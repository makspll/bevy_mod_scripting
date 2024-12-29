use std::{
    any::{type_name, TypeId},
    borrow::Cow,
    ffi::{CStr, CString, OsStr, OsString},
    path::{Path, PathBuf},
};

use bevy::reflect::{
    Access, DynamicEnum, DynamicList, DynamicTuple, DynamicVariant, OffsetAccess, ParsedPath,
    PartialReflect, Reflect, ReflectFromReflect, TypeData,
};

use crate::{
    error::{InteropError, InteropErrorInner, ScriptError, ScriptResult},
    reflection_extensions::{PartialReflectExt, TypeIdExtensions, TypeInfoExtensions},
};

use super::{pretty_print::DisplayWithWorld, ReflectReference, WorldGuard};

/// An abstraction of values that can be passed to and from scripts.
/// This allows us to re-use logic between scripting languages.
#[derive(Debug, Clone, PartialEq, Reflect)]
#[reflect(opaque)]
pub enum ScriptValue {
    /// Represents the absence of a value.
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
    /// Represents a reference to a value.
    Reference(ReflectReference),
    /// Represents any error, will be thrown when returned to a script
    Error(InteropError),
}

impl ScriptValue {
    pub fn type_name(&self) -> String {
        match self {
            ScriptValue::Unit => "Unit".to_owned(),
            ScriptValue::Bool(_) => "Bool".to_owned(),
            ScriptValue::Integer(_) => "Integer".to_owned(),
            ScriptValue::Float(_) => "Float".to_owned(),
            ScriptValue::String(_) => "String".to_owned(),
            ScriptValue::List(_) => "List".to_owned(),
            ScriptValue::Reference(_) => "Reference".to_owned(),
            ScriptValue::Error(_) => "Error".to_owned(),
        }
    }
}

impl From<()> for ScriptValue {
    fn from(_: ()) -> Self {
        ScriptValue::Unit
    }
}

impl From<bool> for ScriptValue {
    fn from(value: bool) -> Self {
        ScriptValue::Bool(value)
    }
}

impl From<i64> for ScriptValue {
    fn from(value: i64) -> Self {
        ScriptValue::Integer(value)
    }
}

impl From<f64> for ScriptValue {
    fn from(value: f64) -> Self {
        ScriptValue::Float(value)
    }
}

impl From<&'static str> for ScriptValue {
    fn from(value: &'static str) -> Self {
        ScriptValue::String(value.into())
    }
}

impl From<String> for ScriptValue {
    fn from(value: String) -> Self {
        ScriptValue::String(value.into())
    }
}

impl From<Cow<'static, str>> for ScriptValue {
    fn from(value: Cow<'static, str>) -> Self {
        ScriptValue::String(value)
    }
}

impl From<Vec<ScriptValue>> for ScriptValue {
    fn from(value: Vec<ScriptValue>) -> Self {
        ScriptValue::List(value)
    }
}

impl From<ReflectReference> for ScriptValue {
    fn from(value: ReflectReference) -> Self {
        ScriptValue::Reference(value)
    }
}

// impl From<ScriptError> for ScriptValue {
//     fn from(value: ScriptError) -> Self {
//         ScriptValue::Error(value)
//     }
// }

impl From<InteropError> for ScriptValue {
    fn from(value: InteropError) -> Self {
        ScriptValue::Error(value)
    }
}

impl<T: Into<ScriptValue>> From<Option<T>> for ScriptValue {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(v) => v.into(),
            None => ScriptValue::Unit,
        }
    }
}

impl<T: Into<ScriptValue>, E: Into<InteropError>> From<Result<T, E>> for ScriptValue {
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(v) => v.into(),
            Err(e) => ScriptValue::Error(e.into()),
        }
    }
}

impl TryFrom<ScriptValue> for ParsedPath {
    type Error = InteropError;
    fn try_from(value: ScriptValue) -> Result<Self, Self::Error> {
        Ok(match value {
            ScriptValue::Integer(i) => ParsedPath::from(vec![OffsetAccess {
                access: bevy::reflect::Access::ListIndex(i as usize),
                offset: Some(1),
            }]),
            ScriptValue::Float(v) => {
                return Err(InteropError::invalid_index(
                    value,
                    "Floating point numbers cannot be used to index into reflected values"
                        .to_owned(),
                ))
            }
            ScriptValue::String(cow) => {
                if let Some(tuple_struct_index) = cow.strip_prefix("_") {
                    if let Ok(index) = tuple_struct_index.parse::<usize>() {
                        let parsed_path = ParsedPath::from(vec![OffsetAccess {
                            access: bevy::reflect::Access::TupleIndex(index),
                            offset: Some(1),
                        }]);
                        return Ok(parsed_path);
                    }
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
                ))
            }
            _ => ParsedPath(vec![]),
        })
    }
}
