#![doc=include_str!("../readme.md")]

pub mod core {
    pub use bevy_mod_scripting_core::*;
}

#[cfg(feature = "lua")]
pub mod lua {
    pub use bevy_mod_scripting_lua::*;
}

#[cfg(feature = "rhai")]
pub mod rhai {
    pub use bevy_mod_scripting_rhai::*;
}

#[cfg(feature = "rune")]
pub mod rune {
    pub use bevy_mod_scripting_rune::*;
}

pub mod prelude {
    pub use bevy_mod_scripting_core::prelude::*;

    #[cfg(feature = "lua")]
    pub use bevy_mod_scripting_lua::prelude::*;

    #[cfg(feature = "rhai")]
    pub use bevy_mod_scripting_rhai::prelude::*;

    // #[cfg(feature = "rune")]
    // pub use bevy_mod_scripting_rune::prelude::*;
}

pub use bevy_mod_scripting_functions::*;
