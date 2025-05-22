//! Implementations of the [`IntoScript`] trait for various types.

use bevy::reflect::Reflect;
use std::{borrow::Cow, collections::HashMap, ffi::OsString, path::PathBuf};

use super::{DynamicScriptFunction, DynamicScriptFunctionMut, Union, Val};
use crate::{
    bindings::{ReflectReference, ScriptValue, WorldGuard},
    error::InteropError,
};

/// Converts a value into a [`ScriptValue`].
pub trait IntoScript {
    /// Convert this value into a [`ScriptValue`].
    fn into_script(self, world: WorldGuard) -> Result<ScriptValue, InteropError>;

    /// Convert this value into a [`ScriptValue`], returning an error as a ScriptValue if an error occurs.
    fn into_script_inline_error(self, world: WorldGuard) -> ScriptValue
    where
        Self: Sized,
    {
        self.into_script(world).unwrap_or_else(ScriptValue::Error)
    }
}

impl IntoScript for ScriptValue {
    fn into_script(self, _world: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(self)
    }
}

#[profiling::all_functions]
impl IntoScript for () {
    fn into_script(self, _world: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(ScriptValue::Unit)
    }
}

#[profiling::all_functions]
impl IntoScript for DynamicScriptFunctionMut {
    fn into_script(self, _world: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(ScriptValue::FunctionMut(self))
    }
}

#[profiling::all_functions]
impl IntoScript for DynamicScriptFunction {
    fn into_script(self, _world: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(ScriptValue::Function(self))
    }
}

#[profiling::all_functions]
impl IntoScript for bool {
    fn into_script(self, _world: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(ScriptValue::Bool(self))
    }
}

macro_rules! impl_into_with_downcast {
    ($variant:tt as $cast:ty [$($ty:ty),*]) => {
        $(
            #[profiling::all_functions]
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
            #[profiling::all_functions]
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

#[profiling::all_functions]
impl IntoScript for &'static str {
    fn into_script(self, _world: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(ScriptValue::String(Cow::Borrowed(self)))
    }
}

#[profiling::all_functions]
impl IntoScript for ReflectReference {
    fn into_script(self, _world: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(ScriptValue::Reference(self))
    }
}

#[profiling::all_functions]
impl<T: Reflect> IntoScript for Val<T> {
    fn into_script(self, world: WorldGuard) -> Result<ScriptValue, InteropError> {
        let boxed = Box::new(self.0);
        let allocator = world.allocator();
        let mut allocator = allocator.write();

        Ok(ScriptValue::Reference(
            ReflectReference::new_allocated_boxed(boxed, &mut allocator),
        ))
    }
}

#[profiling::all_functions]
impl<T: IntoScript> IntoScript for Option<T> {
    fn into_script(self, world: WorldGuard) -> Result<ScriptValue, InteropError> {
        match self {
            Some(val) => val.into_script(world),
            None => Ok(ScriptValue::Unit),
        }
    }
}

#[profiling::all_functions]
impl<T: IntoScript> IntoScript for Vec<T> {
    fn into_script(self, world: WorldGuard) -> Result<ScriptValue, InteropError> {
        let mut values = Vec::with_capacity(self.len());
        for val in self {
            values.push(val.into_script(world.clone())?);
        }
        Ok(ScriptValue::List(values))
    }
}

#[profiling::all_functions]
impl<T: IntoScript, const N: usize> IntoScript for [T; N] {
    fn into_script(self, world: WorldGuard) -> Result<ScriptValue, InteropError> {
        let mut values = Vec::with_capacity(N);
        for val in self {
            values.push(val.into_script(world.clone())?);
        }
        Ok(ScriptValue::List(values))
    }
}

#[profiling::all_functions]
impl<T1: IntoScript, T2: IntoScript> IntoScript for Union<T1, T2> {
    fn into_script(self, world: WorldGuard) -> Result<ScriptValue, InteropError> {
        match self.into_left() {
            Ok(left) => left.into_script(world),
            Err(right) => right.into_script(world),
        }
    }
}

#[profiling::all_functions]
impl<V: IntoScript> IntoScript for HashMap<String, V> {
    fn into_script(self, world: WorldGuard) -> Result<ScriptValue, InteropError> {
        let mut map = HashMap::new();
        for (key, value) in self {
            map.insert(key, value.into_script(world.clone())?);
        }
        Ok(ScriptValue::Map(map))
    }
}

#[profiling::all_functions]
impl IntoScript for InteropError {
    fn into_script(self, _world: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(ScriptValue::Error(self))
    }
}

macro_rules! impl_into_script_tuple {
    ($( $ty:ident ),* ) => {
        #[allow(non_snake_case)]
        #[profiling::all_functions]
        impl<$($ty: IntoScript),*> IntoScript for ($($ty,)*) {
        fn into_script(self, world: WorldGuard) -> Result<ScriptValue, InteropError> {
            let ($($ty,)*) = self;
            Ok(ScriptValue::List(vec![$($ty.into_script(world.clone())?),*]))
        }
    }
}
}

variadics_please::all_tuples!(impl_into_script_tuple, 1, 14, T);
