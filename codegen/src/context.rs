use std::collections::HashMap;

use cargo_metadata::camino::Utf8PathBuf;
use indexmap::IndexMap;
use log::debug;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::{AdtDef, TyCtxt};
use serde::Serialize;

use crate::{ImportPathFinder, MetaLoader, TemplateContext};

pub(crate) struct BevyCtxt<'tcx> {
    pub(crate) tcx: TyCtxt<'tcx>,
    pub(crate) meta_loader: MetaLoader,
    pub(crate) reflect_types: IndexMap<DefId, ReflectType<'tcx>>,
    pub(crate) cached_traits: CachedTraits,
    pub(crate) path_finder: ImportPathFinder<'tcx>,

    /// the template context used for generating code
    pub(crate) template_context: Option<TemplateContext>,
}

impl<'tcx> BevyCtxt<'tcx> {
    /// Creates a new context with the provided TyCtxt and meta directories
    pub(crate) fn new(
        tcx: TyCtxt<'tcx>,
        meta_dirs: &[Utf8PathBuf],
        workspace_meta: crate::WorkspaceMeta,
        include_private_paths: bool,
        import_path_processor: Option<Box<dyn Fn(&str) -> String>>,
    ) -> Self {
        Self {
            tcx,
            reflect_types: Default::default(),
            cached_traits: Default::default(),
            meta_loader: MetaLoader::new(meta_dirs.to_vec(), workspace_meta),
            template_context: Default::default(),
            path_finder: ImportPathFinder::new(tcx, include_private_paths, import_path_processor),
        }
    }

    /// Clears all data structures in the context
    /// the context unusable
    pub(crate) fn clear(&mut self) {
        debug!("Clearing all context");
        *self = Self::new(
            self.tcx,
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        );
    }
}

#[derive(Clone, Default, Debug)]
pub(crate) struct ReflectType<'tcx> {
    /// Map from traits to their implementations for the reflect type (from a selection)
    pub(crate) trait_impls: Option<HashMap<DefId, Vec<DefId>>>,
    /// Information about the ADT structure, fields, and variants
    pub(crate) variant_data: Option<AdtDef<'tcx>>,
    /// Functions passing criteria to be proxied
    pub(crate) valid_functions: Option<Vec<FunctionContext>>,

    /// Mapping from fields to the reflection strategy
    field_reflection_types: IndexMap<DefId, ReflectionStrategy>,
}

impl ReflectType<'_> {
    pub(crate) fn set_field_reflection_strategies<
        I: Iterator<Item = (DefId, ReflectionStrategy)>,
    >(
        &mut self,
        field_strats: I,
    ) {
        self.field_reflection_types = field_strats.collect();
    }

    pub(crate) fn get_field_reflection_strat(&self, field: DefId) -> Option<&ReflectionStrategy> {
        self.field_reflection_types.get(&field)
    }
}

pub(crate) const DEF_PATHS_BMS_FROM_SCRIPT: [&str; 2] = [
    "bevy_mod_scripting_core::bindings::FromScript",
    "bindings::FromScript",
];
pub(crate) const DEF_PATHS_BMS_INTO_SCRIPT: [&str; 2] = [
    "bevy_mod_scripting_core::bindings::IntoScript",
    "bindings::IntoScript",
];

pub(crate) const DEF_PATHS_REFLECT: [&str; 2] =
    ["bevy_reflect::PartialReflect", "reflect::PartialReflect"];
pub(crate) const DEF_PATHS_GET_TYPE_REGISTRATION: [&str; 2] = [
    "bevy_reflect::GetTypeRegistration",
    "type_registry::GetTypeRegistration",
];

/// A collection of traits which we search for in the codebase, some are included purely for the methods they provide,
/// others are later used for quick lookup of the type "does this type implement Display" etc.
pub(crate) const STD_ONLY_TRAITS: [&str; 1] = ["std::string::ToString"];

pub(crate) const STD_OR_CORE_TRAITS: [&str; 13] = [
    // PRINTING
    "fmt::Debug",
    "fmt::Display",
    // OWNERSHIP
    "clone::Clone",
    // OPERATORS
    "ops::Neg",
    "ops::Mul",
    "ops::Add",
    "ops::Sub",
    "ops::Div",
    "ops::Rem",
    "cmp::Eq",
    "cmp::PartialEq",
    "cmp::Ord", // we don't use these fully cuz of the output types not being lua primitives, but keeping it for the future
    "cmp::PartialOrd",
];

/// A collection of common traits stored for quick access.
#[derive(Default)]
pub(crate) struct CachedTraits {
    pub(crate) bms_into_script: Option<DefId>,
    pub(crate) bms_from_script: Option<DefId>,
    pub(crate) bevy_reflect_reflect: Option<DefId>,
    pub(crate) bevy_reflect_get_type_registration: Option<DefId>,
    /// Map from def_path_str to DefId of common std traits we work with
    /// these are the only trait impls from which we generate methods
    pub(crate) std_only_traits: HashMap<String, DefId>,
    pub(crate) std_or_core_traits: HashMap<String, DefId>,
}

impl CachedTraits {
    pub(crate) fn missing_bms_traits(&self) -> Vec<&'static str> {
        let mut missing = Vec::new();
        if self.bms_into_script.is_none() {
            missing.extend(DEF_PATHS_BMS_INTO_SCRIPT);
        }
        if self.bms_from_script.is_none() {
            missing.extend(DEF_PATHS_BMS_FROM_SCRIPT);
        }
        missing
    }

    pub(crate) fn missing_bevy_traits(&self) -> Vec<&'static str> {
        let mut missing = Vec::new();
        if self.bevy_reflect_reflect.is_none() {
            missing.extend(DEF_PATHS_REFLECT);
        }
        if self.bevy_reflect_get_type_registration.is_none() {
            missing.extend(DEF_PATHS_GET_TYPE_REGISTRATION);
        }
        missing
    }

    pub(crate) fn missing_std_traits(&self) -> Vec<String> {
        let mut missing = Vec::new();
        for trait_name in STD_ONLY_TRAITS {
            if !self.std_only_traits.contains_key(trait_name) {
                missing.push(trait_name.to_owned());
            }
        }
        for trait_name in STD_OR_CORE_TRAITS {
            for prefix in ["std::", "core::"] {
                let full_trait_name = format!("{prefix}{trait_name}");
                if !self.std_or_core_traits.contains_key(&full_trait_name) {
                    missing.push(full_trait_name);
                }
            }
        }
        missing
    }

    // pub(crate) fn has_all_std_source_traits(&self) -> bool {
    //     STD_SOURCE_TRAITS
    //         .iter()
    //         .all(|t| self.std_source_traits.contains_key(*t))
    // }

    // pub(crate) fn missing_std_source_traits(&self) -> Vec<String> {
    //     STD_SOURCE_TRAITS
    //         .iter()
    //         .filter(|t| !self.std_source_traits.contains_key(**t))
    //         .map(|s| (*s).to_owned())
    //         .collect()
    // }
}

#[derive(Clone, Debug)]
pub(crate) struct FunctionContext {
    pub(crate) def_id: DefId,
    pub(crate) has_self: bool,
    pub(crate) is_unsafe: bool,
    pub(crate) trait_and_impl_did: Option<(DefId, DefId)>,
    /// strategies for input and output (last element is the output)
    pub(crate) reflection_strategies: Vec<ReflectionStrategy>,
}

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Debug)]
pub(crate) enum ReflectionStrategy {
    /// The will have a known wrapper we can use
    Proxy,
    /// The type is a primitive with the right traits to be used directly in arguments and return values
    Primitive,
    /// Use a reflection primitive i.e. 'ReflectedValue', dynamic runtime reflection
    Reflection,
    /// Either ignored via 'reflect(ignore)' or not visible
    Filtered,
}
