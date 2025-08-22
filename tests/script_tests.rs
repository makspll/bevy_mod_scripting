#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, missing_docs)]

use std::path::PathBuf;

use libtest_mimic::{Arguments, Failed, Trial};
use script_integration_test_harness::{execute_integration_test, scenario::Scenario};
use test_utils::{Test, discover_all_tests};

trait TestExecutor {
    fn execute(self) -> Result<(), Failed>;
    fn name(&self) -> String;
}

impl TestExecutor for Test {
    fn execute(self) -> Result<(), Failed> {
        let script_asset_path = self.script_asset_path;
        let scenario_path = self.scenario_path.ok_or_else(|| {
            Failed::from("Test does not have a scenario.txt file near to use for test".to_string())
        })?;
        println!(
            "Running test: {}, with scenario: {}",
            script_asset_path.display(),
            scenario_path.display()
        );

        let scenario = Scenario::from_scenario_file(&script_asset_path, &scenario_path)
            .map_err(|e| format!("{e:?}"))?; // print whole error from anyhow including source and backtrace

        execute_integration_test(scenario)?;

        Ok(())
    }

    fn name(&self) -> String {
        format!(
            "script_test - {} - {}",
            self.kind,
            self.script_asset_path
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

    let tests = discover_all_tests(manifest_dir, |p| p.script_asset_path.starts_with("tests"))
        .into_iter()
        .map(|t| Trial::test(t.name(), move || t.execute()))
        .collect::<Vec<_>>();

    libtest_mimic::run(&args, tests).exit();
}
