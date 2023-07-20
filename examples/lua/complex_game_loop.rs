use bevy::core::FrameCount;
use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::*;
use bevy_mod_scripting::prelude::*;
use rand::prelude::SliceRandom;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;

#[derive(Clone)]
/// The type we will be using to send data to Lua
pub struct MyLuaArg(usize);

impl<'lua> ToLua<'lua> for MyLuaArg {
    fn to_lua(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        self.0.to_lua(lua)
    }
}

/// Used to assign events unique ids
static COUNTER: AtomicU32 = AtomicU32::new(0);

/// The event firing logic we will use at the each stage of the game loop
/// Fires a random event from the pool of all events i.e. one of:
/// - on_pre_physics, priority: 0
/// - on_post_physics, priority: 11
/// - on_pre_physics, priority: 21
fn fire_random_event(w: &mut PriorityEventWriter<LuaEvent<mlua::Variadic<MyLuaArg>>>) {
    let mut rng = rand::thread_rng();
    let id = COUNTER.fetch_add(1, Relaxed);
    let arg = MyLuaArg(id as usize);
    let (event, prio) = [
        ("on_pre_physics", 0),
        ("on_post_physics", 11),
        ("on_post_update", 21),
    ]
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

/// physics stage logic, represents a bunch of systems sending events of various types and priorities
fn do_physics(mut w: PriorityEventWriter<LuaEvent<mlua::Variadic<MyLuaArg>>>) {
    info!("Physics, firing:");

    for _ in 0..5 {
        fire_random_event(&mut w);
    }
}

/// update stage logic, fired each frame, represents a bunch of systems sending events of various types and priorities
/// we fire just one since we want to keep the output clean
fn do_update(mut w: PriorityEventWriter<LuaEvent<mlua::Variadic<MyLuaArg>>>) {
    info!("Update, firing:");

    fire_random_event(&mut w);
}

/// We will run this system at the end of each update to make the output easier to read
fn print_frame_count(frame: Res<FrameCount>) {
    info!("================ Frame no {} End ================", frame.0);
}

fn load_our_script(server: Res<AssetServer>, mut commands: Commands) {
    let path = "scripts/complex_game_loop.lua";
    let handle = server.load::<LuaFile, &str>(path);

    commands.spawn(()).insert(ScriptCollection::<LuaFile> {
        scripts: vec![Script::<LuaFile>::new(path.to_string(), handle)],
    });
}

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
enum ComplexGameLoopSet {
    Physics,
    PrePhysicsScripts,
    PostPhysicsScripts,
    PostUpdateScripts,
    EndFrame,
}

fn main() -> std::io::Result<()> {
    const TIMESTEP_2_PER_SECOND: f32 = 30.0 / 60.0;

    let mut app = App::new();

    // first let's configure the set orders:
    // we run the pre-physics scripts before physics (duh)
    // we run the post-physics scripts after physics
    // we run the post-update scripts after post-update
    // pretty straightforward, note we use FixedUpdate for physics, which means it runs less often than Update
    app.add_plugins(DefaultPlugins)
        .insert_resource(FixedTime::new_from_secs(TIMESTEP_2_PER_SECOND))
        .add_plugins(ScriptingPlugin)
        .add_systems(Startup, load_our_script)
        .configure_set(FixedUpdate, ComplexGameLoopSet::Physics.after(ComplexGameLoopSet::))
        .configure_set(
            FixedUpdate,
            ComplexGameLoopSet::PrePhysicsScripts.before(ComplexGameLoopSet::Physics),
        )
        .configure_set(
            FixedUpdate,
            ComplexGameLoopSet::PostPhysicsScripts.after(ComplexGameLoopSet::Physics),
        )
        .configure_set(
            PostUpdate,
            ComplexGameLoopSet::EndFrame.after(ComplexGameLoopSet::PostUpdateScripts),
        );

    // Now let's configure our game's main logic/engine systems
    app.add_systems(FixedUpdate, do_physics.in_set(ComplexGameLoopSet::Physics))
        // main update logic system set (every frame)
        .add_systems(Update, do_update)
        .add_systems(
            PostUpdate,
            print_frame_count.in_set(ComplexGameLoopSet::EndFrame),
        );

    // Finally let's configure the scripting systems.
    // Think of the priority value of events as their "order"
    // Events with priority "1" go before events of priority "2"
    app
        // --- script handler system sets
        // pre_physics,     event priority: [0,10] inclusive
        // handlers always ignore the events of lower priority than their range
        // meaning this one will only handle pre_physics events
        .add_script_handler_to_set::<LuaScriptHost<mlua::Variadic<MyLuaArg>>, 0, 10>(
            FixedUpdate,
            ComplexGameLoopSet::PrePhysicsScripts,
        )
        // post_physics,    event priority: [11,20] inclusive
        // This handler one will only ever handle post_physics events,
        // events of higher priority [0-11] are discarded completely
        // (the logic being: if we are at a point in time where we are handling post_physics events, we don't care about pre_physics events)
        .add_script_handler_to_set::<LuaScriptHost<mlua::Variadic<MyLuaArg>>, 11, 20>(
            FixedUpdate,
            ComplexGameLoopSet::PostPhysicsScripts,
        )
        // post_update,     priority: [21,30] inclusive
        // similar to before, only post_update events are handled
        .add_script_handler_to_set::<LuaScriptHost<mlua::Variadic<MyLuaArg>>, 21, 30>(
            PostUpdate,
            ComplexGameLoopSet::PostUpdateScripts,
        )
        // finally we add core script host systems to PostUpdate
        // these handle the scripts themselves i.e. add/remove/modify them when necessary
        .add_script_host_to_set::<LuaScriptHost<mlua::Variadic<MyLuaArg>>>(
            PostUpdate,
            ComplexGameLoopSet::PostUpdateScripts,
        );
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
    // in our case, Physics systems can generate events which can be handled by post_update,
    // but Update cannot send events which are handled by anything other than post_update

    // note that regardless of the order in which the events were spawned
    // priority decides the order in which they are executed
    // in case of identical priority, order is the tie-breaker (earlier events launch first)

    // interestingly, because the Main bevy scheduler runs FixedUpdate systems *before* any Update systems, in this case
    // on_pre_physics events will *never* be handled! (they are discarded by the post_physics handler, and the update system never runs before physics)
    app.run();

    Ok(())
}
