use bevy::{core::FixedTimestep, prelude::*};
use bevy_console::ConsolePlugin;
use bevy_event_priority::PriorityEventWriter;
use bevy_scripting::{
    APIProvider, AddScriptHost, AddScriptHostHandler, LuaCallbackArgument, LuaEvent, LuaFile,
    RLuaScriptHost, Script, ScriptCollection, ScriptingPlugin,
};
use rand::prelude::SliceRandom;
use rlua::Lua;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{atomic::AtomicU32, Mutex};

#[derive(Default)]
pub struct LuaAPIProvider {}

/// the custom Lua api, world is provided via a global pointer,
/// and callbacks are defined only once at script creation
impl APIProvider for LuaAPIProvider {
    type Ctx = Mutex<Lua>;
    fn attach_api(ctx: &mut Self::Ctx) {
        // callbacks can receive any `ToLuaMulti` arguments, here '()' and
        // return any `FromLuaMulti` arguments, here a `usize`
        // check the Rlua documentation for more details
        RLuaScriptHost::<Self>::register_api_callback(
            "print",
            |_ctx, msg: String| {
                info!("{}", msg);
                Ok(())
            },
            ctx,
        )
    }
}

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// utility for generating random events from a list
fn fire_random_event(w: &mut PriorityEventWriter<LuaEvent>, events: &[ScriptEventData]) {
    let mut rng = rand::thread_rng();
    let id = COUNTER.fetch_add(1, Relaxed);
    let arg = LuaCallbackArgument::Integer(id as usize);
    let (event, prio) = events
        .choose(&mut rng)
        .map(|v| {
            (
                LuaEvent {
                    hook_name: v.0.to_string(),
                    args: vec![arg],
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

fn do_some_shit_before_physics(mut w: PriorityEventWriter<LuaEvent>) {
    info!("PrePhysics, firing:");

    for _ in 0..5 {
        // fire random event, for any of the stages
        fire_random_event(&mut w, &ALL_EVENTS);
    }
}

fn do_physics(mut w: PriorityEventWriter<LuaEvent>) {
    info!("Physics, firing:");

    for _ in 0..5 {
        // fire random event, for any stages,
        fire_random_event(&mut w, &ALL_EVENTS);
    }
}

fn do_update(mut w: PriorityEventWriter<LuaEvent>) {
    info!("Update, firing:");

    // fire random event, for any stages
    fire_random_event(&mut w, &ALL_EVENTS);
}

#[derive(Clone, Copy)]
pub struct ScriptEventData(&'static str, u32);

static ON_PRE_PHYSICS_ONE: ScriptEventData = ScriptEventData("on_pre_physics_one", 0);
static ON_PRE_PHYSICS_TWO: ScriptEventData = ScriptEventData("on_pre_physics_two", 1);
static ON_POST_PHYSICS_ONE: ScriptEventData = ScriptEventData("on_post_physics_one", 11);
static ON_POST_PHYSICS_TWO: ScriptEventData = ScriptEventData("on_post_physics_two", 12);
static ON_POST_UPDATE_ONE: ScriptEventData = ScriptEventData("on_post_update_one", 21);
static ON_POST_UPDATE_TWO: ScriptEventData = ScriptEventData("on_post_update_two", 22);
static ALL_EVENTS: [ScriptEventData; 6] = [
    ON_PRE_PHYSICS_ONE,
    ON_PRE_PHYSICS_TWO,
    ON_POST_PHYSICS_ONE,
    ON_POST_PHYSICS_TWO,
    ON_POST_UPDATE_ONE,
    ON_POST_UPDATE_TWO,
];

fn load_our_script(server: Res<AssetServer>, mut commands: Commands) {
    let path = "scripts/complex_game_loop.lua";
    let handle = server.load::<LuaFile, &str>(path);

    commands.spawn().insert(ScriptCollection::<LuaFile> {
        scripts: vec![Script::<LuaFile>::new::<RLuaScriptHost<LuaAPIProvider>>(
            path.to_string(),
            handle,
        )],
    });
}

fn main() -> std::io::Result<()> {
    static PRE_PHYSICS: &str = "pre_physics";
    static PHYSICS: &str = "physics";
    static PRE_PHYSICS_SCRIPTS: &str = "pre_physics_scripts";
    static POST_PHYSICS_SCRIPTS: &str = "post_physics_scripts";
    static POST_UPDATE_SCRIPTS: &str = "post_update_scripts";

    const TIMESTEP_2_PER_SECOND: f64 = 30.0 / 60.0;

    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(ScriptingPlugin)
        .add_plugin(ConsolePlugin)
        .add_startup_system(load_our_script)
        // --- main systems stages
        // physics logic stage (twice a second)
        .add_stage_before(CoreStage::Update, PHYSICS, SystemStage::parallel())
        .add_system_to_stage(
            PHYSICS,
            do_physics.with_run_criteria(FixedTimestep::step(TIMESTEP_2_PER_SECOND)),
        )
        // pre physics logic stage (twice a second)
        .add_stage_before(PHYSICS, PRE_PHYSICS, SystemStage::parallel())
        .add_system_to_stage(
            PRE_PHYSICS,
            do_some_shit_before_physics
                .with_run_criteria(FixedTimestep::step(TIMESTEP_2_PER_SECOND)),
        )
        // main update logic stage (every frame)
        .add_system(do_update)
        // --- script handler stages
        // pre_physics,     priority: [0,10] inclusive
        .add_stage_after(PRE_PHYSICS, PRE_PHYSICS_SCRIPTS, SystemStage::single_threaded())
        .add_script_handler_stage_with_criteria::<RLuaScriptHost<LuaAPIProvider>, _, _, _, 0, 10>(
            PRE_PHYSICS_SCRIPTS,
            FixedTimestep::step(TIMESTEP_2_PER_SECOND),
        )
        // post_physics,    priority: [11,20] inclusive
        // since the previous system will consume all events in the [0,10] range
        .add_stage_after(
            PHYSICS,
            POST_PHYSICS_SCRIPTS,
            SystemStage::single_threaded(),
        )
        .add_script_handler_stage_with_criteria::<RLuaScriptHost<LuaAPIProvider>, _, _, _, 11, 20>(
            POST_PHYSICS_SCRIPTS,
            FixedTimestep::step(TIMESTEP_2_PER_SECOND),
        )
        // post_update,     priority: [21,30] inclusive
        // note we do not use the CoreStage version since our scripts might want
        // to modify transforms etc which some core systems synchronise in here
        .add_stage_after(
            CoreStage::Update,
            POST_UPDATE_SCRIPTS,
            SystemStage::single_threaded(),
        )
        .add_script_handler_stage::<RLuaScriptHost<LuaAPIProvider>, _, 21, 30>(POST_UPDATE_SCRIPTS)
        // this stage handles addition and removal of script contexts, we can safely use `CoreStage::PostUpdate`
        .add_script_host::<RLuaScriptHost<LuaAPIProvider>, _>(CoreStage::PostUpdate);

    // We have 3 core systems

    // PrePhysics (twice per second), fires 5 random events
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
