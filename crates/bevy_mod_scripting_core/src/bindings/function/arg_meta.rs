//! Trait implementations to help with function dispatch.

use std::{ffi::OsString, path::PathBuf};

use bevy_platform::collections::HashMap;

use crate::{
    bindings::{ReflectReference, ScriptValue},
    docgen::TypedThrough,
    error::InteropError,
};

use super::{
    from::{FromScript, Mut, Ref, Union, Val},
    into::IntoScript,
    script_function::{DynamicScriptFunction, DynamicScriptFunctionMut, FunctionCallContext},
    type_dependencies::GetTypeDependencies,
};

/// Marker trait for types that can be used as arguments to a script function.
pub trait ScriptArgument: ArgMeta + FromScript + GetTypeDependencies {}
impl<T: ArgMeta + FromScript + GetTypeDependencies> ScriptArgument for T {}

/// Marker trait for types that can be used as arguments to a script function. And contain type information.
pub trait TypedScriptArgument: TypedThrough + ScriptArgument {}
impl<T: TypedThrough + ScriptArgument> TypedScriptArgument for T {}

/// Marker trait for types that can be used as return values from a script function.
pub trait ScriptReturn: IntoScript + GetTypeDependencies {}
impl<T: IntoScript + GetTypeDependencies> ScriptReturn for T {}

/// Marker trait for types that can be used as return values from a script function. And contain type information.
pub trait TypedScriptReturn: TypedThrough + ScriptReturn {}
impl<T: TypedThrough + ScriptReturn> TypedScriptReturn for T {}

/// Describes an argument to a script function. Provides necessary information for the function to handle dispatch.
pub trait ArgMeta {
    /// The default value for the argument. Used when the argument is not provided.
    fn default_value() -> Option<ScriptValue> {
        None
    }
}

impl ArgMeta for ScriptValue {}

macro_rules! impl_arg_info {
    ($($ty:ty),*) => {
        $(
            impl ArgMeta for $ty {}
        )*
    };
}

impl_arg_info!(
    bool,
    i8,
    i16,
    i32,
    i64,
    i128,
    u8,
    u16,
    u32,
    u64,
    u128,
    f32,
    f64,
    usize,
    isize,
    String,
    PathBuf,
    OsString,
    char,
    ReflectReference,
    &'static str
);

impl<T1, T2> ArgMeta for Union<T1, T2> {}
impl<T> ArgMeta for Val<T> {}
impl<T> ArgMeta for Ref<'_, T> {}
impl<T> ArgMeta for Mut<'_, T> {}

impl<T> ArgMeta for Result<T, InteropError> {}

impl<T> ArgMeta for Option<T> {
    fn default_value() -> Option<ScriptValue> {
        Some(ScriptValue::Unit)
    }
}

impl<T> ArgMeta for Vec<T> {}
impl<T, const N: usize> ArgMeta for [T; N] {}

impl<K, V> ArgMeta for HashMap<K, V> {}
impl<K, V> ArgMeta for std::collections::HashMap<K, V> {}

impl_arg_info!(DynamicScriptFunction, DynamicScriptFunctionMut);

impl ArgMeta for () {
    fn default_value() -> Option<ScriptValue> {
        Some(ScriptValue::Unit)
    }
}
impl<T> ArgMeta for (T,) {}
impl<T1, T2> ArgMeta for (T1, T2) {}
impl<T1, T2, T3> ArgMeta for (T1, T2, T3) {}
impl<T1, T2, T3, T4> ArgMeta for (T1, T2, T3, T4) {}
impl<T1, T2, T3, T4, T5> ArgMeta for (T1, T2, T3, T4, T5) {}
impl<T1, T2, T3, T4, T5, T6> ArgMeta for (T1, T2, T3, T4, T5, T6) {}
impl<T1, T2, T3, T4, T5, T6, T7> ArgMeta for (T1, T2, T3, T4, T5, T6, T7) {}
impl<T1, T2, T3, T4, T5, T6, T7, T8> ArgMeta for (T1, T2, T3, T4, T5, T6, T7, T8) {}
impl<T1, T2, T3, T4, T5, T6, T7, T8, T9> ArgMeta for (T1, T2, T3, T4, T5, T6, T7, T8, T9) {}
impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> ArgMeta
    for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10)
{
}
impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> ArgMeta
    for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11)
{
}
impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> ArgMeta
    for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12)
{
}
impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> ArgMeta
    for (T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13)
{
}

impl ArgMeta for FunctionCallContext {}
