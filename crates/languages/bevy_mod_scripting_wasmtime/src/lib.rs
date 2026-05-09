//! Wasmtime Component Model scripting plugin for bevy_mod_scripting.  
//!  
//! Guests compile to WebAssembly components using WIT bindings generated from the LAD file.  
//! The host implements all BMS functions as component model imports, with ReflectReferences  
//! represented as opaque resource handles.  
#![allow(
    clippy::all,
    missing_docs,
    clippy::expect_used,
    clippy::panic,
    clippy::todo,
    clippy::unwrap_used,
    reason = "cooking"
)]

mod context;
mod handler;
mod plugin;
mod runtime;
pub use context::*;
pub use handler::*;
pub use plugin::*;
pub use runtime::*;
