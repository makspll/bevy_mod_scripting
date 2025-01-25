//! Trait implementations to help with function dispatch.

use std::{ffi::OsString, path::PathBuf};

use crate::bindings::{script_value::ScriptValue, ReflectReference};

use super::{
    from::{FromScript, Mut, Ref, Val},
    into::IntoScript,
    script_function::{DynamicScriptFunction, DynamicScriptFunctionMut, GetInnerTypeDependencies},
};

/// Marker trait for types that can be used as arguments to a script function.
pub trait ScriptArgument: ArgInfo + FromScript + GetInnerTypeDependencies {}
impl<T: ArgInfo + FromScript + GetInnerTypeDependencies> ScriptArgument for T {}

/// Marker trait for types that can be used as return values from a script function.
pub trait ScriptReturn: IntoScript + GetInnerTypeDependencies {}

/// Describes an argument to a script function. Provides necessary information for the function to handle dispatch.
pub trait ArgInfo {
    fn default_value() -> Option<ScriptValue> {
        None
    }
}

impl ArgInfo for ScriptValue {}

impl ArgInfo for () {
    fn default_value() -> Option<ScriptValue> {
        Some(ScriptValue::Unit)
    }
}

macro_rules! impl_arg_info {
    ($($ty:ty),*) => {
        $(
            impl ArgInfo for $ty {}
        )*
    };
}

impl_arg_info!(bool, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64, usize, isize);

impl_arg_info!(String, PathBuf, OsString);

impl_arg_info!(char);

impl_arg_info!(ReflectReference);

impl<T> ArgInfo for Val<T> {}
impl<T> ArgInfo for Ref<'_, T> {}
impl<T> ArgInfo for Mut<'_, T> {}

impl<T> ArgInfo for Option<T> {
    fn default_value() -> Option<ScriptValue> {
        Some(ScriptValue::Unit)
    }
}

impl<T> ArgInfo for Vec<T> {}
impl<T, const N: usize> ArgInfo for [T; N] {}

impl_arg_info!(DynamicScriptFunction, DynamicScriptFunctionMut);
