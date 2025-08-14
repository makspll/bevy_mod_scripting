use std::{
    fs::{self, DirEntry},
    io,
    path::{Path, PathBuf},
};

pub mod test_data;
pub mod test_plugin;

#[derive(Debug, Clone, Copy)]
pub enum TestKind {
    Lua,
    Rhai,
}

impl std::fmt::Display for TestKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestKind::Lua => write!(f, "Lua"),
            TestKind::Rhai => write!(f, "Rhai"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Test {
    pub script_asset_path: PathBuf,
    pub kind: TestKind,
    /// If the test contains an explicit scenario, this will be set.
    pub scenario_path: Option<PathBuf>,
}

fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    } else {
        panic!("Not a directory: {dir:?}");
    }
    Ok(())
}

/// searches for the nearest ancestor of the given path that satisfies the condition.
/// stops the search upon reaching the manifest path.
fn find_nearest_ancestor(path: &Path, condition: impl Fn(&Path) -> bool) -> Option<PathBuf> {
    // check path is within the manifest path
    let manifest_path = std::env::var("CARGO_MANIFEST_DIR")
        .ok()
        .map(PathBuf::from)
        .unwrap();

    let manifest_path_ancestors = path
        .ancestors()
        .filter(|p| p.starts_with(&manifest_path) && p.is_dir())
        .collect::<Vec<_>>();

    for ancestor in manifest_path_ancestors {
        let siblings = fs::read_dir(ancestor).ok()?;
        let entries = siblings.filter_map(Result::ok).collect::<Vec<_>>();
        for entry in entries {
            let entry_path = entry.path();
            if entry_path.is_file() && condition(&entry_path) {
                return Some(entry_path);
            }
        }
    }

    None
}

pub fn discover_all_tests(manifest_dir: PathBuf, filter: impl Fn(&Test) -> bool) -> Vec<Test> {
    let assets_root = manifest_dir.join("assets");
    let mut test_files = Vec::new();
    visit_dirs(&assets_root, &mut |entry| {
        let path = entry.path();
        if let Some(kind) = path
            .extension()
            .and_then(|e| match e.to_string_lossy().as_ref() {
                "lua" => Some(TestKind::Lua),
                "rhai" => Some(TestKind::Rhai),
                _ => None,
            })
        {
            // only take the path from the assets  bit
            let relative = path.strip_prefix(&assets_root).unwrap();

            let scenario_path = find_nearest_ancestor(&path, |p| {
                p.file_name()
                    .and_then(|f| f.to_str())
                    .is_some_and(|p| p == "scenario.txt" || p == "group_scenario.txt")
            });

            // if the scenario has a `// #main_script filename` line, check if this script is the main script in the scenario
            // if not ignore it. we only want to run against the main script in the scenario.
            let main_script_path = scenario_path.as_ref().and_then(|scenario| {
                let scenario_content = fs::read_to_string(scenario).unwrap_or_default();
                scenario_content.lines().find_map(|line| {
                    let main_script_line = line.contains("#main_script");
                    if !main_script_line {
                        return None;
                    }

                    let main_script_path = line
                        .split_once("#main_script ")
                        .map(|(_, main_script_path)| main_script_path.trim())
                        .unwrap_or_default();
                    let main_script_path = PathBuf::from(main_script_path);
                    Some(main_script_path)
                })
            });

            let is_main_script_in_scenario =
                main_script_path.as_ref().is_none_or(|main_script_path| {
                    relative.file_name() == main_script_path.file_name()
                });

            if !is_main_script_in_scenario {
                return;
            }

            let test = Test {
                script_asset_path: relative.to_path_buf(),
                kind,
                scenario_path,
            };

            if !filter(&test) {
                return;
            }
            test_files.push(test);
        }
    })
    .unwrap();

    test_files
}
