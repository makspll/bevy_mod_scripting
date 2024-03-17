use indexmap::IndexMap;
use rustc_hir::{def_id::DefId, Variant, VariantData};
use rustc_middle::ty::TyCtxt;

pub struct BevyCtxt<'tcx> {
    pub tcx: TyCtxt<'tcx>,
    pub reflect_types: Vec<DefId>,
    /// Contains field/variant information for each reflect type
    pub variant_data: IndexMap<DefId, FilteredVariant<'tcx>>,
    pub cached_traits: CachedTraits,
}

/// A collection of common traits stored for quick access.
#[derive(Default)]
pub struct CachedTraits {
    pub mlua_from_lua: Option<DefId>,
    pub mlua_to_lua: Option<DefId>,
    pub bevy_reflect_reflect: Option<DefId>,
}

impl CachedTraits {
    pub fn has_all_mlua_traits(&self) -> bool {
        self.mlua_from_lua.is_some() && self.mlua_to_lua.is_some()
    }

    pub fn has_all_bevy_traits(&self) -> bool {
        self.bevy_reflect_reflect.is_some()
    }
}

pub const DEF_PATHS_FROM_LUA: [&str; 2] = ["value::FromLua", "mlua::FromLua"];
pub const DEF_PATHS_TO_LUA: [&str; 2] = ["value::ToLua", "mlua::ToLua"];
pub const DEF_PATHS_REFLECT: [&str; 2] = ["bevy_reflect::Reflect", "reflect::Reflect"];

impl<'tcx> BevyCtxt<'tcx> {
    pub fn new(tcx: TyCtxt<'tcx>) -> Self {
        Self {
            tcx,
            reflect_types: Default::default(),
            variant_data: Default::default(),
            cached_traits: Default::default(),
        }
    }
}

pub enum FilteredVariant<'tcx> {
    Enum(FilteredEnumData<'tcx>),
    Struct(VariantData<'tcx>),
}

pub struct FilteredEnumData<'tcx> {
    pub variants: Vec<FilteredEnumVariant<'tcx>>,
}

pub enum FilteredEnumVariant<'tcx> {
    Variant(&'tcx Variant<'tcx>),
    Filtered,
}
