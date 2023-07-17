use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::*;
use bevy_mod_scripting::prelude::*;
use rand::prelude::SliceRandom;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{atomic::AtomicU32, Mutex};

#[derive(Clone)]
pub struct MyLuaArg(usize);

impl<'lua> ToLua<'lua> for MyLuaArg {
    fn to_lua(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        self.0.to_lua(lua)
    }
}

#[derive(Default)]
pub struct LuaAPIProvider;

/// the custom Lua api, world is provided via a global pointer,
/// and callbacks are defined only once at script creation
impl APIProvider for LuaAPIProvider {
    type APITarget = Mutex<Lua>;
    type DocTarget = LuaDocFragment;
    type ScriptContext = Mutex<Lua>;

    fn attach_api(&mut self, ctx: &mut Self::APITarget) -> Result<(), ScriptError> {
        // callbacks can receive any `ToLuaMulti` arguments, here '()' and
        // return any `FromLuaMulti` arguments, here a `usize`
        // check the Rlua documentation for more details

        let ctx = ctx.get_mut().unwrap();

        ctx.globals()
            .set(
                "print",
                ctx.create_function(|_ctx, msg: String| {
                    info!("{}", msg);
                    Ok(())
                })
                .map_err(ScriptError::new_other)?,
            )
            .map_err(ScriptError::new_other)?;

        Ok(())
    }
}

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// utility for generating random events from a list
fn fire_random_event(
    w: &mut PriorityEventWriter<LuaEvent<mlua::Variadic<MyLuaArg>>>,
    events: &[ScriptEventData],
) {
    let mut rng = rand::thread_rng();
    let id = COUNTER.fetch_add(1, Relaxed);
    let arg = MyLuaArg(id as usize);
    let (event, prio) = events
        .choose(&mut rng)
        .map(|v| {
            let mut args = mlua::Variadic::new();
            args.push(arg);
            (
                LuaEvent {
                    hook_name: v.0.to_string(),
                    args,
                    recipients: Recipients::All,
                },
                v.1,
            )
        })
        .unwrap();

    info!(
        "\t - event: {},\t prio: {},\t id: {}",
        event.hook_name, prio, id
    );
    w.send(event, prio);
}

fn do_physics(mut w: PriorityEventWriter<LuaEvent<mlua::Variadic<MyLuaArg>>>) {
    info!("Physics, firing:");

    for _ in 0..5 {
        // fire random event, for any of the system sets
        fire_random_event(&mut w, &ALL_EVENTS);
    }
}

fn do_update(mut w: PriorityEventWriter<LuaEvent<mlua::Variadic<MyLuaArg>>>) {
    info!("Update, firing:");

    // fire random event, for any of the system sets
    fire_random_event(&mut w, &ALL_EVENTS);
}

#[derive(Clone, Copy)]
pub struct ScriptEventData(&'static str, u32);

static ON_PRE_PHYSICS: ScriptEventData = ScriptEventData("on_pre_physics", 0);
static ON_POST_PHYSICS: ScriptEventData = ScriptEventData("on_post_physics", 11);
static ON_POST_UPDATE: ScriptEventData = ScriptEventData("on_post_update", 21);
static ALL_EVENTS: [ScriptEventData; 3] = [ON_PRE_PHYSICS, ON_POST_PHYSICS, ON_POST_UPDATE];

fn load_our_script(server: Res<AssetServer>, mut commands: Commands) {
    let path = "scripts/complex_game_loop.lua";
    let handle = server.load::<LuaFile, &str>(path);

    commands.spawn(()).insert(ScriptCollection::<LuaFile> {
        scripts: vec![Script::<LuaFile>::new(path.to_string(), handle)],
    });
}

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
enum ComplexGameLoopSet {
    PrePhysics,
    Physics,
    PrePhysicsScripts,
    PostPhysicsScripts,
    PostUpdateScripts,
}

fn main() -> std::io::Result<()> {
    const TIMESTEP_2_PER_SECOND: f32 = 30.0 / 60.0;

    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .insert_resource(FixedTime::new_from_secs(TIMESTEP_2_PER_SECOND))
        .add_plugins(ScriptingPlugin)
        .add_systems(Startup, load_our_script)
        // --- main systems sets
        // pre physics logic system set (twice a second)
        .configure_set(
            FixedUpdate,
            ComplexGameLoopSet::PrePhysics.before(ComplexGameLoopSet::Physics),
        )
        // physics logic system set (twice a second)
        .add_systems(FixedUpdate, do_physics.in_set(ComplexGameLoopSet::Physics))
        // main update logic system set (every frame)
        .add_systems(Update, do_update)
        // --- script handler system sets
        // pre_physics,     priority: [0,10] inclusive
        .configure_set(
            FixedUpdate,
            ComplexGameLoopSet::PrePhysicsScripts.after(ComplexGameLoopSet::PrePhysics),
        )
        .add_script_handler_to_set::<LuaScriptHost<mlua::Variadic<MyLuaArg>>, 0, 10>(
            FixedUpdate,
            ComplexGameLoopSet::PrePhysicsScripts,
        )
        // post_physics,    priority: [11,20] inclusive
        // since the previous system will consume all events in the [0,10] range
        .configure_set(
            FixedUpdate,
            ComplexGameLoopSet::PostPhysicsScripts.after(ComplexGameLoopSet::Physics),
        )
        .add_script_handler_to_set::<LuaScriptHost<mlua::Variadic<MyLuaArg>>, 11, 20>(
            FixedUpdate,
            ComplexGameLoopSet::PostPhysicsScripts,
        )
        // post_update,     priority: [21,30] inclusive
        .add_script_handler_to_set::<LuaScriptHost<mlua::Variadic<MyLuaArg>>, 21, 30>(
            PostUpdate,
            ComplexGameLoopSet::PostUpdateScripts,
        )
        // this system set handles addition and removal of script contexts, we can safely use `CoreSet::PostUpdate`
        .add_script_host_to_set::<LuaScriptHost<mlua::Variadic<MyLuaArg>>>(
            PostUpdate,
            ComplexGameLoopSet::PostUpdateScripts,
        )
        .add_api_provider::<LuaScriptHost<mlua::Variadic<MyLuaArg>>>(Box::new(LuaAPIProvider));
    // We have 2 core systems

    // Physics (twice per second),    fires 5 random events
    // Update (every frame),          fires 1 random event

    // and 3 event handlers

    // pre_physics (twice per second)
    // post_physics (twice per second)
    // post_update (every frame)

    // each of those spawns a single random event from the pool of all events
    // when a handler encounters an event of higher priority outside its range, that event is discarded
    // when a handler encounters an event of lower priority outside its range, it's left in the queue
    // therefore
    // in our case, PrePhysics systems can generate events which can be handled by post_update,
    // but Update cannot send events which are handled by anything other than post_update

    // note that regardless of the order in which the events were spawned
    // priority decides the order in which they are executed
    // in case of identical priority, order is the tie-breaker (earlier events launch first)
    app.run();

    Ok(())
}
