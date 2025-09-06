use std::str::FromStr;

use bevy_mod_scripting_asset::Language;
use bevy_mod_scripting_bindings::{
    error::InteropError,
    function::script_function::{DynamicScriptFunction, FunctionCallContext},
    script_value::ScriptValue,
};
use rhai::{Dynamic, EvalAltResult, FnPtr, Map, NativeCallContext};

use crate::IntoRhaiError;

use super::reference::RhaiReflectReference;

/// The default function call context for rhai
pub const RHAI_CALLER_CONTEXT: FunctionCallContext = FunctionCallContext::new(Language::Rhai);

/// A function curried with one argument, i.e. the receiver
pub struct FunctionWithReceiver {
    /// The function
    pub function: DynamicScriptFunction,
    /// The receiver
    pub receiver: ScriptValue,
}

impl FunctionWithReceiver {
    /// Create a new function with receiver
    pub fn curry(function: DynamicScriptFunction, receiver: ScriptValue) -> Self {
        Self { function, receiver }
    }
}

impl IntoDynamic for FunctionWithReceiver {
    fn into_dynamic(self) -> Result<Dynamic, Box<EvalAltResult>> {
        #[allow(deprecated)]
        Ok(Dynamic::from(FnPtr::from_fn(
            self.function.name().to_string(),
            move |_ctxt: NativeCallContext, args: &mut [&mut Dynamic]| {
                let convert_args = args
                    .iter_mut()
                    .map(|arg| ScriptValue::from_dynamic(arg.clone()))
                    .collect::<Result<Vec<_>, _>>()?;

                let out = self
                    .function
                    .call(
                        std::iter::once(self.receiver.clone()).chain(convert_args),
                        RHAI_CALLER_CONTEXT,
                    )
                    .map_err(IntoRhaiError::into_rhai_error)?;

                out.into_dynamic()
            },
        )?))
    }
}

/// A trait for converting types into a [`Dynamic`] value
pub trait IntoDynamic {
    /// Convert the type into a [`Dynamic`] value
    fn into_dynamic(self) -> Result<Dynamic, Box<EvalAltResult>>;
}

#[allow(clippy::todo)]
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
            ScriptValue::List(_vec) => Dynamic::from_array(
                _vec.into_iter()
                    .map(|v| v.into_dynamic())
                    .collect::<Result<Vec<_>, _>>()?,
            ),
            ScriptValue::Map(map) => {
                let rhai_map: Map = map
                    .into_iter()
                    .map(|(k, v)| Ok((k.into(), v.into_dynamic()?)))
                    .collect::<Result<_, Box<EvalAltResult>>>()?;
                Dynamic::from_map(rhai_map)
            }
            ScriptValue::Reference(reflect_reference) => {
                Dynamic::from(RhaiReflectReference(reflect_reference))
            }
            #[allow(deprecated)]
            ScriptValue::FunctionMut(func) => Dynamic::from(FnPtr::from_fn(
                func.name().to_string(),
                move |_ctxt: NativeCallContext, args: &mut [&mut Dynamic]| {
                    let convert_args = args
                        .iter_mut()
                        .map(|arg| ScriptValue::from_dynamic(arg.clone()))
                        .collect::<Result<Vec<_>, _>>()?;

                    let out = func
                        .call(convert_args, RHAI_CALLER_CONTEXT)
                        .map_err(IntoRhaiError::into_rhai_error)?;

                    out.into_dynamic()
                },
            )?),
            #[allow(deprecated)]
            ScriptValue::Function(func) => Dynamic::from(FnPtr::from_fn(
                func.name().to_string(),
                move |_ctxt: NativeCallContext, args: &mut [&mut Dynamic]| {
                    let convert_args = args
                        .iter_mut()
                        .map(|arg| ScriptValue::from_dynamic(arg.clone()))
                        .collect::<Result<Vec<_>, _>>()?;

                    let out = func
                        .call(convert_args, RHAI_CALLER_CONTEXT)
                        .map_err(IntoRhaiError::into_rhai_error)?;

                    out.into_dynamic()
                },
            )?),
            ScriptValue::Error(interop_error) => {
                return Err(EvalAltResult::ErrorSystem(
                    "Interop error in rhai script".to_string(),
                    interop_error.into(),
                )
                .into());
            }
        })
    }
}

/// A trait for converting a [`Dynamic`] value into a type
pub trait FromDynamic: Sized {
    /// Convert a [`Dynamic`] value into a type
    fn from_dynamic(dynamic: Dynamic) -> Result<Self, Box<EvalAltResult>>;
}

#[allow(clippy::unwrap_used)]
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
            mut d if d.is_map() => Ok(ScriptValue::Map(
                d.as_map_mut()
                    .map_err(|_| {
                        Box::new(EvalAltResult::ErrorSystem(
                            "FromDynamic".to_string(),
                            InteropError::invariant("d is proved to be a map").into(),
                        ))
                    })?
                    .iter()
                    .map(|(k, v)| Ok((k.to_string(), ScriptValue::from_dynamic(v.clone())?)))
                    .collect::<Result<_, Box<EvalAltResult>>>()?,
            )),
            d if d.is_array() => Ok(ScriptValue::List(
                d.into_array()
                    .map_err(|_| InteropError::invariant("d is proved to be an array"))
                    .map_err(IntoRhaiError::into_rhai_error)?
                    .into_iter()
                    .map(ScriptValue::from_dynamic)
                    .collect::<Result<Vec<_>, _>>()?,
            )),
            d => {
                let type_name = d.type_name();
                if let Some(v) = d.try_cast::<RhaiReflectReference>() {
                    Ok(ScriptValue::Reference(v.0))
                } else {
                    Err(Box::new(EvalAltResult::ErrorSystem(
                        "FromDynamic".to_string(),
                        Box::new(InteropError::string(format!(
                            "unsupported dynamic type for conversion to ScriptValue from dynamic type: {type_name:?}",
                        ))),
                    )))
                }
            }
        }
    }
}
