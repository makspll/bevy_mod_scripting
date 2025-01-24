use bevy_mod_scripting_core::{
    bindings::{
        function::script_function::{DynamicScriptFunction, FunctionCallContext},
        script_value::ScriptValue,
    },
    error::InteropError,
};
use rhai::{Dynamic, EvalAltResult, FnPtr, NativeCallContext};
use std::str::FromStr;

use super::reference::RhaiReflectReference;

pub const RHAI_CALLER_CONTEXT: FunctionCallContext = FunctionCallContext {
    convert_to_0_indexed: false,
};

// impl PluginFunc for FuncWrapper {
//     fn call(
//         &self,
//         _context: Option<rhai::NativeCallContext>,
//         _args: &mut [&mut Dynamic],
//     ) -> rhai::plugin::RhaiResult {
//         // let convert_args = _args
//         //     .iter_mut()
//         //     .map(|arg| ScriptValue::from_dynamic(arg.clone()))
//         //     .collect::<Result<Vec<_>, _>>()?;

//         // let out = self.0.call(
//         //     rhai_caller_context(self.0.info.namespace()),
//         //     WorldCallbackAccess::from_guard(ThreadWorldContainer.get_world()),
//         //     convert_args,
//         // );

//         // out.into_dynamic()
//         todo!()
//     }

//     fn is_method_call(&self) -> bool {
//         // TODO: is this correct? do we care if it's a method call?
//         false
//     }

//     fn has_context(&self) -> bool {
//         false
//     }
// }

// impl PluginFunc for FuncMutWrapper {
//     fn call(
//         &self,
//         _context: Option<rhai::NativeCallContext>,
//         _args: &mut [&mut Dynamic],
//     ) -> rhai::plugin::RhaiResult {
//         // let convert_args = _args
//         //     .iter_mut()
//         //     .map(|arg| ScriptValue::from_dynamic(arg.clone()))
//         //     .collect::<Result<Vec<_>, _>>()?;

//         // let out = self.0.call(
//         //     rhai_caller_context(self.0.info.namespace()),
//         //     WorldCallbackAccess::from_guard(ThreadWorldContainer.get_world()),
//         //     convert_args,
//         // );

//         // out.into_dynamic()
//         todo!()
//     }

//     fn is_method_call(&self) -> bool {
//         false
//     }

//     fn has_context(&self) -> bool {
//         false
//     }
// }

/// A function curried with one argument, i.e. the receiver
pub struct FunctionWithReceiver {
    pub function: DynamicScriptFunction,
    pub receiver: ScriptValue,
}

impl FunctionWithReceiver {
    pub fn curry(function: DynamicScriptFunction, receiver: ScriptValue) -> Self {
        Self { function, receiver }
    }
}

impl IntoDynamic for FunctionWithReceiver {
    fn into_dynamic(self) -> Result<Dynamic, Box<EvalAltResult>> {
        Ok(Dynamic::from(FnPtr::from_fn(
            self.function.name().to_string(),
            move |_ctxt: NativeCallContext, args: &mut [&mut Dynamic]| {
                let convert_args = args
                    .iter_mut()
                    .map(|arg| ScriptValue::from_dynamic(arg.clone()))
                    .collect::<Result<Vec<_>, _>>()?;

                let out = self.function.call(
                    std::iter::once(self.receiver.clone()).chain(convert_args),
                    RHAI_CALLER_CONTEXT,
                )?;

                out.into_dynamic()
            },
        )?))
    }
}

pub trait IntoDynamic {
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
            ScriptValue::Reference(reflect_reference) => {
                Dynamic::from(RhaiReflectReference(reflect_reference))
            }
            ScriptValue::FunctionMut(func) => Dynamic::from(FnPtr::from_fn(
                func.name().to_string(),
                move |_ctxt: NativeCallContext, args: &mut [&mut Dynamic]| {
                    let convert_args = args
                        .iter_mut()
                        .map(|arg| ScriptValue::from_dynamic(arg.clone()))
                        .collect::<Result<Vec<_>, _>>()?;

                    let out = func.call(convert_args, RHAI_CALLER_CONTEXT)?;

                    out.into_dynamic()
                },
            )?),
            ScriptValue::Function(func) => Dynamic::from(FnPtr::from_fn(
                func.name().to_string(),
                move |_ctxt: NativeCallContext, args: &mut [&mut Dynamic]| {
                    let convert_args = args
                        .iter_mut()
                        .map(|arg| ScriptValue::from_dynamic(arg.clone()))
                        .collect::<Result<Vec<_>, _>>()?;

                    let out = func.call(convert_args, RHAI_CALLER_CONTEXT)?;

                    out.into_dynamic()
                },
            )?),
            ScriptValue::Error(interop_error) => {
                return Err(EvalAltResult::ErrorSystem(
                    "Interop error in rhai script".to_string(),
                    interop_error.into(),
                )
                .into())
            }
        })
    }
}

pub trait FromDynamic: Sized {
    fn from_dynamic(dynamic: Dynamic) -> Result<Self, Box<EvalAltResult>>;
}

#[allow(clippy::unwrap_used, clippy::todo)]
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
            d if d.is_array() => Ok(ScriptValue::List(
                d.into_array()
                    .map_err(|_| InteropError::invariant("d is proved to be an array"))?
                    .into_iter()
                    .map(ScriptValue::from_dynamic)
                    .collect::<Result<Vec<_>, _>>()?,
            )),
            d => {
                if let Some(v) = d.try_cast::<RhaiReflectReference>() {
                    Ok(ScriptValue::Reference(v.0))
                } else {
                    todo!("from conversion not implemented yet")
                }
            }
        }
    }
}
