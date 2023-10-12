extern crate bevy;

pub mod error;
#[cfg(feature = "lua")]
pub mod lua;
#[cfg(feature = "rhai")]
pub mod rhai;

pub mod common;

pub mod script_ref;
pub mod sub_reflect;
pub mod wrappers;

pub use {script_ref::*, sub_reflect::*};

pub mod prelude {
    #[cfg(feature = "lua")]
    pub use crate::{
        impl_lua_newtype,
        lua::{
            bevy::LuaBevyAPIProvider, std::LuaVec, FromLuaProxy, LuaProxyable, ReflectLuaProxyable,
            ToLuaProxy,
        },
    };

    #[cfg(feature = "rhai")]
    pub use crate::rhai::{
        bevy::RhaiBevyAPIProvider,
        std::{RhaiCopy, RhaiVec},
        FromRhaiProxy, ReflectRhaiProxyable, RhaiProxyable, ToRhaiProxy,
    };

    pub use crate::{common::bevy::GetWorld, impl_script_newtype, ValueIndex};
}

// re-export derive macros from other langs
pub use bevy_mod_scripting_derive::impl_script_newtype;
#[cfg(feature = "lua")]
pub use bevy_mod_scripting_lua_derive::impl_lua_newtype; //LuaProxy};

#[cfg(not(any(target_feature = "sse2", target_feature = "simd128")))]
pub(crate) mod generated_scalar;

#[cfg(any(target_feature = "sse2", target_feature = "simd128"))]
pub(crate) mod generated_simd;

pub use parking_lot;
