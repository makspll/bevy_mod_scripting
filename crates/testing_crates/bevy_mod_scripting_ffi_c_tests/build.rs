//! Compiles `c/round_trip.c` against the blessed `c/overlays.h` with the platform C compiler.
//! The header's `_Static_assert`s check the C layout of every overlay equals the Rust one, at C compile time.

fn main() {
    println!("cargo::rustc-check-cfg=cfg(bms_ffi_bless)");
    println!("cargo::rerun-if-env-changed=BLESS_MODE");
    println!("cargo::rerun-if-changed=c");

    // an outdated header may not compile, so blessing only regenerates it
    if std::env::var_os("BLESS_MODE").is_some() {
        println!("cargo::rustc-cfg=bms_ffi_bless");
        return;
    }

    let mut build = cc::Build::new();
    build
        .file("c/round_trip.c")
        .include("c")
        .std("c11")
        .warnings_into_errors(true);

    // gcc on Linux: undefined behaviour (misaligned or out of range accesses) in the C code aborts the tests
    let compiler = build.get_compiler();
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("linux")
        && compiler.is_like_gnu()
        && !compiler.is_like_clang()
    {
        build
            .flag("-fsanitize=undefined")
            .flag("-fno-sanitize-recover=all");
        println!("cargo::rustc-link-lib=ubsan");
    }

    build.compile("bms_ffi_round_trip");
}
