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

impl TealData for LuaTypeRegistration {
    fn add_fields<'lua, F: tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut F) {
        fields.document("The [short name](https://docs.rs/bevy/latest/bevy/reflect/struct.TypeRegistration.html#method.get_short_name) of a type");
        fields.add_field_method_get("short_name", |_, s| Ok(s.0.short_name().to_owned()));

        fields.document("The full name of the type");
        fields.add_field_method_get("type_name", |_, s| Ok(s.0.type_name()));
    }
}

impl From<ScriptTypeRegistration> for LuaTypeRegistration {
    fn from(value: ScriptTypeRegistration) -> Self {
        Self(value)
    }
}

impl From<&LuaTypeRegistration> for ScriptTypeRegistration {
    fn from(value: &LuaTypeRegistration) -> Self {
        value.0.clone()
    }
}

impl LuaProxied for ScriptTypeRegistration {
    type Proxy = LuaTypeRegistration;
}
