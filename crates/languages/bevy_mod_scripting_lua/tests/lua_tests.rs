use bevy::{
    app::App,
    asset::{AssetPlugin, AssetServer},
    prelude::{AppTypeRegistry, Entity, World},
    MinimalPlugins,
};
use bevy_mod_scripting_core::{
    bindings::{Proxy, ReflectAllocator, ReflectReference, ReflectValProxy, WorldCallbackAccess},
    context::ContextLoadingSettings,
    error::ScriptError,
    event::CallbackLabel,
    script::ScriptId,
};
use bevy_mod_scripting_lua::{
    bindings::{
        providers::bevy_ecs::LuaEntity,
        proxy::{LuaProxied, LuaReflectValProxy},
        world::{GetWorld, LuaWorld},
    },
    lua_context_load, lua_handler,
    prelude::{Lua, LuaHookTriggers},
    register_lua_values, LuaScriptingPlugin, ReflectLuaValue,
};
use libtest_mimic::{Arguments, Failed, Trial};
use std::{
    any::TypeId,
    borrow::Cow,
    fs::{self, DirEntry},
    io,
    path::{Path, PathBuf},
};
use test_utils::test_data::{setup_world, EnumerateTestComponents};

/// Initializes world for tests
fn init_app() -> App {
    let mut app = App::new();

    let world = setup_world(|_, _| {});

    *app.world_mut() = world;

    // we probably should cut down some fat in here, but it's fast enough so meh
    app.add_plugins(AssetPlugin::default())
        .add_plugins(LuaScriptingPlugin::<()>::default())
        .add_plugins(bevy_mod_scripting_lua::bindings::providers::LuaBevyScriptingPlugin);

    app
}

fn init_lua_test_utils(_script_name: &Cow<'static, str>, lua: &mut Lua) -> Result<(), ScriptError> {
    let _get_entity_with_test_component = lua
        .create_function(|l, s: String| {
            let world = l.get_world().unwrap();
            let opt_entity = world.with_resource::<ReflectAllocator, _, _>(|_, mut allocator| {
                let a = World::enumerate_test_components()
                    .iter()
                    .find(|(name, _, _)| name.contains(&s))
                    .map(|(_, _, c)| {
                        let reference = ReflectReference::new_allocated(
                            c.unwrap_or(Entity::from_raw(9999)),
                            &mut allocator,
                        );
                        <<Entity as LuaProxied>::Proxy>::from(reference)
                    });

                a
            });

            Ok(opt_entity)
        })
        .unwrap();

    lua.globals()
        .set(
            "_get_entity_with_test_component",
            _get_entity_with_test_component,
        )
        .unwrap();

    Ok(())
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
        let mut context_settings: ContextLoadingSettings<Lua, ()> = app
            .world_mut()
            .remove_resource()
            .ok_or("could not find context loading settings")?;
        context_settings
            .context_initializers
            .push(init_lua_test_utils);

        let mut lua = lua_context_load(
            &(self.name()).into(),
            self.code.as_bytes(),
            &context_settings.context_initializers,
            &context_settings.context_pre_handling_initializers,
            app.world_mut(),
            &mut (),
        )?;

        lua_handler(
            (),
            Entity::from_raw(1),
            &(self.name()).into(),
            &CallbackLabel::new("test").ok_or("invalid callback label")?,
            &mut lua,
            &context_settings.context_pre_handling_initializers,
            &mut (),
            app.world_mut(),
        )?;

        // WorldCallbackAccess::with_callback_access(app.world_mut(), |world| {
        //     lua.globals().set("world", LuaWorld(world.clone())).unwrap();

        //     let code = lua.load(self.code).set_name(self.path.to_string_lossy());
        //     code.exec().map_err(|e| e.to_string())
        // })?;
        Ok(())
    }

    fn name(&self) -> String {
        format!("lua_test - {}", self.path.to_string_lossy())
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
