use bevy::prelude::*;
use bevy_mod_scripting::prelude::*;
use rand::prelude::SliceRandom;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;

#[derive(Clone)]
pub struct MyRuneArg(usize);

impl Args for MyRuneArg {
    fn into_stack(self, stack: &mut rune::runtime::Stack) -> rune::runtime::VmResult<()> {
        (self.0,).into_stack(stack)
    }

    fn try_into_vec(self) -> rune::runtime::VmResult<rune::alloc::Vec<rune::Value>> {
        (self.0,).try_into_vec()
    }

    fn count(&self) -> usize {
        1
    }
}

/// A custom Rune API.
#[derive(Default)]
pub struct RuneAPIProvider;

impl APIProvider for RuneAPIProvider {
    type APITarget = Context;
    type DocTarget = RuneDocFragment;
    type ScriptContext = RuneScriptContext;

    fn attach_api(&mut self, ctx: &mut Self::APITarget) -> Result<(), ScriptError> {
        let mut module = rune::Module::new();

        module
            .function("info", |msg: String| info!("{msg}"))
            .build()
            .map_err(ScriptError::new_other)?;

        ctx.install(module).map_err(ScriptError::new_other)?;

        Ok(())
    }
}

static COUNTER: AtomicU32 = AtomicU32::new(0);

/// Utility for generating random events from a list.
fn fire_random_event(
    w: &mut PriorityEventWriter<RuneEvent<MyRuneArg>>,
    events: &[ScriptEventData],
) {
    let mut rng = rand::thread_rng();
    let id = COUNTER.fetch_add(1, Relaxed);
    let arg = MyRuneArg(id as usize);
    let event = events
        .choose(&mut rng)
        .map(|v| RuneEvent {
            hook_name: v.0.to_string(),
            args: arg,
            recipients: v.1.clone(),
        })
        .unwrap();

    info!(
        "\t - event: {},\t recipients: {:?},\t id: {}",
        event.hook_name, event.recipients, id
    );
    w.send(event, 0);
}

fn do_update(mut w: PriorityEventWriter<RuneEvent<MyRuneArg>>) {
    info!("Update, firing:");

    let all_events = [
        ScriptEventData("on_event", Recipients::All),
        ScriptEventData("on_event", Recipients::ScriptID(0)),
        ScriptEventData("on_event", Recipients::ScriptID(1)),
        ScriptEventData(
            "on_event",
            Recipients::ScriptName("scripts/event_recipients.rune".to_owned()),
        ),
    ];

    // fire random event, for any of the system sets
    fire_random_event(&mut w, &all_events);
}

#[derive(Clone)]
pub struct ScriptEventData(&'static str, Recipients);

fn load_scripts(server: Res<AssetServer>, mut commands: Commands) {
    // Spawn two identical scripts.
    // Their id's will be 0 and 1.
    let path = "scripts/event_recipients.rune";
    let handle = server.load::<RuneFile>(path);
    let scripts = (0..2)
        .map(|_| Script::<RuneFile>::new(path.to_string(), handle.clone()))
        .collect();

    commands
        .spawn(())
        .insert(ScriptCollection::<RuneFile> { scripts });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ScriptingPlugin)
        .add_systems(Startup, load_scripts)
        // Randomly fire events for either all scripts, the script with an id of `0`,
        // or the script with an id of `1`.
        .add_systems(Update, do_update)
        .add_script_handler::<RuneScriptHost<MyRuneArg>, 0, 0>(PostUpdate)
        .add_script_host::<RuneScriptHost<MyRuneArg>>(PostUpdate)
        .add_api_provider::<RuneScriptHost<MyRuneArg>>(Box::new(RuneAPIProvider))
        .run()
}
