use bevy::{prelude::*, reflect::Reflect};
use bevy_mod_scripting_core::{
    event::ScriptLoaded,
    prelude::{APIProvider, PriorityEventWriter, Recipients, Script, ScriptCollection},
    AddScriptApiProvider, AddScriptHost, AddScriptHostHandler, ScriptingPlugin,
};
use bevy_mod_scripting_rhai::{
    prelude::{RhaiDocFragment, RhaiFile},
    rhai::{Engine, FuncArgs},
    RhaiContext, RhaiEvent, RhaiScriptHost,
};
use bevy_script_api::prelude::RhaiBevyAPIProvider;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ScriptingPlugin))
        .add_systems(Startup, setup_entities)
        .add_systems(Update, (call_init, call_update))
        .add_script_host::<RhaiScriptHost<ScriptArgs>>(PostUpdate)
        .add_api_provider::<RhaiScriptHost<ScriptArgs>>(Box::new(RhaiBevyAPIProvider))
        .add_script_handler::<RhaiScriptHost<ScriptArgs>, 0, 1>(PostUpdate)
        .run();
}

#[derive(Default)]
pub struct MyCustomAPI;

impl APIProvider for MyCustomAPI {
    type APITarget = Engine;
    type ScriptContext = RhaiContext;
    type DocTarget = RhaiDocFragment;

    fn attach_api(
        &mut self,
        api: &mut Self::APITarget,
    ) -> Result<(), bevy_mod_scripting::prelude::ScriptError> {
        api.set_max_expr_depths(0, 0);

        Ok(())
    }
}

#[derive(Debug, Clone, Reflect, Default)]
struct ScriptArgs {
    entity_name: Option<String>,
    delta_time: Option<f32>,
}

impl FuncArgs for ScriptArgs {
    fn parse<ARGS: Extend<bevy_mod_scripting_rhai::rhai::Dynamic>>(self, args: &mut ARGS) {
        if let Some(entity_name) = self.entity_name {
            args.extend(vec![entity_name.into()]);
        }
        if let Some(delta_time) = self.delta_time {
            args.extend(vec![delta_time.to_string().into()]);
        }
    }
}

fn setup_entities(mut commands: Commands, asset_server: Res<AssetServer>) {
    let script_path = "scripts/multiple_events_rhai.rhai";

    for i in 0..10 {
        let entity_name = format!("Test Entity {}", i);
        commands.spawn((
            NewlyAddedEntityCallInit,
            Name::from(entity_name),
            ScriptCollection::<RhaiFile> {
                scripts: vec![Script::new(
                    script_path.to_owned(),
                    asset_server.load(script_path),
                )],
            },
        ));
    }
}

#[derive(Debug, Clone, Copy, Reflect, Default, Component)]
#[reflect(Component)]
pub struct NewlyAddedEntityCallInit;

fn call_update(
    mut events: PriorityEventWriter<RhaiEvent<ScriptArgs>>,
    time: Res<Time>,
    to_update: Query<
        (Entity, Option<&Name>),
        (
            With<ScriptCollection<RhaiFile>>,
            Without<NewlyAddedEntityCallInit>,
        ),
    >,
) {
    to_update.for_each(|(entity, name)| {
        events.send(
            RhaiEvent {
                hook_name: "on_update".to_owned(),
                args: ScriptArgs {
                    delta_time: Some(time.delta_seconds()),
                    entity_name: name.map(|n| n.to_string()),
                },
                recipients: Recipients::Entity(entity),
            },
            1,
        );
    });
}

fn call_init(
    mut events: PriorityEventWriter<RhaiEvent<ScriptArgs>>,
    mut loaded_scripts: EventReader<ScriptLoaded>,
    entity_query: Query<(Entity, Option<&Name>, Option<&ScriptCollection<RhaiFile>>)>,
    mut commands: Commands,
) {
    // Call init on all entities that have a script that was just loaded, and remove the Marker component, so that Update can be called
    'outer: for loaded_script in loaded_scripts.iter() {
        for (entity, name, scripts) in entity_query.iter() {
            if let Some(scripts) = scripts {
                if scripts.scripts.iter().any(|s| s.id() == loaded_script.sid) {
                    events.send(
                        RhaiEvent {
                            hook_name: "on_init".to_owned(),
                            args: ScriptArgs {
                                entity_name: name.map(|n| n.to_string()),
                                ..Default::default()
                            },
                            recipients: Recipients::ScriptID(loaded_script.sid),
                        },
                        0,
                    );

                    continue 'outer;
                }
            }

            commands.entity(entity).remove::<NewlyAddedEntityCallInit>();
        }
    }
}
