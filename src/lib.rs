#![doc=include_str!("../readme.md")]

pub mod core {
    pub use bevy_mod_scripting_core::*;
}

#[cfg(feature = "lua")]
pub mod lua {
    pub use bevy_mod_scripting_lua::*;
}

// #[cfg(feature = "rhai")]
// pub mod rhai {
//     pub use bevy_mod_scripting_rhai::*;
// }

// #[cfg(feature = "rune")]
// pub mod rune {
//     pub use bevy_mod_scripting_rune::*;
// }

pub use bevy_mod_scripting_functions::*;
