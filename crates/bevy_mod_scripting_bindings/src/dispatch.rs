//! Language agnostic dispatch of script operations to Rust.
//!
//! Language implementations map their operators and metamethods onto these functions, i.e. Lua's `__index` calls [`index`],
//! and `__add` calls [`binary`] with [`ScriptOperatorNames::Addition`]. This keeps the lookup rules the same in every language.
//!
//! What stays with the language is how a method gets its receiver: [`index`] returns [`Member::Method`] for functions called on the value,
//! which Lua passes with its colon syntax (`v:length()`).

use std::{any::TypeId, borrow::Cow};

use bevy_mod_scripting_bindings_domain::ScriptOperatorNames;
use bevy_mod_scripting_display::OrFakeId;
use bevy_mod_scripting_world::WorldGuard;

use crate::{
    DynamicScriptFunction, EnumVariantRef, FunctionCallContext, FunctionScope, InlineValue,
    InteropError, ReflectReference, ScriptValue, ThroughTypeInfo, WorldExtensions,
    world_extensions::FunctionLookups,
};

/// What indexing a value by a name found.
pub enum Member {
    /// A function called on the indexed value, i.e. `v:length()` in Lua.
    ///
    /// The value is not bound to the function, languages without a method call syntax should bind it with [`DynamicScriptFunction::curry`].
    Method(DynamicScriptFunction),
    /// Anything else, i.e. a field, or a static member reached through a type registration.
    Static(ScriptValue),
}

impl From<Member> for ScriptValue {
    fn from(member: Member) -> Self {
        match member {
            Member::Method(func) => ScriptValue::Function(func),
            Member::Static(value) => value,
        }
    }
}

/// Whether a function found on a value of the given type is called on that value, i.e. not a static member of a registration.
fn is_method(func: &DynamicScriptFunction, type_id: TypeId) -> bool {
    type_id == TypeId::of::<EnumVariantRef>()
        || matches!(func.info.namespace, FunctionScope::OnType(on)
            if on == type_id || FunctionScope::OnType(on) == FunctionScope::all_references())
}

/// Indexes a reference, i.e. `v.x` or `v.length`.
///
/// String keys are looked up as members first (see [`WorldExtensions::lookup_member`]),
/// anything not found falls back to the [`crate::MagicFunctions::get`] function, which accesses fields and elements.
pub fn index(
    world: WorldGuard,
    reference: ReflectReference,
    key: ScriptValue,
    ctx: FunctionCallContext,
) -> Result<Member, InteropError> {
    match key.as_string() {
        Ok(name) => index_name(world, Cow::Owned(reference), name, ctx),
        Err(key) => magic_get(world, reference, key, ctx),
    }
}

/// [`index`] with a string key, which is only copied when it names no member, i.e. for field access.
///
/// The reference is only cloned for field access too.
pub fn index_name<'n>(
    world: WorldGuard,
    reference: Cow<'_, ReflectReference>,
    name: impl Into<Cow<'n, str>>,
    ctx: FunctionCallContext,
) -> Result<Member, InteropError> {
    profiling::function_scope!("dispatch::index");
    let type_id = reference.tail_type_id(world.clone())?.or_fake_id();
    match world.lookup_member(&reference, type_id, name) {
        Ok(ScriptValue::Function(func)) if is_method(&func, type_id) => Ok(Member::Method(func)),
        Ok(member) => Ok(Member::Static(member)),
        Err(name) => magic_get(
            world,
            reference.into_owned(),
            ScriptValue::String(Cow::Owned(name.into_owned())),
            ctx,
        ),
    }
}

/// Accesses a field or element with the [`crate::MagicFunctions::get`] function.
fn magic_get(
    world: WorldGuard,
    reference: ReflectReference,
    key: ScriptValue,
    ctx: FunctionCallContext,
) -> Result<Member, InteropError> {
    let registry = world.script_function_registry();
    let registry = registry.read();
    registry
        .magic_functions
        .get(ctx, reference, key)
        .map(Member::Static)
}

/// Assigns to an index of a reference, i.e. `v.x = 1`, through the [`crate::MagicFunctions::set`] function.
pub fn set_index(
    world: WorldGuard,
    reference: ReflectReference,
    key: ScriptValue,
    value: ScriptValue,
    ctx: FunctionCallContext,
) -> Result<(), InteropError> {
    profiling::function_scope!("dispatch::set_index");
    let registry = world.script_function_registry();
    let registry = registry.read();
    registry.magic_functions.set(ctx, reference, key, value)
}

/// Indexes a type, i.e. `Vec3.new` where `Vec3` is a [`ScriptValue::Type`], see [`WorldExtensions::lookup_static_member`].
pub fn static_index(
    world: WorldGuard,
    type_id: TypeId,
    key: ScriptValue,
    ctx: FunctionCallContext,
) -> Result<ScriptValue, InteropError> {
    profiling::function_scope!("dispatch::static_index");
    let key = match key.as_string() {
        Ok(name) => match world.lookup_static_member(type_id, name) {
            Ok(value) => return Ok(value),
            Err(name) => ScriptValue::String(name),
        },
        Err(key) => key,
    };
    Err(InteropError::missing_function(
        format!("{key:#?}"),
        FunctionScope::OnType(type_id),
        Some(ctx),
    ))
}

/// What indexing an inline value by a name found, see [`index_value`].
pub enum ValueMember {
    /// A function of the value's type which doesn't mutate it, called with a copy of the value, i.e. `v:length()` in Lua.
    Method(DynamicScriptFunction),
    /// Anything else needs the value to have a place: the language promotes it in place with
    /// [`crate::InlineValue::into_reference`] and indexes the reference with [`index`].
    NeedsPlace,
}

/// Indexes an inline value, see [`crate::InlineValue`] for why only some members can be reached without a place.
pub fn index_value(world: WorldGuard, value: &InlineValue, name: &str) -> ValueMember {
    profiling::function_scope!("dispatch::index_value");
    match world.lookup_static_member(value.type_id(), name) {
        Ok(ScriptValue::Function(func)) => {
            let mutates = func
                .info
                .arg_info
                .first()
                .is_some_and(|arg| matches!(arg.type_info, Some(ThroughTypeInfo::Mut(_))));
            if mutates {
                ValueMember::NeedsPlace
            } else {
                ValueMember::Method(func)
            }
        }
        _ => ValueMember::NeedsPlace,
    }
}

/// Applies a binary operator, i.e. `v * 2`.
///
/// The operator's function is called on the operand which is a reference or an inline value.
/// When that is the right one, the operator's reflected function is called instead, see [`ScriptOperatorNames::reflected_script_function_name`].
pub fn binary(
    world: WorldGuard,
    operator: ScriptOperatorNames,
    lhs: ScriptValue,
    rhs: ScriptValue,
    ctx: FunctionCallContext,
) -> Result<ScriptValue, InteropError> {
    profiling::function_scope!("dispatch::binary");
    let receiver_type = |world: &WorldGuard, receiver: &ScriptValue| match receiver {
        ScriptValue::Reference(r) => r.tail_type_id(world.clone()).map(|t| Some(t.or_fake_id())),
        ScriptValue::Value(v) => Ok(Some(v.type_id())),
        _ => Ok(None),
    };
    let (receiver, other, name, type_id) = match receiver_type(&world, &lhs)? {
        Some(type_id) => (lhs, rhs, operator.script_function_name(), type_id),
        None => match (
            receiver_type(&world, &rhs)?,
            operator.reflected_script_function_name(),
        ) {
            (Some(type_id), Some(name)) => (rhs, lhs, name, type_id),
            (Some(_), None) => {
                return Err(InteropError::str(
                    "the right operand of this operator can't be a value",
                ));
            }
            (None, _) => {
                return Err(InteropError::str(
                    "neither operand of this operator is a value",
                ));
            }
        },
    };
    world.call_operator(type_id, name, vec![receiver, other], ctx)
}

/// Applies a unary operator to a reference or an inline value, i.e. `-v`.
pub fn unary(
    world: WorldGuard,
    operator: ScriptOperatorNames,
    value: ScriptValue,
    ctx: FunctionCallContext,
) -> Result<ScriptValue, InteropError> {
    profiling::function_scope!("dispatch::unary");
    let type_id = match &value {
        ScriptValue::Reference(r) => r.tail_type_id(world.clone())?.or_fake_id(),
        ScriptValue::Value(v) => v.type_id(),
        _ => {
            return Err(InteropError::str(
                "the operand of this operator is not a value",
            ));
        }
    };
    world.call_operator(type_id, operator.script_function_name(), vec![value], ctx)
}

/// The length of a value, i.e. `#v` in Lua.
///
/// Lists are measured directly, references through their `len` function, which types can override.
/// Other values have no length, and return [`ScriptValue::Unit`].
pub fn len(
    world: WorldGuard,
    value: ScriptValue,
    ctx: FunctionCallContext,
) -> Result<ScriptValue, InteropError> {
    profiling::function_scope!("dispatch::len");
    match value {
        ScriptValue::List(list) => Ok(ScriptValue::Integer(list.len() as i64)),
        value @ (ScriptValue::Reference(_) | ScriptValue::Value(_)) => {
            unary(world, ScriptOperatorNames::Length, value, ctx)
        }
        _ => Ok(ScriptValue::Unit),
    }
}

/// Calls a function shared by all references, which languages call implicitly, i.e. `iter` for `pairs(v)`.
fn call_reserved(
    world: WorldGuard,
    operator: ScriptOperatorNames,
    reference: ReflectReference,
    ctx: FunctionCallContext,
) -> Result<ScriptValue, InteropError> {
    world
        .lookup_reserved_function(operator.script_function_name(), ctx.clone())?
        .call(vec![ScriptValue::Reference(reference)], ctx)
}

/// An iterator over a reference, i.e. `pairs(v)` in Lua: a function returning the next item on each call, and [`ScriptValue::Unit`] at the end.
pub fn iter(
    world: WorldGuard,
    reference: ReflectReference,
    ctx: FunctionCallContext,
) -> Result<ScriptValue, InteropError> {
    profiling::function_scope!("dispatch::iter");
    call_reserved(world, ScriptOperatorNames::Iteration, reference, ctx)
}

fn into_string(value: ScriptValue, name: &'static str) -> Result<String, InteropError> {
    match value {
        ScriptValue::String(string) => Ok(string.into_owned()),
        _ => Err(InteropError::invariant(format!(
            "{name} did not return a string"
        ))),
    }
}

/// The display string of a reference, i.e. `tostring(v)` in Lua.
pub fn display(
    world: WorldGuard,
    reference: ReflectReference,
    ctx: FunctionCallContext,
) -> Result<String, InteropError> {
    profiling::function_scope!("dispatch::display");
    into_string(
        call_reserved(world, ScriptOperatorNames::DisplayPrint, reference, ctx)?,
        "display",
    )
}

/// The debug string of a reference.
pub fn debug(
    world: WorldGuard,
    reference: ReflectReference,
    ctx: FunctionCallContext,
) -> Result<String, InteropError> {
    profiling::function_scope!("dispatch::debug");
    into_string(
        call_reserved(world, ScriptOperatorNames::DebugPrint, reference, ctx)?,
        "debug",
    )
}
