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
    pub path: PathBuf,
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
            let test = Test {
                path: relative.to_path_buf(),
                kind,
                scenario_path: relative
                    .parent()
                    .and_then(|p| p.join("scenario.json").to_str().map(PathBuf::from))
                    .filter(|p| p.exists()),
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
