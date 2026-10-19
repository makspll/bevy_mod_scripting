//! The modules available to every script besides the types, see [`crate::types`].
use std::sync::{Arc, Mutex};

use bevy_mod_scripting_bindings::{InteropError, ScriptValue, builtins::builtins};
use bevy_platform::collections::HashMap;
use rune::{
    Any, ContextError, Module,
    runtime::{Function, Protocol, Value},
};

use crate::{
    error::vm,
    types::{ScriptFunction, call_raw, converter},
    value::{from_rune, to_rune},
};

/// The values stored by a script with `state()`, kept as long as its context.
///
/// Rune values can't be kept in a context, which has to be sendable, so these are plain script values.
/// Reading from the state returns a copy, a new value has to be assigned to change it.
pub type StateStore = Arc<Mutex<HashMap<String, ScriptValue>>>;

/// The script's state store, see [`StateStore`].
#[derive(Any, Clone)]
#[rune(item = ::bms, name = State)]
pub struct RuneState(StateStore);

fn lock(state: &StateStore) -> std::sync::MutexGuard<'_, HashMap<String, ScriptValue>> {
    state.lock().unwrap_or_else(|e| e.into_inner())
}

/// Registers the builtin functions as globals, `catch` and `state`.
pub fn register_globals(
    module: &mut Module,
    state: StateStore,
    register_callback: fn(String, Function) -> Result<(), InteropError>,
) -> Result<(), ContextError> {
    for (name, function) in builtins() {
        let function = ScriptFunction::Plain(function);
        let convert = converter(name);
        module
            .raw_function(
                name,
                call_raw(move |values: &[Value]| {
                    let args = values
                        .iter()
                        .map(from_rune)
                        .collect::<Result<Vec<_>, _>>()?;
                    convert(function.call(args)?)
                }),
            )
            .build()?;
    }

    module
        .function(
            "register_callback",
            move |callback: String, function: Function| vm(register_callback(callback, function)),
        )
        .build()?;

    // runs a function, like Lua's `pcall`: `Ok(value)`, or `Err(message)` if it failed
    module
        // the function is borrowed, one taken by value couldn't be called again
        .function("catch", |function: &Function| -> Result<Value, String> {
            match function.call::<Value>(()).into_result() {
                Ok(value) => Ok(value),
                Err(error) => Err(error.to_string()),
            }
        })
        .build()?;

    module.ty::<RuneState>()?;
    module
        .function("state", move || RuneState(state.clone()))
        .build()?;
    // the key is borrowed, a `String` parameter would take the string out of the variable the script passed
    module.associated_function(&Protocol::INDEX_GET, |this: &RuneState, key: &str| {
        vm(match lock(&this.0).get(key) {
            Some(value) => to_rune(value.clone()),
            None => to_rune(ScriptValue::Unit),
        })
    })?;
    module.associated_function(
        &Protocol::INDEX_SET,
        |this: &RuneState, key: &str, value: Value| {
            vm((|| {
                let value = from_rune(&value)?;
                lock(&this.0).insert(key.to_owned(), value);
                Ok::<_, InteropError>(())
            })())
        },
    )?;
    Ok(())
}

/// The math Lua has in its standard library, which Rune doesn't have.
pub fn math_module() -> Result<Module, ContextError> {
    let mut module = Module::with_item(["math"])?;
    module.constant("PI", std::f64::consts::PI).build()?;
    module.constant("TAU", std::f64::consts::TAU).build()?;
    module.constant("E", std::f64::consts::E).build()?;
    macro_rules! unary {
        ($($name:ident),*) => {
            $(module.function(stringify!($name), |x: f64| x.$name()).build()?;)*
        };
    }
    unary!(
        sin, cos, tan, asin, acos, atan, sinh, cosh, tanh, exp, ln, log2, log10, trunc, fract,
        signum, to_degrees, to_radians
    );
    module
        .function("atan2", |y: f64, x: f64| y.atan2(x))
        .build()?;
    module
        .function("log", |x: f64, base: f64| x.log(base))
        .build()?;
    Ok(module)
}
