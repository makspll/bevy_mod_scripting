//! This module contains the `ScriptValue` enum which is used to pass values between scripting languages and Rust.

use bevy_platform::collections::HashMap;
use bevy_reflect::{Access, OffsetAccess, ParsedPath, Reflect};
use std::borrow::Cow;

use crate::error::InteropError;

use super::{
    ReflectReference,
    function::script_function::{DynamicScriptFunction, DynamicScriptFunctionMut},
};

/// An abstraction of values that can be passed to and from scripts.
/// This allows us to re-use logic between scripting languages.
#[derive(Debug, Clone, PartialEq, Reflect, Default)]
#[reflect(opaque)]
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
    /// Represents any error, will be thrown when returned to a script
    Error(InteropError),
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
#[profiling::all_functions]
impl TryFrom<ScriptValue> for ParsedPath {
    type Error = InteropError;
    fn try_from(value: ScriptValue) -> Result<Self, Self::Error> {
        Ok(match value {
            ScriptValue::Integer(i) => ParsedPath::from(vec![OffsetAccess {
                access: Access::ListIndex(i as usize),
                offset: Some(1),
            }]),
            ScriptValue::Float(_) => {
                return Err(InteropError::invalid_index(
                    value,
                    "Floating point numbers cannot be used to index into reflected values"
                        .to_owned(),
                ));
            }
            ScriptValue::String(cow) => {
                if let Some(tuple_struct_index) = cow.strip_prefix("_")
                    && let Ok(index) = tuple_struct_index.parse::<usize>()
                {
                    let parsed_path = ParsedPath::from(vec![OffsetAccess {
                        access: Access::TupleIndex(index),
                        offset: Some(1),
                    }]);
                    return Ok(parsed_path);
                }

                match cow {
                    Cow::Borrowed(v) => ParsedPath::parse_static(v)
                        .map_err(|e| InteropError::reflection_path_error(e.to_string(), None))?,
                    Cow::Owned(o) => ParsedPath::parse(&o)
                        .map_err(|e| InteropError::reflection_path_error(e.to_string(), None))?,
                }
            }
            ScriptValue::Reference(reflect_reference) => {
                return Err(InteropError::invalid_index(
                    ScriptValue::Reference(reflect_reference),
                    "References cannot be used to index into reflected values".to_owned(),
                ));
            }
            _ => ParsedPath(vec![]),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_script_value_to_parsed_path() {
        let value = ScriptValue::String("test".into());
        let parsed_path = ParsedPath::from(vec![OffsetAccess {
            access: Access::Field("test".to_owned().into()),
            offset: Some(4),
        }]);
        assert_eq!(parsed_path, ParsedPath::try_from(value).unwrap());

        let value = ScriptValue::String("_0".into());
        let parsed_path = ParsedPath::from(vec![OffsetAccess {
            access: Access::TupleIndex(0),
            offset: Some(1),
        }]);
        assert_eq!(parsed_path, ParsedPath::try_from(value).unwrap());

        let value = ScriptValue::Integer(0);
        let parsed_path = ParsedPath::from(vec![OffsetAccess {
            access: Access::ListIndex(0),
            offset: Some(1),
        }]);
        assert_eq!(parsed_path, ParsedPath::try_from(value).unwrap());

        let value = ScriptValue::Float(0.0);
        assert!(ParsedPath::try_from(value).is_err());
    }
}
