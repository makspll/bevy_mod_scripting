use bevy::prelude::*;
use bevy_mod_scripting::prelude::*;

fn main() {
    // Collect command-line arguments, skipping the program name.
    let mut args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: run-script <file1> <file2> ...");
        std::process::exit(1);
    }

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BMSPlugin)
        .add_plugins(add_logging)
        .add_systems(
            Startup,
            move |asset_server: Res<AssetServer>, mut commands: Commands| {
                let mut handles = vec![];
                for file_name in args.drain(..) {
                    handles.push(asset_server.load::<ScriptAsset>(file_name));
                }
                commands.spawn(ScriptComponent(handles));
            },
        )
        .add_systems(Update, info_on_asset_event::<ScriptAsset>())
        .run();
}

fn add_logging(app: &mut App) {
    let world = app.world_mut();
    NamespaceBuilder::<World>::new_unregistered(world)
        .register("info", |s: String| {
            bevy::log::info!("{}", s);
        })
        .register("warn", |s: String| {
            bevy::log::warn!("{}", s);
        })
        .register("error", |s: String| {
            bevy::log::error!("{}", s);
        })
        .register("debug", |s: String| {
            bevy::log::debug!("{}", s);
        })
        .register("trace", |s: String| {
            bevy::log::trace!("{}", s);
        });
}

pub fn info_on_asset_event<T: Asset>() -> impl FnMut(EventReader<AssetEvent<T>>) {
    // The events need to be consumed, so that there are no false positives on subsequent
    // calls of the run condition. Simply checking `is_empty` would not be enough.
    // PERF: note that `count` is efficient (not actually looping/iterating),
    // due to Bevy having a specialized implementation for events.
    move |mut reader: EventReader<AssetEvent<T>>| {
        for event in reader.read() {
            match event {
                AssetEvent::Modified { .. } => (),
                _ => {
                    info!("ASSET EVENT {:?}", &event);
                }
            }
        }
    }
}
