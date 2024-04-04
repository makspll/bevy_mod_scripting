use crate::{Args, BevyCtxt};

mod cache_traits;
mod codegen;
mod crawl_paths;
mod find_methods_and_fields;
mod find_reflect_types;
mod find_trait_impls;
mod populate_template_data;
mod write_meta;

/// A single pass using the bevy context
pub(crate) struct Pass {
    pub name: &'static str,
    pub description: &'static str,
    pub cb: fn(&mut BevyCtxt<'_>, &Args) -> bool,
}

pub(crate) const FIND_REFLECT_TYPES: Pass = Pass {
    name: "Find Reflect Types",
    description: "Finding all non-generic, public and reflectable types in the crate",
    cb: find_reflect_types::find_reflect_types,
};

pub(crate) const FIND_METHODS_AND_FIELDS: Pass = Pass {
    name: "Find Methods and Fields",
    description: "Finding and filtering methods and fields to ones we can expose",
    cb: find_methods_and_fields::find_methods_and_fields,
};

pub(crate) const CACHE_TRAITS: Pass = Pass {
    name: "Cache traits",
    description: "Searching for necessary traits",
    cb: cache_traits::cache_traits,
};

pub(crate) const WRITE_META: Pass = Pass {
    name: "Write meta",
    description: "Writing meta files",
    cb: write_meta::write_meta,
};

pub(crate) const POPULATE_TEMPLATE_DATA: Pass = Pass {
    name: "Populate template data",
    description: "Preparing template data for code generation",
    cb: populate_template_data::populate_template_data,
};

pub(crate) const CODEGEN: Pass = Pass {
    name: "Codegen",
    description: "Generating code files",
    cb: codegen::codegen,
};

pub(crate) const FIND_TRAIT_IMPLS: Pass = Pass {
    name: "Find Trait Impl",
    description: "Finding common trait implementations and methods to expose",
    cb: find_trait_impls::find_trait_impls,
};

pub(crate) const CRAWL_PATHS: Pass = Pass {
    name: "Crawl Paths",
    description: "Crawling crates to generate good import paths",
    cb: crawl_paths::crawl_paths,
};

pub(crate) const ALL_PASSES: [Pass; 8] = [
    CACHE_TRAITS,
    FIND_REFLECT_TYPES,
    FIND_TRAIT_IMPLS, // we have to do this before meta as we still filter some things here
    WRITE_META, // the earlier this happens the better, as other crates will depend on our meta files but not the rest of the passes!
    FIND_METHODS_AND_FIELDS,
    CRAWL_PATHS,
    POPULATE_TEMPLATE_DATA,
    CODEGEN,
];
