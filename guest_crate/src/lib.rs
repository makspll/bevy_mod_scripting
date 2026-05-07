//! BMS WebAssembly Guest Example  
//!   
//! This example demonstrates spawning an entity with a Transform component  
//! using the wasmtime component model integration with bevy_mod_scripting.  

use wit_bindgen::rt::*;

// Import the generated WIT bindings
// This would be generated from the WIT file produced by the WITLadBackendPlugin
wit_bindgen::generate!({
    world: "bms-guest",
    path: "bindings.wit",
});

struct MyHost;

impl Guest for MyHost {
    /// Called when the script is first loaded  
    fn on_script_loaded() {
        println!("[Rust Guest] Script loaded!");
        panic!("asd");

        // // Spawn an entity with Transform
        // match spawn_entity_with_transform() {
        //     Ok(entity_id) => println!("[Rust Guest] Spawned entity with ID: {:?}", entity_id),
        //     Err(e) => println!("[Rust Guest] Failed to spawn entity: {:?}", e),
        // }
    }

    /// Called when the script is unloaded  
    fn on_script_unloaded() {
        println!("[Rust Guest] Script unloaded!");
    }

    /// Called every frame update  
    fn on_update() {
        // Update logic here
        // For this example, we'll just print a message every 60 frames (approximately 1 second)
        static mut FRAME_COUNTER: u32 = 0;
        unsafe {
            FRAME_COUNTER += 1;
            if FRAME_COUNTER % 60 == 0 {
                println!("[Rust Guest] Update tick: {}", FRAME_COUNTER);
            }
        }
    }

    fn on_custom_event() -> () {
        println!("[Rust Guest] Script custom event!");
    }
}

// export! defines that the `MyHost` struct defined below is going to define
// the exports of the `world`, namely the `run` function.
export!(MyHost);
