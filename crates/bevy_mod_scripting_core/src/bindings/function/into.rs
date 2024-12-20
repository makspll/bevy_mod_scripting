use std::{
    borrow::Cow,
    ffi::OsString,
    path::{Path, PathBuf},
};

use bevy::reflect::{GetTypeRegistration, PartialReflect, ReflectRef};

use crate::{
    bindings::{ReflectReference, WorldGuard},
    error::InteropError,
    prelude::ScriptValue,
};

use super::from::Val;

pub trait IntoScript {
    fn into_script(self, world: WorldGuard) -> Result<ScriptValue, InteropError>;
}

impl IntoScript for ScriptValue {
    fn into_script(self, _world: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(self)
    }
}

impl IntoScript for () {
    fn into_script(self, _world: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(ScriptValue::Unit)
    }
}

impl IntoScript for bool {
    fn into_script(self, world: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(ScriptValue::Bool(self))
    }
}

macro_rules! impl_into_with_downcast {
    ($variant:tt as $cast:ty [$($ty:ty),*]) => {
        $(
            impl IntoScript for $ty {
                fn into_script(self, _world: WorldGuard) -> Result<ScriptValue, InteropError> {
                    Ok(ScriptValue::$variant(self as $cast))
                }
            }
        )*
    }

}

impl_into_with_downcast!(Integer as i64 [i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize, isize]);
impl_into_with_downcast!(Float as f64 [f32, f64]);

macro_rules! impl_into_stringlike {
    ($id:ident,[ $(($ty:ty => $conversion:expr)),*]) => {
        $(
            impl IntoScript for $ty {
                fn into_script(self, _world: WorldGuard) -> Result<ScriptValue, InteropError> {
                    let $id = self;
                    let converted: String = $conversion;
                    Ok(ScriptValue::String(converted.into()))
                }
            }
        )*
    }
}

impl_into_stringlike!(
    s,
    [
        (String => s),
        (char => s.to_string()),
        (PathBuf => s.to_string_lossy().to_string()),
        (OsString => s.into_string().map_err(|e| InteropError::unsupported_operation(None, Some(Box::new(e)), "Could not convert OsString to String".to_owned()))?)
    ]
);

impl IntoScript for &'static str {
    fn into_script(self, _world: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(ScriptValue::String(Cow::Borrowed(self)))
    }
}

impl IntoScript for ReflectReference {
    fn into_script(self, _world: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(ScriptValue::Reference(self))
    }
}

impl<T: PartialReflect> IntoScript for Val<T> {
    fn into_script(self, world: WorldGuard) -> Result<ScriptValue, InteropError> {
        let boxed: Box<dyn PartialReflect> = Box::new(self.0);
        let allocator = world.allocator();
        let mut allocator = allocator.write();

        Ok(ScriptValue::Reference(
            ReflectReference::new_allocated_boxed(boxed, &mut allocator),
        ))
    }
}

impl<T: IntoScript> IntoScript for Option<T> {
    fn into_script(self, world: WorldGuard) -> Result<ScriptValue, InteropError> {
        match self {
            Some(val) => val.into_script(world),
            None => Ok(ScriptValue::Unit),
        }
    }
}

impl<T: IntoScript> IntoScript for Vec<T> {
    fn into_script(self, world: WorldGuard) -> Result<ScriptValue, InteropError> {
        let mut values = Vec::with_capacity(self.len());
        for val in self {
            values.push(val.into_script(world.clone())?);
        }
        Ok(ScriptValue::List(values))
    }
}

impl IntoScript for InteropError {
    fn into_script(self, _world: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(ScriptValue::Error(self))
    }
}
