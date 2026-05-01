//! Example demonstrating how to run WebAssembly scripts using the wasmtime component model plugin.  
//!  
//! This example shows:  
//! 1. Setting up the wasmtime scripting plugin  
//! 2. Loading a WASM component script  
//! 3. Attaching it to an entity  
//! 4. Handling script callbacks  

use bevy::prelude::*;
use bevy_mod_scripting::prelude::*;
use bevy_mod_scripting_core::event::ScriptCallbackResponseEvent;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // Add the wasmtime scripting plugin
            WasmtimeScriptingPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (trigger_events, handle_script_events))
        .run();
}

/// Setup system that loads and attaches the WASM script  
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Setting up WASM script example...");

    // Load the WASM component script
    // This should be a .component.wasm file compiled from a guest crate
    let wasm_handle: Handle<ScriptAsset> = asset_server.load("scripts/example.component.wasm");

    // Spawn an entity and attach the WASM script to it
    commands.spawn((
        ScriptComponent::new([wasm_handle]),
        // Add any other components the script might need to interact with
        Name::new("WasmScriptEntity".to_string()),
    ));

    println!("WASM script loaded and attached to entity");
}

/// System that triggers events for scripts to handle  
fn trigger_events(mut writer: MessageWriter<ScriptCallbackEvent>) {
    // Send a custom event every 60 frames (approximately 1 second at 60 FPS)
    static mut FRAME_COUNTER: u32 = 0;
    unsafe {
        FRAME_COUNTER += 1;
        if FRAME_COUNTER % 60 == 0 {
            println!("Triggering script event...");

            // Define a callback label
            callback_labels!(OnCustomEvent => "on_custom_event");

            // Send the event to all scripts
            writer.send(ScriptCallbackEvent::new_for_all_scripts(
                OnCustomEvent,
                vec![ScriptValue::String("Hello from Rust!".into())],
            ));
        }
    }
}

/// System that handles responses from scripts  
fn handle_script_events(mut reader: MessageReader<ScriptCallbackResponseEvent>) {
    for event in reader.read() {
        match &event.response {
            Ok(value) => {
                println!("Script responded with: {:?}", value);
            }
            Err(error) => {
                println!("Script error: {:?}", error);
            }
        }
    }
}
