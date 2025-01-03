#![allow(deprecated)]
use std::sync::Mutex;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    image::ImageSampler,
    prelude::*,
    reflect::Reflect,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
    window::{PrimaryWindow, WindowResized},
};

use bevy_mod_scripting::prelude::*;
use bevy_mod_scripting_lua::LuaScriptingPlugin;
use bindings::script_value::ScriptValue;

#[derive(Debug, Default, Clone, Reflect, Component)]
#[reflect(Component)]
pub struct LifeState {
    pub cells: Vec<u8>,
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
            physical_grid_dimensions: (88, 50),
            display_grid_dimensions: (0, 0),
        }
    }
}

pub fn init_game_of_life_state(
    mut commands: Commands,
    mut assets: ResMut<Assets<Image>>,
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

    commands.spawn(Camera2d);
    commands
        .spawn(Sprite {
            image: assets.add(image),
            custom_size: Some(Vec2::new(
                settings.display_grid_dimensions.0 as f32,
                settings.display_grid_dimensions.1 as f32,
            )),
            color: Color::srgb(1.0, 0.388, 0.278), // TOMATO
            ..Default::default()
        })
        .insert(LifeState {
            cells: vec![
                0u8;
                (settings.physical_grid_dimensions.0 * settings.physical_grid_dimensions.1)
                    as usize
            ],
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
    query: Query<(&LifeState, &Sprite)>,
) {
    for (new_state, old_rendered_state) in query.iter() {
        let old_rendered_state = assets
            .get_mut(&old_rendered_state.image)
            .expect("World is not setup correctly");

        old_rendered_state.data = new_state.cells.clone();
    }
}

callback_labels!(
    OnUpdate => "on_update",
    Init => "init"
);

/// Sends events allowing scripts to drive update logic
pub fn send_on_update(mut events: EventWriter<ScriptCallbackEvent>) {
    events.send(ScriptCallbackEvent::new_for_all(
        OnUpdate,
        vec![ScriptValue::Unit],
    ));
}

const UPDATE_FREQUENCY: f32 = 1.0 / 60.0;

fn main() -> std::io::Result<()> {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .insert_resource(Time::<Fixed>::from_seconds(UPDATE_FREQUENCY.into()))
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(LuaScriptingPlugin::default())
        .init_resource::<Settings>()
        .add_systems(Startup, init_game_of_life_state)
        .add_systems(Update, sync_window_size)
        .add_systems(FixedUpdate, update_rendered_state.after(sync_window_size))
        .add_systems(FixedUpdate, send_on_update.after(update_rendered_state))
        .add_systems(FixedUpdate, event_handler::<OnUpdate, LuaScriptingPlugin>);

    app.run();

    Ok(())
}
