use anyhow::Result;
use cargo_metadata::semver::Version;
use std::path::PathBuf;

use log::info;

use crate::{
    args::GlobalArgs, codegen_crate_dir, main_workspace_cargo_metadata, read_rust_toolchain,
    relative_workspace_dir, run_system_command, workspace_dir,
};

pub struct CodegenSettings {
    pub bevy_repo_app_settings: GlobalArgs,
    pub main_workspace_app_settings: GlobalArgs,
    pub bevy_version: Version,
    pub bevy_dir: PathBuf,
    pub output_dir: PathBuf,
    pub bevy_features: Vec<String>,
    pub template_args: String,
    pub bms_bindings_path: PathBuf,
}

pub fn prepare_codegen(
    app_settings: GlobalArgs,
    output_dir: Option<PathBuf>,
    bevy_features: Vec<String>,
) -> Result<CodegenSettings> {
    let main_workspace_app_settings = app_settings;
    let bevy_dir = relative_workspace_dir(&main_workspace_app_settings, "target/codegen/bevy")?;
    let bevy_target_dir = bevy_dir.join("target");
    // clear the bevy target dir if it exists
    info!("Clearing bevy target dir: {bevy_target_dir:?}");
    if bevy_target_dir.exists() {
        std::fs::remove_dir_all(&bevy_target_dir)?;
    }

    info!("Cleaning output dir: {output_dir:?}");
    let output_dir = if let Some(output_dir) = output_dir {
        // safety measure
        let output_dir = relative_workspace_dir(&main_workspace_app_settings, output_dir)?;
        if output_dir.exists() && output_dir.to_string_lossy().contains("target") {
            std::fs::remove_dir_all(&output_dir)?;
        }
        std::fs::create_dir_all(&output_dir)?;
        output_dir
    } else {
        Default::default()
    };

    let api_gen_dir = codegen_crate_dir(&main_workspace_app_settings)?;
    let codegen_toolchain = read_rust_toolchain(&api_gen_dir);
    let codegen_app_settings = main_workspace_app_settings
        .clone()
        .with_workspace_dir(api_gen_dir.clone());
    // .with_toolchain(codegen_toolchain.clone()); // don't think it's needed, the rust toolchain file sorts that out

    let bevy_repo_app_settings = main_workspace_app_settings
        .clone()
        .with_workspace_dir(bevy_dir.clone())
        .with_toolchain(codegen_toolchain.clone());

    // run cargo install
    log::info!("Running bevy_api_gen against toolchain: {codegen_toolchain}");
    run_system_command(
        &codegen_app_settings,
        "cargo",
        "Failed to install bevy_api_gen",
        vec!["install", "--path", "."],
        None,
        false,
    )?;

    let metadata = main_workspace_cargo_metadata()?;

    let bevy_version = metadata
        .packages
        .iter()
        .filter_map(|p| (p.name.as_str() == "bevy").then_some(&p.version))
        .max()
        .expect("could not find bevy package in metadata");
    log::info!("Using bevy version {bevy_version}");

    // create directories if they don't already exist
    std::fs::create_dir_all(&bevy_dir)?;

    // git clone bevy repo
    let _ = run_system_command(
        &bevy_repo_app_settings,
        "git",
        "Failed to clone bevy repo",
        vec![
            "clone",
            "https://github.com/bevyengine/bevy",
            "--branch",
            format!("v{}", &bevy_version).as_str(),
            "--depth",
            "1",
            ".",
        ],
        None,
        false,
    );

    // fetch the tags
    run_system_command(
        &bevy_repo_app_settings,
        "git",
        "Failed to fetch bevy tags",
        vec!["fetch", "--tags"],
        Some(&bevy_dir),
        false,
    )?;

    // checkout the version tag
    run_system_command(
        &bevy_repo_app_settings,
        "git",
        "Failed to checkout bevy tag",
        vec!["checkout", format!("v{}", &bevy_version).as_str()],
        Some(&bevy_dir),
        false,
    )?;

    // run bevy_api_gen

    // nothing in template args for now.
    let template_args = CodegenTemplateArgs {};

    let template_args = serde_json::to_string(&template_args)?;
    let bms_bindings_path = workspace_dir(&main_workspace_app_settings)?
        .join("crates/bevy_mod_scripting_bindings")
        .to_path_buf();

    Ok(CodegenSettings {
        bevy_repo_app_settings: bevy_repo_app_settings,
        main_workspace_app_settings: main_workspace_app_settings,
        bevy_version: bevy_version.clone(),
        bevy_dir: bevy_dir,
        output_dir: output_dir,
        bevy_features,
        template_args,
        bms_bindings_path,
    })
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CodegenTemplateArgs {}
