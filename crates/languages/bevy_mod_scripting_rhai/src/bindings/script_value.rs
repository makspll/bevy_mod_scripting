use std::{str::FromStr, sync::Arc};

use bevy_mod_scripting_core::{
    bindings::{
        function::script_function::{
            CallerContext, DynamicScriptFunction, DynamicScriptFunctionMut,
        },
        script_value::ScriptValue,
    },
    error::InteropError,
};
use rhai::{
    plugin::{PluginFunc, RhaiFunc},
    Dynamic, EvalAltResult, FnPtr, RhaiNativeFunc,
};

pub const RHAI_CALLER_CONTEXT: CallerContext = CallerContext {
    convert_to_0_indexed: false,
};

#[allow(dead_code)]
struct FuncWrapper(DynamicScriptFunction);

#[allow(dead_code)]
struct FuncMutWrapper(DynamicScriptFunctionMut);

impl RhaiNativeFunc for FuncWrapper {
    fn into_rhai_function(self, is_pure: bool, is_volatile: bool) -> RhaiFunc {
        todo!()
    }

    fn param_types() -> [std::any::TypeId; N] {
        todo!()
    }
}

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

#[allow(dead_code)]
pub(crate) fn to_rhai_fn(func: DynamicScriptFunction) -> RhaiFunc {
    RhaiFunc::Plugin {
        func: Arc::new(FuncWrapper(func)),
    }
    .into()
    // FnPtr {
    //     name: todo!(),
    //     curry: todo!(),
    //     environ: todo!(),
    //     fn_def: todo!(),
    // }
}

pub(crate) fn to_rhai_fn_mut(func: DynamicScriptFunctionMut) -> RhaiFunc {
    RhaiFunc::Plugin {
        func: Arc::new(FuncMutWrapper(func)),
    }
}

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
            ScriptValue::FunctionMut(func) => Dynamic::from(to_rhai_fn_mut(func)),
            ScriptValue::Function(func) => Dynamic::from(to_rhai_fn(func)),
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
