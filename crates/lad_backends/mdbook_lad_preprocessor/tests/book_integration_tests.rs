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
    paths.insert(0, dir.to_owned());
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

fn copy_ladfile_to_book_dir(book_dir: &std::path::Path) {
    let ladfile = ladfile::EXAMPLE_LADFILE;
    let book_ladfile_path = book_dir.join("src").join("test.lad.json");
    std::fs::write(book_ladfile_path, ladfile).expect("failed to copy LAD file");
}

fn all_files_in_dir_recursive(dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut files = vec![];
    for entry in std::fs::read_dir(dir).expect("failed to read dir") {
        let entry = entry.expect("failed to get entry");
        let path = entry.path();
        if path.is_dir() {
            files.extend(all_files_in_dir_recursive(&path));
        } else {
            files.push(path);
        }
    }
    files
}

/// normalize line endings
fn normalize_file(file: String) -> String {
    file.replace("\r\n", "\n")
}

#[test]
fn test_on_example_ladfile() {
    // invoke mdbook build
    // assert that the output contains the expected content

    add_executable_dir_to_path();

    let books_dir = get_books_dir();
    let book = "example_ladfile";

    copy_ladfile_to_book_dir(&books_dir.join(book));

    Command::new("mdbook")
        .env("RUST_LOG", "trace")
        .current_dir(books_dir.join(book))
        .arg("build")
        .assert()
        .success();

    // compare the sub directories (expected vs book in the book dir), existing files in expected must have a corresponding, identical file in the book dir with the same path
    let expected_dir = books_dir.join(book).join("expected");
    let book_dir = books_dir.join(book).join("book");

    let expected_files = all_files_in_dir_recursive(&expected_dir);
    let book_files = all_files_in_dir_recursive(&book_dir);

    for expected_file in expected_files {
        let relative_path = expected_file.strip_prefix(&expected_dir).unwrap();
        let book_file = book_dir.join(relative_path);
        assert!(
            book_files.contains(&book_file),
            "File not found: {book_file:?}"
        );
        let expected_content =
            std::fs::read_to_string(&expected_file).expect("failed to read file");
        let book_content = std::fs::read_to_string(&book_file).expect("failed to read file");
        pretty_assertions::assert_eq!(
            normalize_file(expected_content),
            normalize_file(book_content)
        );
    }
}
