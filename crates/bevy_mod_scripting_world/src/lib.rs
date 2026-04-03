//! Abstractions for interacting with the bevy world without knowing compile time type information safely.

mod access_map;
mod world;

pub use access_map::*;
pub use world::*;
