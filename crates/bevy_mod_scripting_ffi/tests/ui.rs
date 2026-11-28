//! Overlays that must not compile. Each fixture's header names the error it must fail with;
//! the `.stderr` snapshots pin the exact message.

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/fail/*.rs");
}
