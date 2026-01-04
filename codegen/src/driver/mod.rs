//! The module dealing with how the CLI interacts with each instantiation of the
//! plugin across the workspace.

mod plugin;

use std::{
    env,
    ops::Deref,
    path::{Path, PathBuf},
    process::{Command, Stdio, exit},
};

use cargo_metadata::{Metadata, camino::Utf8PathBuf};
pub use plugin::*;
use rustc_session::{EarlyDiagCtxt, config::ErrorOutputType};

pub const CARGO_VERBOSE: &str = "CARGO_VERBOSE";
pub const WORKSPACE_GRAPH_FILE_ENV: &str = "WORKSPACE_GRAPH_FILE";

pub fn fetch_target_directory(metadata: &Metadata) -> Utf8PathBuf {
    let plugin_subdir = format!("plugin-{}", crate::CHANNEL);
    metadata.target_directory.join(plugin_subdir)
}

/// The top-level function that should be called in your user-facing binary.
pub fn cli_main<T: RustcPlugin>(plugin: T, metadata: &Metadata) {
    if env::args().any(|arg| arg == "-V") {
        println!("{}", T::version());
        return;
    }

    let target_dir = fetch_target_directory(metadata);

    let mut cmd = Command::new("cargo");
    cmd.env("CARGO_TERM_COLOR", "always");
    cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());

    let mut path = env::current_exe()
        .expect("current executable path invalid")
        .with_file_name(plugin.driver_name().as_ref());

    if cfg!(windows) {
        path.set_extension("exe");
    }

    cmd.env("RUSTC_WORKSPACE_WRAPPER", path)
        .args(["check", "--target-dir"])
        .arg(&target_dir);

    if env::var(CARGO_VERBOSE).is_ok() {
        cmd.arg("-vv");
    } else {
        cmd.arg("-q");
    }

    plugin.serialize_to_env();

    plugin.modify_cargo(&mut cmd);

    let exit_status = cmd.status().expect("failed to wait for cargo?");

    exit(exit_status.code().unwrap_or(-1));
}

fn with_next<I>(iter: I) -> impl Iterator<Item = (I::Item, Option<I::Item>)>
where
    I: Iterator,
    I::Item: Clone,
{
    let mut it = iter.peekable();
    std::iter::from_fn(move || {
        let current = it.next()?;
        let next = it.peek().cloned();
        Some((current, next))
    })
}

/// If a command-line option matches `find_arg`, then apply the predicate `pred` on its value. If
/// true, then return it. The parameter is assumed to be either `--arg=value` or `--arg value`.
fn arg_values<'a, T: Deref<Target = str>>(
    args: &'a [T],
    find_arg: &'a str,
    pred: impl Fn(&'a str) -> bool + 'static,
) -> impl Iterator<Item = &'a str> {
    with_next(args.iter().map(Deref::deref)).filter_map(move |(arg, next_arg)| {
        let mut split_arg = arg.splitn(2, '=');
        if split_arg.next() != Some(find_arg) {
            return None;
        }
        match split_arg.next().or(next_arg) {
            Some(v) if pred(v) => Some(v),
            _ => None,
        }
    })
}

pub fn arg_value<'a, T: Deref<Target = str>>(
    args: &'a [T],
    find_arg: &'a str,
    pred: impl Fn(&'a str) -> bool + 'static,
) -> Option<&'a str> {
    arg_values(args, find_arg, pred).next()
}

fn toolchain_path(home: Option<String>, toolchain: Option<String>) -> Option<PathBuf> {
    home.and_then(|home| {
        toolchain.map(|toolchain| {
            let mut path = PathBuf::from(home);
            path.push("toolchains");
            path.push(toolchain);
            path
        })
    })
}

fn get_sysroot(orig_args: &[String]) -> (bool, String) {
    // Get the sysroot, looking from most specific to this invocation to the least:
    // - command line
    // - runtime environment
    //    - SYSROOT
    //    - RUSTUP_HOME, MULTIRUST_HOME, RUSTUP_TOOLCHAIN, MULTIRUST_TOOLCHAIN
    // - sysroot from rustc in the path
    // - compile-time environment
    //    - SYSROOT
    //    - RUSTUP_HOME, MULTIRUST_HOME, RUSTUP_TOOLCHAIN, MULTIRUST_TOOLCHAIN
    let sys_root_arg = arg_value(orig_args, "--sysroot", |_| true);
    let have_sys_root_arg = sys_root_arg.is_some();
    let sys_root = sys_root_arg
        .map(PathBuf::from)
        .or_else(|| std::env::var("MIRI_SYSROOT").ok().map(PathBuf::from))
        .or_else(|| std::env::var("SYSROOT").ok().map(PathBuf::from))
        .or_else(|| {
            let home = std::env::var("RUSTUP_HOME")
                .or_else(|_| std::env::var("MULTIRUST_HOME"))
                .ok();
            let toolchain = std::env::var("RUSTUP_TOOLCHAIN")
                .or_else(|_| std::env::var("MULTIRUST_TOOLCHAIN"))
                .ok();
            toolchain_path(home, toolchain)
        })
        .or_else(|| {
            Command::new("rustc")
                .arg("--print")
                .arg("sysroot")
                .output()
                .ok()
                .and_then(|out| String::from_utf8(out.stdout).ok())
                .map(|s| PathBuf::from(s.trim()))
        })
        .or_else(|| option_env!("SYSROOT").map(PathBuf::from))
        .or_else(|| {
            let home = option_env!("RUSTUP_HOME")
                .or(option_env!("MULTIRUST_HOME"))
                .map(ToString::to_string);
            let toolchain = option_env!("RUSTUP_TOOLCHAIN")
                .or(option_env!("MULTIRUST_TOOLCHAIN"))
                .map(ToString::to_string);
            toolchain_path(home, toolchain)
        })
        .map(|pb| pb.to_string_lossy().to_string())
        .expect(
            "need to specify SYSROOT env var during clippy compilation, or use rustup or multirust",
        );
    (have_sys_root_arg, sys_root)
}

/// The top-level function that should be called by your internal driver binary.
pub fn rustc_driver_main<T: RustcPlugin>(plugin: T) {
    let early_dcx = EarlyDiagCtxt::new(ErrorOutputType::default());
    rustc_driver::init_rustc_env_logger(&early_dcx);

    exit(rustc_driver::catch_with_exit_code(move || {
        let mut orig_args: Vec<String> = env::args().collect();

        let (have_sys_root_arg, sys_root) = get_sysroot(&orig_args);

        if orig_args.iter().any(|a| a == "--version" || a == "-V") {
            println!("{}", T::version());
            exit(0);
        }

        // Setting RUSTC_WRAPPER causes Cargo to pass 'rustc' as the first argument.
        // We're invoking the compiler programmatically, so we ignore this
        let wrapper_mode =
            orig_args.get(1).map(Path::new).and_then(Path::file_stem) == Some("rustc".as_ref());

        if wrapper_mode {
            // we still want to be able to invoke it normally though
            orig_args.remove(1);
        }

        // this conditional check for the --sysroot flag is there so users can call
        // the driver directly without having to pass --sysroot or anything
        let mut args: Vec<String> = orig_args.clone();
        if !have_sys_root_arg {
            args.extend(["--sysroot".into(), sys_root.clone()]);
        };

        plugin.modify_rustc(&mut args);

        // On a given invocation of rustc, we have to decide whether to act as rustc,
        // or actually execute the plugin. There are two conditions for executing the plugin:
        // 1. Either we're supposed to run on all crates, or CARGO_PRIMARY_PACKAGE is set.
        // 2. --print is NOT passed, since Cargo does that to get info about rustc.
        // let primary_package = env::var("CARGO_PRIMARY_PACKAGE").is_ok();
        let crate_being_built = arg_value(&args, "--crate-name", |_| true)
            .as_ref()
            .map(|s| s.to_string())
            .unwrap_or_default();

        // let run_mode = plugin.should_run(&crate_being_built);
        // log::info!("crate {crate_being_built}, run mode: {run_mode:?}");
        // let (should_run_plugin, should_skip) = match run_mode {
        //     RunMode::NormalRustc => (false, false),
        //     RunMode::Plugin => (true, false),
        //     RunMode::Skip => (false, true),
        // };

        let normal_rustc_cli_override = arg_value(&args, "--print", |_| true).is_some();

        // TODO: this is dangerous
        // ignore all lints that could break the comp in crates that we don't care about
        // args.extend([String::from("--cap-lints"), String::from("warn")]);

        log::debug!("Running plugin on crate: {crate_being_built}");
        plugin.run(&crate_being_built, args, normal_rustc_cli_override);
    }))
}
