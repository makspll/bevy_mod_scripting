use anyhow::{Context, Result, bail};
use log::info;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::{Command, Output},
};

use crate::GlobalArgs;

pub fn run_system_command<I: IntoIterator<Item = impl AsRef<OsStr>>>(
    app_settings: &GlobalArgs,
    command: &str,
    context: &str,
    add_args: I,
    dir: Option<&Path>,
    capture_streams_in_output: bool,
) -> Result<Output> {
    info!("Running system command: {command}");

    let working_dir = match dir {
        Some(d) => relative_workspace_dir(app_settings, d)?,
        None => workspace_dir(app_settings)?,
    };

    let mut cmd = Command::new(command);
    cmd.args(add_args).current_dir(working_dir);

    if !capture_streams_in_output {
        cmd.stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit());
    }

    info!("Using command: {cmd:?}");

    let output = cmd.output();
    if capture_streams_in_output {
        info!("Command status: {:?}", output.as_ref().map(|o| o.status));
    } else {
        info!("Command output: {output:?}");
    }

    let output = output.with_context(|| context.to_owned())?;
    match output.status.code() {
        Some(0) => Ok(output),
        _ => bail!(
            "{} failed with exit code: {}",
            context,
            output.status.code().unwrap_or(-1)
        ),
    }
}

pub fn run_workspace_command<I: IntoIterator<Item = impl AsRef<OsStr>>>(
    app_settings: &GlobalArgs,
    command: &str,
    context: &str,
    add_args: I,
    dir: Option<&Path>,
    capture_streams_in_output: bool,
) -> Result<Output> {
    let coverage_mode = if app_settings.coverage {
        "with coverage"
    } else {
        Default::default()
    };

    info!("Running workspace command {coverage_mode}: {command}");

    let mut args = vec![];

    if let Some(ref toolchain) = app_settings.override_toolchain {
        args.push(format!("+{toolchain}"));
    }

    args.push(command.to_owned());

    if command != "fmt" && command != "bms-codegen" && command != "install" {
        // fmt doesn't care about features, workspaces or profiles
        if command != "run" {
            args.push("--workspace".to_owned());

            if let Some(profile) = app_settings.profile.as_ref() {
                let use_profile = if profile == "ephemeral-build" && app_settings.coverage {
                    // use special profile for coverage as it needs debug information
                    // but also don't want it too slow
                    "ephemeral-coverage"
                } else {
                    profile
                };

                if !app_settings.coverage {
                    args.push("--profile".to_owned());
                    args.push(use_profile.to_owned());
                }

                if let Some(jobs) = app_settings.jobs {
                    args.push("--jobs".to_owned());
                    args.push(jobs.to_string());
                }
            }
        }

        args.extend(app_settings.features.to_cargo_args());
    }

    args.extend(add_args.into_iter().map(|s| {
        s.as_ref()
            .to_str()
            .expect("invalid command argument")
            .to_owned()
    }));

    let working_dir = match dir {
        Some(d) => relative_workspace_dir(app_settings, d)?,
        None => workspace_dir(app_settings)?,
    };

    let mut cmd = Command::new("cargo");
    cmd.args(args).current_dir(working_dir);

    if !capture_streams_in_output {
        cmd.stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit());
    }

    info!("Using command: {cmd:?}");

    let output = cmd.output().with_context(|| context.to_owned())?;
    match output.status.code() {
        Some(0) => Ok(output),
        _ => bail!(
            "{} failed with exit code: {}. Features: {}. output {output:?}",
            context,
            output.status.code().unwrap_or(-1),
            app_settings.features
        ),
    }
}

/// Reads the metadata from the main workspace
pub fn main_workspace_cargo_metadata() -> Result<cargo_metadata::Metadata> {
    let cargo_manifest_path = std::env::var("MAIN_CARGO_MANIFEST_PATH").unwrap();
    let path = PathBuf::from(cargo_manifest_path);
    let parent_dir = path.parent().unwrap().parent().unwrap().join("Cargo.toml");
    let mut cmd = cargo_metadata::MetadataCommand::new();
    cmd.manifest_path(parent_dir.to_string_lossy().to_string());
    let out = cmd.exec()?;
    Ok(out)
}

pub fn workspace_dir(app_settings: &GlobalArgs) -> Result<std::path::PathBuf> {
    if let Some(dir) = &app_settings.override_workspace_dir {
        return Ok(dir.into());
    }

    let metadata = main_workspace_cargo_metadata()?;
    let workspace_root = metadata.workspace_root;
    Ok(workspace_root.into())
}

pub fn codegen_crate_dir(app_settings: &GlobalArgs) -> Result<std::path::PathBuf> {
    let workspace_dir = workspace_dir(app_settings)?;
    Ok(workspace_dir.join("codegen"))
}

pub fn relative_workspace_dir<P: AsRef<Path>>(
    app_settings: &GlobalArgs,
    dir: P,
) -> Result<std::path::PathBuf> {
    let workspace_dir = workspace_dir(app_settings)?;
    Ok(workspace_dir.join(dir))
}

/// reads rust-toolchain.toml file at the given directory and returns the toolchain string
pub fn read_rust_toolchain(path: &Path) -> String {
    let rust_toolchain_path = path.join("rust-toolchain.toml");

    let rust_toolchain =
        std::fs::read_to_string(rust_toolchain_path).expect("Could not read rust_toolchain.toml");

    // parse the toml file
    let toml: toml::Value =
        toml::from_str(&rust_toolchain).expect("Could not parse rust_toolchain.toml");

    let toolchain = toml
        .get("toolchain")
        .expect("Could not find toolchain in rust_toolchain.toml");
    let channel = toolchain
        .get("channel")
        .expect("Could not find channel in rust_toolchain.toml");
    let channel = channel
        .as_str()
        .expect("Channel in rust_toolchain.toml is not a string");

    // set the toolchain
    channel.to_string()
}
