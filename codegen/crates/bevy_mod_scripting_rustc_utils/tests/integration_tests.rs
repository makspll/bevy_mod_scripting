use std::{path::PathBuf, process::Output};
use assert_cmd::Command;


pub fn test_harness(args: impl IntoIterator<Item=String>) -> Output {
    let mut cmd = Command::cargo_bin("cargo-bms-rustc-utils").unwrap();
    cmd.arg("skipped-arg");
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let test_lib_dir = PathBuf::from(crate_dir).parent().unwrap().join("test_crates").join("rust_lib_tests");
    unsafe { std::env::set_var("CARGO_TARGET_DIR", test_lib_dir.join("target")) };
    let mut clean_cmd = std::process::Command::new("cargo");
    clean_cmd.arg("clean").current_dir(&test_lib_dir);
    println!("{clean_cmd:?}, {:?}", clean_cmd.get_current_dir());
    clean_cmd.status().unwrap();

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
        "SimpleTrait",
        "--type",
        "TargetType",
        "--function",
        "simple_fn",
        "--arg",
        "arg"
    ].into_iter().map(String::from).collect::<Vec<_>>());
    
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "yes\n");
    assert!(output.status.success());
}


#[test]
pub fn test_complex_trait() {
    let output = test_harness([
        "arg-implements",
        "--trait",
        "WithAssocItem",
        "--type",
        "TargetType",
        "--function",
        "with_assoc_fn",
        "--arg",
        "arg"
    ].into_iter().map(String::from).collect::<Vec<_>>());
    
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "yes\n");
    assert!(output.status.success());
}


#[test]
pub fn test_not_implementing_arg() {
    let output = test_harness([
        "arg-implements",
        "--trait",
        "SimpleTrait",
        "--type",
        "TargetType",
        "--function",
        "simple_fn_negative",
        "--arg",
        "arg"
    ].into_iter().map(String::from).collect::<Vec<_>>());
    
    assert_eq!(String::from_utf8(output.stdout).unwrap(), "no: the trait bound `std::string::String: SimpleTrait` is not satisfied\n");
    assert!(output.status.success());
}

