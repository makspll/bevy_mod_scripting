use anyhow::{Context, Result, bail};
use log::info;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::{Command, Output},
};

use crate::GlobalArgs;

pub fn make_system_command<I: IntoIterator<Item = impl AsRef<OsStr>>>(
    app_settings: &GlobalArgs,
    command: &str,
    add_args: I,
    dir: Option<&Path>,
    capture_streams_in_output: bool,
) -> Result<Command> {
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
    Ok(cmd)
}

pub fn run_system_command<I: IntoIterator<Item = impl AsRef<OsStr>>>(
    app_settings: &GlobalArgs,
    command: &str,
    context: &str,
    add_args: I,
    dir: Option<&Path>,
    capture_streams_in_output: bool,
) -> Result<Output> {
    let mut cmd = make_system_command(
        app_settings,
        command,
        add_args,
        dir,
        capture_streams_in_output,
    )?;

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
    info!("Running workspace command: {command}");

    let mut args = vec![];

    if let Some(ref toolchain) = app_settings.override_toolchain {
        args.push(format!("+{toolchain}"));
    }

    args.push(command.to_owned());

    if command != "fmt" && command != "install" {
        // fmt doesn't care about features, workspaces or profiles
        if command != "run" {
            args.push("--workspace".to_owned());

            if let Some(profile) = app_settings.profile.as_ref() {
                args.push("--profile".to_owned());
                args.push(profile.to_owned());

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

pub fn relative_workspace_dir<P: AsRef<Path>>(
    app_settings: &GlobalArgs,
    dir: P,
) -> Result<std::path::PathBuf> {
    let workspace_dir = workspace_dir(app_settings)?;
    Ok(workspace_dir.join(dir))
}

