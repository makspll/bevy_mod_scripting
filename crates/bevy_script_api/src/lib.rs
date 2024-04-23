extern crate bevy;

pub mod error;
#[cfg(feature = "lua")]
pub mod lua;
#[cfg(feature = "rhai")]
pub mod rhai;
#[cfg(feature = "rune")]
pub mod rune;

pub mod common;

#[cfg(feature = "lua")]
pub mod core_providers;
// for now providers do not support any other lang so just remove this whole module if they are not needed
#[cfg(feature = "lua")]
pub mod providers;

pub mod script_ref;
pub mod sub_reflect;
pub mod wrappers;

pub use {script_ref::*, sub_reflect::*};

pub mod prelude {
    #[cfg(feature = "lua")]
    pub use crate::{
        core_providers::LuaCoreBevyAPIProvider,
        lua::{std::LuaVec, FromLuaProxy, IntoLuaProxy, LuaProxyable, ReflectLuaProxyable},
        providers::LuaBevyAPIProvider,
        LuaProxy,
    };

    #[cfg(feature = "rhai")]
    pub use crate::rhai::{
        bevy::RhaiBevyAPIProvider,
        std::{RhaiCopy, RhaiVec},
        FromRhaiProxy, ReflectRhaiProxyable, RhaiProxyable, ToRhaiProxy,
    };

    #[cfg(feature = "rune")]
    pub use crate::rune;

    pub use crate::{common::bevy::GetWorld, ValueIndex};
}

#[cfg(feature = "lua")]
pub use bevy_mod_scripting_lua_derive::LuaProxy;

pub use parking_lot;
