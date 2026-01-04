use std::collections::HashMap;

use cargo_metadata::camino::Utf8PathBuf;
use crate_feature_graph::WorkspaceGraph;
use indexmap::IndexMap;
use log::debug;
use rustc_hir::def_id::{CrateNum, DefId, DefIndex};
use rustc_middle::ty::{AdtDef, TyCtxt};
use serde::Serialize;

use crate::{ImportPathFinder, MetaLoader, TemplateContext, candidate::GenerationCandidate};

pub(crate) struct BevyCtxt<'tcx> {
    pub(crate) tcx: TyCtxt<'tcx>,
    pub(crate) meta_loader: MetaLoader,
    pub(crate) reflect_types: IndexMap<DefId, GenerationCandidate<'tcx, AdtDef<'tcx>>>,
    pub(crate) excluded_reflect_types: Vec<GenerationCandidate<'tcx, Option<AdtDef<'tcx>>>>,
    pub(crate) cached_traits: CachedItems,
    pub(crate) path_finder: ImportPathFinder<'tcx>,
    pub(crate) workspace: WorkspaceGraph,

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
        workspace: WorkspaceGraph,
    ) -> Self {
        Self {
            tcx,
            reflect_types: Default::default(),
            excluded_reflect_types: Default::default(),
            cached_traits: Default::default(),
            meta_loader: MetaLoader::new(meta_dirs.to_vec(), workspace_meta),
            template_context: Default::default(),
            path_finder: ImportPathFinder::new(tcx, include_private_paths, import_path_processor),
            workspace,
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
            Default::default(),
        );
    }
}

pub(crate) const DEF_PATHS_BMS_FROM_SCRIPT: [&str; 2] =
    ["bevy_mod_scripting_bindings::FromScript", "FromScript"];
pub(crate) const DEF_PATHS_BMS_INTO_SCRIPT: [&str; 2] =
    ["bevy_mod_scripting_bindings::IntoScript", "IntoScript"];

pub(crate) const DEF_PATHS_REFLECT: [&str; 2] = [
    "bevy_reflect::reflect::PartialReflect",
    "reflect::PartialReflect",
];
pub(crate) const DEF_PATHS_GET_TYPE_REGISTRATION: [&str; 2] = [
    "bevy_reflect::type_registry::GetTypeRegistration",
    "type_registry::GetTypeRegistration",
];
pub(crate) const DEF_PATHS_BMS_REF_WRAPPER: [&str; 2] = [
    "bevy_mod_scripting_bindings::Ref",
    "bevy_mod_scripting_bindings::function::Ref",
];
pub(crate) const DEF_PATHS_BMS_MUT_WRAPPER: [&str; 2] = [
    "bevy_mod_scripting_bindings::Mut",
    "bevy_mod_scripting_bindings::function::Mut",
];
pub(crate) const DEF_PATHS_BMS_VAL_WRAPPER: [&str; 2] = [
    "bevy_mod_scripting_bindings::Val",
    "bevy_mod_scripting_bindings::function::Val",
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

pub(crate) struct BmsTypes {
    #[allow(dead_code)]
    pub bindings_crate: CrateNum,
    pub into_script: DefId,
    pub from_script: DefId,
    pub ref_wrapper: DefId,
    pub val_wrapper: DefId,
    pub mut_wrapper: DefId,
}

pub(crate) struct BevyTypes {
    #[allow(dead_code)]
    pub reflect_crate: CrateNum,
    pub reflect: DefId,
    #[allow(dead_code)]
    pub get_type_registration: DefId,
}

impl Default for BevyTypes {
    fn default() -> Self {
        Self {
            reflect_crate: INVALID_CRATE_NUM,
            reflect: INVALID_DEF_ID,
            get_type_registration: INVALID_DEF_ID,
        }
    }
}

const INVALID_CRATE_NUM: CrateNum = CrateNum::from_u16(u16::MAX);
const INVALID_DEF_ID: DefId = DefId {
    index: DefIndex::from_u16(u16::MAX),
    krate: INVALID_CRATE_NUM,
};

impl Default for BmsTypes {
    fn default() -> Self {
        Self {
            bindings_crate: INVALID_CRATE_NUM,
            into_script: INVALID_DEF_ID,
            from_script: INVALID_DEF_ID,
            ref_wrapper: INVALID_DEF_ID,
            val_wrapper: INVALID_DEF_ID,
            mut_wrapper: INVALID_DEF_ID,
        }
    }
}

/// A collection of common traits stored for quick access.
#[derive(Default)]
pub(crate) struct CachedItems {
    pub(crate) bms_types: BmsTypes,
    pub(crate) bevy_types: BevyTypes,
    /// Map from def_path_str to DefId of common std traits we work with
    /// these are the only trait impls from which we generate methods
    pub(crate) std_only_traits: HashMap<String, DefId>,
    pub(crate) std_or_core_traits: HashMap<String, DefId>,
}

impl CachedItems {
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

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Debug, Default)]
pub(crate) enum ReflectionStrategy {
    /// The will have a known wrapper we can use
    Proxy,
    /// The type is a primitive with the right traits to be used directly in arguments and return values
    Primitive,
    /// Use a reflection primitive i.e. 'ReflectedValue', dynamic runtime reflection
    Reflection,
    /// Either ignored via 'reflect(ignore)' or not visible
    #[default]
    Filtered,
}
