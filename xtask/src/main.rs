use std::{
    collections::HashMap,
    ffi::OsString,
    io::{Write, stdout},
    path::{Path, PathBuf},
    process::{Child, Command, Output},
    str::FromStr,
};

use anyhow::{Context, *};
use clap::Parser;
use itertools::Itertools;
use json_comments::StripComments;
use log::*;
use serde::{Deserialize, Serialize};
use strum::{IntoEnumIterator, VariantNames};
use xtask::{
    Feature, FeatureGroup, Features, GlobalArgs, main_workspace_cargo_metadata,
    make_system_command, relative_workspace_dir, run_system_command, run_workspace_command,
    workspace_dir,
};

/// Enumerates the binaries available in the project and their paths
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    strum::EnumString,
    strum::AsRefStr,
    strum::VariantNames,
)]
#[strum(serialize_all = "snake_case")]
enum Binary {
    MdbookPreprocessor,
}

impl Binary {
    pub fn path(self) -> PathBuf {
        PathBuf::from(match self {
            Binary::MdbookPreprocessor => "./crates/lad_backends/mdbook_lad_preprocessor/",
        })
    }

    pub fn to_placeholder() -> clap::builder::Str {
        format!("[{}]", Binary::VARIANTS.join("|")).into()
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    strum::EnumString,
    strum::AsRefStr,
    strum::Display,
    strum::VariantArray,
)]
enum CiOs {
    #[strum(serialize = "windows-latest")]
    Windows,
    #[strum(serialize = "macos-latest")]
    Macos,
    #[strum(serialize = "ubuntu-latest")]
    Ubuntu,
}

impl CiOs {
    fn is_main_os(&self) -> bool {
        matches!(self, CiOs::Ubuntu)
    }
}

#[derive(Debug, Clone, Parser)]
struct App {
    #[clap(subcommand)]
    subcmd: Xtasks,

    #[clap(flatten)]
    global_args: GlobalArgs,
}

impl App {
    fn into_command(self) -> Command {
        let mut cmd = Command::new("cargo");
        cmd.arg("xtask");

        if self.global_args.features != Features::default() {
            cmd.arg("--features")
                .arg(self.global_args.features.to_string());
        }

        if let Some(profile) = self.global_args.profile {
            cmd.arg("--profile").arg(profile);
        }

        if let Some(jobs) = self.global_args.jobs {
            cmd.arg("--jobs").arg(jobs.to_string());
        }

        match self.subcmd {
            Xtasks::Macros { macro_name } => {
                cmd.arg("macros").arg(macro_name.as_ref());
            }
            Xtasks::Init {
                dont_update_ide,
                tools,
            } => {
                let arg = cmd.arg("init");

                if dont_update_ide {
                    arg.arg("--dont-update-ide");
                }

                if let Some(tools) = tools {
                    arg.arg("--tools");
                    arg.arg(tools.iter().map(|t| t.as_ref()).join(","));
                }
            }
            Xtasks::Build { timings } => {
                cmd.arg("build");
                if timings {
                    cmd.arg("--timings");
                }
            }
            Xtasks::Check {
                ide_mode,
                kind,
                mobile,
            } => {
                cmd.arg("check");
                if ide_mode {
                    cmd.arg("--ide-mode");
                }
                cmd.arg("--kind").arg(kind.as_ref());
                if let Some(mobile) = mobile {
                    cmd.arg("--mobile").arg(mobile.as_ref());
                }
            }
            Xtasks::Docs {
                open,
                no_rust_docs,
                exclude_generated_bindings,
                test_bindings,
            } => {
                cmd.arg("docs");
                if open {
                    cmd.arg("--open");
                }
                if no_rust_docs {
                    cmd.arg("--no-rust-docs");
                }
                if exclude_generated_bindings {
                    cmd.arg("--exclude-generated-bindings");
                }
                if test_bindings {
                    cmd.arg("--test-bindings");
                }
            }
            Xtasks::Test {
                name,
                package,
                mobile,
            } => {
                cmd.arg("test");
                if let Some(name) = name {
                    cmd.arg("--name").arg(name);
                }
                if let Some(package) = package {
                    cmd.arg("--package").arg(package);
                }
                if let Some(mobile) = mobile {
                    cmd.arg("--mobile").arg(mobile.as_ref());
                }
            }
            Xtasks::BlessLad => {
                cmd.arg("bless-lad");
            }
            Xtasks::CiCheck => {
                cmd.arg("ci-check");
            }
            Xtasks::CiMatrix { full, device_tests } => {
                cmd.arg("ci-matrix");
                if full {
                    cmd.arg("--full");
                }
                if device_tests {
                    cmd.arg("--device-tests");
                }
            }
            Xtasks::Example { example } => {
                cmd.arg("example").arg(example);
            }
            Xtasks::Install { binary } => {
                cmd.arg("install").arg(binary.as_ref());
            }
            Xtasks::Bencher { publish } => {
                cmd.arg("bencher");

                if publish {
                    cmd.arg("--publish");
                }
            }
            Xtasks::Bench {
                name,
                enable_profiling: profile,
                tracy_capture_name,
            } => {
                cmd.arg("bench");

                if let Some(name) = name {
                    cmd.arg("--name").arg(name);
                }

                if profile {
                    cmd.arg("--profile");
                }

                if let Some(capture_tracy) = tracy_capture_name {
                    cmd.arg("--capture-tracy").arg(capture_tracy);
                }
            }
        }

        cmd
    }

    pub(crate) fn into_command_string(self) -> OsString {
        let cmd = self.into_command();
        let program = cmd.get_program();
        let args = cmd.get_args();
        let len = args.len();
        let mut os_string = OsString::new();
        os_string.push(program);
        os_string.push(" ");
        for (i, arg) in args.enumerate() {
            os_string.push(arg);
            if i < len - 1 {
                os_string.push(" ");
            }
        }

        os_string
    }

    pub(crate) fn into_ci_row(self, os: CiOs) -> CiMatrixRow {
        let mobile = self.subcmd.mobile();
        // mobile builds don't share their cache with the desktop ones
        let target = mobile
            .map(|m| format!("-{}", m.as_ref()))
            .unwrap_or_default();
        CiMatrixRow {
            command: self.clone().into_command_string().into_string().unwrap(),
            name: format!(
                "{}({}{}) - {}",
                self.subcmd.as_ref(),
                os,
                target.replace('-', ", "),
                if self.global_args.features == Features::all_features() {
                    "all features".to_owned()
                } else if mobile.is_some() && self.global_args.features.0.len() == 1 {
                    // these only enable the language to check
                    self.global_args.features.to_string()
                } else {
                    self.global_args.features.display_no_default()
                }
            ),
            os: os.to_string(),
            init_args: self.subcmd.ci_init_args(),
            cache_key: format!("{}{target}-{os}", self.subcmd.as_ref().to_lowercase()),
            run_on_forks: !matches!(self.subcmd, Xtasks::Bencher { .. } | Xtasks::Bench { .. }),
            requires_gpu: matches!(self.subcmd, Xtasks::Docs { .. }),
            requires_android_emulator: matches!(
                self.subcmd,
                Xtasks::Test {
                    mobile: Some(MobileTarget::Android),
                    ..
                }
            ),
        }
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default,
    strum::EnumString,
    strum::VariantNames,
    strum::AsRefStr,
)]
#[strum(serialize_all = "snake_case")]
enum CheckKind {
    #[default]
    All,
    Main,
}

impl CheckKind {
    fn to_placeholder() -> clap::builder::Str {
        format!("[{}]", CheckKind::VARIANTS.join("|")).into()
    }
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    strum::EnumString,
    strum::AsRefStr,
    strum::VariantNames,
)]
#[strum(serialize_all = "snake_case")]
enum Macro {
    /// Integration tests for all script plugins
    ScriptTests,
}

impl Macro {
    pub fn to_placeholder() -> clap::builder::Str {
        format!("[{}]", Macro::VARIANTS.join("|")).into()
    }
}

/// Mobile platforms which `cargo xtask check --mobile` can type-check for
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, strum::AsRefStr, clap::ValueEnum)]
#[strum(serialize_all = "snake_case")]
#[clap(rename_all = "snake_case")]
enum MobileTarget {
    /// needs the Android NDK
    Android,
    /// needs macOS with Xcode
    Ios,
}

impl MobileTarget {
    fn triple(self) -> &'static str {
        match self {
            MobileTarget::Android => "aarch64-linux-android",
            MobileTarget::Ios => "aarch64-apple-ios",
        }
    }

    /// The languages which build for this target, other than the ones enabled with all features
    fn other_languages(self) -> &'static [Feature] {
        match self {
            // LuaJIT has no iOS support in `luajit-src` (and iOS forbids JIT anyway) and Lua <= 5.3 use `system`,
            // which the iOS SDK marks unavailable
            MobileTarget::Ios => &[Feature::Luau],
            MobileTarget::Android => &[
                Feature::Lua51,
                Feature::Lua52,
                Feature::Lua53,
                Feature::Luajit,
                Feature::Luajit52,
                Feature::Luau,
            ],
        }
    }

    /// The languages other than lua54 and rune the script tests run with on the device, kept small as device runs are slow
    fn device_test_languages(self) -> &'static [Feature] {
        match self {
            MobileTarget::Android => &[Feature::Luajit, Feature::Luau],
            MobileTarget::Ios => &[Feature::Luau],
        }
    }

    /// The rust target of the binaries running on the emulator or simulator, which matches the CI host's architecture
    fn test_triple(self) -> &'static str {
        match self {
            MobileTarget::Android => "x86_64-linux-android",
            MobileTarget::Ios => "aarch64-apple-ios-sim",
        }
    }

    /// The CI OS providing the toolchain for this target
    fn os(self) -> CiOs {
        match self {
            MobileTarget::Android => CiOs::Ubuntu,
            MobileTarget::Ios => CiOs::Macos,
        }
    }
}

/// Optional tools which `cargo xtask init` can install, each is only needed by some of the xtasks
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    strum::EnumString,
    strum::AsRefStr,
    strum::VariantNames,
    strum::VariantArray,
    clap::ValueEnum,
)]
#[strum(serialize_all = "snake_case")]
#[clap(rename_all = "snake_case")]
enum InitTool {
    /// `bencher` cli, used by the `bencher` xtask
    Bencher,
    /// `mdbook` and `mdbook-mermaid`, used by the `docs` xtask
    Mdbook,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, clap::Subcommand, strum::AsRefStr)]
#[clap(
    name = "xtask",
    bin_name = "cargo xtask",
    about = "A set of xtasks for managing the project. Run 'cargo xtask init' to get started."
)]
enum Xtasks {
    /// Set of tasks with predefined settings for running a combination of the other commands
    Macros {
        #[clap(
            required = true,
            value_parser=clap::value_parser!(Macro),
            value_name=Macro::to_placeholder(),
            help = "The macro to run"
        )]
        macro_name: Macro,
    },
    /// Performs first time local-development environment setup
    Init {
        /// Prevents updating the IDE settings, defaults to false
        #[clap(long, default_value = "false")]
        dont_update_ide: bool,
        /// Only install the given optional tools, if the flag is omitted all of them are installed.
        /// Passing the flag with no values installs none of them.
        #[clap(long, value_enum, value_delimiter = ',', num_args = 0..)]
        tools: Option<Vec<InitTool>>,
    },
    /// Build the main workspace only
    Build {
        /// Emit cargo build timings
        #[clap(
            long,
            default_value = "false",
            help = "Emit cargo build timings via --timinigs"
        )]
        timings: bool,
    },
    /// Build the main workspace, apply all prefferred lints
    Check {
        #[clap(
            long,
            default_value = "false",
            help = "Run in the expected format for rust-analyzer's override check command"
        )]
        ide_mode: bool,

        #[clap(
            long,
            default_value = "all",
            value_parser=clap::value_parser!(CheckKind),
            value_name=CheckKind::to_placeholder(),
            help = "The kind of check to perform",
        )]
        kind: CheckKind,

        #[clap(
            long,
            value_enum,
            help = "Only type-check the `bevy_mod_scripting` crate for the given mobile target, needs the target's toolchain"
        )]
        mobile: Option<MobileTarget>,
    },
    /// Run the example with the given name
    Example {
        /// The example to run
        example: String,
    },
    /// Installs a binary produced by the workspace
    Install {
        /// The binary to install
        #[clap(
            value_parser=clap::value_parser!(Binary),
            value_name=Binary::to_placeholder(),
        )]
        binary: Binary,
    },
    /// Build the rust crates.io docs as well as any other docs, with all the bindings.
    ///
    /// This also generates the ladfile and the Lua language server files into `assets/definitions`
    Docs {
        /// Open in browser
        /// This will open the generated docs in the default browser
        #[clap(long)]
        open: bool,

        /// Skip building rust docs
        #[clap(long)]
        no_rust_docs: bool,

        /// Skip the generated `bms_bindings` crates when building rust docs, there are many of them and
        /// they are slow to document
        #[clap(long)]
        exclude_generated_bindings: bool,

        /// Also document the functions available to the script tests, to get completions while writing tests
        #[clap(long)]
        test_bindings: bool,
    },
    /// Build the main workspace, and then run all tests
    Test {
        /// Run tests containing the given name only
        #[clap(long)]
        name: Option<String>,

        /// Run tests in the given package only
        #[clap(long)]
        package: Option<String>,

        #[clap(
            long,
            value_enum,
            help = "Run the script tests on an Android emulator or iOS simulator instead, the emulator has to be running (`adb`)"
        )]
        mobile: Option<MobileTarget>,
    },
    /// Rewrites the expected outputs of the LAD tests: the LAD file, then the Lua declarations and mdbook pages generated from it,
    /// and checks them. Needs `mdbook` on the PATH (`cargo xtask init`).
    BlessLad,
    /// Perform a full check as it would be done in CI, except not parallelised
    CiCheck,
    /// Generate a json job matrix, containing, describing maximally parallelised jobs for running in CI/CD.
    ///
    /// The format of the output is:
    ///
    /// [
    ///    {
    ///       "command": "the command to run"
    ///    }
    /// ]
    ///
    CiMatrix {
        /// Emit the full matrix (every OS, every feature combination, benchmarks) instead of the reduced
        /// one used for pull requests
        #[clap(long)]
        full: bool,

        /// Include the tests running on an Android emulator and iOS simulator, which the full matrix includes already
        #[clap(long)]
        device_tests: bool,
    },
    /// Runs bencher in dry mode by default if not on the main branch
    /// To publish main branch defaults set publish mode to true
    Bencher {
        /// Publish the benchmarks when on main
        #[clap(long, default_value = "false", help = "Publish the benchmarks")]
        publish: bool,
    },
    /// Runs criterion benchmarks generates json required to be published by bencher and generates html performance report
    Bench {
        /// Whether or not to enable tracy profiling
        #[clap(long, default_value = "false", help = "Enable tracy profiling")]
        enable_profiling: bool,
        #[clap(
            long,
            help = "The name to give to the captured trace, if not passed won't capture trace"
        )]
        tracy_capture_name: Option<String>,
        /// The name argument passed to `cargo bench`, can be used in combination with profile to selectively profile benchmarks
        #[clap(long, help = "The name argument passed to `cargo bench`")]
        name: Option<String>,
    },
}

#[derive(Serialize, Clone)]
struct CiMatrixRow {
    /// The command to run
    command: String,
    /// The display name of the job
    name: String,
    /// The os to run this on
    os: String,
    /// The arguments to pass to `cargo xtask init`, so only the tools this job needs get installed
    init_args: String,
    /// Identifies the cargo cache this job restores from and saves to, jobs doing the same kind of work share one
    cache_key: String,
    /// If this step requires a gpu
    requires_gpu: bool,
    /// If this step runs on an Android emulator, which the CI has to start
    requires_android_emulator: bool,
    /// if it should run on fork PR's
    run_on_forks: bool,
}

impl Xtasks {
    /// The `cargo xtask init` arguments installing only the optional tools this xtask needs when run in CI
    fn ci_init_args(&self) -> String {
        let tools: &[InitTool] = match self {
            // the preprocessor tests build a book with `mdbook`
            Xtasks::Docs { .. } | Xtasks::Test { .. } => &[InitTool::Mdbook],
            Xtasks::Bencher { .. } | Xtasks::Bench { .. } => &[InitTool::Bencher],
            _ => &[],
        };

        // the IDE settings are of no use in CI
        format!(
            "--dont-update-ide --tools {}",
            tools.iter().map(|t| t.as_ref()).join(",")
        )
    }

    /// The mobile target this xtask builds for, if it is a mobile one
    fn mobile(&self) -> Option<MobileTarget> {
        match self {
            Xtasks::Check { mobile, .. } | Xtasks::Test { mobile, .. } => *mobile,
            _ => None,
        }
    }

    /// The OS which provides the toolchain needed to build for the mobile target of this xtask, if it is a mobile one
    fn mobile_os(&self) -> Option<CiOs> {
        self.mobile().map(MobileTarget::os)
    }

    fn run(self, app_settings: GlobalArgs) -> Result<String> {
        match self {
            Xtasks::Build { timings } => Self::build(timings, app_settings),
            Xtasks::Check {
                ide_mode,
                kind,
                mobile,
            } => Self::check(app_settings, ide_mode, kind, mobile),
            Xtasks::Docs {
                open,
                no_rust_docs,
                exclude_generated_bindings,
                test_bindings,
            } => Self::docs(
                app_settings,
                open,
                no_rust_docs,
                exclude_generated_bindings,
                test_bindings,
            ),
            Xtasks::Test {
                name,
                mobile: Some(mobile),
                ..
            } => Self::test_mobile(app_settings, mobile, name),
            Xtasks::Test {
                name,
                package,
                mobile: None,
            } => Self::test(app_settings, package, name),
            Xtasks::BlessLad => Self::bless_lad(app_settings),
            Xtasks::CiCheck => Self::cicd(app_settings),
            Xtasks::Init {
                dont_update_ide,
                tools,
            } => Self::init(app_settings, dont_update_ide, tools),
            Xtasks::Example { example } => Self::example(app_settings, example),
            Xtasks::Macros { macro_name } => match macro_name {
                Macro::ScriptTests => {
                    let mut settings = app_settings.clone();
                    settings.features = Features::all_features();
                    Self::test(settings, Some("script_test".to_owned()), None)
                }
            },
            Xtasks::CiMatrix { full, device_tests } => {
                let mut output = Self::ci_matrix(app_settings, full, device_tests)?;
                output.sort_by(|e1, e2| e1.subcmd.cmp(&e2.subcmd));
                let mut rows = Vec::default();
                for os in <CiOs as strum::VariantArray>::VARIANTS {
                    for row in output.iter() {
                        // mobile checks need the target's toolchain, so they run on the OS providing it
                        // regardless of the matrix size
                        if let Some(mobile_os) = row.subcmd.mobile_os() {
                            if *os == mobile_os {
                                rows.push(row.clone().into_ci_row(*os));
                            }
                            continue;
                        }

                        // these only need to run on one OS, the others are covered by the OS specific
                        // checks and tests. Benchmarks are the exception, each OS publishes to its own
                        // bencher testbed on main
                        let step_should_run_on_main_os =
                            matches!(row.subcmd, Xtasks::Build { .. } | Xtasks::Docs { .. });

                        if !os.is_main_os() && step_should_run_on_main_os {
                            continue;
                        }

                        // pull requests only run on the main OS, the full matrix runs on main and staging
                        if !full && !os.is_main_os() {
                            continue;
                        }

                        rows.push(row.clone().into_ci_row(*os));
                    }
                }

                let json = serde_json::to_string_pretty(&rows)?;
                return Ok(json);
            }
            Xtasks::Install { binary } => Self::install(app_settings, binary),
            Xtasks::Bencher { publish } => Self::bencher(app_settings, publish),
            Xtasks::Bench {
                name,
                enable_profiling,
                tracy_capture_name,
            } => {
                let _ = Self::bench(
                    app_settings,
                    enable_profiling,
                    name,
                    tracy_capture_name,
                    false,
                )?;
                Ok(())
            }
        }?;

        Ok("".into())
    }

    fn build(timings: bool, app_settings: GlobalArgs) -> Result<()> {
        // build workspace using the given features
        let mut args = vec!["--all-targets", "--examples"];
        if timings {
            args.push("--timings");
        }

        run_workspace_command(
            &app_settings,
            "build",
            "Failed to build workspace",
            args,
            None,
            false,
        )?;
        Ok(())
    }

    fn check_main_workspace(app_settings: GlobalArgs, ide_mode: bool) -> Result<()> {
        // start with cargo clippy
        let mut clippy_args = vec![];
        if ide_mode {
            clippy_args.push("--message-format=json");
        }

        clippy_args.extend(["--all-targets", "--examples"]);

        let keep_going = std::env::var(XTASK_KEEP_GOING).is_ok();
        if !keep_going {
            clippy_args.extend(vec!["--", "-D", "warnings"]);
        }

        run_workspace_command(
            &app_settings,
            "clippy",
            "Failed to run clippy",
            clippy_args,
            None,
            false,
        )?;

        if ide_mode {
            return Ok(());
        }

        // run cargo fmt checks
        run_workspace_command(
            &app_settings,
            "fmt",
            "Failed to run cargo fmt",
            vec!["--all", "--", "--check"],
            None,
            false,
        )?;

        Ok(())
    }

    fn check(
        app_settings: GlobalArgs,
        ide_mode: bool,
        kind: CheckKind,
        mobile: Option<MobileTarget>,
    ) -> Result<()> {
        if let Some(mobile) = mobile {
            return Self::check_mobile(app_settings, mobile);
        }

        match kind {
            // the main workspace is the only one left
            CheckKind::All | CheckKind::Main => Self::check_main_workspace(app_settings, ide_mode),
        }
    }

    /// Type-checks the `bevy_mod_scripting` crate with the given features for the given mobile target
    fn check_mobile(app_settings: GlobalArgs, mobile: MobileTarget) -> Result<()> {
        let mut cmd = Self::mobile_cargo_command(
            &app_settings,
            mobile,
            mobile.triple(),
            &["check", "--package", "bevy_mod_scripting"],
        )?;

        let status = cmd.status().context("Failed to check for mobile target")?;
        ensure!(
            status.success(),
            "Checking for {} failed with features: {}",
            mobile.triple(),
            app_settings.features
        );
        Ok(())
    }

    /// Runs the script tests on an Android emulator (needs a running one reachable with `adb`) or an iOS simulator.
    ///
    /// The test binary is built for the device and run through a cargo runner. The device needs the `assets`,
    /// which the tests find through `CARGO_MANIFEST_DIR`.
    fn test_mobile(
        app_settings: GlobalArgs,
        mobile: MobileTarget,
        name: Option<String>,
    ) -> Result<()> {
        let target = mobile.test_triple();
        let mut cmd = Self::mobile_cargo_command(
            &app_settings,
            mobile,
            target,
            &[
                "test",
                "--package",
                "bevy_mod_scripting",
                "--test",
                "script_tests",
            ],
        )?;

        let runner_env = format!(
            "CARGO_TARGET_{}_RUNNER",
            target.to_uppercase().replace('-', "_")
        );
        let workspace = workspace_dir(&app_settings)?;
        match mobile {
            MobileTarget::Android => {
                let remote = "/data/local/tmp";
                let adb = |args: &[&str]| {
                    run_system_command(&app_settings, "adb", "Failed to run adb", args, None, false)
                };
                adb(&["wait-for-device"])?;
                adb(&[
                    "shell",
                    "mkdir",
                    "-p",
                    &format!("{remote}/bevy_mod_scripting"),
                ])?;
                adb(&[
                    "push",
                    &workspace.join("assets").to_string_lossy(),
                    &format!("{remote}/bevy_mod_scripting/"),
                ])?;

                // the C++ runtime of luau
                if let Some(toolchain) = android_ndk_toolchain() {
                    let libcxx = toolchain
                        .join("sysroot/usr/lib")
                        .join(target)
                        .join("libc++_shared.so");
                    adb(&["push", &libcxx.to_string_lossy(), &format!("{remote}/")])?;
                }

                let runner = std::env::temp_dir().join("bms_android_test_runner.sh");
                std::fs::write(
                    &runner,
                    format!(
                        "#!/bin/bash\n\
                         set -e\n\
                         bin=\"$1\"; shift\n\
                         adb push \"$bin\" {remote}/ > /dev/null\n\
                         adb shell \"cd {remote} && chmod +x ./$(basename \"$bin\") && \
                         LD_LIBRARY_PATH={remote} RUST_BACKTRACE=1 \
                         CARGO_MANIFEST_DIR={remote}/bevy_mod_scripting \
                         ./$(basename \"$bin\") $(printf '%q ' \"$@\")\"\n"
                    ),
                )?;
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    std::fs::set_permissions(&runner, std::fs::Permissions::from_mode(0o755))?;
                }
                cmd.env(runner_env, runner);
            }
            MobileTarget::Ios => {
                let udid = Self::boot_ios_simulator(&app_settings)?;
                cmd.env(runner_env, format!("xcrun simctl spawn {udid}"));
                // the simulator shares the host's file system, `simctl` forwards the variables with this prefix
                cmd.env("SIMCTL_CHILD_CARGO_MANIFEST_DIR", &workspace);
                cmd.env("SIMCTL_CHILD_RUST_BACKTRACE", "1");
            }
        }

        if let Some(name) = name {
            cmd.args(["--", &name]);
        }

        let status = cmd.status().context("Failed to run tests on the device")?;
        ensure!(
            status.success(),
            "Tests on {target} failed with features: {}",
            app_settings.features
        );
        Ok(())
    }

    /// Boots an iPhone simulator if none is, and returns its id
    fn boot_ios_simulator(app_settings: &GlobalArgs) -> Result<String> {
        let output = run_system_command(
            app_settings,
            "xcrun",
            "Failed to list simulators",
            ["simctl", "list", "devices", "available", "-j"],
            None,
            true,
        )?;
        let devices: serde_json::Value = serde_json::from_slice(&output.stdout)?;
        let udid = devices["devices"]
            .as_object()
            .into_iter()
            .flatten()
            .filter(|(runtime, _)| runtime.contains("SimRuntime.iOS"))
            .flat_map(|(_, devices)| devices.as_array().into_iter().flatten())
            .find(|d| d["name"].as_str().is_some_and(|n| n.starts_with("iPhone")))
            .and_then(|d| d["udid"].as_str())
            .context("No iPhone simulator available")?
            .to_owned();

        // boots the simulator if needed and waits for it
        run_system_command(
            app_settings,
            "xcrun",
            "Failed to boot the simulator",
            ["simctl", "bootstatus", &udid, "-b"],
            None,
            false,
        )?;
        Ok(udid)
    }

    /// A cargo command running `args` for the given mobile target with the features of `app_settings`
    fn mobile_cargo_command(
        app_settings: &GlobalArgs,
        mobile: MobileTarget,
        target: &str,
        args: &[&str],
    ) -> Result<Command> {
        run_system_command(
            app_settings,
            "rustup",
            "Failed to add the rust target",
            ["target", "add", target],
            None,
            false,
        )?;

        let mut args = args.iter().map(|a| a.to_string()).collect::<Vec<_>>();
        args.extend(["--target".to_owned(), target.to_owned()]);
        args.extend(app_settings.features.to_cargo_args());

        let android = mobile == MobileTarget::Android;
        if android {
            // bevy's android glue needs an activity implementation, apps normally pick one through their `bevy` dependency
            args.extend([
                "--features".to_owned(),
                "bevy/android-native-activity".to_owned(),
            ]);
        }

        let mut cmd = make_system_command(app_settings, "cargo", args, None, false)?;
        if android {
            cmd.envs(android_ndk_env(target));
        }
        Ok(cmd)
    }

    fn docs(
        mut app_settings: GlobalArgs,
        open: bool,
        no_rust_docs: bool,
        exclude_generated_bindings: bool,
        test_bindings: bool,
    ) -> Result<()> {
        // find [package.metadata."docs.rs"] key in Cargo.toml
        info!("installing mdbook ladfile preprocessor binary");
        Self::install(app_settings.clone(), Binary::MdbookPreprocessor)?;

        info!("Running docgen example to generate ladfiles");
        let mut docgen_settings = app_settings.clone();
        docgen_settings.features = docs_features();
        let example_args = if test_bindings { &["testlib"][..] } else { &[] };
        Self::example_with_args(docgen_settings, "docgen", example_args)?;

        // copy the `<workspace>/assets/bindings.lad.json` file to it's path in the book
        let ladfile_path =
            relative_workspace_dir(&app_settings, "assets/definitions/bindings.lad.json")?;
        let destination_path =
            relative_workspace_dir(&app_settings, "docs/src/ladfiles/bindings.lad.json")?;

        info!("Copying generated ladfile from: {ladfile_path:?} to: {destination_path:?}");
        std::fs::create_dir_all(destination_path.parent().unwrap())?;
        std::fs::copy(ladfile_path, destination_path)
            .with_context(|| "copying generated ladfile")?;

        if !no_rust_docs {
            info!("Building rust docs");
            let metadata = main_workspace_cargo_metadata()?;

            let package = metadata
                .packages
                .iter()
                .find(|p| p.name.as_str() == "bevy_mod_scripting")
                .expect("Could not find bevy_mod_scripting package in metadata");

            info!("Building with root package: {}", package.name);

            let docs_rs = package
                .metadata
                .get("docs.rs")
                .expect("no docs.rs metadata");

            let features = docs_rs
                .as_object()
                .expect("docs.rs metadata is not an object")
                .get("features")
                .expect("no 'features' in docs.rs metadata");

            info!("Using docs.rs metadata: {docs_rs:?}");
            let string_list = features
                .as_array()
                .expect("docs.rs metadata is not an array")
                .iter()
                .map(|v| v.as_str().expect("docs.rs metadata is not a string"))
                .map(|s| Feature::from_str(s).expect("invalid feature"))
                .collect::<Vec<_>>();

            // include default features
            let default_features = Features::default();
            let mut features = Features::new(string_list);
            features.0.extend(default_features.0);

            app_settings.features = features;

            let mut args = Vec::default();
            args.push("--all".to_owned());
            if open {
                // dependencies are only worth documenting if someone is going to browse them
                args.push("--open".to_owned());
            } else {
                args.push("--no-deps".to_owned());
            }

            if exclude_generated_bindings {
                for package in metadata.workspace_packages() {
                    if package
                        .manifest_path
                        .components()
                        .any(|c| c.as_str() == "bindings")
                        && package.name.as_str().ends_with("_bms_bindings")
                    {
                        args.push("--exclude".to_owned());
                        args.push(package.name.to_string());
                    }
                }
            }
            run_workspace_command(
                &app_settings,
                "doc",
                "Failed to build crates.io docs",
                args,
                None,
                false,
            )?;
        }

        // build mdbook
        info!("Building mdbook docs");
        let args = if !open { vec!["build"] } else { vec!["serve"] };

        run_system_command(
            &app_settings,
            "mdbook",
            "Failed to build or serve mdbook docs",
            args,
            Some(Path::new("docs")),
            false,
        )?;

        Ok(())
    }

    fn bench(
        app_settings: GlobalArgs,
        profile: bool,
        name: Option<String>,
        tracy_capture_name: Option<String>,
        capture_streams_in_output: bool,
    ) -> Result<Output> {
        log::info!("Profiling enabled: {profile}");

        struct DroppingChild(Child);
        impl Drop for DroppingChild {
            fn drop(&mut self) {
                let _ = self.0.kill();
            }
        }

        let mut features = Features::default();
        let mut tracy_child_capture: Option<DroppingChild> = None;
        if profile {
            unsafe { std::env::set_var("ENABLE_PROFILING", "1") };
            features.0.insert(Feature::ProfileWithTracy);
            if let Some(trace_name) = tracy_capture_name {
                let mut cmd = make_system_command(
                    &app_settings,
                    "tracy-capture",
                    ["-o", &trace_name, "-a", "localhost"],
                    None,
                    capture_streams_in_output,
                )?;
                tracy_child_capture = Some(DroppingChild(cmd.spawn()?));
            }
        } else {
            unsafe { std::env::set_var("RUST_LOG", "bevy_mod_scripting=error") };
        }

        let mut args = vec!["--timings".to_owned()];

        if let Some(name) = name {
            args.extend(["--".to_owned(), name]);
        };

        let mut output = run_workspace_command(
            // run with just lua54
            &app_settings.clone().with_features(features),
            "bench",
            "Failed to run benchmarks",
            args,
            None,
            capture_streams_in_output,
        )
        .with_context(|| "when executing criterion benchmarks")?;

        let workspace_dir = workspace_dir(&app_settings)?;
        let metadata = main_workspace_cargo_metadata()?;
        let crates = metadata
            .workspace_packages()
            .iter()
            .map(|p| p.name.to_string())
            .collect::<Vec<String>>();

        info!("Gathering build time for crates: {}", crates.join(","));

        let additional_benchmark_lines = xtask::read_cargo_timings_report(&workspace_dir, crates)?;

        if !capture_streams_in_output {
            stdout().write(additional_benchmark_lines.as_bytes())?;
        } else {
            output.stdout.write(additional_benchmark_lines.as_bytes())?;
        }
        drop(tracy_child_capture);
        Ok(output)
    }

    fn bencher(app_settings: GlobalArgs, publish: bool) -> Result<()> {
        // // first of all figure out which branch we're on
        // // run // git rev-parse --abbrev-ref HEAD
        let workspace_dir = workspace_dir(&app_settings).unwrap();
        let command = Command::new("git")
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .current_dir(workspace_dir.clone())
            .output()
            .with_context(|| "Trying to figure out which branch we're on in benchmarking")?;
        let branch = String::from_utf8(command.stdout)?.trim().replace("\n", "");

        let is_main = branch.trim() == "main";

        // figure out if we're running in github actions
        let github_token = std::env::var("GITHUB_TOKEN").ok();

        // get testbed
        // we want this to be a combination of
        // is_github_ci?
        // OS
        // machine id

        let os = std::env::consts::OS;

        let testbed = format!(
            "{os}{}",
            if github_token.is_some() {
                "-gha"
            } else {
                Default::default()
            }
        );

        // also figure out if we're on a fork

        let token = std::env::var("BENCHER_API_TOKEN").ok();

        // first of all run bench, and save output to a file

        let result = Self::bench(app_settings, false, None, None, true)?;
        let bench_file_path = PathBuf::from("./bencher_output.txt");
        let mut file = std::fs::File::create(&bench_file_path)?;
        file.write_all(&result.stdout)?;

        let mut bencher_cmd = Command::new("bencher");
        bencher_cmd
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .arg("run")
            .args(["--project", "bms"])
            .args(["--branch", &branch])
            .args(["--token", &token.unwrap_or_default()])
            .args(["--testbed", &testbed])
            // .args(["--build-time"])
            .args(["--threshold-measure", "latency"])
            .args(["--threshold-test", "t_test"])
            .args(["--threshold-max-sample-size", "10"])
            .args(["--threshold-upper-boundary", "0.99"])
            .args(["--thresholds-reset"]);

        if let Some(token) = &github_token {
            bencher_cmd.args(["--github-actions", token]);
        }

        if !is_main || !publish {
            bencher_cmd.args(["--dry-run"]);
        }

        bencher_cmd
            .args(["--adapter", "rust_criterion"])
            .arg("--file")
            .arg(bench_file_path);

        log::info!("Running bencher command: {bencher_cmd:?}");

        let out = bencher_cmd
            .output()
            .with_context(|| "Could not trigger bencher command")?;
        if !out.status.success() {
            bail!("Failed to run bencher: {:?}", out);
        }

        // if we're on linux and publishing and on main synch graphs
        if os == "linux" && is_main && publish && github_token.is_some() {
            Self::synch_bencher_graphs()?;
        }

        Ok(())
    }

    fn synch_bencher_graphs() -> Result<()> {
        // first run `bencher benchmark list bms
        // this produces list of objects each containing a `uuid` and `name`

        let parse_list_of_dicts = |bytes: Vec<u8>| {
            if bytes.is_empty() {
                bail!("Empty input");
            }
            serde_json::from_slice::<Vec<HashMap<String, serde_json::Value>>>(&bytes)
                .map(|map| {
                    map.into_iter()
                        .map(|map| {
                            map.into_iter()
                                .map(|(k, v)| (k, v.as_str().unwrap_or_default().to_string()))
                                .collect::<HashMap<_, _>>()
                        })
                        .collect::<Vec<_>>()
                })
                .with_context(|| "Could not parse bencher output")
        };

        let token = std::env::var("BENCHER_API_TOKEN").ok();
        let mut bencher_cmd = Command::new("bencher");
        let benchmarks = bencher_cmd
            .stdout(std::process::Stdio::piped())
            .arg("benchmark")
            .args(["list", "bms"])
            .args(["--per-page", "255"])
            .args(["--token", &token.clone().unwrap_or_default()])
            .output()
            .with_context(|| "Could not list benchmarks")?;
        if !benchmarks.status.success() {
            bail!("Failed to list benchmarks: {:?}", benchmarks);
        }

        // parse teh name and uuid pairs
        let benchmarks = parse_list_of_dicts(benchmarks.stdout)
            .with_context(|| "Reading benchmarks")?
            .into_iter()
            .map(|p| {
                let name = p.get("name").expect("no name in project");
                let uuid = p.get("uuid").expect("no uuid in project");
                (name.clone(), uuid.clone())
            })
            .collect::<Vec<_>>();

        // delete all plots using bencher plot list bms to get "uuid's"
        // then bencher plot delete bms <uuid>

        let bencher_cmd = Command::new("bencher")
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::inherit())
            .args(["plot", "list", "bms"])
            .args(["--per-page", "255"])
            .args(["--token", &token.clone().unwrap_or_default()])
            .output()
            .with_context(|| "Could not list plots")?;

        if !bencher_cmd.status.success() {
            bail!("Failed to list plots: {:?}", bencher_cmd);
        }

        let plots = parse_list_of_dicts(bencher_cmd.stdout)
            .with_context(|| "reading plots")?
            .into_iter()
            .map(|p| {
                log::info!("Plot to delete: {p:?}");
                let uuid = p.get("uuid").expect("no uuid in plot");
                uuid.clone()
            })
            .collect::<Vec<_>>();

        for uuid in plots {
            let bencher_cmd = Command::new("bencher")
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit())
                .args(["plot", "delete", "bms", &uuid])
                .args(["--token", &token.clone().unwrap_or_default()])
                .output()
                .with_context(|| "Could not delete plot")?;

            if !bencher_cmd.status.success() {
                bail!("Failed to delete plot: {:?}", bencher_cmd);
            }
        }

        const MAIN_BRANCH_UUID: &str = "1d70a4e3-d416-43fc-91bd-4b1c8f9e9580";
        const LATENCY_MEASURE_UUID: &str = "6820b034-5163-4cdd-95f5-5640dd0ff298";
        const LINUX_GHA_TESTBED: &str = "467e8580-a67a-435e-a602-b167541f332c";
        const MACOS_GHA_TESTBAD: &str = "f8aab940-27d2-4b52-93df-4518fe68abfb";
        const WINDOWS_GHA_TESTBED: &str = "be8ff546-31d3-40c4-aacc-763e5e8a09c4";

        let testbeds = [
            ("linux-gha", LINUX_GHA_TESTBED),
            ("macos-gha", MACOS_GHA_TESTBAD),
            ("windows-gha", WINDOWS_GHA_TESTBED),
        ];

        let group_to_benchmark_map: HashMap<_, Vec<_>> =
            benchmarks
                .iter()
                .fold(HashMap::new(), |mut acc, (name, uuid)| {
                    let group = name.split('/').next().unwrap_or_default();
                    acc.entry(group.to_owned()).or_default().push(uuid.clone());
                    acc
                });

        // create plot using
        // bencher plot create --x-axis date_time --branches main --testbeds <uuids> --benchmarks <uuids> --measures latency

        // bencher rejects plots with more than this many benchmarks
        const MAX_BENCHMARKS_PER_PLOT: usize = 8;

        for (group, uuids) in group_to_benchmark_map.iter().sorted() {
            if uuids.len() > MAX_BENCHMARKS_PER_PLOT {
                log::warn!(
                    "Skipping plots for benchmark group `{group}`: {} benchmarks, bencher allows at most {MAX_BENCHMARKS_PER_PLOT} per plot",
                    uuids.len()
                );
                continue;
            }

            for (testbed_name, testbed_uuid) in testbeds.iter() {
                let without_gha = testbed_name.replace("-gha", "");
                let plot_name = format!("{without_gha} {group}");

                let window_months = 12;
                let window_seconds = window_months * 30 * 24 * 60 * 60;
                let mut bencher_cmd = Command::new("bencher");
                bencher_cmd
                    .stdout(std::process::Stdio::inherit())
                    .stderr(std::process::Stdio::inherit())
                    .args(["plot", "create", "bms"])
                    .args(["--title", &plot_name])
                    .args(["--x-axis", "version"])
                    .args(["--window", &window_seconds.to_string()])
                    .args(["--branches", MAIN_BRANCH_UUID])
                    .args(["--testbeds", testbed_uuid])
                    .args(["--measures", LATENCY_MEASURE_UUID])
                    .args(["--token", &token.clone().unwrap_or_default()]);

                for benchmark_uuid in uuids {
                    bencher_cmd.arg("--benchmarks").arg(benchmark_uuid);
                }

                let bencher_cmd = bencher_cmd
                    .output()
                    .with_context(|| "Could not create plot")?;

                if !bencher_cmd.status.success() {
                    bail!("Failed to create plot: {:?}", bencher_cmd);
                }
            }
        }

        Ok(())
    }

    fn test(app_settings: GlobalArgs, package: Option<String>, name: Option<String>) -> Result<()> {
        // run cargo test
        let mut test_args = vec![];
        if let Some(package) = package {
            test_args.push("--package".to_owned());
            test_args.push(package);
        }

        if let Some(name) = name {
            test_args.push(name);
        }

        test_args.push("--exclude".to_owned());
        test_args.push("xtask".to_owned());

        if std::env::var("BLESS_MODE").is_ok() {
            test_args.push("--no-fail-fast".to_owned())
        }

        run_workspace_command(
            &app_settings,
            "test",
            "Failed to run tests",
            test_args,
            None,
            false,
        )?;

        Ok(())
    }

    /// The crates whose tests compare against blessed LAD outputs, in the order the outputs feed each other
    const LAD_PACKAGES: [&str; 3] = [
        "ladfile_builder",
        "lua_language_server_lad_backend",
        "mdbook_lad_preprocessor",
    ];

    fn bless_lad(app_settings: GlobalArgs) -> Result<()> {
        // SAFETY: xtask runs no other threads which could read the environment at this point
        unsafe { std::env::set_var("BLESS_MODE", "1") };
        for package in Self::LAD_PACKAGES {
            // tests which bless their expected output fail on purpose, the check below reruns them
            let _ = Self::test(app_settings.clone(), Some(package.to_owned()), None);
        }
        // SAFETY: as above
        unsafe { std::env::remove_var("BLESS_MODE") };
        for package in Self::LAD_PACKAGES {
            Self::test(app_settings.clone(), Some(package.to_owned()), None)?;
        }
        Ok(())
    }

    fn ci_matrix(app_settings: GlobalArgs, full: bool, device_tests: bool) -> Result<Vec<App>> {
        // split up the tests we want to run into different jobs
        // everything that can be parallelised should be
        // each row in the output will correspond to a separate job in the matrix.

        let mut output = vec![];

        // first of all we run a powerset of check commands with various features enabled. all of which can be run in parallel
        let available_features =
            Features::new(<Feature as strum::VariantArray>::VARIANTS.iter().cloned());

        let grouped = available_features.split_by_group();

        let features_to_combine = grouped
            .get(&FeatureGroup::BMSFeature)
            .expect("no bms features were found at all, bms definitely has feature flags");

        // run powerset with all language features enabled without mutually exclusive
        let mut powersets = features_to_combine
            .iter()
            .cloned()
            .powerset()
            .map(Features::new)
            .collect::<Vec<_>>();

        // start with longest to compile all first
        powersets.reverse();

        if !full {
            // the powerset is a lot of runner time, which pull requests have to share. The all-features build
            // is covered by the tests, so only check the smallest feature set (catches missing feature gates)
            powersets = powersets.pop().into_iter().collect();
        }
        info!("Powerset: {powersets:?}");

        let default_args = app_settings
            .clone()
            .with_features(Features::all_features().without(Feature::ProfileWithTracy))
            .with_profile(
                app_settings
                    .profile
                    .clone()
                    .or(Some("ephemeral-build".to_owned())),
            );

        for feature_set in powersets.iter_mut() {
            // choose language features
            for exclusive_category in FeatureGroup::iter().filter(|g| g.is_exclusive()) {
                feature_set.0.insert(exclusive_category.default_feature());
            }

            // include all non-bms features
            if let Some(f) = grouped.get(&FeatureGroup::ForExternalCrate) {
                feature_set.0.extend(f.iter().cloned());
            }

            // include all features which are excluded from powersetting
            if let Some(f) = grouped.get(&FeatureGroup::BMSFeatureNotInPowerset) {
                feature_set.0.extend(f.iter().cloned());
            }

            // replace args with powerset
            output.push(App {
                global_args: default_args.clone().with_features(feature_set.clone()),
                subcmd: Xtasks::Build { timings: false },
            })
        }

        log::info!("Powerset command combinations: {output:?}");

        // next run a full lint check with all features
        output.push(App {
            global_args: default_args.clone(),
            subcmd: Xtasks::Check {
                ide_mode: false,
                kind: CheckKind::All,
                mobile: None,
            },
        });

        // the language runtimes are built with the C toolchain of the target platform. Pull requests check
        // all features once per mobile platform, the full matrix also checks the other runtimes the platform supports
        for mobile in [MobileTarget::Android, MobileTarget::Ios] {
            let check = |features: Features| App {
                global_args: default_args.clone().with_features(features),
                subcmd: Xtasks::Check {
                    ide_mode: false,
                    kind: CheckKind::All,
                    mobile: Some(mobile),
                },
            };

            output.push(check(default_args.features.clone()));
            if full {
                output.extend(
                    mobile
                        .other_languages()
                        .iter()
                        .map(|language| check(Features::new([*language]))),
                );
            }
        }

        // the script tests run on emulators and simulators, which is slow, so they only run on main
        // (or when asked for)
        if full || device_tests {
            for mobile in [MobileTarget::Android, MobileTarget::Ios] {
                let test = |features: Features| App {
                    global_args: default_args.clone().with_features(features),
                    subcmd: Xtasks::Test {
                        name: None,
                        package: None,
                        mobile: Some(mobile),
                    },
                };

                // all features includes lua54 and rune
                output.push(test(default_args.features.clone()));
                for language in mobile.device_test_languages() {
                    let mut features = default_args.features.clone().without(Feature::Lua54);
                    features.0.insert(*language);
                    output.push(test(features));
                }
            }
        }

        // then run docs
        output.push(App {
            global_args: default_args.clone(),
            subcmd: Xtasks::Docs {
                open: false,
                no_rust_docs: false,
                // the generated crates are documented on docs.rs, no need to wait on them here
                exclude_generated_bindings: true,
                test_bindings: false,
            },
        });

        // benchmarks publish to bencher, so they only run for the full matrix. Pull requests run them on demand,
        // see the `Run benchmarks on PR` workflow
        if full {
            output.push(App {
                // benchmarks need to be representative, so they keep cargo's own bench profile
                global_args: default_args
                    .clone()
                    .with_profile(app_settings.profile.clone()),
                subcmd: Xtasks::Bencher { publish: true },
            });
        }

        // and finally run tests
        output.push(App {
            global_args: default_args.clone(),
            subcmd: Xtasks::Test {
                name: None,
                package: None,
                mobile: None,
            },
        });

        Ok(output)
    }

    fn cicd(app_settings: GlobalArgs) -> Result<()> {
        // get the ci matrix
        let matrix = Self::ci_matrix(app_settings.clone(), true, true)?;
        // the mobile checks need toolchains which are not generally available
        let matrix = matrix
            .into_iter()
            .filter(|app| app.subcmd.mobile_os().is_none())
            .collect::<Vec<_>>();
        let length = matrix.len();
        for (i, app) in matrix.into_iter().enumerate() {
            info!("Running CI job {}/{}. {:?}", i + 1, length, app.subcmd);
            app.subcmd.run(app_settings.clone())?;
        }

        Ok(())
    }

    fn init(
        app_settings: GlobalArgs,
        dont_update_ide: bool,
        tools: Option<Vec<InitTool>>,
    ) -> Result<()> {
        // by default install everything, CI narrows this down to what each job needs
        let tools = tools.unwrap_or_else(|| <InitTool as strum::VariantArray>::VARIANTS.to_vec());
        let wants = |tool: InitTool| tools.contains(&tool);

        // install alsa et al
        if cfg!(target_os = "linux") {
            let sudo = if !is_root::is_root() { "sudo" } else { "" };
            let install_cmd = format!(
                "{sudo} apt-get update && {sudo} apt-get install --no-install-recommends -y libasound2-dev libudev-dev libwayland-dev"
            );
            run_system_command(
                &app_settings,
                "sh",
                "Failed to install Linux dependencies",
                vec!["-c", install_cmd.as_str()],
                None,
                false,
            )?;
        }

        // install bencher
        // linux curl --proto '=https' --tlsv1.2 -sSfL https://bencher.dev/download/install-cli.sh | sh
        // windows irm https://bencher.dev/download/install-cli.ps1 | iex
        if wants(InitTool::Bencher) {
            // no `--force`: with the cargo bin directory cached this is a no-op unless upstream changed
            run_system_command(
                &app_settings,
                "cargo",
                "Failed to install bencher",
                vec![
                    "install",
                    "--git",
                    "https://github.com/bencherdev/bencher",
                    "--branch",
                    "main",
                    "--locked",
                    "bencher_cli",
                ],
                None,
                false,
            )?;
        }

        if wants(InitTool::Mdbook) {
            // install cargo mdbook
            run_system_command(
                &app_settings,
                "cargo",
                "Failed to install mdbook",
                vec!["install", "mdbook"],
                None,
                false,
            )?;

            // install mdbook-mermaid
            run_system_command(
                &app_settings,
                "cargo",
                "Failed to install mdbook-mermaid",
                vec!["install", "mdbook-mermaid"],
                None,
                false,
            )?;
        }

        let rustup_components_args = [
            "component",
            "add",
            "rust-src",
            "rustc-dev",
            "clippy",
            "rustfmt",
        ];

        // install components for the stable toolchain
        run_system_command(
            &app_settings,
            "rustup",
            "Failed to install rust components",
            rustup_components_args,
            None,
            false,
        )?;

        // create .vscode settings
        // read from templates at compile time
        if !dont_update_ide {
            let vscode_settings = include_str!("../templates/settings.json.tera");
            let mut tera = tera::Tera::default();
            let mut context = tera::Context::new();
            let workspace_dir = workspace_dir(&app_settings)?;
            let json_workspace_dir = serde_json::to_string(&workspace_dir)?; // make sure this works as a json string
            context.insert("dir", &json_workspace_dir.trim_matches('\"'));

            let templated_settings = tera.render_str(vscode_settings, &context)?;
            let templated_settings_json =
                Self::read_json_with_comments(templated_settings.as_bytes())
                    .with_context(|| "reading templated vscode settings")?;
            let vscode_dir = relative_workspace_dir(&app_settings, ".vscode")?;
            std::fs::create_dir_all(&vscode_dir)?;
            let vscode_settings_path = vscode_dir.join("settings.json");

            // if the file already exists, merge the settings otherwise create it
            info!(
                "Merging vscode settings at {vscode_settings_path:?}. With overrides generated by template."
            );
            if vscode_settings_path.exists() {
                let existing_settings = std::fs::read_to_string(&vscode_settings_path)?;
                let mut existing_settings =
                    Self::read_json_with_comments(existing_settings.as_bytes())
                        .with_context(|| "reading existing vscode settings file")?;
                Self::merge_json(templated_settings_json, &mut existing_settings);
                let merged_settings = serde_json::to_string_pretty(&existing_settings)?;
                std::fs::write(&vscode_settings_path, merged_settings)?;
            } else {
                std::fs::write(&vscode_settings_path, templated_settings)?;
            }
        }

        Ok(())
    }

    fn read_json_with_comments(bytes: &[u8]) -> Result<serde_json::Value> {
        let stripped = StripComments::new(bytes);
        let mut reader = serde_json::Deserializer::from_reader(stripped);
        let value = serde_json::Value::deserialize(&mut reader)
            .with_context(|| format!("deserializing json:\n{}", String::from_utf8_lossy(bytes)))?;
        Ok(value)
    }

    /// Override the target json file with some overrides. Will replace values if they already exist, or insert them otherwise.
    fn merge_json(overrides: serde_json::Value, target: &mut serde_json::Value) {
        if let (serde_json::Value::Object(overrides), serde_json::Value::Object(target)) =
            (overrides, target)
        {
            for (key, value) in overrides {
                // simply replace
                info!("Replacing json key: {key} with value: {value}");
                target.insert(key.clone(), value.clone());
            }
        } else {
            warn!("Could not merge json, overrides and target are not objects");
        }
    }

    fn example(app_settings: GlobalArgs, example: String) -> std::result::Result<(), Error> {
        Self::example_with_args(app_settings, &example, &[])
    }

    /// Runs the example, passing it the given arguments
    fn example_with_args(
        app_settings: GlobalArgs,
        example: &str,
        example_args: &[&str],
    ) -> std::result::Result<(), Error> {
        // find the required features for the example named this in the cargo.toml of the main workspace
        // the keys look like
        // [[example]]
        // name = "docgen"
        // path = "examples/docgen.rs"
        // required-features = []

        // let metadata = main_workspace_cargo_metadata()?;
        // let metadata = &metadata.root_package().unwrap().targets;
        // println!("{metadata:#?}");

        // run the example
        let mut args = vec!["--example", example];
        if !example_args.is_empty() {
            args.push("--");
            args.extend(example_args);
        }
        run_workspace_command(
            &app_settings,
            "run",
            "Failed to run example",
            args,
            None,
            false,
        )?;

        Ok(())
    }

    fn install(app_settings: GlobalArgs, binary: Binary) -> std::result::Result<(), Error> {
        // run cargo install --path
        let binary_path = relative_workspace_dir(&app_settings, binary.path())?;
        run_system_command(
            &app_settings,
            "cargo",
            "Failed to install binary",
            vec!["install", "--path", binary_path.to_str().unwrap()],
            None,
            false,
        )?;

        Ok(())
    }
}

/// The features the docs are generated with.
///
/// The docgen example documents whatever bindings are compiled in, so it needs all of them,
/// the default features only cover a few of the bevy crates.
fn docs_features() -> Features {
    Features::all_features()
}

/// The directory of the NDK's LLVM toolchain, which is named after the host (i.e. `linux-x86_64`)
fn android_ndk_toolchain() -> Option<PathBuf> {
    let ndk = [
        "ANDROID_NDK_HOME",
        "ANDROID_NDK_LATEST_HOME",
        "ANDROID_NDK_ROOT",
    ]
    .into_iter()
    .find_map(std::env::var_os)?;

    let prebuilt = PathBuf::from(ndk).join("toolchains/llvm/prebuilt");
    Some(
        std::fs::read_dir(prebuilt)
            .ok()?
            .find_map(|e| e.ok())?
            .path(),
    )
}

/// cc-rs can't find a compiler for android targets by itself, this points it (and the linker) at the clang shipped with the NDK.
fn android_ndk_env(target: &str) -> Vec<(String, PathBuf)> {
    const API_LEVEL: u32 = 24;

    let Some(bin) = android_ndk_toolchain().map(|t| t.join("bin")) else {
        warn!("No Android NDK found (ANDROID_NDK_HOME), relying on the compilers being in PATH");
        return vec![];
    };

    let target_env = target.replace('-', "_");
    let clang = bin.join(format!("{target}{API_LEVEL}-clang"));
    vec![
        (format!("CC_{target_env}"), clang.clone()),
        (
            format!("CXX_{target_env}"),
            bin.join(format!("{target}{API_LEVEL}-clang++")),
        ),
        (format!("AR_{target_env}"), bin.join("llvm-ar")),
        (
            format!("CARGO_TARGET_{}_LINKER", target_env.to_uppercase()),
            clang,
        ),
    ]
}

/// Because we are likely already runnnig in the context of a cargo invocation,
/// some environment variables may be set that we don't want to inherit.
/// Set them to MAIN_CARGO_<VARIABLE> so that we can reference them but so they dont get inherited by further cargo commands
fn pop_cargo_env() -> Result<()> {
    let env = std::env::vars().collect::<Vec<_>>();
    let exclude_list = ["CARGO_HOME"];
    let include_list = []; //"LD_LIBRARY_PATH"

    for (key, value) in env.iter() {
        let key_str = &(key.as_str());
        if (include_list.contains(key_str) || key.starts_with("CARGO_"))
            && !exclude_list.contains(key_str)
        {
            let new_key = format!("MAIN_{key}");
            unsafe { std::env::set_var(new_key, value) };
            unsafe { std::env::remove_var(key) };
        }
    }

    // unset some other variables
    let remove_vars = ["RUSTUP_TOOLCHAIN"];
    for var in remove_vars.iter() {
        if exclude_list.contains(var) {
            continue;
        }
        unsafe { std::env::remove_var(var) };
    }

    Ok(())
}

fn try_main() -> Result<()> {
    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Info)
        .init();

    pop_cargo_env()?;

    let args = App::try_parse()?;
    info!(
        "Default toolchain: {:?}",
        args.global_args.override_toolchain
    );

    let out = args.subcmd.run(args.global_args)?;
    // push any output to stdout
    if !out.is_empty() {
        std::io::stdout().write_all(out.as_bytes())?;
        println!();
    }
    Ok(())
}

const XTASK_KEEP_GOING: &str = "XTASK_KEEP_GOING";

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{e:?}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_init_tools(args: &[&str]) -> Option<Vec<InitTool>> {
        let app = App::try_parse_from(["xtask", "init"].iter().chain(args)).unwrap();
        match app.subcmd {
            Xtasks::Init { tools, .. } => tools,
            other => panic!("expected init, got {other:?}"),
        }
    }

    #[test]
    fn init_installs_all_tools_by_default() {
        assert_eq!(parse_init_tools(&[]), None);
    }

    #[test]
    fn docs_document_every_binding() {
        let features = docs_features();

        for feature in <Feature as strum::VariantArray>::VARIANTS {
            if feature.to_string().ends_with("_bindings") {
                assert!(features.0.contains(feature), "{feature} is not documented");
            }
        }

        // these are not part of the default features
        assert!(features.0.contains(&Feature::BevyAssetBindings));
        // and the files are only generated with this one
        assert!(features.0.contains(&Feature::LuaLanguageServerFiles));
    }

    #[test]
    fn init_tools_flag_without_values_installs_none() {
        assert_eq!(parse_init_tools(&["--tools"]), Some(vec![]));
        assert_eq!(
            parse_init_tools(&["--dont-update-ide", "--tools"]),
            Some(vec![])
        );
    }

    #[test]
    fn init_tools_flag_accepts_a_list() {
        assert_eq!(
            parse_init_tools(&["--tools", "bencher,mdbook"]),
            Some(vec![InitTool::Bencher, InitTool::Mdbook])
        );
    }

    #[test]
    fn ci_init_args_round_trip() {
        for tool in <InitTool as strum::VariantArray>::VARIANTS {
            let args = format!("--tools {}", tool.as_ref());
            let args = args.split(' ').collect::<Vec<_>>();
            assert_eq!(parse_init_tools(&args), Some(vec![*tool]));
        }
    }

    #[test]
    fn docs_can_include_the_test_bindings() {
        let test_bindings = |args: &[&str]| {
            let app = App::try_parse_from(["xtask", "docs"].iter().chain(args)).unwrap();
            match app.subcmd {
                Xtasks::Docs { test_bindings, .. } => test_bindings,
                other => panic!("expected docs, got {other:?}"),
            }
        };
        assert!(!test_bindings(&[]));
        assert!(test_bindings(&["--test-bindings"]));

        // the flag survives being passed on to another invocation, as the CI matrix does
        let app = App::try_parse_from(["xtask", "docs", "--test-bindings"]).unwrap();
        let command = app.into_command_string();
        assert!(command.to_string_lossy().contains("--test-bindings"));
    }

    #[test]
    fn docgen_is_part_of_docs() {
        assert!(App::try_parse_from(["xtask", "docgen"]).is_err());
    }

    #[test]
    fn mobile_checks_run_on_every_matrix_size() {
        let mobile_rows = |full: bool| {
            let args = ["xtask", "ci-matrix"]
                .into_iter()
                .chain(full.then_some("--full"));
            let app = App::try_parse_from(args).unwrap();
            let json = app.subcmd.run(app.global_args).unwrap();
            let rows = serde_json::from_str::<Vec<serde_json::Value>>(&json).unwrap();
            rows.into_iter()
                .filter(|r| {
                    r["command"]
                        .as_str()
                        .is_some_and(|c| c.contains("--mobile"))
                })
                .collect::<Vec<_>>()
        };

        // pull requests check each platform once, on the OS providing its toolchain
        let pr_rows = mobile_rows(false);
        let platforms = pr_rows
            .iter()
            .map(|r| r["os"].as_str().unwrap().to_owned())
            .sorted()
            .collect::<Vec<_>>();
        assert_eq!(platforms, ["macos-latest", "ubuntu-latest"]);

        // and the full matrix adds the individual languages
        assert!(mobile_rows(true).len() > pr_rows.len());
    }

    #[test]
    fn device_tests_only_run_when_asked_for() {
        let device_rows = |flag: Option<&str>| {
            let args = ["xtask", "ci-matrix"].into_iter().chain(flag);
            let app = App::try_parse_from(args).unwrap();
            let json = app.subcmd.run(app.global_args).unwrap();
            let rows = serde_json::from_str::<Vec<serde_json::Value>>(&json).unwrap();
            rows.into_iter()
                .filter(|r| {
                    r["command"]
                        .as_str()
                        .is_some_and(|c| c.contains(" test --mobile"))
                })
                .collect::<Vec<_>>()
        };

        assert!(device_rows(None).is_empty());
        let rows = device_rows(Some("--device-tests"));
        // all features + luajit + luau on android, all features + luau on ios
        assert_eq!(rows.len(), 5);
        // only the android ones need the emulator
        assert_eq!(
            rows.iter()
                .filter(|r| r["requires_android_emulator"] == true)
                .count(),
            3
        );
        assert_eq!(device_rows(Some("--full")).len(), rows.len());
    }

    #[test]
    fn mobile_flag_survives_being_passed_on() {
        let app = App::try_parse_from(["xtask", "check", "--mobile", "ios"]).unwrap();
        assert_eq!(app.subcmd.mobile_os(), Some(CiOs::Macos));
        assert!(
            app.into_command_string()
                .to_string_lossy()
                .ends_with("--mobile ios")
        );
    }
}
