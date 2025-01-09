use std::str::FromStr;

use bevy_mod_scripting_core::{bindings::script_value::ScriptValue, error::InteropError};
use rhai::{Dynamic, EvalAltResult};

pub trait IntoDynamic {
    fn into_dynamic(self) -> Result<Dynamic, Box<EvalAltResult>>;
}

impl IntoDynamic for ScriptValue {
    fn into_dynamic(self) -> Result<Dynamic, Box<EvalAltResult>> {
        Ok(match self {
            ScriptValue::Unit => Dynamic::UNIT,
            ScriptValue::Bool(b) => Dynamic::from_bool(b),
            ScriptValue::Integer(i) => Dynamic::from_int(i),
            ScriptValue::Float(f) => Dynamic::from_float(f),
            ScriptValue::String(cow) => Dynamic::from_str(&cow).map_err(|_| {
                EvalAltResult::ErrorSystem(
                    "error in converting string to rhai value".to_owned(),
                    InteropError::unsupported_operation(
                        None,
                        Some(Box::new(cow.clone())),
                        "string to rhai value".to_owned(),
                    )
                    .into(),
                )
            })?,
            ScriptValue::List(_vec) => todo!(),
            ScriptValue::Reference(_reflect_reference) => todo!(),
            ScriptValue::Function(_dynamic_script_function_mut) => todo!(),
            ScriptValue::Error(_interop_error) => todo!(),
        })
    }
}

pub trait FromDynamic: Sized {
    fn from_dynamic(dynamic: Dynamic) -> Result<Self, Box<EvalAltResult>>;
}

impl FromDynamic for ScriptValue {
    fn from_dynamic(dynamic: Dynamic) -> Result<Self, Box<EvalAltResult>> {
        match dynamic {
            d if d.is_unit() => Ok(ScriptValue::Unit),
            d if d.is_bool() => Ok(ScriptValue::Bool(d.as_bool().unwrap())),
            d if d.is_int() => Ok(ScriptValue::Integer(d.as_int().unwrap())),
            d if d.is_float() => Ok(ScriptValue::Float(d.as_float().unwrap())),
            d if d.is_string() => Ok(ScriptValue::String(
                d.into_immutable_string().unwrap().to_string().into(),
            )),
            _ => todo!(),
        }
    }
}
