use bevy::prelude::*;
use bevy_mod_scripting::lua::*;
use bevy_mod_scripting::prelude::*;
use bevy_mod_scripting_core::pipeline::ScriptPipelineState;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    ScriptLoading,
    Running,
}

// create a large number of scripts which will take some time to process
pub fn initialize_script_loading(
    // mut script_assets: ResMut<Assets<ScriptAsset>>,
    asset_server: ResMut<AssetServer>,
    mut commands: Commands,
) {
    for _ in 0..1000 {
        // you can create assets in memory instead of loading
        // the result is the same, apart from the fact your strong handles won't contain path information
        let script = ScriptAsset::new(
            "
            function on_script_loaded()
            end
            "
            .to_string(),
        );
        let handle = asset_server.add(script);
        // in this case though, reusing the same asset means you only get one logical "script" and one asset handle
        // we want to simulate many unique scripts
        // let handle = asset_server
        //     .load(AssetPath::from_static("scripts/dummy.lua"));
        commands.spawn(ScriptComponent(vec![handle]));
    }
}

/// Marker component for the UI root
#[derive(Component)]
struct LoadingUiRoot;

/// Marker components for UI updates
#[derive(Component)]
struct LoadingBarFill;

#[derive(Component)]
struct ScriptNameText;

#[derive(Component)]
struct PercentageText;

/// Creates a centered loading UI
fn setup_loading_ui(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(400.0),
                        height: Val::Px(120.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(10.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.3)),
                ))
                .with_children(|column| {
                    // Script name text
                    column.spawn((
                        Text::new("Loading: (none)"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        ScriptNameText,
                    ));

                    // Bar background
                    column
                        .spawn((
                            Node {
                                width: Val::Px(300.0),
                                height: Val::Px(24.0),
                                justify_content: JustifyContent::FlexStart,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            BackgroundColor(Color::darker(&Color::WHITE, 0.5)),
                        ))
                        .with_children(|bar_bg| {
                            // Filled portion of bar
                            bar_bg.spawn((
                                Node {
                                    width: Val::Percent(0.0), // updated dynamically
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                BackgroundColor(Color::linear_rgb(0.0, 1.0, 0.0)),
                                LoadingBarFill,
                            ));
                        });

                    // Percentage text
                    column.spawn((
                        Text::new("0%"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        PercentageText,
                    ));
                });
        });
}

/// Updates the loading UI elements based on progress
fn update_loading_ui(
    pipeline_state: ScriptPipelineState<LuaScriptingPlugin>,
    mut fill_query: Query<&mut Node, With<LoadingBarFill>>,
    mut name_query: Query<&mut Text, (With<ScriptNameText>, Without<PercentageText>)>,
    mut percent_query: Query<&mut Text, (With<PercentageText>, Without<ScriptNameText>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // this is the progress of the currently loading batch of scripts
    // if some scripts are reloading these will be included in the count
    let (progress, loaded, total) = pipeline_state.progress();

    let current_script = pipeline_state
        .currently_loading_script()
        .and_then(|handle| handle.path().cloned())
        .map(|p| p.to_string())
        .unwrap_or(String::from("script"));

    // Update bar fill width
    if let Ok(mut node) = fill_query.single_mut() {
        node.width = Val::Percent(progress);
    }

    // Update script name
    if let Ok(mut text) = name_query.single_mut() {
        *text = Text::new(format!("Loading: {current_script}"));
    }

    // Update percentage text
    if let Ok(mut text) = percent_query.single_mut() {
        *text = Text::new(format!("{progress:.0}% ({loaded}/{total})"));
    }

    if pipeline_state.processing_batch_completed() && loaded > 0 {
        next_state.set(GameState::Running)
    }
}

fn set_loading_ui_completed(
    mut name_query: Query<&mut Text, (With<ScriptNameText>, Without<PercentageText>)>,
) {
    // Update script name
    if let Ok(mut text) = name_query.single_mut() {
        *text = Text::new("Loaded all scripts!".to_string());
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BMSPlugin)
        .init_state::<GameState>()
        .add_systems(Startup, (initialize_script_loading, setup_loading_ui))
        .add_systems(
            Update,
            update_loading_ui.run_if(in_state(GameState::ScriptLoading)),
        )
        .add_systems(OnEnter(GameState::Running), set_loading_ui_completed)
        .run();
}
