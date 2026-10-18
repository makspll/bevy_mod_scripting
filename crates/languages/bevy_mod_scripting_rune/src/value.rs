//! Conversions between [`ScriptValue`] and Rune values.
use std::{
    any::TypeId,
    collections::{HashMap as StdHashMap, VecDeque},
    sync::{
        Arc, Mutex, OnceLock, Weak,
        atomic::{AtomicU64, Ordering},
    },
};

use bevy_mod_scripting_asset::Language;
use bevy_mod_scripting_bindings::{
    FunctionInfo, InteropError, ScriptValue, VariadicTuple,
    function::script_function::{DynamicScriptFunction, FunctionCallContext},
};
use bevy_platform::collections::HashMap;
use rune::{
    alloc,
    runtime::{Function, Object, OwnedTuple, TypeHash, Value, VmResult},
};

use crate::{
    error::{IntoInteropError, RuneError},
    send::SendWrapper,
    types::{RuneFunction, RuneNamespace, RuneRef, RuneType, ScriptFunction},
};

/// The context for calling a function from Rune
pub const RUNE_CALLER_CONTEXT: FunctionCallContext = FunctionCallContext::new(Language::Rune);

fn new_value<T: rune::Any>(value: T) -> Result<Value, InteropError> {
    Value::new(value).map_err(IntoInteropError::to_bms_error)
}

fn alloc_vec(list: VecDeque<ScriptValue>) -> Result<alloc::Vec<Value>, InteropError> {
    let mut vec =
        alloc::Vec::try_with_capacity(list.len()).map_err(IntoInteropError::to_bms_error)?;
    for item in list {
        vec.try_push(to_rune(item)?)
            .map_err(IntoInteropError::to_bms_error)?;
    }
    Ok(vec)
}

/// Converts a value from a script function to a Rune value, errors returned by functions are returned as errors.
pub fn to_rune(value: ScriptValue) -> Result<Value, InteropError> {
    Ok(match value {
        ScriptValue::Unit => rune::to_value(()).map_err(IntoInteropError::to_bms_error)?,
        ScriptValue::Bool(b) => rune::to_value(b).map_err(IntoInteropError::to_bms_error)?,
        ScriptValue::Integer(i) => rune::to_value(i).map_err(IntoInteropError::to_bms_error)?,
        ScriptValue::Float(f) => rune::to_value(f).map_err(IntoInteropError::to_bms_error)?,
        ScriptValue::String(s) => {
            let string =
                alloc::String::try_from(s.as_ref()).map_err(IntoInteropError::to_bms_error)?;
            rune::to_value(string).map_err(IntoInteropError::to_bms_error)?
        }
        ScriptValue::List(list) => {
            Value::vec(alloc_vec(list)?).map_err(IntoInteropError::to_bms_error)?
        }
        ScriptValue::Tuple(VariadicTuple(list)) => {
            Value::tuple(alloc_vec(list)?).map_err(IntoInteropError::to_bms_error)?
        }
        ScriptValue::Map(map) => {
            let mut object = Object::new();
            for (key, item) in map {
                let key = alloc::String::try_from(key.as_str())
                    .map_err(IntoInteropError::to_bms_error)?;
                object
                    .insert(key, to_rune(item)?)
                    .map_err(IntoInteropError::to_bms_error)?;
            }
            new_value(object)?
        }
        ScriptValue::Reference(reference) => new_value(RuneRef(reference))?,
        ScriptValue::Type(type_id) => new_value(RuneType(type_id))?,
        ScriptValue::Function(function) => function_to_rune(function, None)?,
        ScriptValue::FunctionMut(function) => {
            new_value(RuneFunction(ScriptFunction::Mut(function)))?
        }
        ScriptValue::Error(error) => return Err(error),
    })
}

/// Converts the map returned by `import_namespace` to a namespace, which can call its functions with the method syntax.
pub fn namespace_to_rune(value: ScriptValue) -> Result<Value, InteropError> {
    match value {
        ScriptValue::Map(map) => new_value(RuneNamespace(std::sync::Arc::new(map))),
        other => to_rune(other),
    }
}

fn type_error(value: &Value) -> InteropError {
    InteropError::external(RuneError(format!(
        "can't pass a value of type `{}` to a function",
        value.type_info()
    )))
}

/// Converts a Rune value to be passed to a script function.
pub fn from_rune(value: &Value) -> Result<ScriptValue, InteropError> {
    let hash = value.type_hash();
    if hash == <() as TypeHash>::HASH {
        return Ok(ScriptValue::Unit);
    }
    if hash == <bool as TypeHash>::HASH {
        return rune::from_value::<bool>(value.clone())
            .map(ScriptValue::Bool)
            .map_err(IntoInteropError::to_bms_error);
    }
    if hash == <i64 as TypeHash>::HASH {
        return rune::from_value::<i64>(value.clone())
            .map(ScriptValue::Integer)
            .map_err(IntoInteropError::to_bms_error);
    }
    if hash == <u64 as TypeHash>::HASH {
        return rune::from_value::<u64>(value.clone())
            .map(|u| ScriptValue::Integer(u as i64))
            .map_err(IntoInteropError::to_bms_error);
    }
    if hash == <f64 as TypeHash>::HASH {
        return rune::from_value::<f64>(value.clone())
            .map(ScriptValue::Float)
            .map_err(IntoInteropError::to_bms_error);
    }
    if hash == <alloc::String as TypeHash>::HASH {
        let string = value
            .borrow_string_ref()
            .map_err(IntoInteropError::to_bms_error)?;
        return Ok(ScriptValue::String(string.to_string().into()));
    }
    if hash == <rune::runtime::Vec as TypeHash>::HASH {
        let vec = value
            .borrow_ref::<rune::runtime::Vec>()
            .map_err(IntoInteropError::to_bms_error)?;
        return vec
            .iter()
            .map(from_rune)
            .collect::<Result<VecDeque<_>, _>>()
            .map(ScriptValue::List);
    }
    if hash == <OwnedTuple as TypeHash>::HASH {
        let tuple = value
            .borrow_ref::<OwnedTuple>()
            .map_err(IntoInteropError::to_bms_error)?;
        return tuple
            .iter()
            .map(from_rune)
            .collect::<Result<VecDeque<_>, _>>()
            .map(|list| ScriptValue::Tuple(VariadicTuple(list)));
    }
    if hash == <Object as TypeHash>::HASH {
        let object = value
            .borrow_ref::<Object>()
            .map_err(IntoInteropError::to_bms_error)?;
        let mut map = HashMap::default();
        for (key, item) in object.iter() {
            map.insert(key.to_string(), from_rune(item)?);
        }
        return Ok(ScriptValue::Map(map));
    }
    if let Ok(reference) = value.borrow_ref::<RuneRef>() {
        return Ok(ScriptValue::Reference(reference.0.clone()));
    }
    if let Ok(type_ref) = value.borrow_ref::<RuneType>() {
        return Ok(ScriptValue::Type(type_ref.0));
    }
    if let Ok(function) = value.borrow_ref::<RuneFunction>() {
        return Ok(match &function.0 {
            ScriptFunction::Plain(f) => ScriptValue::Function(f.clone()),
            ScriptFunction::Mut(f) => ScriptValue::FunctionMut(f.clone()),
        });
    }
    if hash == <Function as TypeHash>::HASH {
        return Ok(ScriptValue::Function(rune_function_to_script(value.clone())));
    }
    Err(type_error(value))
}

/// Wraps a function from a script to be called by BMS.
///
/// The function is remembered, when it comes back to a script it is that function and not a wrapper of the wrapper.
pub fn rune_function_to_script(function: Value) -> DynamicScriptFunction {
    static NEXT_ID: AtomicU64 = AtomicU64::new(0);
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    let function = Arc::new(SendWrapper::new(function));
    {
        let mut functions = rune_functions().lock().unwrap_or_else(|e| e.into_inner());
        // forget the functions which were dropped
        functions.retain(|_, function| function.strong_count() > 0);
        functions.insert(id, Arc::downgrade(&function));
    }
    DynamicScriptFunction::from(move |_context: FunctionCallContext, args: VecDeque<ScriptValue>| {
        let result = rune::from_value::<Function>(function.get().clone())
            .map_err(IntoInteropError::to_bms_error)
            .and_then(|function| call_rune_function(&function, args));
        match result {
            Ok(value) => value,
            Err(error) => ScriptValue::Error(error),
        }
    })
    .with_info(FunctionInfo::new().with_name(format!("{RUNE_FUNCTION_PREFIX}{id}")))
}

/// Calls a Rune function with the given arguments.
pub fn call_rune_function(
    function: &Function,
    args: VecDeque<ScriptValue>,
) -> Result<ScriptValue, InteropError> {
    let args = args
        .into_iter()
        .map(to_rune)
        .collect::<Result<std::vec::Vec<_>, _>>()?;
    let value = function
        .call::<Value>(args)
        .into_result()
        .map_err(IntoInteropError::to_bms_error)?;
    from_rune(&value)
}

/// The prefix of the names of functions made from Rune functions, followed by an id, see [`rune_function_to_script`].
const RUNE_FUNCTION_PREFIX: &str = "rune function #";

/// The Rune functions which were passed to BMS, so they come back as the very same function.
fn rune_functions() -> &'static Mutex<StdHashMap<u64, Weak<SendWrapper<Value>>>> {
    static FUNCTIONS: OnceLock<Mutex<StdHashMap<u64, Weak<SendWrapper<Value>>>>> =
        OnceLock::new();
    FUNCTIONS.get_or_init(Default::default)
}

/// The most arguments a function value can take to be called as `f(a, b)`, which is as many as Rune can make functions for, see [`function_to_rune`].
const MAX_CALLABLE_ARITY: usize = 5;

/// Converts a function to a Rune value.
///
/// Functions which are values in scripts are called like any function, `f(a, b)`, when their number of arguments is known
/// and not variable. Exports of scripts and functions made from closures are not like that. Rune checks that number, so arguments which have defaults have to be passed too.
/// Other functions are [`RuneFunction`]s, which are called with `f.call(a, b)`.
///
/// `bound` is the number of arguments which are already given to the function, i.e. its receiver.
pub fn function_to_rune(
    function: DynamicScriptFunction,
    bound: Option<usize>,
) -> Result<Value, InteropError> {
    // functions from scripts come back as they were
    if let Some(id) = function
        .name()
        .strip_prefix(RUNE_FUNCTION_PREFIX)
        .and_then(|id| id.parse::<u64>().ok())
    {
        let original = rune_functions()
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .get(&id)
            .and_then(Weak::upgrade);
        if let Some(original) = original {
            return Ok(original.get().clone());
        }
    }

    // functions made from closures, i.e. exports, don't say how many arguments they take
    let unknown_arguments = function.name().contains("{{closure}}");
    let variadic = unknown_arguments
        || function
            .info
        .arg_info
        .iter()
        .any(|arg| arg.type_id == TypeId::of::<VariadicTuple>());
    let arity = function
        .info
        .arg_info
        .len()
        .checked_sub(bound.unwrap_or(0));
    let function = ScriptFunction::Plain(function);
    match arity {
        Some(arity) if !variadic && arity <= MAX_CALLABLE_ARITY => {
            callable_function(function, arity)
        }
        _ => new_value(RuneFunction(function)),
    }
}

fn call_values(function: &ScriptFunction, args: &[&Value]) -> VmResult<Value> {
    crate::error::vm((|| {
        let args = args
            .iter()
            .map(|arg| from_rune(arg))
            .collect::<Result<Vec<_>, _>>()?;
        match function.call(args)? {
            ScriptValue::Error(error) => Err(error),
            result => to_rune(result),
        }
    })())
}

/// Makes a Rune function which takes the given number of arguments, which Rune needs to know, from a function.
fn callable_function(function: ScriptFunction, arity: usize) -> Result<Value, InteropError> {
    macro_rules! callable {
        ($($arg:ident),*) => {{
            let function = function.clone();
            Function::new(move |$($arg: Value),*| call_values(&function, &[$(&$arg),*]))
        }};
    }
    let function = match arity {
        0 => callable!(),
        1 => callable!(a),
        2 => callable!(a, b),
        3 => callable!(a, b, c),
        4 => callable!(a, b, c, d),
        5 => callable!(a, b, c, d, e),
        _ => callable!(a, b, c, d, e),
    };
    rune::to_value(function).map_err(IntoInteropError::to_bms_error)
}
