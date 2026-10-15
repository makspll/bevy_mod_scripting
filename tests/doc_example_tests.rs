#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, missing_docs)]
//! Runs the `multi-lang-example` code blocks of the doc comments under `crates/` and of the book as script tests.
//!
//! Each block is written to `assets/doc_examples/generated/` and run with the scenario named by its `scenario=<name>`
//! attribute, i.e. `assets/doc_examples/<name>.bmsscenario`, or `default`. Blocks with the `ignore` attribute are skipped.
//! See the "Writing Bindings" page of the book.

use std::{
    fs,
    path::{Path, PathBuf},
};

use libtest_mimic::{Arguments, Failed, Trial};
use script_integration_test_harness::{execute_integration_test, scenario::Scenario};

/// Where the example scripts are written to, relative to the assets folder.
const GENERATED_DIR: &str = "doc_examples/generated";

/// The categories of the bevy bindings crates, and whether their feature is enabled.
/// Examples of disabled ones are ignored.
const BINDINGS: &[(&str, bool)] = &[
    ("animation", cfg!(feature = "bevy_animation_bindings")),
    ("asset", cfg!(feature = "bevy_asset_bindings")),
    ("audio", cfg!(feature = "bevy_audio_bindings")),
    ("camera", cfg!(feature = "bevy_camera_bindings")),
    ("color", cfg!(feature = "bevy_color_bindings")),
    ("ecs", cfg!(feature = "bevy_ecs_bindings")),
    ("gizmos", cfg!(feature = "bevy_gizmos_bindings")),
    ("input", cfg!(feature = "bevy_input_bindings")),
    ("math", cfg!(feature = "bevy_math_bindings")),
    ("render", cfg!(feature = "bevy_render_bindings")),
    ("sprite", cfg!(feature = "bevy_sprite_bindings")),
    ("state", cfg!(feature = "bevy_state_bindings")),
    ("text", cfg!(feature = "bevy_text_bindings")),
    ("time", cfg!(feature = "bevy_time_bindings")),
    ("transform", cfg!(feature = "bevy_transform_bindings")),
    ("ui", cfg!(feature = "bevy_ui_bindings")),
    ("window", cfg!(feature = "bevy_window_bindings")),
];

/// A `multi-lang-example` code block.
struct Example {
    /// The file containing the block, relative to the workspace root
    source: PathBuf,
    /// The line of the opening fence, starting at 1
    line: usize,
    /// The script file extension of the block's language, i.e. `lua`
    language: String,
    /// The scenario to run the block with
    scenario: String,
    ignore: bool,
    code: String,
}

impl Example {
    /// Parses the info string after "```multi-lang-example,", i.e. `lua,scenario=window`.
    fn new(source: &Path, line: usize, info: &str) -> Self {
        let mut attributes = info.split(',').map(str::trim);
        let language = attributes.next().unwrap_or_default().to_owned();
        let mut example = Self {
            source: source.to_path_buf(),
            line,
            language,
            scenario: "default".to_owned(),
            ignore: false,
            code: String::new(),
        };
        for attribute in attributes {
            match attribute.split_once('=') {
                Some(("scenario", scenario)) => example.scenario = scenario.to_owned(),
                None if attribute == "ignore" => example.ignore = true,
                _ => panic!(
                    "{}:{line}: unknown multi-lang-example attribute `{attribute}`",
                    source.display()
                ),
            }
        }
        example
    }

    fn name(&self) -> String {
        format!(
            "doc_example - {} - {}:{}",
            self.language,
            self.source.display(),
            self.line
        )
    }

    /// The path of the generated script, relative to the assets folder.
    fn script_path(&self) -> PathBuf {
        let file = format!(
            "{}_L{}.{}",
            self.source.to_string_lossy().replace(['/', '\\', '.'], "_"),
            self.line,
            self.language
        );
        Path::new(GENERATED_DIR).join(file)
    }

    /// Why the example can't run in this build, if it can't.
    fn disabled_reason(&self) -> Option<String> {
        let language_enabled = match self.language.as_str() {
            "lua" => cfg!(feature = "lua"),
            "rhai" => cfg!(feature = "rhai"),
            _ => return Some(format!("unsupported language `{}`", self.language)),
        };
        if !language_enabled {
            return Some(format!("the {} feature is disabled", self.language));
        }
        let crate_name = self
            .source
            .components()
            .nth(2)
            .map(|c| c.as_os_str().to_string_lossy().into_owned())
            .unwrap_or_default();
        let category = crate_name
            .strip_prefix("bevy_mod_scripting_bevy_")
            .and_then(|c| c.strip_suffix("_bindings"))?;
        match BINDINGS.iter().find(|(name, _)| *name == category) {
            Some((_, true)) => None,
            _ => Some(format!("the bevy_{category}_bindings feature is disabled")),
        }
    }
}

/// Finds the `multi-lang-example` blocks in a rust file's doc comments, or in a markdown file.
fn extract_examples(source: &Path, content: &str) -> Vec<Example> {
    let is_rust = source.extension().is_some_and(|e| e == "rs");
    let mut examples = Vec::new();
    let mut current: Option<Example> = None;
    for (index, line) in content.lines().enumerate() {
        let text = if is_rust {
            let trimmed = line.trim_start();
            match trimmed
                .strip_prefix("///")
                .or_else(|| trimmed.strip_prefix("//!"))
            {
                Some(doc) => doc.strip_prefix(' ').unwrap_or(doc),
                None => {
                    if let Some(example) = &current {
                        panic!("{}: unterminated code block", example.name());
                    }
                    continue;
                }
            }
        } else {
            line
        };
        let fence = text.trim();
        match current.as_mut() {
            Some(_) if fence == "```" => examples.extend(current.take()),
            Some(example) => {
                example.code.push_str(text);
                example.code.push('\n');
            }
            None => {
                if let Some(info) = fence.strip_prefix("```multi-lang-example,") {
                    current = Some(Example::new(source, index + 1, info));
                }
            }
        }
    }
    examples
}

/// Visits the files under `dir` with the given extension, skipping build outputs.
fn visit_files(dir: &Path, extension: &str, visit: &mut impl FnMut(&Path)) {
    let mut entries = fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect::<Vec<_>>();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            if path.file_name().is_some_and(|n| n != "target") {
                visit_files(&path, extension, visit);
            }
        } else if path.extension().is_some_and(|e| e == extension) {
            visit(&path);
        }
    }
}

fn run(example: &Example, manifest_dir: &Path) -> Result<(), Failed> {
    let scenario_path = manifest_dir
        .join("assets/doc_examples")
        .join(format!("{}.bmsscenario", example.scenario));
    let scenario = Scenario::from_scenario_file(&example.script_path(), &scenario_path)
        .map_err(|e| format!("{e:?}"))?;
    Ok(execute_integration_test(scenario)?)
}

fn main() {
    let mut args = Arguments::from_args();
    // the script tests use thread locals
    args.test_threads = Some(1);
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let mut examples = Vec::new();
    for (dir, extension) in [("crates", "rs"), ("docs/src", "md")] {
        visit_files(&manifest_dir.join(dir), extension, &mut |path| {
            let source = path.strip_prefix(&manifest_dir).unwrap();
            let content = fs::read_to_string(path).unwrap();
            examples.extend(extract_examples(source, &content));
        });
    }

    let generated = manifest_dir.join("assets").join(GENERATED_DIR);
    let _ = fs::remove_dir_all(&generated);
    fs::create_dir_all(&generated).unwrap();
    let tests = examples
        .into_iter()
        .map(|example| {
            let disabled = example.disabled_reason();
            if disabled.is_none() && !example.ignore {
                fs::write(
                    manifest_dir.join("assets").join(example.script_path()),
                    &example.code,
                )
                .unwrap();
            }
            let name = example.name();
            let ignored = example.ignore || disabled.is_some();
            let manifest_dir = manifest_dir.clone();
            Trial::test(name, move || run(&example, &manifest_dir)).with_ignored_flag(ignored)
        })
        .collect::<Vec<_>>();

    libtest_mimic::run(&args, tests).exit();
}
