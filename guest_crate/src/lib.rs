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

// The guest exports
struct Guest;

impl Guest {
    /// Called when the script is first loaded  
    fn on_script_loaded() {
        println!("[Rust Guest] Script loaded!");

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
}

// /// Spawn an entity with a Transform component
// fn spawn_entity_with_transform() -> Result<(), String> {
//     // Spawn a new entity
//     let entity = world::spawn_entity().map_err(|e| format!("Failed to spawn entity: {:?}", e))?;

//     // Get the Transform component registration
//     // In a real scenario, you'd get this from the component registry
//     let transform_registration = ComponentRegistration {
//         // This would be populated with the actual component registration
//         // For now, we'll assume it's available

//     };

//     // Add Transform component to the entity
//     world::add_component(
//         &entity,
//         &transform_registration,
//         &Transform {
//             translation: Vec3 {
//                 x: 0.0,
//                 y: 1.0,
//                 z: 0.0,
//             },
//             rotation: Quat {
//                 x: 0.0,
//                 y: 0.0,
//                 z: 0.0,
//                 w: 1.0,
//             },
//             scale: Vec3 {
//                 x: 1.0,
//                 y: 1.0,
//                 z: 1.0,
//             },
//         },
//     )
//     .map_err(|e| format!("Failed to add Transform component: {:?}", e))?;

//     println!("[Rust Guest] Successfully spawned entity at position (0, 1, 0)");
//     Ok(())
// }

// // Component types (these would be generated from the WIT file)
// #[derive(Clone, Debug)]
// pub struct Vec3 {
//     pub x: f32,
//     pub y: f32,
//     pub z: f32,
// }

// #[derive(Clone, Debug)]
// pub struct Quat {
//     pub x: f32,
//     pub y: f32,
//     pub z: f32,
//     pub w: f32,
// }

// #[derive(Clone, Debug)]
// pub struct Transform {
//     pub translation: Vec3,
//     pub rotation: Quat,
//     pub scale: Vec3,
// }

// #[derive(Clone, Debug)]
// pub struct ComponentRegistration {
//     // Component registration details
//     // This would be properly defined based on the WIT interface
// }
