use bevy_mod_scripting_core::{bindings::{pretty_print::DisplayWithWorld, ThreadWorldContainer, WorldContainer}, error::ScriptError, AddContextInitializer};
use bevy_mod_scripting_lua::LuaScriptingPlugin;
use libtest_mimic::{Arguments, Failed, Trial};
use mlua::{Function, Lua, MultiValue};
use script_integration_test_harness::execute_integration_test;
use std::{
    fs::{self, DirEntry},
    io, panic,
    path::{Path, PathBuf},
};

struct Test {
    code: String,
    path: PathBuf,
}

impl Test {
    fn execute(self) -> Result<(), Failed> {
        execute_integration_test::<LuaScriptingPlugin, _, _>(
            |world, type_registry| {
                let _ = world;
                let _ = type_registry;
            },
            |app| {
                app.add_plugins(LuaScriptingPlugin::default());
                app.add_context_initializer::<LuaScriptingPlugin>(|_,ctxt: &mut Lua| {
                    let globals = ctxt.globals();
                    globals.set(
                        "assert_throws",
                        ctxt.create_function(|_lua, (f, reg): (Function, String)| {
                            let world =  ThreadWorldContainer.get_world();
                            let result = f.call::<()>(MultiValue::new());
                            let err = match result {
                                Ok(_) => {
                                    return Err(mlua::Error::external(
                                        "Expected function to throw error, but it did not.",
                                    ))
                                }
                                Err(e) => 
                                    ScriptError::from_mlua_error(e).display_with_world(world)
                                ,
                            };
            
                            let regex = regex::Regex::new(&reg).unwrap();
                            if regex.is_match(&err) {
                                Ok(())
                            } else {
                                Err(mlua::Error::external(
                                    format!(
                                        "Expected error message to match the regex: \n{}\n\nBut got:\n{}",
                                        regex.as_str(),
                                        err
                                    ),
                                ))
                            }
                        })?,
                    )?;
                    Ok(())
                });
            },
            self.path.as_os_str().to_str().unwrap(),
            self.code.as_bytes(),
        )
        .map_err(Failed::from)
    }

    fn name(&self) -> String {
        format!(
            "script_test - lua - {}",
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
