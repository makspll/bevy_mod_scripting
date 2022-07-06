#![allow(unused_variables,unused_parens)]
use bevy::reflect::TypeData;
use bevy::reflect::TypeRegistry;
use bevy::prelude::*;
use parking_lot::RwLock;
use std::sync::Weak;
use std::{fmt,fmt::{Debug}};
use phf::{phf_map, Map};
use crate::ScriptRefBase;
use crate::PrintableReflect;
use crate::ScriptRef;
use crate::{ReflectPtr,lua::LuaEntity};
use crate::util::impl_tealr_type;

use bevy_mod_scripting_derive::{impl_lua_newtypes,replace};
use tealr::{mlu::{mlua,TealDataMethods,TealData,mlua::{prelude::*,Error,MetaMethod,Value}}};
use std::ops::Deref;

