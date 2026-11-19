//! Typed calls and native values in Rune, see [`bevy_mod_scripting_bindings::native`].
//!
//! Native values are [`RuneNative`]s holding a [`NativeBox`]. Their fields are read and written through their type's typed
//! accessors, their methods are the functions registered on the type, called with the values as they are when typed.
//! Everything else goes through a reference to a copy of the value, like any other value BMS doesn't know natively.
#![allow(unsafe_code)]

use std::any::TypeId;

use bevy_mod_scripting_bindings::{
    DynamicScriptFunction, InteropError, ReflectReference, ScriptValue, WorldExtensions, dispatch,
    native::{ArgSource, NativeBox, NativeField, NativeStorage, NativeVTable, ReturnSink},
};
use bevy_mod_scripting_bindings_domain::ScriptOperatorNames;
use rune::{Any, TypeHash, runtime::Value};

use crate::{
    error::IntoInteropError,
    types::{RuneRef, apply_operator, call_member_inner, get_ref_member, world},
    value::{RUNE_CALLER_CONTEXT, from_rune, new_value, to_rune},
};

/// A value BMS stores natively, see [`bevy_mod_scripting_bindings::native`].
#[derive(Any)]
#[rune(item = ::bms, name = Native)]
pub struct RuneNative(pub NativeBox);

impl RuneNative {
    fn type_id(&self) -> TypeId {
        // Safety: the box holds initialised storage
        unsafe { NativeStorage::header(self.0.base()) }
            .vtable
            .type_id
    }

    /// A copy of the value, as a reference.
    pub(crate) fn to_script(&self) -> Result<ScriptValue, InteropError> {
        // Safety: the box holds initialised storage
        unsafe { NativeStorage::to_script_value(self.0.base()) }
    }

    fn to_reference(&self) -> Result<ReflectReference, InteropError> {
        match self.to_script()? {
            ScriptValue::Reference(reference) => Ok(reference),
            _ => Err(InteropError::str(
                "a native value is copied into a reference",
            )),
        }
    }
}

/// An argument of a typed call: a value on the stack, or the receiver of a protocol, which Rune gives borrowed.
pub(crate) enum RuneArg<'a> {
    Value(&'a Value),
    Native(&'a RuneNative),
}

struct RuneArgs<'a>(&'a [RuneArg<'a>]);

impl RuneArgs<'_> {
    fn value(&self, i: usize) -> Option<&Value> {
        match self.0.get(i)? {
            RuneArg::Value(value) => Some(value),
            RuneArg::Native(_) => None,
        }
    }
}

// Safety: native bases come from `NativeBox`es of `RuneNative`s, which the arguments keep alive for the call
unsafe impl ArgSource for RuneArgs<'_> {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn number(&self, i: usize) -> Option<f64> {
        let value = self.value(i)?;
        (value.type_hash() == <f64 as TypeHash>::HASH)
            .then(|| rune::from_value::<f64>(value.clone()).ok())
            .flatten()
    }

    fn integer(&self, i: usize) -> Option<i64> {
        let value = self.value(i)?;
        let hash = value.type_hash();
        if hash == <i64 as TypeHash>::HASH {
            return rune::from_value::<i64>(value.clone()).ok();
        }
        if hash == <u64 as TypeHash>::HASH {
            return rune::from_value::<u64>(value.clone())
                .ok()
                .map(|u| u as i64);
        }
        None
    }

    fn boolean(&self, i: usize) -> Option<bool> {
        let value = self.value(i)?;
        (value.type_hash() == <bool as TypeHash>::HASH)
            .then(|| rune::from_value::<bool>(value.clone()).ok())
            .flatten()
    }

    fn native(&self, i: usize) -> Option<*mut u8> {
        match self.0.get(i)? {
            RuneArg::Native(native) => Some(native.0.base()),
            RuneArg::Value(value) => value.borrow_ref::<RuneNative>().ok().map(|n| n.0.base()),
        }
    }

    fn script_value(&self, i: usize) -> Result<ScriptValue, InteropError> {
        match self.0.get(i) {
            Some(RuneArg::Native(native)) => native.to_script(),
            Some(RuneArg::Value(value)) => from_rune(value),
            None => Ok(ScriptValue::Unit),
        }
    }
}

/// The result of a typed call.
#[derive(Default)]
struct RuneReturn {
    value: Option<Result<Value, InteropError>>,
}

impl RuneReturn {
    fn set(&mut self, value: Result<Value, InteropError>) {
        self.value = Some(value);
    }

    fn finish(self) -> Result<Value, InteropError> {
        match self.value {
            Some(value) => value,
            None => to_rune(ScriptValue::Unit),
        }
    }
}

impl ReturnSink for RuneReturn {
    fn unit(&mut self) {}

    fn number(&mut self, n: f64) {
        self.set(rune::to_value(n).map_err(IntoInteropError::to_bms_error));
    }

    fn integer(&mut self, i: i64) {
        self.set(rune::to_value(i).map_err(IntoInteropError::to_bms_error));
    }

    fn boolean(&mut self, b: bool) {
        self.set(rune::to_value(b).map_err(IntoInteropError::to_bms_error));
    }

    fn native(&mut self, vtable: &'static NativeVTable, write: &mut dyn FnMut(*mut u8)) -> bool {
        match NativeBox::new(vtable, write) {
            Ok(native) => {
                self.set(new_value(RuneNative(native)));
                true
            }
            Err(_) => false,
        }
    }

    fn script_value(&mut self, value: ScriptValue) -> Result<(), InteropError> {
        self.set(to_rune(value));
        Ok(())
    }
}

/// Calls `function`: its typed entry with the arguments as they are, or its dynamic one with them converted.
pub(crate) fn call(
    function: &DynamicScriptFunction,
    args: &[RuneArg],
) -> Result<Value, InteropError> {
    let mut ret = RuneReturn::default();
    match function.call_native(&RuneArgs(args), &mut ret, RUNE_CALLER_CONTEXT) {
        Some(result) => result.and_then(|()| ret.finish()),
        None => {
            let args = RuneArgs(args);
            let values = (0..args.len())
                .map(|i| args.script_value(i))
                .collect::<Result<Vec<_>, _>>()?;
            to_rune(function.call(values, RUNE_CALLER_CONTEXT)?)
        }
    }
}

/// The field `name` of the native value's type, if it has one.
fn field(this: &RuneNative, name: &str) -> Result<Option<NativeField>, InteropError> {
    let world = world()?;
    let registry = world.script_function_registry();
    let registry = registry.read();
    Ok(registry
        .native_type(this.type_id())
        .and_then(|native| native.fields.get(name).cloned()))
}

/// The function `name` called on the native value: its type's, then the ones shared by all references.
fn method(this: &RuneNative, name: &str) -> Result<Option<DynamicScriptFunction>, InteropError> {
    let world = world()?;
    let registry = world.script_function_registry();
    let registry = registry.read();
    Ok([this.type_id(), TypeId::of::<ReflectReference>()]
        .into_iter()
        .find_map(|type_id| registry.get_function(type_id, name).ok().cloned()))
}

/// `v.name`
pub(crate) fn get(this: &RuneNative, name: &str) -> Result<Value, InteropError> {
    if let Some(field) = field(this, name)? {
        return call(&field.get, &[RuneArg::Native(this)]);
    }
    // methods as values and anything else, through a copy
    get_ref_member(&RuneRef(this.to_reference()?), name)
}

/// `v.name = x`
pub(crate) fn set(this: &RuneNative, name: &str, value: &Value) -> Result<(), InteropError> {
    match field(this, name)?.and_then(|field| field.set) {
        Some(set) => call(&set, &[RuneArg::Native(this), RuneArg::Value(value)]).map(|_| ()),
        None => Err(InteropError::string(format!(
            "`{name}` is not a field which can be set on a native value"
        ))),
    }
}

/// `v.name += x` and the other compound assignments.
pub(crate) fn assign(
    operator: ScriptOperatorNames,
    this: &RuneNative,
    name: &str,
    value: &Value,
) -> Result<(), InteropError> {
    let current = from_rune(&get(this, name)?)?;
    let result = to_rune(apply_operator(operator, current, from_rune(value)?)?)?;
    set(this, name, &result)
}

/// `v.name(..)`, the receiver is the first value.
pub(crate) fn call_method(
    name: &str,
    convert: fn(ScriptValue) -> Result<Value, InteropError>,
    values: &[Value],
) -> Result<Value, InteropError> {
    let (receiver, rest) = values
        .split_first()
        .ok_or_else(|| InteropError::str("a method needs a receiver"))?;
    let this = receiver
        .borrow_ref::<RuneNative>()
        .map_err(IntoInteropError::to_bms_error)?;
    if let Some(function) = method(&this, name)? {
        let mut args = Vec::with_capacity(values.len());
        args.push(RuneArg::Native(&this));
        args.extend(rest.iter().map(RuneArg::Value));
        return call(&function, &args);
    }
    // anything else, i.e. a function stored in a field, through a copy
    let mut values = values.to_vec();
    values[0] = new_value(RuneRef(this.to_reference()?))?;
    drop(this);
    call_member_inner(name, convert, &values)
}

/// `v[key]`
pub(crate) fn index_get(this: &RuneNative, key: &Value) -> Result<Value, InteropError> {
    match from_rune(key)? {
        ScriptValue::String(name) => get(this, &name),
        key => {
            let member = dispatch::index(world()?, this.to_reference()?, key, RUNE_CALLER_CONTEXT)?;
            to_rune(member.into())
        }
    }
}

/// `v[key] = x`
pub(crate) fn index_set(this: &RuneNative, key: &Value, value: &Value) -> Result<(), InteropError> {
    match from_rune(key)? {
        ScriptValue::String(name) => set(this, &name, value),
        _ => Err(InteropError::str("native values are indexed with names")),
    }
}

/// `v + x` and the other operators, the native value is on the left.
pub(crate) fn binary(
    operator: ScriptOperatorNames,
    this: &RuneNative,
    other: &Value,
) -> Result<Value, InteropError> {
    if let Some(function) = method(this, operator.script_function_name())? {
        return call(&function, &[RuneArg::Native(this), RuneArg::Value(other)]);
    }
    let out = dispatch::binary(
        world()?,
        operator,
        this.to_script()?,
        from_rune(other)?,
        RUNE_CALLER_CONTEXT,
    )?;
    to_rune(out)
}

/// `v == x`
pub(crate) fn eq(this: &RuneNative, other: &Value) -> Result<bool, InteropError> {
    rune::from_value::<bool>(binary(ScriptOperatorNames::Equality, this, other)?)
        .map_err(IntoInteropError::to_bms_error)
}

/// The display or debug string.
pub(crate) fn display(this: &RuneNative, debug: bool) -> Result<String, InteropError> {
    let reference = this.to_reference()?;
    match debug {
        true => dispatch::debug(world()?, reference, RUNE_CALLER_CONTEXT),
        false => dispatch::display(world()?, reference, RUNE_CALLER_CONTEXT),
    }
}
