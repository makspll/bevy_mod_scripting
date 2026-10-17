use std::{any::TypeId, borrow::Cow};

use bevy_mod_scripting_bindings::{ReflectReference, WorldGuard, dispatch, error::InteropError};
use bevy_mod_scripting_bindings_domain::ScriptOperatorNames;
use bevy_mod_scripting_world::ThreadWorldContainer;
use mlua::{FromLua, IntoLua, MetaMethod, UserData, UserDataMethods};

use crate::IntoMluaError;

use super::script_value::{LUA_CALLER_CONTEXT, LuaScriptValue, registered_function_into_lua};

/// Lua UserData wrapper for [`ReflectReference`].
/// Acts as a lua reflection interface. Any value which is registered in the type registry can be interacted with using this type.
#[derive(Debug, Clone, PartialEq, mlua::FromLua)]
pub struct LuaReflectReference(pub ReflectReference);

impl AsRef<ReflectReference> for LuaReflectReference {
    fn as_ref(&self) -> &ReflectReference {
        &self.0
    }
}

impl From<LuaReflectReference> for ReflectReference {
    fn from(value: LuaReflectReference) -> Self {
        value.0
    }
}

impl From<ReflectReference> for LuaReflectReference {
    fn from(value: ReflectReference) -> Self {
        Self(value)
    }
}

/// Runs a [`dispatch`] function with the world of the running script.
fn with_world<O>(
    f: impl FnOnce(WorldGuard<'static>) -> Result<O, InteropError>,
) -> mlua::Result<O> {
    let world = ThreadWorldContainer
        .try_get_context()
        .map_err(IntoMluaError::to_lua_error)?
        .world;
    f(world).map_err(IntoMluaError::to_lua_error)
}

/// The metamethod of a binary operator, see [`dispatch::binary`].
///
/// Lua calls it with the operands in their original order, so for `2 * v` the value is on the right.
fn binary_operator(
    operator: ScriptOperatorNames,
) -> impl Fn(&mlua::Lua, (LuaScriptValue, LuaScriptValue)) -> mlua::Result<LuaScriptValue> {
    move |_, (lhs, rhs)| {
        profiling::function_scope!("MetaMethod::BinaryOperator");
        with_world(|world| {
            dispatch::binary(world, operator, lhs.into(), rhs.into(), LUA_CALLER_CONTEXT)
        })
        .map(LuaScriptValue)
    }
}

impl UserData for LuaReflectReference {
    fn add_methods<T: UserDataMethods<Self>>(m: &mut T) {
        // methods come back unbound, `v:method()` passes the value itself
        m.add_meta_function(
            MetaMethod::Index,
            |lua, (self_, key): (mlua::UserDataRef<LuaReflectReference>, mlua::Value)| {
                profiling::function_scope!("MetaMethod::Index");
                let member = match &key {
                    // names and the receiver are only copied for field access
                    mlua::Value::String(name) => {
                        let name = name.to_str()?;
                        with_world(|world| {
                            dispatch::index_name(
                                world,
                                Cow::Borrowed(&self_.0),
                                &*name,
                                LUA_CALLER_CONTEXT,
                            )
                        })?
                    }
                    _ => {
                        let key = LuaScriptValue::from_lua(key, lua)?;
                        with_world(|world| {
                            dispatch::index(world, self_.0.clone(), key.into(), LUA_CALLER_CONTEXT)
                        })?
                    }
                };
                match member {
                    dispatch::Member::Method(func) => registered_function_into_lua(lua, func),
                    member => LuaScriptValue(member.into()).into_lua(lua),
                }
            },
        );

        m.add_meta_function(
            MetaMethod::NewIndex,
            |_, (self_, key, value): (LuaReflectReference, LuaScriptValue, LuaScriptValue)| {
                profiling::function_scope!("MetaMethod::NewIndex");
                with_world(|world| {
                    dispatch::set_index(
                        world,
                        self_.0,
                        key.into(),
                        value.into(),
                        LUA_CALLER_CONTEXT,
                    )
                })
            },
        );

        for (metamethod, operator) in [
            (MetaMethod::Add, ScriptOperatorNames::Addition),
            (MetaMethod::Sub, ScriptOperatorNames::Subtraction),
            (MetaMethod::Mul, ScriptOperatorNames::Multiplication),
            (MetaMethod::Div, ScriptOperatorNames::Division),
            (MetaMethod::Mod, ScriptOperatorNames::Remainder),
            (MetaMethod::Pow, ScriptOperatorNames::Exponentiation),
            (MetaMethod::Eq, ScriptOperatorNames::Equality),
            (MetaMethod::Lt, ScriptOperatorNames::LessThanComparison),
        ] {
            m.add_meta_function(metamethod, binary_operator(operator));
        }

        m.add_meta_function(MetaMethod::Unm, |_, self_: LuaReflectReference| {
            profiling::function_scope!("MetaMethod::Unm");
            with_world(|world| {
                dispatch::unary(
                    world,
                    ScriptOperatorNames::Negation,
                    self_.0,
                    LUA_CALLER_CONTEXT,
                )
            })
            .map(LuaScriptValue)
        });

        m.add_meta_function(MetaMethod::Len, |_lua, self_: LuaScriptValue| {
            profiling::function_scope!("MetaMethod::Len");
            with_world(|world| dispatch::len(world, self_.into(), LUA_CALLER_CONTEXT))
                .map(LuaScriptValue)
        });

        #[cfg(any(
            feature = "lua54",
            feature = "lua53",
            feature = "lua52",
            feature = "luajit52",
        ))]
        m.add_meta_function(MetaMethod::Pairs, |_, self_: LuaReflectReference| {
            profiling::function_scope!("MetaMethod::Pairs");
            with_world(|world| dispatch::iter(world, self_.0, LUA_CALLER_CONTEXT))
                .map(LuaScriptValue)
        });

        m.add_meta_function(MetaMethod::ToString, |_, self_: LuaReflectReference| {
            profiling::function_scope!("MetaMethod::ToString");
            with_world(|world| dispatch::display(world, self_.0, LUA_CALLER_CONTEXT))
        });
    }
}

/// A type itself, i.e. what [`bevy_mod_scripting_bindings::ScriptValue::Type`] is in Lua. Indexing it reaches the type's static members, i.e. `Entity.from_raw(..)`.
#[derive(Debug, Clone, Copy, PartialEq, mlua::FromLua)]
pub struct LuaTypeRef(pub TypeId);

impl UserData for LuaTypeRef {
    fn add_methods<T: UserDataMethods<Self>>(m: &mut T) {
        m.add_meta_function(
            MetaMethod::Index,
            |_, (self_, key): (LuaTypeRef, LuaScriptValue)| {
                profiling::function_scope!("MetaMethod::Index");
                with_world(|world| {
                    dispatch::static_index(world, self_.0, key.into(), LUA_CALLER_CONTEXT)
                })
                .map(LuaScriptValue)
            },
        );
    }
}
