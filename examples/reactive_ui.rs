//! An inventory of 500 items which live in the ECS, shown and edited by a reactive ui written in Lua.
//!
//! The data flows both ways:
//! - Rust owns the `Item` components and restocks a few of them several times a second. The ui watches
//!   the components, only the rows of the items which changed are updated.
//! - Clicking in the ui changes the same components from the script, which Rust systems see like any change.
//!
//! The rows are built from a row template, no script code runs per row. The stock bars and restock flashes
//! are animated by Rust writing `shown` and `flash`: the rows bind to them, so only the rows which move update.
//! The header shows how long the ui took to update, measured around the systems of the reactive ui.
//!
//! Run with `cargo run --example reactive_ui --features lua54,bevy/file_watcher`, the ui is
//! `assets/scripts/inventory_ui.lua`. Edit the script while it runs: the ui reloads and keeps its state
//! (`file_watcher` is what notices the edit).

use std::time::{Duration, Instant};

use bevy::prelude::*;
use bevy_mod_scripting::prelude::*;
use bevy_mod_scripting_reactive_ui::{ReactiveUiPlugin, ReactiveUiSystems};

const ITEMS: usize = 500;
const KINDS: [&str; 4] = ["weapon", "armour", "potion", "scroll"];

/// An item of the inventory, one entity each.
#[derive(Component, Reflect)]
#[reflect(Component)]
#[type_path = "inventory"]
struct Item {
    name: String,
    kind: String,
    count: u32,
    /// `count`, eased over a few frames, for the stock bars
    shown: f32,
    /// 1 when Rust restocks the item, fading to 0
    flash: f32,
}

/// What the ui shows about itself, on one entity.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[type_path = "inventory"]
struct UiStats {
    /// How long the last ui update which had work to do took
    update_micros: u32,
    /// How many times Rust restocked an item
    restocked: u32,
}

/// When the ui systems started this frame.
#[derive(Resource)]
struct UiUpdateStart(Instant);

#[derive(Resource)]
struct RestockTimer(Timer);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(UiStats::default());
    for i in 0..ITEMS {
        let kind = KINDS[i % KINDS.len()];
        let count = (i % 7) as u32;
        commands.spawn(Item {
            name: format!("{kind} {}", i / KINDS.len() + 1),
            kind: kind.to_owned(),
            count,
            shown: count as f32,
            flash: 0.0,
        });
    }
    // the script builds the ui when it is loaded
    commands.spawn(ScriptComponent(vec![
        asset_server.load("scripts/inventory_ui.lua"),
    ]));
}

/// Rust changing the world: a few items get one more, several times a second.
fn restock(
    time: Res<Time>,
    mut timer: ResMut<RestockTimer>,
    mut items: Query<&mut Item>,
    mut stats: Query<&mut UiStats>,
    mut next: Local<usize>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    let mut restocked = 0;
    for mut item in items.iter_mut().skip(*next % ITEMS).step_by(97).take(5) {
        item.count += 1;
        item.flash = 1.0;
        restocked += 1;
    }
    *next += 31;
    for mut stats in &mut stats {
        stats.restocked += restocked;
    }
}

/// Eases `shown` towards `count` and fades `flash`. Items at rest are not touched, so their rows don't update.
fn animate(time: Res<Time>, mut items: Query<&mut Item>) {
    let dt = time.delta_secs();
    let ease = 1.0 - (-12.0 * dt).exp();
    for mut item in &mut items {
        let target = item.count as f32;
        if item.shown == target && item.flash == 0.0 {
            continue;
        }
        let item = &mut *item;
        item.shown += (target - item.shown) * ease;
        if (target - item.shown).abs() < 0.02 {
            item.shown = target;
        }
        item.flash = (item.flash - dt * 2.0).max(0.0);
    }
}

fn start_ui_update(mut start: ResMut<UiUpdateStart>) {
    start.0 = Instant::now();
}

fn stop_ui_update(start: Res<UiUpdateStart>, mut stats: Query<&mut UiStats>) {
    let micros = start.0.elapsed().as_micros() as u32;
    // idle updates take a few microseconds, they are not what is interesting
    if micros < 20 {
        return;
    }
    for mut stats in &mut stats {
        stats.update_micros = micros;
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BMSPlugin)
        .add_plugins(ReactiveUiPlugin)
        .register_type::<Item>()
        .register_type::<UiStats>()
        .insert_resource(UiUpdateStart(Instant::now()))
        .insert_resource(RestockTimer(Timer::new(
            Duration::from_millis(200),
            TimerMode::Repeating,
        )))
        .add_systems(Startup, setup)
        .add_systems(Update, (restock, animate).chain())
        .add_systems(
            PostUpdate,
            (
                start_ui_update.before(ReactiveUiSystems),
                stop_ui_update.after(ReactiveUiSystems),
            ),
        )
        .run();
}
