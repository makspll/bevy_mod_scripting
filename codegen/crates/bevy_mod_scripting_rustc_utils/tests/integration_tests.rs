use std::{path::{PathBuf}, process::Output};
use assert_cmd::Command;


pub fn test_harness(args: impl IntoIterator<Item=String>) -> Output {
    let mut cmd = Command::cargo_bin("cargo-bms-rustc-utils").unwrap();
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let test_lib_dir = PathBuf::from(crate_dir).parent().unwrap().join("test_crates").join("rust_lib_tests");
    cmd
        .current_dir(test_lib_dir)
        .args(args)
        .output()
        .unwrap()
}

#[test]
pub fn test_plain_argument_direct_impl() {
    let output = test_harness([
        "arg-implements",
        "--trait",
        "TargetTrait",
        "--type",
        "TestType",
        "--function",
        "test_func",
        "--arg",
        "my_arg"
    ].into_iter().map(String::from).collect::<Vec<_>>());
    
    assert_eq!(String::from_utf8(output.stderr).unwrap(), "");
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "aasd");
    assert!(output.status.success());
}
