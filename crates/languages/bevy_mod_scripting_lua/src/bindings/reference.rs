use std::any::TypeId;

use bevy_mod_scripting_bindings::{
    ReflectReference, WorldExtensions, error::InteropError, script_value::ScriptValue,
};
use bevy_mod_scripting_bindings_domain::ScriptOperatorNames;
use bevy_mod_scripting_display::OrFakeId;
use bevy_mod_scripting_world::ThreadWorldContainer;
use mlua::{ExternalError, MetaMethod, UserData, UserDataMethods};

use crate::IntoMluaError;

use super::script_value::{LUA_CALLER_CONTEXT, LuaScriptValue};

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

/// The metamethod of a binary arithmetic operator.
///
/// Lua calls it with the operands in their original order, so for `2 * v` the value is on the right.
/// Then the operator's reflected function is called on the value instead, see [`ScriptOperatorNames::reflected_script_function_name`].
fn binary_operator(
    operator: ScriptOperatorNames,
) -> impl Fn(&mlua::Lua, (LuaScriptValue, LuaScriptValue)) -> mlua::Result<LuaScriptValue> {
    move |_, (lhs, rhs)| {
        profiling::function_scope!("MetaMethod::BinaryOperator");
        let world = ThreadWorldContainer
            .try_get_context()
            .map_err(IntoMluaError::to_lua_error)?
            .world;
        let (receiver, other, name) = match (ScriptValue::from(lhs), ScriptValue::from(rhs)) {
            (ScriptValue::Reference(receiver), other) => {
                (receiver, other, operator.script_function_name())
            }
            (other, ScriptValue::Reference(receiver)) => {
                match operator.reflected_script_function_name() {
                    Some(name) => (receiver, other, name),
                    None => {
                        return Err(mlua::Error::runtime(
                            "the right operand of this operator can't be a value",
                        ));
                    }
                }
            }
            _ => {
                return Err(mlua::Error::runtime(
                    "neither operand of this operator is a value",
                ));
            }
        };
        let type_id = receiver
            .tail_type_id(world.clone())
            .map_err(IntoMluaError::to_lua_error)?
            .or_fake_id();
        world
            .call_operator(
                type_id,
                name,
                vec![ScriptValue::Reference(receiver), other],
                LUA_CALLER_CONTEXT,
            )
            .map(LuaScriptValue)
            .map_err(IntoMluaError::to_lua_error)
    }
}

impl UserData for LuaReflectReference {
    fn add_methods<T: UserDataMethods<Self>>(m: &mut T) {
        m.add_meta_function(
            MetaMethod::Index,
            |_, (self_, key): (LuaReflectReference, LuaScriptValue)| {
                profiling::function_scope!("MetaMethod::Index");
                let world = ThreadWorldContainer
                    .try_get_context()
                    .map_err(IntoMluaError::to_lua_error)?
                    .world;
                let self_: ReflectReference = self_.into();
                let type_id = self_
                    .tail_type_id(world.clone())
                    .map_err(IntoMluaError::to_lua_error)?
                    .or_fake_id();

                let key: ScriptValue = key.into();
                let key = match key.as_string() {
                    Ok(string) => match world.lookup_instance_function(type_id, string) {
                        Ok(func) => return Ok(LuaScriptValue(ScriptValue::Function(func))),

                        Err(e) => ScriptValue::String(e),
                    },
                    Err(key) => key,
                };

                // call the default magic getter
                let registry = world.script_function_registry();
                let registry = registry.read();

                let out = registry
                    .magic_functions
                    .get(LUA_CALLER_CONTEXT, self_, key)
                    .map_err(IntoMluaError::to_lua_error)?;
                Ok(LuaScriptValue(out))
            },
        );

        m.add_meta_function(
            MetaMethod::NewIndex,
            |_, (self_, key, value): (LuaReflectReference, LuaScriptValue, LuaScriptValue)| {
                profiling::function_scope!("MetaMethod::NewIndex");
                let world = ThreadWorldContainer
                    .try_get_context()
                    .map_err(IntoMluaError::to_lua_error)?
                    .world;
                let self_: ReflectReference = self_.into();
                let key: ScriptValue = key.into();
                let value: ScriptValue = value.into();

                let registry = world.script_function_registry();
                let registry = registry.read();

                registry
                    .magic_functions
                    .set(LUA_CALLER_CONTEXT, self_, key, value)
                    .map_err(IntoMluaError::to_lua_error)?;

                Ok(())
            },
        );

        m.add_meta_function(
            MetaMethod::Sub,
            binary_operator(ScriptOperatorNames::Subtraction),
        );
        m.add_meta_function(
            MetaMethod::Add,
            binary_operator(ScriptOperatorNames::Addition),
        );
        m.add_meta_function(
            MetaMethod::Mul,
            binary_operator(ScriptOperatorNames::Multiplication),
        );
        m.add_meta_function(
            MetaMethod::Div,
            binary_operator(ScriptOperatorNames::Division),
        );
        m.add_meta_function(
            MetaMethod::Mod,
            binary_operator(ScriptOperatorNames::Remainder),
        );

        m.add_meta_function(MetaMethod::Unm, |_, self_: LuaReflectReference| {
            profiling::function_scope!("MetaMethod::Unm");
            let world = ThreadWorldContainer
                .try_get_context()
                .map_err(IntoMluaError::to_lua_error)?
                .world;
            let self_: ReflectReference = self_.into();
            let target_type_id = self_
                .tail_type_id(world.clone())
                .map_err(IntoMluaError::to_lua_error)?
                .or_fake_id();
            let args = vec![ScriptValue::Reference(self_)];
            let out = world
                .call_operator(
                    target_type_id,
                    ScriptOperatorNames::Negation.script_function_name(),
                    args,
                    LUA_CALLER_CONTEXT,
                )
                .map_err(IntoMluaError::to_lua_error)?;
            Ok(LuaScriptValue(out))
        });

        m.add_meta_function(
            MetaMethod::Pow,
            binary_operator(ScriptOperatorNames::Exponentiation),
        );

        m.add_meta_function(
            MetaMethod::Eq,
            |_, (self_, other): (LuaReflectReference, LuaScriptValue)| {
                profiling::function_scope!("MetaMethod::Eq");
                let world = ThreadWorldContainer
                    .try_get_context()
                    .map_err(IntoMluaError::to_lua_error)?
                    .world;
                let self_: ReflectReference = self_.into();
                let other: ScriptValue = other.into();
                let target_type_id = self_
                    .tail_type_id(world.clone())
                    .map_err(IntoMluaError::to_lua_error)?
                    .or_fake_id();
                let args = vec![ScriptValue::Reference(self_), other];
                let out = world
                    .call_operator(
                        target_type_id,
                        ScriptOperatorNames::Equality.script_function_name(),
                        args,
                        LUA_CALLER_CONTEXT,
                    )
                    .map_err(IntoMluaError::to_lua_error)?;
                Ok(LuaScriptValue(out))
            },
        );

        m.add_meta_function(
            MetaMethod::Lt,
            |_, (self_, other): (LuaReflectReference, LuaScriptValue)| {
                profiling::function_scope!("MetaMethod::Lt");
                let world = ThreadWorldContainer
                    .try_get_context()
                    .map_err(IntoMluaError::to_lua_error)?
                    .world;
                let self_: ReflectReference = self_.into();
                let other: ScriptValue = other.into();
                let target_type_id = self_
                    .tail_type_id(world.clone())
                    .map_err(IntoMluaError::to_lua_error)?
                    .or_fake_id();
                let args = vec![ScriptValue::Reference(self_), other];
                let out = world
                    .call_operator(
                        target_type_id,
                        ScriptOperatorNames::LessThanComparison.script_function_name(),
                        args,
                        LUA_CALLER_CONTEXT,
                    )
                    .map_err(IntoMluaError::to_lua_error)?;
                Ok(LuaScriptValue(out))
            },
        );

        m.add_meta_function(MetaMethod::Len, |_lua, self_: LuaScriptValue| {
            profiling::function_scope!("MetaMethod::Len");
            let world = ThreadWorldContainer
                .try_get_context()
                .map_err(IntoMluaError::to_lua_error)?
                .world;
            let script_value: ScriptValue = self_.into();
            Ok(match script_value {
                ScriptValue::Reference(r) => r.len(world).map_err(IntoMluaError::to_lua_error)?,
                ScriptValue::List(l) => Some(l.len()),
                _ => None,
            })
        });

        #[cfg(any(
            feature = "lua54",
            feature = "lua53",
            feature = "lua52",
            feature = "luajit52",
        ))]
        m.add_meta_function(MetaMethod::Pairs, |_, s: LuaReflectReference| {
            profiling::function_scope!("MetaMethod::Pairs");
            // let mut iter_func = lookup_dynamic_function_typed::<ReflectReference>(l, "iter")
            //     .expect("No iter function registered");
            let world = ThreadWorldContainer
                .try_get_context()
                .map_err(IntoMluaError::to_lua_error)?
                .world;

            let iter_func = world
                .lookup_reserved_function(
                    ScriptOperatorNames::Iteration.script_function_name(),
                    LUA_CALLER_CONTEXT,
                )
                .map_err(IntoMluaError::to_lua_error)?;

            Ok(LuaScriptValue::from(
                iter_func
                    .call(vec![ScriptValue::Reference(s.into())], LUA_CALLER_CONTEXT)
                    .map_err(IntoMluaError::to_lua_error)?,
            ))
        });

        m.add_meta_function(MetaMethod::ToString, |_, self_: LuaReflectReference| {
            profiling::function_scope!("MetaMethod::ToString");
            let world = ThreadWorldContainer
                .try_get_context()
                .map_err(IntoMluaError::to_lua_error)?
                .world;
            let reflect_reference: ReflectReference = self_.into();

            let func = world
                .lookup_reserved_function(
                    ScriptOperatorNames::DisplayPrint.script_function_name(),
                    LUA_CALLER_CONTEXT,
                )
                .map_err(IntoMluaError::to_lua_error)?;
            let out = func
                .call(
                    vec![ScriptValue::Reference(reflect_reference)],
                    LUA_CALLER_CONTEXT,
                )
                .map_err(IntoMluaError::to_lua_error)?;
            Ok(LuaScriptValue(out))
        });
    }
}

/// A reference to just a type. This is used to provide a static call mechanism when we know the type we want to call a function on.
///
/// For example if we want `Entity::from_raw(usize)` to be callable as `Entity.from_raw(usize)` in lua, we can set the global `Entity` to a `LuaStaticReflectReference(Entity::type_id())`.

#[derive(Debug, Clone, Copy, PartialEq, mlua::FromLua)]
pub struct LuaStaticReflectReference(pub TypeId);

impl UserData for LuaStaticReflectReference {
    fn add_methods<T: UserDataMethods<Self>>(m: &mut T) {
        m.add_meta_function(
            MetaMethod::Index,
            |_, (self_, key): (LuaStaticReflectReference, LuaScriptValue)| {
                profiling::function_scope!("MetaMethod::Index");
                let world = ThreadWorldContainer
                    .try_get_context()
                    .map_err(IntoMluaError::to_lua_error)?
                    .world;
                let type_id = self_.0;

                let key: ScriptValue = key.into();

                let key = match key.as_string() {
                    Ok(name) => match world.lookup_static_member(type_id, name) {
                        Ok(value) => return Ok(LuaScriptValue(value)),
                        Err(key) => ScriptValue::String(key),
                    },
                    Err(key) => key,
                };
                Err(InteropError::missing_function(
                    format!("{key:#?}"),
                    type_id.into(),
                    Some(LUA_CALLER_CONTEXT),
                )
                .into_lua_err())
            },
        );
    }
}
