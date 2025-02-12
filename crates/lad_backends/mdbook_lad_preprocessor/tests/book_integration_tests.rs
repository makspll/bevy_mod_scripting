#![allow(missing_docs, clippy::expect_used, clippy::unwrap_used)]

use std::path::PathBuf;

use assert_cmd::Command;
fn add_executable_dir_to_path() {
    let command_path = Command::cargo_bin("mdbook-lad-preprocessor")
        .expect("failed to find mdbook-lad-preprocessor binary");
    let command_path = command_path.get_program();
    let command_path = PathBuf::from(command_path);
    let dir = command_path
        .parent()
        .expect("failed to get parent directory");
    let mut paths = std::env::split_paths(&std::env::var("PATH").expect("failed to get PATH"))
        .collect::<Vec<_>>();
    paths.push(dir.to_owned());
    std::env::set_var(
        "PATH",
        std::env::join_paths(paths).expect("failed to join paths"),
    );
}

// use cargo manifest dir
fn get_books_dir() -> std::path::PathBuf {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_dir = std::path::PathBuf::from(manifest_dir);
    manifest_dir.join("tests").join("books")
}

#[test]
fn test_on_example_ladfile() {
    // invoke mdbook build
    // assert that the output contains the expected content

    add_executable_dir_to_path();

    let books_dir = get_books_dir();
    println!("books_dir: {:?}", books_dir);
    let book = "example_ladfile";

    Command::new("mdbook")
        .current_dir(books_dir.join(book))
        .arg("build")
        .assert()
        .success();
}
