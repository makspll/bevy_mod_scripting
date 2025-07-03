#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, missing_docs)]

use std::path::PathBuf;

use libtest_mimic::{Arguments, Failed, Trial};
use script_integration_test_harness::{
    execute_lua_integration_test,
};

use test_utils::{discover_all_tests, Test, TestKind};

trait TestExecutor {
    fn execute(self) -> Result<(), Failed>;
    fn name(&self) -> String;
}

impl TestExecutor for Test {
    fn execute(self) -> Result<(), Failed> {

        match self.kind {
            TestKind::Lua => {
                println!("Running test: {:?}", self.path);
                execute_lua_integration_test(&self.path.to_string_lossy())?
            },
            TestKind::Rhai => {
                if cfg!(feature = "rhai") {
                    println!("Running test: {:?}", self.path);
                    #[cfg(feature = "rhai")]
                    script_integration_test_harness::execute_rhai_integration_test(&self.path.to_string_lossy())?
                } else {
                    println!("Skipping test: {:?}", self.path);
                }
            },

        }

        Ok(())
    }

    fn name(&self) -> String {
        format!(
            "script_test - {} - {}",
            self.kind,
            self.path
                .to_string_lossy()
                .split(&format!("tests{}data", std::path::MAIN_SEPARATOR))
                .last()
                .unwrap()
        )
    }
}

// run this with `cargo test --features lua54 --package bevy_mod_scripting_lua --test lua_tests`
// or filter using the prefix "lua test -"
fn main() {
    // Parse command line arguments
    let args = Arguments::from_args();
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let tests = discover_all_tests(manifest_dir, |p| p.path.starts_with("tests"))
        .into_iter()
        .map(|t| Trial::test(t.name(), move || t.execute()))
        .collect::<Vec<_>>();

    libtest_mimic::run(&args, tests).exit();
}
