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

use bevy_reflect::{PartialReflect, ReflectRef};

use crate::{
    DynamicScriptFunction, EnumVariantRef, FromScriptErased, FunctionCallContext, FunctionScope,
    InteropError, OwnedValue, ReferencePart, ReflectReference, ScriptValue, WorldExtensions,
    copy_into_script,
    world_extensions::{FunctionLookups, Lookups},
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
    // one registry read for the member and the getter, which is called once unlocked
    let (member, get) = Lookups::with(&world, |lookups| {
        let member = lookups.member(&reference, type_id, name);
        (member, lookups.registry.magic_functions.get)
    });
    match member {
        Ok(ScriptValue::Function(func)) if is_method(&func, type_id) => Ok(Member::Method(func)),
        Ok(member) => Ok(Member::Static(member)),
        Err(name) => get(
            ctx,
            reference.into_owned(),
            ScriptValue::String(Cow::Owned(name.into_owned())),
        )
        .map(Member::Static),
    }
}

/// Accesses a field or element with the [`crate::MagicFunctions::get`] function.
fn magic_get(
    world: WorldGuard,
    reference: ReflectReference,
    key: ScriptValue,
    ctx: FunctionCallContext,
) -> Result<Member, InteropError> {
    // copied out, so the registry isn't locked during the call
    let get = world.script_function_registry().read().magic_functions.get;
    get(ctx, reference, key).map(Member::Static)
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
    // copied out, so the registry isn't locked during the call
    let set = world.script_function_registry().read().magic_functions.set;
    set(ctx, reference, key, value)
}

/// [`index_name`] for a value owned by the script, see [`OwnedValue`].
///
/// Fields are copied out, without the world's access map.
pub fn index_owned_name(
    world: WorldGuard,
    value: &OwnedValue,
    name: &str,
    ctx: FunctionCallContext,
) -> Result<Member, InteropError> {
    profiling::function_scope!("dispatch::index_owned");
    let type_id = value.type_id();
    // functions callable on the value: its type's, then the ones shared by all references
    let func = Lookups::with(&world, |lookups| {
        lookups
            .function([type_id, TypeId::of::<ReflectReference>()], name)
            .ok()
    });
    if let Some(func) = func {
        return Ok(match is_method(&func, type_id) {
            true => Member::Method(func),
            false => Member::Static(ScriptValue::Function(func)),
        });
    }
    if let ReflectRef::Struct(fields) = value.as_reflect().reflect_ref()
        && let Some(field) = fields.field(name)
    {
        return copy_into_script(field, world).map(Member::Static);
    }
    index_owned(
        world,
        value,
        ScriptValue::String(Cow::Owned(name.to_owned())),
        ctx,
    )
}

/// [`index`] for a value owned by the script, see [`index_owned_name`].
pub fn index_owned(
    world: WorldGuard,
    value: &OwnedValue,
    key: ScriptValue,
    ctx: FunctionCallContext,
) -> Result<Member, InteropError> {
    let key = match key.as_string() {
        Ok(name) if value.as_reflect().reflect_ref().as_struct().is_err() => {
            ScriptValue::String(name)
        }
        Ok(name) => return index_owned_name(world, value, &name, ctx),
        Err(key) => key,
    };
    let part = owned_path_part(&world, key, &ctx)?;
    let field = {
        let registry = world.type_registry().read();
        part.reflect_element(value.as_reflect().as_partial_reflect(), &registry)
    };
    match field {
        Ok(Some(field)) => copy_into_script(field, world).map(Member::Static),
        _ => Err(InteropError::string(format!(
            "'{part}' is not a member of a value of type {:?}",
            value.as_reflect().reflect_type_path()
        ))),
    }
}

/// [`set_index`] for a value owned by the script, i.e. `v.x = 1`, which sets the value itself.
pub fn set_index_owned(
    world: WorldGuard,
    value: &mut OwnedValue,
    key: ScriptValue,
    new: ScriptValue,
    ctx: FunctionCallContext,
) -> Result<(), InteropError> {
    profiling::function_scope!("dispatch::set_index_owned");
    let part = owned_path_part(&world, key, &ctx)?;
    let registry = world.type_registry().clone();
    let registry = registry.read();
    let field = part
        .reflect_element_mut(value.as_reflect_mut().as_partial_reflect_mut(), &registry)
        .ok()
        .flatten();
    let Some(field) = field else {
        return Err(InteropError::string(format!(
            "'{part}' is not a member of a value of type {:?}",
            value.as_reflect().reflect_type_path()
        )));
    };
    let target_type_id = field
        .get_represented_type_info()
        .map(|i| i.type_id())
        .or_fake_id();
    drop(registry);
    let new = <Box<dyn PartialReflect>>::from_script_erased(target_type_id, new, world)?;
    field
        .try_apply(new.as_partial_reflect())
        .map_err(InteropError::reflect_apply_error)
}

fn owned_path_part(
    world: &WorldGuard,
    key: ScriptValue,
    ctx: &FunctionCallContext,
) -> Result<ReferencePart, InteropError> {
    ReferencePart::new_from_script_val(key, ctx.language(), Some(world.clone())).map_err(|e| {
        InteropError::InvalidIndex {
            index: Box::new(e),
            reason: Box::new("Cannot convert to valid reflection path".to_owned()),
        }
    })
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

/// Applies a binary operator, i.e. `v * 2`.
///
/// The operator's function is called on the operand which is a reference.
/// When that is the right one, the operator's reflected function is called instead, see [`ScriptOperatorNames::reflected_script_function_name`].
pub fn binary(
    world: WorldGuard,
    operator: ScriptOperatorNames,
    lhs: ScriptValue,
    rhs: ScriptValue,
    ctx: FunctionCallContext,
) -> Result<ScriptValue, InteropError> {
    profiling::function_scope!("dispatch::binary");
    let is_receiver =
        |v: &ScriptValue| matches!(v, ScriptValue::Reference(_) | ScriptValue::Owned(_));
    let (receiver, other, name) = match (lhs, rhs) {
        (receiver, other) if is_receiver(&receiver) => {
            (receiver, other, operator.script_function_name())
        }
        (other, receiver) if is_receiver(&receiver) => {
            match operator.reflected_script_function_name() {
                Some(name) => (receiver, other, name),
                None => {
                    return Err(InteropError::str(
                        "the right operand of this operator can't be a value",
                    ));
                }
            }
        }
        _ => {
            return Err(InteropError::str(
                "neither operand of this operator is a value",
            ));
        }
    };
    let type_id = receiver_type_id(&world, &receiver)?;
    world.call_operator(type_id, name, vec![receiver, other], ctx)
}

/// The type of a reference or owned value whose functions are called for it.
fn receiver_type_id(world: &WorldGuard, receiver: &ScriptValue) -> Result<TypeId, InteropError> {
    Ok(match receiver {
        ScriptValue::Reference(reference) => reference.tail_type_id(world.clone())?.or_fake_id(),
        ScriptValue::Owned(owned) => owned.type_id(),
        _ => TypeId::of::<()>(),
    })
}

/// Applies a unary operator to a reference, i.e. `-v`.
pub fn unary(
    world: WorldGuard,
    operator: ScriptOperatorNames,
    value: ReflectReference,
    ctx: FunctionCallContext,
) -> Result<ScriptValue, InteropError> {
    unary_value(world, operator, ScriptValue::Reference(value), ctx)
}

/// [`unary`] for a reference or an owned value.
pub fn unary_value(
    world: WorldGuard,
    operator: ScriptOperatorNames,
    value: ScriptValue,
    ctx: FunctionCallContext,
) -> Result<ScriptValue, InteropError> {
    profiling::function_scope!("dispatch::unary");
    let type_id = receiver_type_id(&world, &value)?;
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
        value @ (ScriptValue::Reference(_) | ScriptValue::Owned(_)) => {
            unary_value(world, ScriptOperatorNames::Length, value, ctx)
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
