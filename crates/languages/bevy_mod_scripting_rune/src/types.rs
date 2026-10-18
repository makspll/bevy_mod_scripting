//! The types scripts see: references, types, namespaces and functions, and how their members are resolved.
//!
//! Rune resolves `v.name` and `v.name()` by the hash of the name, in a table fixed when the context is built.
//! There is no fallback for names it doesn't know, so every name which can be a member (function names, field names, type names, ..)
//! is registered as a method and a field of each of the types here, see [`register_types`]. `v["name"]` never needs that, it goes through
//! the index protocol, which gets the name as a value.
use std::{any::TypeId, borrow::Cow, cmp::Ordering, collections::BTreeSet, sync::Arc};

use bevy_mod_scripting_bindings::{
    InteropError, ReflectReference, ScriptValue, WorldGuard,
    dispatch::{self, Member},
    function::script_function::{DynamicScriptFunction, DynamicScriptFunctionMut},
};
use bevy_mod_scripting_bindings_domain::ScriptOperatorNames;
use bevy_mod_scripting_world::ThreadWorldContainer;
use bevy_platform::collections::HashMap;
use rune::{
    Any, ContextError, Module,
    alloc::fmt::TryWrite,
    runtime::{Formatter, InstAddress, Memory, Output, Protocol, Value, VmResult},
};

use crate::{
    error::vm,
    value::{RUNE_CALLER_CONTEXT, from_rune, function_to_rune, namespace_to_rune, to_rune},
};

/// A script function, either stateless or stateful.
#[derive(Clone)]
pub enum ScriptFunction {
    /// A stateless function
    Plain(DynamicScriptFunction),
    /// A function which keeps state
    Mut(DynamicScriptFunctionMut),
}

impl ScriptFunction {
    /// Calls the function
    pub fn call(&self, args: Vec<ScriptValue>) -> Result<ScriptValue, InteropError> {
        match self {
            ScriptFunction::Plain(f) => f.call(args, RUNE_CALLER_CONTEXT),
            ScriptFunction::Mut(f) => f.call(args, RUNE_CALLER_CONTEXT),
        }
    }
}

/// A reference to a reflected value, any value which is registered in the type registry can be interacted with using this type, see [`ScriptValue::Reference`].
#[derive(Any, Clone)]
#[rune(item = ::bms, name = Ref)]
pub struct RuneRef(pub ReflectReference);

/// A type itself, i.e. `world`. Its members are the type's static members, see [`ScriptValue::Type`].
#[derive(Any, Clone, Copy)]
#[rune(item = ::bms, name = Type)]
pub struct RuneType(pub TypeId);

/// The values of a namespace, as returned by `import_namespace`, whose functions can be called with the method syntax.
#[derive(Any, Clone)]
#[rune(item = ::bms, name = Namespace)]
pub struct RuneNamespace(pub Arc<HashMap<String, ScriptValue>>);

/// A function from BMS, call it with `f.call(..)`.
#[derive(Any, Clone)]
#[rune(item = ::bms, name = Function)]
pub struct RuneFunction(pub ScriptFunction);

/// An iterator over a reference.
#[derive(Any)]
#[rune(item = ::bms, name = Iter)]
pub struct RuneIter(ScriptFunction);

fn world() -> Result<WorldGuard<'static>, InteropError> {
    Ok(ThreadWorldContainer.try_get_context()?.world)
}

/// Converts the result of a call into the value a script gets, a bound method for a method which was not called.
fn member_to_rune(
    member: Member,
    receiver: Option<&ReflectReference>,
) -> Result<Value, InteropError> {
    match member {
        Member::Method(func) => match receiver {
            // the receiver is not one of the arguments of the function a script calls
            Some(receiver) => function_to_rune(
                func.curry(ScriptValue::Reference(receiver.clone())),
                Some(1),
            ),
            None => function_to_rune(func, None),
        },
        Member::Static(value) => to_rune(value),
    }
}

fn get_ref_member(this: &RuneRef, name: &str) -> Result<Value, InteropError> {
    let member = dispatch::index_name(
        world()?,
        Cow::Borrowed(&this.0),
        name.to_owned(),
        RUNE_CALLER_CONTEXT,
    )?;
    member_to_rune(member, Some(&this.0))
}

fn get_type_member(this: &RuneType, name: &str) -> Result<Value, InteropError> {
    let value = dispatch::static_index(
        world()?,
        this.0,
        ScriptValue::String(name.to_owned().into()),
        RUNE_CALLER_CONTEXT,
    )?;
    to_rune(value)
}

fn get_namespace_member(this: &RuneNamespace, name: &str) -> Result<Value, InteropError> {
    match this.0.get(name) {
        Some(value) => to_rune(value.clone()),
        None => Err(InteropError::external(crate::error::RuneError(format!(
            "the namespace has no member `{name}`"
        )))),
    }
}

fn call_value(value: ScriptValue, args: Vec<ScriptValue>) -> Result<ScriptValue, InteropError> {
    match value {
        ScriptValue::Function(f) => ScriptFunction::Plain(f).call(args),
        ScriptValue::FunctionMut(f) => ScriptFunction::Mut(f).call(args),
        _ => Err(InteropError::str("the member is not a function")),
    }
}

/// Makes a raw Rune function from one which takes the values on the stack, the first of which is the receiver of a method.
pub fn call_raw<F>(
    f: F,
) -> impl Fn(&mut dyn Memory, InstAddress, usize, Output) -> VmResult<()> + Send + Sync + 'static
where
    F: Fn(&[Value]) -> Result<Value, InteropError> + Send + Sync + 'static,
{
    move |stack: &mut dyn Memory, addr: InstAddress, count: usize, out: Output| {
        let result = {
            let values = rune::vm_try!(stack.slice_at(addr, count));
            vm(f(values))
        };
        let value = rune::vm_try!(result);
        rune::vm_try!(out.store(stack, value));
        VmResult::Ok(())
    }
}

/// Calls the member `name` of the receiver, which is the first value on the stack, with the rest as arguments.
fn call_member_inner(name: &str, values: &[Value]) -> Result<Value, InteropError> {
    let (receiver, rest) = values
        .split_first()
        .ok_or_else(|| InteropError::str("a method needs a receiver"))?;
    let args = rest
        .iter()
        .map(from_rune)
        .collect::<Result<Vec<_>, _>>()?;
    let result = if let Ok(this) = receiver.borrow_ref::<RuneRef>() {
        let member = dispatch::index_name(
            world()?,
            Cow::Borrowed(&this.0),
            name.to_owned(),
            RUNE_CALLER_CONTEXT,
        )?;
        match member {
            Member::Method(func) => {
                let mut with_receiver = Vec::with_capacity(args.len() + 1);
                with_receiver.push(ScriptValue::Reference(this.0.clone()));
                with_receiver.extend(args);
                func.call(with_receiver, RUNE_CALLER_CONTEXT)?
            }
            Member::Static(value) => call_value(value, args)?,
        }
    } else if let Ok(this) = receiver.borrow_ref::<RuneType>() {
        let member = dispatch::static_index(
            world()?,
            this.0,
            ScriptValue::String(name.to_owned().into()),
            RUNE_CALLER_CONTEXT,
        )?;
        call_value(member, args)?
    } else if let Ok(this) = receiver.borrow_ref::<RuneNamespace>() {
        let member = this.0.get(name).cloned().ok_or_else(|| {
            InteropError::external(crate::error::RuneError(format!(
                "the namespace has no member `{name}`"
            )))
        })?;
        call_value(member, args)?
    } else {
        return Err(InteropError::str("unsupported receiver"));
    };
    // namespaces are what `import_namespace` returns
    if name == "import_namespace" {
        namespace_to_rune(result)
    } else {
        to_rune(result)
    }
}

fn binary(operator: ScriptOperatorNames, this: &RuneRef, other: &Value) -> VmResult<Value> {
    vm((|| {
        let rhs = from_rune(other)?;
        let out = dispatch::binary(
            world()?,
            operator,
            ScriptValue::Reference(this.0.clone()),
            rhs,
            RUNE_CALLER_CONTEXT,
        )?;
        to_rune(out)
    })())
}

fn same_reference(this: &RuneRef, other: &Value) -> Result<Option<bool>, InteropError> {
    // like in Lua, values of different kinds are never equal
    if other.borrow_ref::<RuneRef>().is_err() {
        return Ok(Some(false));
    }
    let rhs = from_rune(other)?;
    let out = dispatch::binary(
        world()?,
        ScriptOperatorNames::Equality,
        ScriptValue::Reference(this.0.clone()),
        rhs,
        RUNE_CALLER_CONTEXT,
    )?;
    Ok(match out {
        ScriptValue::Bool(b) => Some(b),
        _ => None,
    })
}

fn less_than(this: &RuneRef, other: &Value) -> Result<bool, InteropError> {
    let rhs = from_rune(other)?;
    let out = dispatch::binary(
        world()?,
        ScriptOperatorNames::LessThanComparison,
        ScriptValue::Reference(this.0.clone()),
        rhs,
        RUNE_CALLER_CONTEXT,
    )?;
    Ok(matches!(out, ScriptValue::Bool(true)))
}

macro_rules! operators {
    ($module:expr, $(($protocol:ident, $operator:ident)),* $(,)?) => {
        $(
            $module.associated_function(
                &Protocol::$protocol,
                |this: &RuneRef, other: Value| {
                    binary(ScriptOperatorNames::$operator, this, &other)
                },
            )?;
        )*
    };
}

/// Interns the names, which are leaked as Rune needs them to live forever, and only once.
pub fn intern(name: &str) -> &'static str {
    use std::sync::{Mutex, OnceLock};
    static NAMES: OnceLock<Mutex<std::collections::HashSet<&'static str>>> = OnceLock::new();
    let mut names = NAMES
        .get_or_init(Default::default)
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    if let Some(interned) = names.get(name) {
        return interned;
    }
    let leaked: &'static str = Box::leak(name.to_owned().into_boxed_str());
    names.insert(leaked);
    leaked
}

/// Registers the types and everything their members can be called, see the module documentation.
pub fn register_types(
    module: &mut Module,
    names: &BTreeSet<&'static str>,
) -> Result<(), ContextError> {
    module.ty::<RuneRef>()?;
    module.ty::<RuneType>()?;
    module.ty::<RuneNamespace>()?;
    module.ty::<RuneFunction>()?;
    module.ty::<RuneIter>()?;

    // `v["name"]` works for any name
    module.associated_function(&Protocol::INDEX_GET, |this: &RuneRef, key: Value| {
        vm((|| match from_rune(&key)? {
            ScriptValue::String(name) => get_ref_member(this, &name),
            key => {
                let member = dispatch::index(world()?, this.0.clone(), key, RUNE_CALLER_CONTEXT)?;
                member_to_rune(member, Some(&this.0))
            }
        })())
    })?;
    module.associated_function(
        &Protocol::INDEX_SET,
        |this: &RuneRef, key: Value, value: Value| {
            vm((|| {
                dispatch::set_index(
                    world()?,
                    this.0.clone(),
                    from_rune(&key)?,
                    from_rune(&value)?,
                    RUNE_CALLER_CONTEXT,
                )
            })())
        },
    )?;
    module.associated_function(&Protocol::INDEX_GET, |this: &RuneType, key: Value| {
        vm((|| match from_rune(&key)? {
            ScriptValue::String(name) => get_type_member(this, &name),
            _ => Err(InteropError::str("types are indexed with names")),
        })())
    })?;
    module.associated_function(&Protocol::INDEX_GET, |this: &RuneNamespace, key: Value| {
        vm((|| match from_rune(&key)? {
            ScriptValue::String(name) => get_namespace_member(this, &name),
            _ => Err(InteropError::str("namespaces are indexed with names")),
        })())
    })?;

    operators!(
        module,
        (ADD, Addition),
        (SUB, Subtraction),
        (MUL, Multiplication),
        (DIV, Division),
        (REM, Remainder),
    );

    module.associated_function(&Protocol::PARTIAL_EQ, |this: &RuneRef, other: Value| {
        vm(same_reference(this, &other).map(|eq| eq.unwrap_or(false)))
    })?;
    module.associated_function(&Protocol::PARTIAL_EQ, |this: &RuneType, other: Value| {
        other
            .borrow_ref::<RuneType>()
            .map(|other| other.0 == this.0)
            .unwrap_or(false)
    })?;
    module.associated_function(
        &Protocol::PARTIAL_CMP,
        |this: &RuneRef, other: Value| -> VmResult<Option<Ordering>> {
            vm((|| {
                if same_reference(this, &other)? == Some(true) {
                    return Ok(Some(Ordering::Equal));
                }
                if less_than(this, &other)? {
                    return Ok(Some(Ordering::Less));
                }
                // greater, when the other is a reference which is less than this one
                match other.borrow_ref::<RuneRef>() {
                    Ok(other) => {
                        let this_value = Value::new(RuneRef(this.0.clone()))
                            .map_err(crate::error::IntoInteropError::to_bms_error)?;
                        Ok(less_than(&other, &this_value)?.then_some(Ordering::Greater))
                    }
                    Err(_) => Ok(None),
                }
            })())
        },
    )?;

    module.associated_function(&Protocol::INTO_ITER, |this: &RuneRef| {
        vm((|| {
            match dispatch::iter(world()?, this.0.clone(), RUNE_CALLER_CONTEXT)? {
                ScriptValue::FunctionMut(f) => Ok(RuneIter(ScriptFunction::Mut(f))),
                ScriptValue::Function(f) => Ok(RuneIter(ScriptFunction::Plain(f))),
                _ => Err(InteropError::str("iterating returned no function")),
            }
        })())
    })?;
    module.associated_function(&Protocol::NEXT, |this: &mut RuneIter| {
        vm((|| match this.0.call(Vec::new())? {
            ScriptValue::Unit => Ok(None),
            value => Ok(Some(to_rune(value)?)),
        })())
    })?;

    module.associated_function(&Protocol::DISPLAY_FMT, |this: &RuneRef, f: &mut Formatter| {
        let text = rune::vm_try!(vm(world().and_then(|world| dispatch::display(
            world,
            this.0.clone(),
            RUNE_CALLER_CONTEXT
        ))));
        rune::vm_write!(f, "{}", text)
    })?;
    module.associated_function(&Protocol::DEBUG_FMT, |this: &RuneRef, f: &mut Formatter| {
        let text = rune::vm_try!(vm(world().and_then(|world| dispatch::debug(
            world,
            this.0.clone(),
            RUNE_CALLER_CONTEXT
        ))));
        rune::vm_write!(f, "{}", text)
    })?;

    // functions of BMS are called with `f.call(..)`
    module
        .raw_function(
            "call",
            call_raw(|values: &[Value]| {
                let (function, rest) = values
                    .split_first()
                    .ok_or_else(|| InteropError::str("a method needs a receiver"))?;
                let function = function
                    .borrow_ref::<RuneFunction>()
                    .map_err(crate::error::IntoInteropError::to_bms_error)?;
                let args = rest
                    .iter()
                    .map(from_rune)
                    .collect::<Result<Vec<_>, _>>()?;
                to_rune(function.0.call(args)?)
            }),
        )
        .build_associated::<RuneFunction>()?;

    for &name in names {
        // `v.name(..)`
        for container in [Container::Ref, Container::Type, Container::Namespace] {
            let builder = module.raw_function(
                name,
                call_raw(move |values: &[Value]| call_member_inner(name, values)),
            );
            match container {
                Container::Ref => builder.build_associated::<RuneRef>()?,
                Container::Type => builder.build_associated::<RuneType>()?,
                Container::Namespace => builder.build_associated::<RuneNamespace>()?,
            };
        }

        // `v.name` and `v.name = x`
        module.field_function(&Protocol::GET, name, move |this: &RuneRef| {
            vm(get_ref_member(this, name))
        })?;
        module.field_function(&Protocol::SET, name, move |this: &RuneRef, value: Value| {
            vm((|| {
                dispatch::set_index(
                    world()?,
                    this.0.clone(),
                    ScriptValue::String(name.to_owned().into()),
                    from_rune(&value)?,
                    RUNE_CALLER_CONTEXT,
                )
            })())
        })?;
        module.field_function(&Protocol::GET, name, move |this: &RuneType| {
            vm(get_type_member(this, name))
        })?;
        module.field_function(&Protocol::GET, name, move |this: &RuneNamespace| {
            vm(get_namespace_member(this, name))
        })?;
    }
    Ok(())
}

#[derive(Clone, Copy)]
enum Container {
    Ref,
    Type,
    Namespace,
}
