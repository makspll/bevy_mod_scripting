use crate::{Args, BevyCtxt};

mod cache_traits;
mod codegen;
mod find_methods_and_fields;
mod find_reflect_types;
mod find_trait_impls;
mod populate_template_data;
mod write_meta;

/// A single pass using the bevy context
pub(crate) struct Pass {
    pub name: &'static str,
    pub cb: fn(&mut BevyCtxt<'_>, &Args) -> bool,
}

pub(crate) const FIND_REFLECT_TYPES: Pass = Pass {
    name: "Find Reflect Types",
    cb: find_reflect_types::find_reflect_types,
};

pub(crate) const FIND_METHODS_AND_FIELDS: Pass = Pass {
    name: "Find Methods and Fields",
    cb: find_methods_and_fields::find_methods_and_fields,
};

pub(crate) const CACHE_TRAITS: Pass = Pass {
    name: "Cache traits",
    cb: cache_traits::cache_traits,
};

pub(crate) const WRITE_META: Pass = Pass {
    name: "Write meta",
    cb: write_meta::write_meta,
};

pub(crate) const POPULATE_TEMPLATE_DATA: Pass = Pass {
    name: "Populate template data",
    cb: populate_template_data::populate_template_data,
};

pub(crate) const CODEGEN: Pass = Pass {
    name: "Codegen",
    cb: codegen::codegen,
};

pub(crate) const FIND_TRAIT_IMPLS: Pass = Pass {
    name: "Find Trait Impl",
    cb: find_trait_impls::find_trait_impls,
};

pub(crate) const ALL_PASSES: [Pass; 7] = [
    CACHE_TRAITS,
    FIND_REFLECT_TYPES,
    WRITE_META, // the earlier this happens the better, as other crates will depend on our meta files but not the rest of the passes!
    FIND_TRAIT_IMPLS,
    FIND_METHODS_AND_FIELDS,
    POPULATE_TEMPLATE_DATA,
    CODEGEN,
];
