use std::sync::Arc;

use bevy::reflect::TypeRegistration;
use tealr::mlu::TealData;

use crate::impl_userdata_from_lua;

/// Caches information about type data
#[derive(Clone, tealr::mlu::UserData, tealr::ToTypename)]
pub struct LuaTypeRegistration(Arc<TypeRegistration>);

impl_userdata_from_lua!(LuaTypeRegistration);

impl TealData for LuaTypeRegistration {}
