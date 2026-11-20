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
use bevy_reflect::TypeInfo;

use crate::{
    DynamicScriptFunction, EnumVariantRef, FunctionCallContext, FunctionScope, InteropError,
    ReferencePart, ReflectReference, ScriptValue, WorldExtensions,
    function::script_function::CachedMember,
    world_extensions::{FunctionLookups, Lookups, members_depend_on_value},
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
/// Fields of structs are found from type info without reading the world, see [`static_field`]. Members come first, Bevy
/// types often have private fields named like their methods, i.e. `Time::delta`.
pub fn index_name<'n>(
    world: WorldGuard,
    reference: Cow<'_, ReflectReference>,
    name: impl Into<Cow<'n, str>>,
    ctx: FunctionCallContext,
) -> Result<Member, InteropError> {
    profiling::function_scope!("dispatch::index");
    let name = name.into();
    let tail = static_tail(&world, &reference);
    let type_id = match tail {
        Some(type_id) => type_id,
        None => reference.tail_type_id(world.clone())?.or_fake_id(),
    };
    // one registry read for the member and the getter, which is called once unlocked
    let (member, field, get) = Lookups::with(&world, |lookups| {
        let get = lookups.registry.magic_functions.get;
        if tail.is_none() {
            return (lookups.member(&reference, type_id, name), None, get);
        }
        if members_depend_on_value(type_id) {
            let field = struct_field(&world, type_id, &name);
            return (lookups.member(&reference, type_id, name), field, get);
        }
        // the same on every value of the type, until the functions change
        let cached = lookups
            .registry
            .member_of_type(type_id, &name, || CachedMember {
                function: match lookups.member(&reference, type_id, &*name) {
                    Ok(ScriptValue::Function(function)) => Some(function),
                    _ => None,
                },
                field: struct_field(&world, type_id, &name),
            });
        let member = cached.function.map(ScriptValue::Function).ok_or(name);
        (member, cached.field, get)
    });
    match member {
        Ok(ScriptValue::Function(func)) if is_method(&func, type_id) => Ok(Member::Method(func)),
        Ok(member) => Ok(Member::Static(member)),
        Err(name) => {
            // the default getter, done without converting the name or reading values which aren't returned
            if let Some((field, is_reference)) = field
                && std::ptr::fn_addr_eq(
                    get,
                    crate::function::magic_functions::MagicFunctions::default_get
                        as fn(_, _, _) -> _,
                )
            {
                let mut reference = reference.into_owned();
                reference.push_path(ReferencePart::StringAccess(Cow::Borrowed(field)));
                // access to the value is checked once it's read
                if is_reference {
                    return Ok(Member::Static(ScriptValue::Reference(reference)));
                }
                return <ReflectReference as crate::IntoScriptErased>::into_script_erased(
                    reference, world,
                )
                .map(Member::Static);
            }
            get(
                ctx,
                reference.into_owned(),
                ScriptValue::String(Cow::Owned(name.into_owned())),
            )
            .map(Member::Static)
        }
    }
}

/// The type of the value `reference` points to, from type info only (no world access).
///
/// `None` if the path goes through anything but struct fields and tuple struct fields.
fn static_tail(world: &WorldGuard, reference: &ReflectReference) -> Option<TypeId> {
    if reference.reflect_path.is_empty() {
        return Some(reference.base.type_id);
    }
    let registry = world.type_registry();
    let registry = registry.read();
    let mut info = registry.get_type_info(reference.base.type_id())?;
    for part in reference.reflect_path.parts() {
        let field_type = match (part, info) {
            (ReferencePart::StringAccess(n), TypeInfo::Struct(s)) => s.field(n)?.type_id(),
            (ReferencePart::IntegerAccess(i), TypeInfo::TupleStruct(s)) => {
                s.field_at(*i as usize)?.type_id()
            }
            _ => return None,
        };
        info = registry.get_type_info(field_type)?;
    }
    Some(info.type_id())
}

/// The field `name` of the struct `type_id`: its name, and whether its value is returned as a reference (not a
/// primitive or an option).
fn struct_field(world: &WorldGuard, type_id: TypeId, name: &str) -> Option<(&'static str, bool)> {
    let registry = world.type_registry();
    let registry = registry.read();
    let TypeInfo::Struct(s) = registry.get_type_info(type_id)? else {
        return None;
    };
    let field = s.field(name)?;
    let is_reference = matches!(
        registry.get_type_info(field.type_id())?,
        TypeInfo::Struct(_)
            | TypeInfo::TupleStruct(_)
            | TypeInfo::List(_)
            | TypeInfo::Array(_)
            | TypeInfo::Map(_)
            | TypeInfo::Set(_)
            | TypeInfo::Tuple(_)
    );
    Some((field.name(), is_reference))
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
    let (receiver, other, on_left) = match (lhs, rhs) {
        (ScriptValue::Reference(receiver), other) => (receiver, other, true),
        (other, ScriptValue::Reference(receiver)) => (receiver, other, false),
        _ => {
            return Err(InteropError::str(
                "neither operand of this operator is a value",
            ));
        }
    };
    let function = binary_function(world, operator, &receiver, on_left, ctx.clone())?;
    function.call([ScriptValue::Reference(receiver), other], ctx)
}

/// The function [`binary`] calls for the reference operand `receiver`, on the left if `on_left`: it takes the receiver
/// first and the other operand second.
///
/// Languages can call it with their own values instead (see [`DynamicScriptFunction::call_with`]), without converting
/// both operands to [`ScriptValue`]s.
pub fn binary_function(
    world: WorldGuard,
    operator: ScriptOperatorNames,
    receiver: &ReflectReference,
    on_left: bool,
    ctx: FunctionCallContext,
) -> Result<DynamicScriptFunction, InteropError> {
    let name = match on_left {
        true => operator.script_function_name(),
        false => operator.reflected_script_function_name().ok_or_else(|| {
            InteropError::str("the right operand of this operator can't be a value")
        })?,
    };
    operator_function(world, name, receiver, ctx)
}

/// Applies a unary operator to a reference, i.e. `-v`.
pub fn unary(
    world: WorldGuard,
    operator: ScriptOperatorNames,
    value: ReflectReference,
    ctx: FunctionCallContext,
) -> Result<ScriptValue, InteropError> {
    profiling::function_scope!("dispatch::unary");
    let function = unary_function(world, operator, &value, ctx.clone())?;
    function.call([ScriptValue::Reference(value)], ctx)
}

/// The function [`unary`] calls for `value`, which takes it as its only argument, see [`binary_function`].
pub fn unary_function(
    world: WorldGuard,
    operator: ScriptOperatorNames,
    value: &ReflectReference,
    ctx: FunctionCallContext,
) -> Result<DynamicScriptFunction, InteropError> {
    operator_function(world, operator.script_function_name(), value, ctx)
}

/// The function `name` of the type of `receiver`, or the generic one on `ReflectReference`, i.e. `eq`.
fn operator_function(
    world: WorldGuard,
    name: &'static str,
    receiver: &ReflectReference,
    ctx: FunctionCallContext,
) -> Result<DynamicScriptFunction, InteropError> {
    let type_id = receiver.tail_type_id(world.clone())?.or_fake_id();
    world
        .lookup_function([type_id, TypeId::of::<ReflectReference>()], name)
        .map_err(|name| {
            InteropError::missing_function(
                name.to_string(),
                FunctionScope::OnType(type_id),
                Some(ctx),
            )
        })
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
        ScriptValue::Reference(reference) => {
            unary(world, ScriptOperatorNames::Length, reference, ctx)
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
