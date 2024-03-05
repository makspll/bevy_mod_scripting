#![doc=include_str!("../readme.md")]

pub mod core {
    pub use bevy_mod_scripting_core::*;
}

#[cfg(feature = "lua")]
pub mod lua {
    pub use bevy_mod_scripting_lua::*;

    #[cfg(feature = "lua_script_api")]
    pub mod api {
        pub use bevy_script_api::lua::*;
    }
}

#[cfg(feature = "rhai")]
pub mod rhai {
    pub use bevy_mod_scripting_rhai::*;

    #[cfg(feature = "rhai_script_api")]
    pub mod api {
        pub use bevy_script_api::rhai::*;
    }
}

#[cfg(feature = "rune")]
pub mod rune {
    pub use bevy_mod_scripting_rune::*;
}

#[cfg(any(feature = "lua_script_api", feature = "rhai_script_api"))]
pub mod api {
    pub use bevy_script_api::*;
}

pub mod prelude {
    pub use bevy_mod_scripting_core::prelude::*;

    #[cfg(feature = "lua")]
    pub use bevy_mod_scripting_lua::prelude::*;

    #[cfg(feature = "rhai")]
    pub use bevy_mod_scripting_rhai::prelude::*;

    #[cfg(feature = "rune")]
    pub use bevy_mod_scripting_rune::prelude::*;

    #[cfg(any(feature = "lua_script_api", feature = "rhai_script_api"))]
    pub use bevy_script_api::prelude::*;
}
