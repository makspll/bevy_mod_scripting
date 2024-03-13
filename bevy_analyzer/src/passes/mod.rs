use crate::BevyCtxt;

mod find_reflect_types;
mod test_pass;

/// A single pass using the bevy context
pub struct Pass {
    pub name: &'static str,
    pub cb: fn(&mut BevyCtxt<'_>),
}

pub const TEST_PASS: Pass = Pass {
    name: "Test Pass",
    cb: test_pass::test_pass,
};

pub const FIND_REFLECT_TYPES: Pass = Pass {
    name: "Find Reflect Types",
    cb: find_reflect_types::find_reflect_types,
};
