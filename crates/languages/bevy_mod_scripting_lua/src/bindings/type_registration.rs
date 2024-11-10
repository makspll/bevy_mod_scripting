use std::sync::Arc;

use bevy::reflect::TypeRegistration;
use bevy_mod_scripting_core::bindings::ScriptTypeRegistration;
use tealr::mlu::TealData;

use crate::impl_userdata_from_lua;

use super::proxy::LuaProxied;

/// Caches information about type data
#[derive(Clone, tealr::mlu::UserData, tealr::ToTypename)]
pub struct LuaTypeRegistration(pub ScriptTypeRegistration);

impl_userdata_from_lua!(LuaTypeRegistration);

impl TealData for LuaTypeRegistration {}

impl From<ScriptTypeRegistration> for LuaTypeRegistration {
    fn from(value: ScriptTypeRegistration) -> Self {
        Self(value)
    }
}

impl LuaProxied for ScriptTypeRegistration {
    type Proxy = LuaTypeRegistration;
}
