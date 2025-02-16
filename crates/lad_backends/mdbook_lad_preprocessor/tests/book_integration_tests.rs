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

fn copy_ladfile_to_book_dir(book_dir: &std::path::Path, ladfile: &str) {
    let ladfile_path = get_books_dir().join(ladfile);
    let book_ladfile_path = book_dir.join("src").join("test.lad.json");
    println!(
        "Copying LAD file from {:?} to {:?}",
        ladfile_path, book_ladfile_path
    );
    std::fs::copy(ladfile_path, book_ladfile_path).expect("failed to copy LAD file");
}

#[test]
fn test_on_example_ladfile() {
    // invoke mdbook build
    // assert that the output contains the expected content

    add_executable_dir_to_path();

    let books_dir = get_books_dir();
    let book = "example_ladfile";

    let ladfile_path = "../../../../ladfile_builder/test_assets/test.lad.json";

    copy_ladfile_to_book_dir(&books_dir.join(book), ladfile_path);

    Command::new("mdbook")
        .env("RUST_LOG", "debug")
        .current_dir(books_dir.join(book))
        .arg("build")
        .assert()
        .success();
}
