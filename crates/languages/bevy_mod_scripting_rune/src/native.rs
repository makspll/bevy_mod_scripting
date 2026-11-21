//! Typed calls and native values in Rune, see [`bevy_mod_scripting_bindings::native`].
//!
//! Native values are [`RuneNative`]s holding a [`NativeBox`]. Their fields are read and written through their type's typed
//! accessors, their methods are the functions registered on the type, called with the values as they are when typed.
//! Everything else goes through a reference to a copy of the value, like any other value BMS doesn't know natively.
#![allow(unsafe_code)]

use std::{any::TypeId, sync::Arc};

use bevy_mod_scripting_bindings::{
    DynamicScriptFunction, InteropError, ReflectReference, ScriptValue, WorldExtensions, dispatch,
    native::{ArgSource, NativeBox, NativeStorage, NativeType, ReturnSink},
};
use bevy_mod_scripting_bindings_domain::ScriptOperatorNames;
use rune::{
    Any, TypeHash,
    runtime::{BorrowRef, Value},
};

use crate::{
    error::IntoInteropError,
    types::{RuneRef, apply_operator, call_member_inner, get_ref_member, world},
    value::{RUNE_CALLER_CONTEXT, from_rune, new_value, to_rune},
};

/// A value BMS stores natively, see [`bevy_mod_scripting_bindings::native`].
#[derive(Any)]
#[rune(item = ::bms, name = Native)]
pub struct RuneNative(pub NativeBox, Arc<NativeType>);

impl RuneNative {
    /// The value, with its type's fields and methods, as the typed function returning it looked them up.
    pub fn new(storage: NativeBox, native: Arc<NativeType>) -> Self {
        Self(storage, native)
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
    Ref(&'a crate::types::RuneRef),
}

/// The arguments of a typed call, holding a borrow of every native value among them for the call, so nothing the call
/// runs (i.e. a callback back into the VM) can take or drop them while their values are borrowed.
struct RuneArgs<'a> {
    args: &'a [RuneArg<'a>],
    guards: Guards<'a>,
}

/// The borrows of native values held by [`RuneArgs`], without a heap allocation for short calls.
enum Guards<'a> {
    Inline([Option<BorrowRef<'a, RuneNative>>; 4]),
    Heap(Vec<Option<BorrowRef<'a, RuneNative>>>),
}

impl<'a> RuneArgs<'a> {
    fn new(args: &'a [RuneArg<'a>]) -> Self {
        let guard = |arg: &'a RuneArg<'a>| match arg {
            // only natives are borrowed: a failed borrow builds an error
            RuneArg::Value(value) if value.type_hash() == <RuneNative as TypeHash>::HASH => {
                value.borrow_ref::<RuneNative>().ok()
            }
            RuneArg::Value(_) | RuneArg::Native(_) | RuneArg::Ref(_) => None,
        };
        let guards = if args.len() <= 4 {
            let mut inline = [None, None, None, None];
            for (slot, arg) in inline.iter_mut().zip(args) {
                *slot = guard(arg);
            }
            Guards::Inline(inline)
        } else {
            Guards::Heap(args.iter().map(guard).collect())
        };
        Self { args, guards }
    }

    fn guard(&self, i: usize) -> Option<&BorrowRef<'a, RuneNative>> {
        match &self.guards {
            Guards::Inline(guards) => guards.get(i)?.as_ref(),
            Guards::Heap(guards) => guards.get(i)?.as_ref(),
        }
    }

    fn value(&self, i: usize) -> Option<&Value> {
        match self.args.get(i)? {
            RuneArg::Value(value) => Some(value),
            RuneArg::Native(_) | RuneArg::Ref(_) => None,
        }
    }
}

// Safety: native bases come from `NativeBox`es of `RuneNative`s, which are borrowed for the call: either by the caller
// (`RuneArg::Native`) or by the guards held here
unsafe impl ArgSource for RuneArgs<'_> {
    fn len(&self) -> usize {
        self.args.len()
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
            // above `i64::MAX` it's not an integer here, the slow path converts it
            return rune::from_value::<u64>(value.clone())
                .ok()
                .and_then(|u| i64::try_from(u).ok());
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
        match self.args.get(i)? {
            RuneArg::Native(native) => Some(native.0.base()),
            RuneArg::Value(_) => self.guard(i).map(|n| n.0.base()),
            RuneArg::Ref(_) => None,
        }
    }

    fn script_value(&self, i: usize) -> Result<ScriptValue, InteropError> {
        match self.args.get(i) {
            Some(RuneArg::Native(native)) => native.to_script(),
            Some(RuneArg::Value(value)) => from_rune(value),
            Some(RuneArg::Ref(reference)) => Ok(ScriptValue::Reference(reference.0.clone())),
            None => Ok(ScriptValue::Unit),
        }
    }
}

/// The result of a typed call.
struct RuneReturn {
    value: Option<Result<Value, InteropError>>,
    /// How a [`ScriptValue`] result is converted, see [`crate::types::converter`]
    convert: fn(ScriptValue) -> Result<Value, InteropError>,
    /// Whether the native value returned next is a read-only copy, see [`ReturnSink::read_only`]
    read_only: bool,
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

// Safety: `native` keeps the value in a `NativeBox`, which calls `write` once when it's made, and doesn't call it otherwise
unsafe impl ReturnSink for RuneReturn {
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

    unsafe fn native(
        &mut self,
        native: &Arc<NativeType>,
        vtable: &'static bevy_mod_scripting_bindings::native::NativeVTable,
        write: &mut dyn FnMut(*mut u8),
    ) -> bool {
        // Safety: `write` writes a value of the vtable's type, see the method's contract
        match unsafe { NativeBox::new(vtable, write) } {
            Ok(storage) => {
                // Safety: initialised storage; a place is the value in the world, never a copy
                unsafe { NativeStorage::header(storage.base()) }
                    .read_only
                    .set(self.read_only && !vtable.place);
                self.set(new_value(RuneNative::new(storage, native.clone())));
                true
            }
            Err(_) => false,
        }
    }

    fn read_only(&mut self) {
        self.read_only = true;
    }

    fn script_value(&mut self, value: ScriptValue) -> Result<(), InteropError> {
        self.set((self.convert)(value));
        Ok(())
    }
}

/// Calls `function` with the arguments as they are.
pub(crate) fn call(
    function: &DynamicScriptFunction,
    args: &[RuneArg],
) -> Result<Value, InteropError> {
    call_with(
        |args, ret| function.call_with(args, ret, RUNE_CALLER_CONTEXT),
        args,
        to_rune,
    )
}

/// Runs `call` with the arguments as they are, a [`ScriptValue`] result is converted with `convert`.
pub(crate) fn call_with(
    call: impl FnOnce(&dyn ArgSource, &mut dyn ReturnSink) -> Result<(), InteropError>,
    args: &[RuneArg],
    convert: fn(ScriptValue) -> Result<Value, InteropError>,
) -> Result<Value, InteropError> {
    let mut ret = RuneReturn {
        value: None,
        convert,
        read_only: false,
    };
    call(&RuneArgs::new(args), &mut ret)?;
    ret.finish()
}

/// The native type of the value: its fields and methods, as the typed function which made it looked them up.
fn native_type(this: &RuneNative) -> Result<Option<&NativeType>, InteropError> {
    Ok(Some(&this.1))
}

/// Calls the function `name` on the native value with the rest of `args`: its type's, then the ones shared by all references.
///
/// `None` if there is no such function.
fn call_method_named(
    this: &RuneNative,
    name: &str,
    args: &[RuneArg],
) -> Option<Result<Value, InteropError>> {
    let native = match native_type(this) {
        Ok(native) => native,
        Err(e) => return Some(Err(e)),
    };
    if let Some(function) = native.as_ref().and_then(|native| native.methods.get(name)) {
        return Some(call(function, args));
    }
    let shared = (|| {
        let world = world()?;
        let registry = world.script_function_registry();
        let registry = registry.read();
        Ok::<_, InteropError>(
            registry
                .get_function(TypeId::of::<ReflectReference>(), name)
                .ok()
                .cloned(),
        )
    })();
    match shared {
        Ok(Some(function)) => Some(call(&function, args)),
        Ok(None) => None,
        Err(e) => Some(Err(e)),
    }
}

/// `v.name`
pub(crate) fn get(this: &RuneNative, name: &str) -> Result<Value, InteropError> {
    if let Some(native) = native_type(this)?
        && let Some(field) = native.fields.get(name)
    {
        return call(&field.get, &[RuneArg::Native(this)]);
    }
    // methods as values and anything else, through a copy
    get_ref_member(&RuneRef(this.to_reference()?), name)
}

/// `v.name = x`
pub(crate) fn set(this: &RuneNative, name: &str, value: &Value) -> Result<(), InteropError> {
    let native = native_type(this)?;
    match native
        .as_ref()
        .and_then(|native| native.fields.get(name)?.set.as_ref())
    {
        Some(set) => call(set, &[RuneArg::Native(this), RuneArg::Value(value)]).map(|_| ()),
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
    // the arguments of short calls stay on the stack
    let result = match rest {
        [] => call_method_named(&this, name, &[RuneArg::Native(&this)]),
        [a] => call_method_named(&this, name, &[RuneArg::Native(&this), RuneArg::Value(a)]),
        [a, b] => call_method_named(
            &this,
            name,
            &[RuneArg::Native(&this), RuneArg::Value(a), RuneArg::Value(b)],
        ),
        _ => {
            let mut args = Vec::with_capacity(values.len());
            args.push(RuneArg::Native(&this));
            args.extend(rest.iter().map(RuneArg::Value));
            call_method_named(&this, name, &args)
        }
    };
    if let Some(result) = result {
        return result;
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
    if let Some(result) = call_method_named(
        this,
        operator.script_function_name(),
        &[RuneArg::Native(this), RuneArg::Value(other)],
    ) {
        return result;
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
    // like references, values of different kinds are never equal
    if other.borrow_ref::<RuneNative>().is_err() && other.borrow_ref::<RuneRef>().is_err() {
        return Ok(false);
    }
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

#[cfg(test)]
mod test {
    use bevy_ecs::world::World;
    use bevy_mod_scripting_bindings::{
        CurrentScriptAttachment, R, Union, V, WorldAccessGuard,
        function::namespace::NamespaceBuilder, script_native,
    };
    use bevy_reflect::Reflect;

    use super::*;

    /// Not Copy: dropping and cloning go through the vtable
    #[derive(Reflect, Clone, Debug, PartialEq)]
    struct Labelled {
        value: f64,
        label: String,
    }

    fn number(value: f64) -> Value {
        rune::to_value(value).unwrap()
    }

    fn as_number(value: Result<Value, InteropError>) -> f64 {
        rune::from_value::<f64>(value.unwrap()).unwrap()
    }

    /// The chain Rune goes through without its VM: typed calls making native values, fields, operators with the value
    /// passed twice, compound assignment, equality, and a copy of the value coming back as a reference.
    #[test]
    fn native_values_in_rune() {
        let mut world = World::default();
        NamespaceBuilder::<Labelled>::new(&mut world)
            .register("new", |value: f64| {
                V(Labelled {
                    value,
                    label: "made".to_owned(),
                })
            })
            .register("add", |a: R<Labelled>, b: Union<R<Labelled>, f64>| {
                V(Labelled {
                    value: a.value
                        + match b.into_left() {
                            Ok(b) => b.value,
                            Err(n) => n,
                        },
                    label: a.label.clone(),
                })
            })
            .register("eq", |a: R<Labelled>, b: R<Labelled>| a.value == b.value);
        script_native!(
            &mut world,
            Labelled {
                value: f64,
                label: String
            }
        );
        let cache = WorldAccessGuard::setup_cache(&world, CurrentScriptAttachment::empty());
        WorldAccessGuard::with_static_guard(&mut world, cache, |world| {
            let function = |name: &str| {
                world
                    .script_function_registry()
                    .read()
                    .get_function(TypeId::of::<Labelled>(), name)
                    .unwrap()
                    .clone()
            };

            let made = call(&function("new"), &[RuneArg::Value(&number(2.0))]).unwrap();
            let this = made.borrow_ref::<RuneNative>().unwrap();
            assert_eq!(as_number(get(&this, "value")), 2.0);

            set(&this, "value", &number(5.0)).unwrap();
            assert_eq!(as_number(get(&this, "value")), 5.0);
            assign(ScriptOperatorNames::Addition, &this, "value", &number(1.0)).unwrap();
            assert_eq!(as_number(get(&this, "value")), 6.0);
            assert!(set(&this, "missing", &number(1.0)).is_err());

            // the value on both sides, and a number
            let sum = binary(ScriptOperatorNames::Addition, &this, &made).unwrap();
            assert_eq!(
                as_number(get(&sum.borrow_ref::<RuneNative>().unwrap(), "value")),
                12.0
            );
            let sum = binary(ScriptOperatorNames::Addition, &this, &number(1.0)).unwrap();
            assert_eq!(
                as_number(get(&sum.borrow_ref::<RuneNative>().unwrap(), "value")),
                7.0
            );
            assert!(eq(&this, &made).unwrap());
            assert!(
                !eq(&this, &number(6.0)).unwrap(),
                "different kinds are unequal"
            );

            // a copy, as a reference, goes back in through the slow path
            let reference = new_value(RuneRef(this.to_reference().unwrap())).unwrap();
            let sum = call(
                &function("add"),
                &[RuneArg::Value(&reference), RuneArg::Native(&this)],
            )
            .unwrap();
            assert_eq!(
                as_number(get(&sum.borrow_ref::<RuneNative>().unwrap(), "value")),
                12.0
            );
        });
    }
}
