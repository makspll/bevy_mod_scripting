extern crate bevy;

pub mod error;
#[cfg(feature = "lua")]
pub mod lua;
pub mod script_ref;
pub mod sub_reflect;
pub mod wrappers;

pub use {script_ref::*, sub_reflect::*};

// re-export derive macros from other langs
pub use bevy_mod_scripting_derive::impl_script_newtype;
#[cfg(feature = "lua")]
pub use bevy_mod_scripting_lua_derive::impl_lua_newtype;

pub(crate) mod generated;

pub use parking_lot;
