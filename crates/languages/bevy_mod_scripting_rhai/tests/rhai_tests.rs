use bevy::{
    app::App,
    asset::AssetPlugin,
    prelude::{Children, Entity, HierarchyPlugin, Parent, World},
    reflect::{Reflect, TypeRegistration},
};
use bevy_mod_scripting_core::{
    bindings::{
        access_map::ReflectAccessId, pretty_print::DisplayWithWorld, script_value::ScriptValue,
        ReflectReference, ScriptTypeRegistration, WorldAccessGuard,
    },
    context::ContextLoadingSettings,
    error::ScriptError,
    event::CallbackLabel,
};
use bevy_mod_scripting_functions::ScriptFunctionsPlugin;
use bevy_mod_scripting_rhai::{RhaiScriptContext, RhaiScriptingPlugin};
use libtest_mimic::{Arguments, Failed, Trial};
use rhai::Engine;
use std::{
    fs::{self, DirEntry},
    io, panic,
    path::{Path, PathBuf},
    sync::Arc,
};
use test_utils::test_data::{setup_integration_test, setup_world, EnumerateTestComponents};

/// Initializes world for tests
fn init_app() -> App {
    let mut app = setup_integration_test(|_, _| {});

    app.add_plugins(RhaiScriptingPlugin::default())
        .add_plugins(ScriptFunctionsPlugin);

    app.finish();
    app.cleanup();

    app
}

struct Test {
    code: String,
    path: PathBuf,
}

impl Test {
    fn execute(self) -> Result<(), Failed> {
        // let lua = Lua::new();
        // set file information
        let mut app = init_app();
        app.add_systems(schedule, systems)

        // let mut lua = lua_context_load(
        //     &(self.name()).into(),
        //     self.code.as_bytes(),
        //     &context_settings.context_initializers,
        //     &context_settings.context_pre_handling_initializers,
        //     app.world_mut(),
        //     &mut (),
        // )
        // .map_err(|e| {
        //     let world = app.world_mut();
        //     let world = WorldAccessGuard::new(world);
        //     let msg = e.display_with_world(Arc::new(world));
        //     Failed::from(msg)
        // })?;

        // lua_handler(
        //     vec![ScriptValue::Unit],
        //     Entity::from_raw(1),
        //     &(self.name()).into(),
        //     &CallbackLabel::new("on_test").ok_or("invalid callback label")?,
        //     &mut lua,
        //     &context_settings.context_pre_handling_initializers,
        //     &mut (),
        //     app.world_mut(),
        // )
        // .map_err(|e| {
        //     let world = app.world_mut();
        //     let world = WorldAccessGuard::new(world);
        //     let msg = e.display_with_world(Arc::new(world));
        //     Failed::from(msg)
        // })?;

        Ok(())
    }

    fn name(&self) -> String {
        format!(
            "lua_test - {}",
            self.path
                .to_string_lossy()
                .split(&format!("tests{}data", std::path::MAIN_SEPARATOR))
                .last()
                .unwrap()
        )
    }
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
        panic!("Not a directory: {:?}", dir);
    }
    Ok(())
}

fn discover_all_tests() -> Vec<Test> {
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let test_root = workspace_root.join("tests").join("data");
    let mut test_files = Vec::new();
    visit_dirs(&test_root, &mut |entry| {
        let path = entry.path();
        let code = fs::read_to_string(&path).unwrap();
        test_files.push(Test { code, path });
    })
    .unwrap();

    test_files
}

// run this with `cargo test --features lua54 --package bevy_mod_scripting_lua --test lua_tests`
// or filter using the prefix "lua test -"
fn main() {
    // Parse command line arguments
    let args = Arguments::from_args();

    // Create a list of tests and/or benchmarks (in this case: two dummy tests).
    let tests = discover_all_tests()
        .into_iter()
        .map(|t| Trial::test(t.name(), move || t.execute()));

    // Run all tests and exit the application appropriatly.
    libtest_mimic::run(&args, tests.collect()).exit();
}
