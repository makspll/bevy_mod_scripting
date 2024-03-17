use crate::BevyCtxt;

mod cache_traits;
mod find_methods_and_fields;
mod find_reflect_types;

/// A single pass using the bevy context
pub struct Pass {
    pub name: &'static str,
    pub cb: fn(&mut BevyCtxt<'_>),
}

pub const FIND_REFLECT_TYPES: Pass = Pass {
    name: "Find Reflect Types",
    cb: find_reflect_types::find_reflect_types,
};

pub const FIND_METHODS_AND_FIELDS: Pass = Pass {
    name: "Find Methods and Fields",
    cb: find_methods_and_fields::find_methods_and_fields,
};

pub const CACHE_TRAITS: Pass = Pass {
    name: "Cache traits",
    cb: cache_traits::cache_traits,
};

pub const ALL_PASSES: [Pass; 3] = [CACHE_TRAITS, FIND_REFLECT_TYPES, FIND_METHODS_AND_FIELDS];
