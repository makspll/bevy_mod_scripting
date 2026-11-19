//! Implementations of the [`IntoScript`] trait for various types.

use super::{Union, V};
use crate::function::{DynamicScriptFunction, DynamicScriptFunctionMut};
use crate::native::{ReturnSink, native_vtable};
use crate::{ReflectReference, ScriptValue, VariadicTuple, error::InteropError};
use bevy_mod_scripting_world::WorldGuard;
use bevy_platform::collections::HashMap;
use bevy_reflect::Reflect;
use std::{borrow::Cow, collections::VecDeque, ffi::OsString, path::PathBuf};

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

    /// Returns this value from a typed call (see [`crate::native`]), in its native form if it has one.
    ///
    /// The default converts it with [`Self::into_script`], types with a faster way override it.
    fn into_ret(self, ret: &mut dyn ReturnSink, world: WorldGuard) -> Result<(), InteropError>
    where
        Self: Sized,
    {
        ret.script_value(self.into_script(world)?)
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

    fn into_ret(self, ret: &mut dyn ReturnSink, _world: WorldGuard) -> Result<(), InteropError> {
        ret.unit();
        Ok(())
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

    fn into_ret(self, ret: &mut dyn ReturnSink, _world: WorldGuard) -> Result<(), InteropError> {
        ret.boolean(self);
        Ok(())
    }
}

macro_rules! impl_into_with_downcast {
    ($variant:tt as $cast:ty, $ret:ident [$($ty:ty),*]) => {
        $(
            #[profiling::all_functions]
            impl IntoScript for $ty {
                fn into_script(self, _world: WorldGuard) -> Result<ScriptValue, InteropError> {
                    Ok(ScriptValue::$variant(self as $cast))
                }

                fn into_ret(self, ret: &mut dyn ReturnSink, _world: WorldGuard) -> Result<(), InteropError> {
                    ret.$ret(self as $cast);
                    Ok(())
                }
            }
        )*
    }

}

impl_into_with_downcast!(Integer as i64, integer [i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize, isize]);
impl_into_with_downcast!(Float as f64, number [f32, f64]);

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
impl<T: Reflect> IntoScript for V<T> {
    fn into_script(self, _world: WorldGuard) -> Result<ScriptValue, InteropError> {
        // one heap allocation, holding the value and its reference counts
        Ok(ScriptValue::Reference(ReflectReference::new_allocated(
            self.0,
        )))
    }

    fn into_ret(self, ret: &mut dyn ReturnSink, world: WorldGuard) -> Result<(), InteropError> {
        // types stored natively are moved into the language's storage, others are allocated as usual
        if let Some(vtable) = native_vtable(&world, std::any::TypeId::of::<T>()) {
            let mut value = Some(self.0);
            let written = ret.native(vtable, &mut |slot| {
                if let Some(value) = value.take() {
                    // Safety: the slot is memory for a value of the vtable's type, which is T
                    unsafe { (slot as *mut T).write(value) }
                }
            });
            match value {
                // not written: the language has no native form for it
                Some(value) if !written => {
                    return V(value)
                        .into_script(world)
                        .and_then(|v| ret.script_value(v));
                }
                _ => return Ok(()),
            }
        }
        ret.script_value(self.into_script(world)?)
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

    fn into_ret(self, ret: &mut dyn ReturnSink, world: WorldGuard) -> Result<(), InteropError> {
        match self {
            Some(val) => val.into_ret(ret, world),
            None => {
                ret.unit();
                Ok(())
            }
        }
    }
}

#[profiling::all_functions]
impl<T: IntoScript> IntoScript for Vec<T> {
    fn into_script(self, world: WorldGuard) -> Result<ScriptValue, InteropError> {
        let mut values = VecDeque::with_capacity(self.len());
        for val in self {
            values.push_back(val.into_script(world.clone())?);
        }
        Ok(ScriptValue::List(values))
    }
}

#[profiling::all_functions]
impl<T: IntoScript> IntoScript for VecDeque<T> {
    fn into_script(self, world: WorldGuard) -> Result<ScriptValue, InteropError> {
        let mut values = VecDeque::with_capacity(self.len());
        for val in self {
            values.push_back(val.into_script(world.clone())?);
        }
        Ok(ScriptValue::List(values))
    }
}

#[profiling::all_functions]
impl<T: IntoScript, const N: usize> IntoScript for [T; N] {
    fn into_script(self, world: WorldGuard) -> Result<ScriptValue, InteropError> {
        let mut values = VecDeque::with_capacity(N);
        for val in self {
            values.push_back(val.into_script(world.clone())?);
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

    fn into_ret(self, ret: &mut dyn ReturnSink, world: WorldGuard) -> Result<(), InteropError> {
        match self.into_left() {
            Ok(left) => left.into_ret(ret, world),
            Err(right) => right.into_ret(ret, world),
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
impl<V: IntoScript> IntoScript for std::collections::HashMap<String, V> {
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

impl IntoScript for VariadicTuple {
    fn into_script(self, _world: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(ScriptValue::Tuple(self))
    }
}

macro_rules! impl_into_script_tuple {
    ($( $ty:ident ),* ) => {
        #[allow(non_snake_case)]
        #[profiling::all_functions]
        impl<$($ty: IntoScript),*> IntoScript for ($($ty,)*) {
        fn into_script(self, world: WorldGuard) -> Result<ScriptValue, InteropError> {
            let ($($ty,)*) = self;
            Ok(ScriptValue::Tuple(crate::script_value::VariadicTuple(VecDeque::from_iter([$($ty.into_script(world.clone())?),*].into_iter()))))
        }
    }
}
}

variadics_please::all_tuples!(impl_into_script_tuple, 1, 14, T);
