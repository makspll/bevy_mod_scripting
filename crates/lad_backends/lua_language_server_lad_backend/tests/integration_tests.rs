#![allow(missing_docs, clippy::expect_used, clippy::unwrap_used, clippy::panic)]

use std::{fs::DirEntry, path::PathBuf};

use assert_cmd::{Command, cargo_bin};
fn add_executable_dir_to_path() {
    let command_path = Command::new(cargo_bin!("lad-lls"));
    let command_path = command_path.get_program();
    let command_path = PathBuf::from(command_path);
    let dir = command_path
        .parent()
        .expect("failed to get parent directory");
    let mut paths = std::env::split_paths(&std::env::var("PATH").expect("failed to get PATH"))
        .collect::<Vec<_>>();
    paths.insert(0, dir.to_owned());
    unsafe {
        std::env::set_var(
            "PATH",
            std::env::join_paths(paths).expect("failed to join paths"),
        );
    }
}

// use cargo manifest dir
fn get_tests_dir() -> std::path::PathBuf {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_dir = std::path::PathBuf::from(manifest_dir);
    manifest_dir.join("tests")
}

fn copy_example_ladfile_to_relevant_test(tests_dir: &std::path::Path) {
    let ladfile = ladfile::EXAMPLE_LADFILE;
    let book_ladfile_path = tests_dir.join("example_ladfile").join("test.lad.json");
    std::fs::write(book_ladfile_path, ladfile).expect("failed to copy LAD file");
}

#[test]
fn main() {
    add_executable_dir_to_path();

    let tests_dir = get_tests_dir();
    if !tests_dir.exists() {
        std::fs::create_dir_all(&tests_dir).expect("failed to create tests directory");
    }
    copy_example_ladfile_to_relevant_test(&tests_dir);

    // for each folder in tests_dir, run the binary with
    // --input <folder>/test.lad.json
    // --output <folder>/generated.lua

    let tests = std::fs::read_dir(&tests_dir)
        .expect("failed to read tests directory")
        .collect::<Result<Vec<DirEntry>, _>>()
        .expect("failed to collect test entries");

    if tests.is_empty() {
        panic!("No tests found in the tests directory. Please add some test folders with LAD files")
    }

    for entry in tests {
        if entry.file_type().expect("failed to get file type").is_dir() {
            let folder_path = entry.path();
            let ladfile_path = folder_path.join("test.lad.json");

            Command::new(cargo_bin!("lad-lls"))
                .arg("--input")
                .arg(&ladfile_path)
                .arg("--output")
                .arg(&folder_path)
                .assert()
                .success();

            // then compare each generated file, `bindings.lua` and one per namespace (`bindings.<namespace>.lua`),
            // with the expected one, `expected.lua` or `expected.<namespace>.lua`
            let mut generated_files = std::fs::read_dir(&folder_path)
                .expect("failed to read test folder")
                .map(|entry| entry.expect("failed to read entry").file_name())
                .map(|name| name.to_string_lossy().to_string())
                .filter(|name| name.starts_with("bindings") && name.ends_with(".lua"))
                .collect::<Vec<_>>();
            generated_files.sort();

            for generated_file in generated_files {
                let expected_path =
                    folder_path.join(generated_file.replacen("bindings", "expected", 1));
                let generated_str = std::fs::read_to_string(folder_path.join(&generated_file))
                    .expect("failed to read generated file");

                if std::env::var("BLESS_MODE").is_ok() {
                    std::fs::write(&expected_path, &generated_str)
                        .expect("failed to write expected file");
                } else {
                    let expected_str =
                        std::fs::read_to_string(&expected_path).unwrap_or_else(|_| {
                            panic!("missing {}, bless the test", expected_path.display())
                        });
                    pretty_assertions::assert_eq!(
                        expected_str,
                        generated_str,
                        "Generated Lua file {generated_file} does not match expected output for {}",
                        folder_path.display()
                    );
                }
            }
            if std::env::var("BLESS_MODE").is_ok() {
                panic!("BLESS_MODE is enabled, please disable it to run the tests");
            }
        }
    }
}
