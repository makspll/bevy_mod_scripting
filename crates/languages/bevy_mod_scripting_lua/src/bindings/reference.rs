use super::script_value::{LuaScriptValue, LUA_CALLER_CONTEXT};
use bevy_mod_scripting_core::{
    bindings::{
        function::{namespace::Namespace, script_function::DynamicScriptFunction},
        pretty_print::DisplayWithWorld,
        script_value::ScriptValue,
        ReflectReference, ThreadWorldContainer, WorldContainer, WorldGuard,
    },
    error::InteropError,
    reflection_extensions::TypeIdExtensions,
};
use mlua::{MetaMethod, UserData, UserDataMethods};
use std::{any::TypeId, borrow::Cow};

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

// Looks up a function in the registry on the given type id
// fn lookup_function(lua: &Lua, key: &str, type_id: TypeId) -> Option<Result<Function, mlua::Error>> {
//     let function = lookup_dynamic_function(lua, key, type_id);

//     function.map(|mut function| {
//         lua.create_function_mut(move |_lua, args: Variadic<LuaScriptValue>| {
//             let world = ThreadWorldContainer.get_world();
//             let out = function.call_script_function(
//                 args.into_iter().map(Into::into),
//                 world,
//                 LUA_CALLER_CONTEXT,
//             )?;

//             Ok(LuaScriptValue::from(out))
//         })
//     })
// }

// fn lookup_function_typed<T: 'static + ?Sized>(
//     lua: &Lua,
//     key: &str,
// ) -> Option<Result<Function, mlua::Error>> {
//     let type_id = TypeId::of::<T>();
//     lookup_function(lua, key, type_id)
// }

// fn lookup_dynamic_function(
//     _lua: &Lua,
//     key: &str,
//     type_id: TypeId,
// ) -> Option<DynamicScriptFunction> {
//     let function_registry = ThreadWorldContainer
//         .get_world()
//         .with_resource(|registry: &AppScriptFunctionRegistry| registry.clone());
//     let registry = function_registry.read();

//     registry
//         .get_function(Namespace::OnType(type_id), key.to_string())
//         .cloned()
// }

// fn lookup_dynamic_function_typed<T: 'static + ?Sized>(
//     lua: &Lua,
//     key: &str,
// ) -> Option<DynamicScriptFunction> {
//     let type_id = TypeId::of::<T>();
//     lookup_dynamic_function(lua, key, type_id)
// }

// fn iter_dynamic_function_overloads(
//     _lua: &Lua,
//     key: &str,
//     type_id: TypeId,
// ) -> impl Iterator<Item = DynamicScriptFunction> {
//     let registry = ThreadWorldContainer
//         .get_world()
//         .with_resource(|registry: &AppScriptFunctionRegistry| registry.clone());
//     let registry = registry.read();

//     registry
//         .iter_overloads(Namespace::OnType(type_id), key.to_string())
//         .cloned()
//         .collect::<Vec<_>>()
//         .into_iter()
// }

// fn try_call_overloads(
//     lua: &Lua,
//     key: &str,
//     type_id: TypeId,
//     args: Vec<ScriptValue>,
//     world: WorldGuard,
// ) -> Result<LuaScriptValue, InteropError> {
//     let overloads = iter_dynamic_function_overloads(lua, key, type_id);
//     let mut last_error = None;
//     for mut overload in overloads {
//         match overload.call_script_function(
//             args.clone(),
//             world.clone(),
//             lua_caller_context(Some(type_id)),
//         ) {
//             Ok(out) => return Ok(out.into()),
//             Err(e) => last_error = Some(e),
//         }
//     }

//     Err(last_error.unwrap_or_else(|| InteropError::missing_function(type_id, key.to_string())))
// }

/// Look up a function on the given type ids
fn lookup_function(
    guard: WorldGuard,
    type_ids: impl IntoIterator<Item = TypeId>,
    name: impl Into<Cow<'static, str>>,
) -> Result<DynamicScriptFunction, Cow<'static, str>> {
    let registry = guard.script_function_registry();
    let registry = registry.read();

    let mut name = name.into();
    for type_id in type_ids {
        name = match registry.get_function(Namespace::OnType(type_id), name) {
            Ok(func) => return Ok(func.clone()),
            Err(name) => name,
        };
    }

    Err(name)
}

fn try_call_overloads(
    guard: WorldGuard,
    type_id: TypeId,
    name: impl Into<Cow<'static, str>>,
    args: Vec<ScriptValue>,
) -> Result<LuaScriptValue, InteropError> {
    let registry = guard.script_function_registry();
    let registry = registry.read();

    let name = name.into();
    let overload_iter = match registry.iter_overloads(Namespace::OnType(type_id), name) {
        Ok(iter) => iter,
        Err(name) => return Err(InteropError::missing_function(type_id, name.to_string())),
    };

    let mut last_error = None;
    for overload in overload_iter {
        match overload.call(args.clone(), guard.clone(), LUA_CALLER_CONTEXT) {
            Ok(out) => return Ok(out.into()),
            Err(e) => last_error = Some(e),
        }
    }

    Err(last_error.expect("invariant, iterator should always return at least one item, and if the call fails it should return an error"))
}

impl UserData for LuaReflectReference {
    fn add_methods<T: UserDataMethods<Self>>(m: &mut T) {
        m.add_meta_function(
            MetaMethod::Index,
            |_, (self_, key): (LuaReflectReference, LuaScriptValue)| {
                let world = ThreadWorldContainer.get_world();
                let self_: ReflectReference = self_.into();
                let type_id = self_.tail_type_id(world.clone())?.or_fake_id();

                let key: ScriptValue = key.into();
                let key = match key.as_string() {
                    Ok(string) => {
                        match lookup_function(
                            world.clone(),
                            [type_id, TypeId::of::<ReflectReference>()],
                            string,
                        ) {
                            Ok(func) => return Ok(LuaScriptValue(ScriptValue::Function(func))),

                            Err(e) => ScriptValue::String(e),
                        }
                    }
                    Err(key) => key,
                };

                let func =
                    lookup_function(world.clone(), [TypeId::of::<ReflectReference>()], "get")
                        .expect("No 'get' function registered for a ReflectReference");
                // call the function with the key
                let out = func.call(
                    vec![ScriptValue::Reference(self_), key],
                    world,
                    LUA_CALLER_CONTEXT,
                )?;
                Ok(LuaScriptValue(out))
                // // call the function with the key
                // let out = index_func.call_script_function(
                //     vec![ScriptValue::Reference(self_), key],
                //     world.clone(),
                //     lua_caller_context(Some(std::any::TypeId::of::<ReflectReference>())),
                // )?;
                // LuaScriptValue::from(out).into_lua(lua)
            },
        );

        m.add_meta_function(
            MetaMethod::NewIndex,
            |_, (self_, key, value): (LuaReflectReference, LuaScriptValue, LuaScriptValue)| {
                let self_: ReflectReference = self_.into();
                let key: ScriptValue = key.into();
                let value: ScriptValue = value.into();

                // lookup_dynamic_function_typed::<ReflectReference>(lua, "set")
                //     .expect("No 'set' function registered for a ReflectReference")
                //     .call_script_function(
                //         vec![ScriptValue::Reference(self_), key, value],
                //         ThreadWorldContainer.get_world(),
                //         lua_caller_context(Some(std::any::TypeId::of::<ReflectReference>())),
                //     )?;
                // Ok(())

                let func = lookup_function(
                    ThreadWorldContainer.get_world(),
                    [TypeId::of::<ReflectReference>()],
                    "set",
                )
                .expect("No 'set' function registered for a ReflectReference");

                let out = func.call(
                    vec![ScriptValue::Reference(self_), key, value],
                    ThreadWorldContainer.get_world(),
                    LUA_CALLER_CONTEXT,
                )?;

                Ok(LuaScriptValue(out))
            },
        );

        m.add_meta_function(
            MetaMethod::Sub,
            |_, (self_, other): (LuaReflectReference, LuaScriptValue)| {
                let world = ThreadWorldContainer.get_world();
                let self_: ReflectReference = self_.into();
                let other: ScriptValue = other.into();
                let target_type_id = self_.tail_type_id(world.clone())?.or_fake_id();
                let args = vec![ScriptValue::Reference(self_), other];
                // Ok(try_call_overloads(lua, "sub", target_type_id, args, world)?)
                Ok(try_call_overloads(world, target_type_id, "sub", args)?)
            },
        );

        m.add_meta_function(
            MetaMethod::Add,
            |_, (self_, other): (LuaReflectReference, LuaScriptValue)| {
                let world = ThreadWorldContainer.get_world();
                let self_: ReflectReference = self_.into();
                let other: ScriptValue = other.into();
                let target_type_id = self_.tail_type_id(world.clone())?.or_fake_id();
                let args = vec![ScriptValue::Reference(self_), other];
                // Ok(try_call_overloads(lua, "add", target_type_id, args, world)?)
                Ok(try_call_overloads(world, target_type_id, "add", args)?)
            },
        );

        m.add_meta_function(
            MetaMethod::Mul,
            |_, (self_, other): (LuaReflectReference, LuaScriptValue)| {
                let world = ThreadWorldContainer.get_world();
                let self_: ReflectReference = self_.into();
                let other: ScriptValue = other.into();
                let target_type_id = self_.tail_type_id(world.clone())?.or_fake_id();
                let args = vec![ScriptValue::Reference(self_), other];
                // Ok(try_call_overloads(lua, "mul", target_type_id, args, world)?)
                Ok(try_call_overloads(world, target_type_id, "mul", args)?)
            },
        );

        m.add_meta_function(
            MetaMethod::Div,
            |_, (self_, other): (LuaReflectReference, LuaScriptValue)| {
                let world = ThreadWorldContainer.get_world();
                let self_: ReflectReference = self_.into();
                let other: ScriptValue = other.into();
                let target_type_id = self_.tail_type_id(world.clone())?.or_fake_id();
                let args = vec![ScriptValue::Reference(self_), other];
                // Ok(try_call_overloads(lua, "div", target_type_id, args, world)?)
                Ok(try_call_overloads(world, target_type_id, "div", args)?)
            },
        );

        m.add_meta_function(
            MetaMethod::Mod,
            |_, (self_, other): (LuaReflectReference, LuaScriptValue)| {
                let world = ThreadWorldContainer.get_world();
                let self_: ReflectReference = self_.into();
                let other: ScriptValue = other.into();
                let target_type_id = self_.tail_type_id(world.clone())?.or_fake_id();
                let args = vec![ScriptValue::Reference(self_), other];
                // Ok(try_call_overloads(lua, "rem", target_type_id, args, world)?)
                Ok(try_call_overloads(world, target_type_id, "rem", args)?)
            },
        );

        m.add_meta_function(MetaMethod::Unm, |_, self_: LuaReflectReference| {
            let world = ThreadWorldContainer.get_world();
            let self_: ReflectReference = self_.into();
            let target_type_id = self_.tail_type_id(world.clone())?.or_fake_id();
            let args = vec![ScriptValue::Reference(self_)];
            // Ok(try_call_overloads(lua, "neg", target_type_id, args, world)?)
            Ok(try_call_overloads(world, target_type_id, "neg", args)?)
        });

        m.add_meta_function(
            MetaMethod::Pow,
            |_, (self_, other): (LuaReflectReference, LuaScriptValue)| {
                let world = ThreadWorldContainer.get_world();
                let self_: ReflectReference = self_.into();
                let other: ScriptValue = other.into();
                let target_type_id = self_.tail_type_id(world.clone())?.or_fake_id();
                let args = vec![ScriptValue::Reference(self_), other];
                // Ok(try_call_overloads(lua, "pow", target_type_id, args, world)?)
                Ok(try_call_overloads(world, target_type_id, "pow", args)?)
            },
        );

        m.add_meta_function(
            MetaMethod::Eq,
            |_, (self_, other): (LuaReflectReference, LuaScriptValue)| {
                let world = ThreadWorldContainer.get_world();
                let self_: ReflectReference = self_.into();
                let other: ScriptValue = other.into();
                let target_type_id = self_.tail_type_id(world.clone())?.or_fake_id();
                let args = vec![ScriptValue::Reference(self_), other];
                // Ok(try_call_overloads(lua, "eq", target_type_id, args, world)?)
                Ok(try_call_overloads(world, target_type_id, "eq", args)?)
            },
        );

        m.add_meta_function(
            MetaMethod::Lt,
            |_, (self_, other): (LuaReflectReference, LuaScriptValue)| {
                let world = ThreadWorldContainer.get_world();
                let self_: ReflectReference = self_.into();
                let other: ScriptValue = other.into();
                let target_type_id = self_.tail_type_id(world.clone())?.or_fake_id();
                let args = vec![ScriptValue::Reference(self_), other];
                // Ok(try_call_overloads(lua, "lt", target_type_id, args, world)?)
                Ok(try_call_overloads(world, target_type_id, "lt", args)?)
            },
        );

        m.add_meta_function(MetaMethod::Len, |_lua, self_: LuaScriptValue| {
            let world = ThreadWorldContainer.get_world();
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
            let iter_func = lookup_function(
                ThreadWorldContainer.get_world(),
                [TypeId::of::<ReflectReference>()],
                "iter",
            )
            .expect("No iter function registered");
            let world = ThreadWorldContainer.get_world();

            Ok(LuaScriptValue::from(iter_func.call(
                vec![ScriptValue::Reference(s.into())],
                world,
                LUA_CALLER_CONTEXT,
            )?))
        });

        m.add_meta_function(MetaMethod::ToString, |_, self_: LuaReflectReference| {
            let world = ThreadWorldContainer.get_world();
            let reflect_reference: ReflectReference = self_.into();

            let func = lookup_function(
                world.clone(),
                [TypeId::of::<ReflectReference>()],
                "display_ref",
            )
            .expect("No 'display' function registered for a ReflectReference");
            let out = func.call(
                vec![ScriptValue::Reference(reflect_reference)],
                world,
                LUA_CALLER_CONTEXT,
            )?;
            Ok(LuaScriptValue(out))
            // let mut display_func =
            //     lookup_dynamic_function_typed::<ReflectReference>(lua, "display_ref")
            //         .expect("No 'display' function registered for a ReflectReference");

            // let out = display_func.call_script_function(
            //     vec![ScriptValue::Reference(self_)],
            //     world,
            //     lua_caller_context(Some(std::any::TypeId::of::<ReflectReference>())),
            // )?;

            // Ok(LuaScriptValue::from(out))
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
                let type_id = self_.0;

                let key: ScriptValue = key.into();

                // if let ScriptValue::String(ref key) = key {
                //     if let Some(func) = lookup_function(lua, key, type_id) {
                //         return func?.into_lua(lua);
                //     }
                // };
                let key = match key.as_string() {
                    Ok(name) => {
                        match lookup_function(ThreadWorldContainer.get_world(), [type_id], name) {
                            Ok(func) => return Ok(LuaScriptValue(ScriptValue::Function(func))),
                            Err(key) => ScriptValue::String(key),
                        }
                    }
                    Err(key) => key,
                };

                let world = ThreadWorldContainer.get_world();
                Err(
                    InteropError::missing_function(type_id, key.display_with_world(world.clone()))
                        .into(),
                )
            },
        );
    }
}
