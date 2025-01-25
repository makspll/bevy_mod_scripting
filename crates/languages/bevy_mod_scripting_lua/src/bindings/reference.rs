use super::script_value::{LuaScriptValue, LUA_CALLER_CONTEXT};
use bevy_mod_scripting_core::{
    bindings::{
        pretty_print::DisplayWithWorld, script_value::ScriptValue, ReflectReference,
        ThreadWorldContainer, WorldContainer,
    },
    error::InteropError,
    reflection_extensions::TypeIdExtensions,
};
use mlua::{MetaMethod, UserData, UserDataMethods};
use std::any::TypeId;

/// Lua UserData wrapper for [`bevy_mod_scripting_core::bindings::ReflectReference`].
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

impl UserData for LuaReflectReference {
    fn add_methods<T: UserDataMethods<Self>>(m: &mut T) {
        m.add_meta_function(
            MetaMethod::Index,
            |_, (self_, key): (LuaReflectReference, LuaScriptValue)| {
                let world = ThreadWorldContainer.try_get_world()?;
                let self_: ReflectReference = self_.into();
                let type_id = self_.tail_type_id(world.clone())?.or_fake_id();

                let key: ScriptValue = key.into();
                let key = match key.as_string() {
                    Ok(string) => {
                        match world
                            .lookup_function([type_id, TypeId::of::<ReflectReference>()], string)
                        {
                            Ok(func) => return Ok(LuaScriptValue(ScriptValue::Function(func))),

                            Err(e) => ScriptValue::String(e),
                        }
                    }
                    Err(key) => key,
                };

                let func = world
                    .lookup_function([TypeId::of::<ReflectReference>()], "get")
                    .map_err(|f| {
                        InteropError::missing_function(TypeId::of::<ReflectReference>(), f)
                    })?;

                // call the function with the key
                let out =
                    func.call(vec![ScriptValue::Reference(self_), key], LUA_CALLER_CONTEXT)?;
                Ok(LuaScriptValue(out))
            },
        );

        m.add_meta_function(
            MetaMethod::NewIndex,
            |_, (self_, key, value): (LuaReflectReference, LuaScriptValue, LuaScriptValue)| {
                let world = ThreadWorldContainer.try_get_world()?;
                let self_: ReflectReference = self_.into();
                let key: ScriptValue = key.into();
                let value: ScriptValue = value.into();

                let func = world
                    .lookup_function([TypeId::of::<ReflectReference>()], "set")
                    .map_err(|f| {
                        InteropError::missing_function(TypeId::of::<ReflectReference>(), f)
                    })?;

                let out = func.call(
                    vec![ScriptValue::Reference(self_), key, value],
                    LUA_CALLER_CONTEXT,
                )?;

                Ok(LuaScriptValue(out))
            },
        );

        m.add_meta_function(
            MetaMethod::Sub,
            |_, (self_, other): (LuaReflectReference, LuaScriptValue)| {
                let world = ThreadWorldContainer.try_get_world()?;
                let self_: ReflectReference = self_.into();
                let other: ScriptValue = other.into();
                let target_type_id = self_.tail_type_id(world.clone())?.or_fake_id();
                let args = vec![ScriptValue::Reference(self_), other];
                let out =
                    world.try_call_overloads(target_type_id, "sub", args, LUA_CALLER_CONTEXT)?;
                Ok(LuaScriptValue(out))
            },
        );

        m.add_meta_function(
            MetaMethod::Add,
            |_, (self_, other): (LuaReflectReference, LuaScriptValue)| {
                let world = ThreadWorldContainer.try_get_world()?;
                let self_: ReflectReference = self_.into();
                let other: ScriptValue = other.into();
                let target_type_id = self_.tail_type_id(world.clone())?.or_fake_id();
                let args = vec![ScriptValue::Reference(self_), other];
                let out =
                    world.try_call_overloads(target_type_id, "add", args, LUA_CALLER_CONTEXT)?;
                Ok(LuaScriptValue(out))
            },
        );

        m.add_meta_function(
            MetaMethod::Mul,
            |_, (self_, other): (LuaReflectReference, LuaScriptValue)| {
                let world = ThreadWorldContainer.try_get_world()?;
                let self_: ReflectReference = self_.into();
                let other: ScriptValue = other.into();
                let target_type_id = self_.tail_type_id(world.clone())?.or_fake_id();
                let args = vec![ScriptValue::Reference(self_), other];
                let out =
                    world.try_call_overloads(target_type_id, "mul", args, LUA_CALLER_CONTEXT)?;
                Ok(LuaScriptValue(out))
            },
        );

        m.add_meta_function(
            MetaMethod::Div,
            |_, (self_, other): (LuaReflectReference, LuaScriptValue)| {
                let world = ThreadWorldContainer.try_get_world()?;
                let self_: ReflectReference = self_.into();
                let other: ScriptValue = other.into();
                let target_type_id = self_.tail_type_id(world.clone())?.or_fake_id();
                let args = vec![ScriptValue::Reference(self_), other];
                let out =
                    world.try_call_overloads(target_type_id, "div", args, LUA_CALLER_CONTEXT)?;
                Ok(LuaScriptValue(out))
            },
        );

        m.add_meta_function(
            MetaMethod::Mod,
            |_, (self_, other): (LuaReflectReference, LuaScriptValue)| {
                let world = ThreadWorldContainer.try_get_world()?;
                let self_: ReflectReference = self_.into();
                let other: ScriptValue = other.into();
                let target_type_id = self_.tail_type_id(world.clone())?.or_fake_id();
                let args = vec![ScriptValue::Reference(self_), other];
                let out =
                    world.try_call_overloads(target_type_id, "rem", args, LUA_CALLER_CONTEXT)?;
                Ok(LuaScriptValue(out))
            },
        );

        m.add_meta_function(MetaMethod::Unm, |_, self_: LuaReflectReference| {
            let world = ThreadWorldContainer.try_get_world()?;
            let self_: ReflectReference = self_.into();
            let target_type_id = self_.tail_type_id(world.clone())?.or_fake_id();
            let args = vec![ScriptValue::Reference(self_)];
            let out = world.try_call_overloads(target_type_id, "neg", args, LUA_CALLER_CONTEXT)?;
            Ok(LuaScriptValue(out))
        });

        m.add_meta_function(
            MetaMethod::Pow,
            |_, (self_, other): (LuaReflectReference, LuaScriptValue)| {
                let world = ThreadWorldContainer.try_get_world()?;
                let self_: ReflectReference = self_.into();
                let other: ScriptValue = other.into();
                let target_type_id = self_.tail_type_id(world.clone())?.or_fake_id();
                let args = vec![ScriptValue::Reference(self_), other];
                let out =
                    world.try_call_overloads(target_type_id, "pow", args, LUA_CALLER_CONTEXT)?;
                Ok(LuaScriptValue(out))
            },
        );

        m.add_meta_function(
            MetaMethod::Eq,
            |_, (self_, other): (LuaReflectReference, LuaScriptValue)| {
                let world = ThreadWorldContainer.try_get_world()?;
                let self_: ReflectReference = self_.into();
                let other: ScriptValue = other.into();
                let target_type_id = self_.tail_type_id(world.clone())?.or_fake_id();
                let args = vec![ScriptValue::Reference(self_), other];
                let out =
                    world.try_call_overloads(target_type_id, "eq", args, LUA_CALLER_CONTEXT)?;
                Ok(LuaScriptValue(out))
            },
        );

        m.add_meta_function(
            MetaMethod::Lt,
            |_, (self_, other): (LuaReflectReference, LuaScriptValue)| {
                let world = ThreadWorldContainer.try_get_world()?;
                let self_: ReflectReference = self_.into();
                let other: ScriptValue = other.into();
                let target_type_id = self_.tail_type_id(world.clone())?.or_fake_id();
                let args = vec![ScriptValue::Reference(self_), other];
                let out =
                    world.try_call_overloads(target_type_id, "lt", args, LUA_CALLER_CONTEXT)?;
                Ok(LuaScriptValue(out))
            },
        );

        m.add_meta_function(MetaMethod::Len, |_lua, self_: LuaScriptValue| {
            let world = ThreadWorldContainer.try_get_world()?;
            let script_value: ScriptValue = self_.into();
            Ok(match script_value {
                ScriptValue::Reference(r) => r.len(world)?,
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
            // let mut iter_func = lookup_dynamic_function_typed::<ReflectReference>(l, "iter")
            //     .expect("No iter function registered");
            let world = ThreadWorldContainer.try_get_world()?;

            let iter_func = world
                .lookup_function([TypeId::of::<ReflectReference>()], "iter")
                .map_err(|f| InteropError::missing_function(TypeId::of::<ReflectReference>(), f))?;

            Ok(LuaScriptValue::from(iter_func.call(
                vec![ScriptValue::Reference(s.into())],
                LUA_CALLER_CONTEXT,
            )?))
        });

        m.add_meta_function(MetaMethod::ToString, |_, self_: LuaReflectReference| {
            let world = ThreadWorldContainer.try_get_world()?;
            let reflect_reference: ReflectReference = self_.into();

            let func = world
                .lookup_function([TypeId::of::<ReflectReference>()], "display_ref")
                .map_err(|f| InteropError::missing_function(TypeId::of::<ReflectReference>(), f))?;
            let out = func.call(
                vec![ScriptValue::Reference(reflect_reference)],
                LUA_CALLER_CONTEXT,
            )?;
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
                let world = ThreadWorldContainer.try_get_world()?;
                let type_id = self_.0;

                let key: ScriptValue = key.into();

                let key = match key.as_string() {
                    Ok(name) => match world.lookup_function([type_id], name) {
                        Ok(func) => return Ok(LuaScriptValue(ScriptValue::Function(func))),
                        Err(key) => ScriptValue::String(key),
                    },
                    Err(key) => key,
                };

                let world = ThreadWorldContainer.try_get_world()?;
                Err(
                    InteropError::missing_function(type_id, key.display_with_world(world.clone()))
                        .into(),
                )
            },
        );
    }
}
