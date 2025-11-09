use std::{
    path::{Path, PathBuf},
    process::Command,
};

fn rustc_path(channel: &str) -> PathBuf {
    let output = Command::new("rustup")
        .args(["which", "--toolchain", channel, "rustc"])
        .output()
        .expect("failed to run rustup which");

    let rustc_path = String::from_utf8(output.stdout).unwrap();

    PathBuf::from(rustc_path.trim())
}

fn target_libdir(rustc: &Path) -> PathBuf {
    let output = Command::new(rustc)
        .args(["--print", "target-libdir"])
        .output()
        .expect("failed to run rustc --print target-libdir");

    let libdir = String::from_utf8(output.stdout).unwrap();

    PathBuf::from(libdir.trim())
}

pub fn main() {
    // Use to set RUSTC_CHANNEL so we can compute target dir from rustc_plugin
    let toolchain_toml = include_str!("rust-toolchain.toml");
    let toolchain_table = toolchain_toml.parse::<toml::Table>().unwrap();
    let toolchain = toolchain_table["toolchain"].as_table().unwrap();
    let channel = toolchain["channel"].as_str().unwrap();
    println!("cargo:rustc-env=RUSTC_CHANNEL={channel}");

    // I believe there was some sort of change in cargo, which motivated this change in the original rustc_plugin:
    // https://github.com/cognitive-engineering-lab/rustc_plugin/pull/41
    //
    // This solves issues with linking to the rustc driver dynamic library, as cargo does not seem to put in the correct LD_LIBRARY_PATH for the driver binary instantiations
    // Embedding this in the binary solves the problem
    let rustc_path = rustc_path(channel);
    let target_libdir = target_libdir(&rustc_path);

    println!(
        "cargo::rustc-link-arg=-Wl,-rpath,{}",
        target_libdir.display()
    );
}
