extern crate bevy;

use crate::{impl_tealr_type};
use anyhow::Result;
use tealr::{mlu::{mlua,mlua::{prelude::*,Value,UserData,MetaMethod}, TealData, TealDataMethods}, TypeName};
use ::std::fmt::Debug;

use ::std::{ops::{Deref,DerefMut, Index},sync::Weak, borrow::Cow, marker::PhantomData};
use parking_lot::{RwLock};
use bevy::{
    prelude::*,
    reflect::{ReflectRef, TypeRegistry, GetPath, TypeData}, ecs::component::ComponentId,
};
use ::std::{
    sync::Arc,
    cell::Ref,
    fmt,
};

pub mod generated;
pub mod wrappers;
pub mod lua;
pub mod script_ref;
pub mod sub_reflect;

pub use {wrappers::*,lua::*, script_ref::*, sub_reflect::*};

