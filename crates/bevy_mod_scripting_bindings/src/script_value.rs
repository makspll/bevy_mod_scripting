//! This module contains the `ScriptValue` enum which is used to pass values between scripting languages and Rust.

use crate::error::InteropError;
use bevy_mod_scripting_derive::DebugWithTypeInfo;
use bevy_mod_scripting_display::{
    DisplayWithTypeInfo, GetTypeInfo, ReflectDisplayWithTypeInfo, WithTypeInfo,
};
use bevy_platform::collections::HashMap;
use bevy_reflect::Reflect;
use std::borrow::Cow;

use super::{
    ReflectReference,
    function::script_function::{DynamicScriptFunction, DynamicScriptFunctionMut},
};

/// An abstraction of values that can be passed to and from scripts.
/// This allows us to re-use logic between scripting languages.
#[derive(Clone, Reflect, Default, DebugWithTypeInfo)]
#[reflect(opaque, DisplayWithTypeInfo)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
pub enum ScriptValue {
    /// Represents the absence of a value.
    #[default]
    Unit,
    /// Represents a boolean value.
    Bool(bool),
    /// Represents an integer value with at most 64 bits.
    Integer(i64),
    /// Represents a floating point value with at most 64 bits.
    Float(f64),
    /// Represents a string value.
    String(Cow<'static, str>),
    /// Represents a list of other things passed by value
    List(Vec<ScriptValue>),
    /// Represents a map of other things passed by value
    Map(HashMap<String, ScriptValue>),
    /// Represents a reference to a value.
    Reference(ReflectReference),
    /// A dynamic script function possibly storing state. Preffer using the [`ScriptValue::Function`] variant instead if possible.
    FunctionMut(DynamicScriptFunctionMut),
    /// A stateless dynamic script function
    Function(DynamicScriptFunction),
    /// Represents any error, will be thrown when returned to a script.
    Error(InteropError),
}

impl DisplayWithTypeInfo for ScriptValue {
    fn display_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        match self {
            ScriptValue::Unit => f.write_str("()"),
            ScriptValue::Bool(v) => write!(f, "{v}"),
            ScriptValue::Integer(v) => write!(f, "{v}"),
            ScriptValue::Float(v) => write!(f, "{v}"),
            ScriptValue::String(v) => write!(f, "{v}"),
            ScriptValue::List(v) => {
                f.write_str("[")?;
                let mut first = true;
                for item in v {
                    if !first {
                        f.write_str(", ")?;
                    }
                    first = false;
                    WithTypeInfo::new_with_opt_info(item, type_info_provider)
                        .display_with_type_info(f, type_info_provider)?;
                }
                f.write_str("]")
            }
            ScriptValue::Map(v) => {
                f.write_str("{")?;
                let mut first = true;
                for (key, value) in v {
                    if !first {
                        f.write_str(", ")?;
                    }
                    first = false;
                    write!(f, "{key}: ")?;
                    WithTypeInfo::new_with_opt_info(value, type_info_provider)
                        .display_with_type_info(f, type_info_provider)?;
                }
                f.write_str("}")
            }
            ScriptValue::Reference(v) => v.display_with_type_info(f, type_info_provider),
            ScriptValue::FunctionMut(v) => v.display_with_type_info(f, type_info_provider),
            ScriptValue::Function(v) => v.display_with_type_info(f, type_info_provider),
            ScriptValue::Error(e) => e.display_with_type_info(f, type_info_provider),
        }
    }
}

impl PartialEq for ScriptValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Unit, Self::Unit) => true,
            (Self::Bool(l0), Self::Bool(r0)) => l0 == r0,
            (Self::Integer(l0), Self::Integer(r0)) => l0 == r0,
            (Self::Float(l0), Self::Float(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Map(l0), Self::Map(r0)) => l0 == r0,
            (Self::Reference(l0), Self::Reference(r0)) => l0 == r0,
            // explicitly don't compare functions and errors
            _ => false,
        }
    }
}

#[profiling::all_functions]
impl ScriptValue {
    /// Returns the contained string if this is a string variant otherwise returns the original value.
    pub fn as_string(self) -> Result<Cow<'static, str>, Self> {
        match self {
            ScriptValue::String(s) => Ok(s),
            other => Err(other),
        }
    }

    /// Returns the variant of the value as a string.
    pub fn type_name(&self) -> String {
        match self {
            ScriptValue::Unit => "Unit".to_owned(),
            ScriptValue::Bool(_) => "Bool".to_owned(),
            ScriptValue::Integer(_) => "Integer".to_owned(),
            ScriptValue::Float(_) => "Float".to_owned(),
            ScriptValue::String(_) => "String".to_owned(),
            ScriptValue::List(_) => "List".to_owned(),
            ScriptValue::Reference(_) => "Reference".to_owned(),
            ScriptValue::FunctionMut(_) => "FunctionMut".to_owned(),
            ScriptValue::Function(_) => "Function".to_owned(),
            ScriptValue::Error(_) => "Error".to_owned(),
            ScriptValue::Map(_) => "Map".to_owned(),
        }
    }
}

#[profiling::all_functions]
impl From<()> for ScriptValue {
    fn from(_: ()) -> Self {
        ScriptValue::Unit
    }
}

#[profiling::all_functions]
impl From<bool> for ScriptValue {
    fn from(value: bool) -> Self {
        ScriptValue::Bool(value)
    }
}

#[profiling::all_functions]
impl From<i64> for ScriptValue {
    fn from(value: i64) -> Self {
        ScriptValue::Integer(value)
    }
}

#[profiling::all_functions]
impl From<f64> for ScriptValue {
    fn from(value: f64) -> Self {
        ScriptValue::Float(value)
    }
}

#[profiling::all_functions]
impl From<&'static str> for ScriptValue {
    fn from(value: &'static str) -> Self {
        ScriptValue::String(value.into())
    }
}

#[profiling::all_functions]
impl From<String> for ScriptValue {
    fn from(value: String) -> Self {
        ScriptValue::String(value.into())
    }
}

#[profiling::all_functions]
impl From<Cow<'static, str>> for ScriptValue {
    fn from(value: Cow<'static, str>) -> Self {
        ScriptValue::String(value)
    }
}

#[profiling::all_functions]
impl From<Vec<ScriptValue>> for ScriptValue {
    fn from(value: Vec<ScriptValue>) -> Self {
        ScriptValue::List(value)
    }
}

#[profiling::all_functions]
impl From<ReflectReference> for ScriptValue {
    fn from(value: ReflectReference) -> Self {
        ScriptValue::Reference(value)
    }
}

#[profiling::all_functions]
impl From<InteropError> for ScriptValue {
    fn from(value: InteropError) -> Self {
        ScriptValue::Error(value)
    }
}

#[profiling::all_functions]
impl<T: Into<ScriptValue>> From<Option<T>> for ScriptValue {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(v) => v.into(),
            None => ScriptValue::Unit,
        }
    }
}

#[profiling::all_functions]
impl<T: Into<ScriptValue>, E: Into<InteropError>> From<Result<T, E>> for ScriptValue {
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(v) => v.into(),
            Err(e) => ScriptValue::Error(e.into()),
        }
    }
}

impl From<HashMap<String, ScriptValue>> for ScriptValue {
    fn from(value: HashMap<String, ScriptValue>) -> Self {
        ScriptValue::Map(value)
    }
}
