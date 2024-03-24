use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    reflect::Reflect,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
        texture::ImageSampler,
    },
    window::{PrimaryWindow, WindowResized},
};

use bevy_mod_scripting::prelude::*;
use bevy_mod_scripting_rhai::rhai::packages::Package;
use bevy_script_api::rhai::{std::RegisterVecType, RegisterForeignRhaiType};
use rhai_rand::RandomPackage;

#[derive(Clone, Debug, Default, Reflect, Component)]
#[reflect(Component)]
pub struct LifeState {
    pub cells: Vec<u8>,
}

#[derive(Default)]
pub struct LifeAPI;

impl APIProvider for LifeAPI {
    type APITarget = Engine;
    type ScriptContext = RhaiContext;
    type DocTarget = RhaiDocFragment;

    fn attach_api(&mut self, api: &mut Self::APITarget) -> Result<(), ScriptError> {
        api.register_vec_functions::<u8>();
        let random = RandomPackage::new();
        api.set_max_expr_depths(999, 999);

        // Load the package into the `Engine`
        random.register_into_engine(api);
        Ok(())
    }

    fn register_with_app(&self, app: &mut App) {
        // this will resolve retrievals of this component to our custom rhai object
        app.register_type::<LifeState>();
        app.register_type::<Settings>();
        app.register_foreign_rhai_type::<Vec<u8>>();
    }
}

#[derive(Reflect, Resource)]
#[reflect(Resource)]
pub struct Settings {
    physical_grid_dimensions: (u32, u32),
    display_grid_dimensions: (u32, u32),
    border_thickness: u32,
    live_color: u8,
    dead_color: u8,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            border_thickness: 1,
            live_color: 255u8,
            dead_color: 0u8,
            physical_grid_dimensions: (44, 25),
            display_grid_dimensions: (0, 0),
        }
    }
}

pub fn setup(
    mut commands: Commands,
    mut assets: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
    settings: Res<Settings>,
) {
    let mut image = Image::new_fill(
        Extent3d {
            width: settings.physical_grid_dimensions.0,
            height: settings.physical_grid_dimensions.1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[0u8],
        TextureFormat::R8Unorm,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );

    image.sampler = ImageSampler::nearest();

    let script_path = "scripts/game_of_life.rhai";

    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(SpriteBundle {
            texture: assets.add(image),
            sprite: Sprite {
                custom_size: Some(Vec2::new(
                    settings.display_grid_dimensions.0 as f32,
                    settings.display_grid_dimensions.1 as f32,
                )),
                color: Color::TOMATO,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(LifeState {
            cells: vec![
                0u8;
                (settings.physical_grid_dimensions.0 * settings.physical_grid_dimensions.1)
                    as usize
            ],
        })
        .insert(ScriptCollection::<RhaiFile> {
            scripts: vec![Script::new(
                script_path.to_owned(),
                asset_server.load(script_path),
            )],
        });
}

pub fn sync_window_size(
    mut resize_event: EventReader<WindowResized>,
    mut settings: ResMut<Settings>,
    mut query: Query<&mut Sprite, With<LifeState>>,
    primary_windows: Query<&Window, With<PrimaryWindow>>,
) {
    if let Some(e) = resize_event
        .read()
        .filter(|e| primary_windows.get(e.window).is_ok())
        .last()
    {
        let primary_window = primary_windows.get(e.window).unwrap();
        settings.display_grid_dimensions = (
            primary_window.physical_width(),
            primary_window.physical_height(),
        );

        // resize all game's of life, retain aspect ratio and fit the entire game in the window
        for mut sprite in query.iter_mut() {
            let scale = if settings.physical_grid_dimensions.0 > settings.physical_grid_dimensions.1
            {
                // horizontal is longer
                settings.display_grid_dimensions.1 as f32
                    / settings.physical_grid_dimensions.1 as f32
            } else {
                // vertical is longer
                settings.display_grid_dimensions.0 as f32
                    / settings.physical_grid_dimensions.0 as f32
            };

            sprite.custom_size = Some(Vec2::new(
                (settings.physical_grid_dimensions.0 as f32) * scale,
                (settings.physical_grid_dimensions.1 as f32) * scale,
            ));
        }
    }
}

/// Runs after LifeState components are updated, updates their rendered representation
pub fn update_rendered_state(
    mut assets: ResMut<Assets<Image>>,
    query: Query<(&LifeState, &Handle<Image>)>,
) {
    for (new_state, old_rendered_state) in query.iter() {
        let old_rendered_state = assets
            .get_mut(old_rendered_state)
            .expect("World is not setup correctly");

        old_rendered_state.data = new_state.cells.clone();
    }
}

/// Sends events allowing scripts to drive update logic
pub fn send_on_update(mut events: PriorityEventWriter<RhaiEvent<()>>) {
    events.send(
        RhaiEvent {
            hook_name: "on_update".to_owned(),
            args: (),
            recipients: Recipients::All,
        },
        1,
    )
}

/// Sends initialization event
pub fn send_init(mut events: PriorityEventWriter<RhaiEvent<()>>) {
    events.send(
        RhaiEvent {
            hook_name: "init".to_owned(),
            args: (),
            recipients: Recipients::All,
        },
        0,
    )
}

/// how often to step the simulation
const UPDATE_FREQUENCY: f64 = 1.0 / 30.0;

fn main() -> std::io::Result<()> {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .insert_resource(Time::<Fixed>::from_seconds(UPDATE_FREQUENCY))
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(ScriptingPlugin)
        .init_resource::<Settings>()
        .add_systems(Startup, setup)
        .add_systems(Startup, send_init)
        .add_systems(Update, sync_window_size)
        .add_systems(FixedUpdate, (update_rendered_state, send_on_update).chain())
        .add_systems(
            FixedUpdate,
            script_event_handler::<RhaiScriptHost<()>, 0, 1>,
        )
        .add_script_host::<RhaiScriptHost<()>>(PostUpdate)
        .add_api_provider::<RhaiScriptHost<()>>(Box::new(RhaiBevyAPIProvider))
        .add_api_provider::<RhaiScriptHost<()>>(Box::new(LifeAPI))
        .update_documentation::<RhaiScriptHost<()>>();

    app.run();

    Ok(())
}
