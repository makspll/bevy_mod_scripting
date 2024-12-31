use std::{
    any::{Any, TypeId},
    borrow::Cow,
    error::Error,
    ffi::{CStr, CString, OsStr, OsString},
    path::{Path, PathBuf},
    sync::Arc,
};

use bevy::{
    ecs::{reflect::AppTypeRegistry, world::Mut},
    prelude::AppFunctionRegistry,
    reflect::{
        func::DynamicFunction, OffsetAccess, ParsedPath, PartialReflect, ReflectFromReflect,
    },
};
use bevy_mod_scripting_core::{
    bindings::{
        function::{
            script_function::{AppScriptFunctionRegistry, CallerContext, DynamicScriptFunction},
            CallScriptFunction,
        },
        pretty_print::{DisplayWithWorld, ReflectReferencePrinter},
        script_value::ScriptValue,
        ReflectAllocator, ReflectRefIter, ReflectReference, ReflectionPathExt, TypeIdSource,
        WorldCallbackAccess, WorldGuard,
    },
    error::{InteropError, ScriptError, ScriptResult},
    reflection_extensions::{PartialReflectExt, TypeIdExtensions},
    Either,
};
use bevy_mod_scripting_functions::namespaced_register::{GetNamespacedFunction, Namespace};
use mlua::{Function, IntoLua, Lua, MetaMethod, UserData, UserDataMethods, Value, Variadic};

use super::{
    // proxy::{LuaProxied, LuaValProxy},
    script_value::LuaScriptValue,
    world::GetWorld,
};
use crate::bindings::{script_value::lua_caller_context, world::LuaWorld};

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

/// Looks up a function in the registry on the given type id
fn lookup_function(lua: &Lua, key: &str, type_id: TypeId) -> Option<Result<Function, mlua::Error>> {
    let function = lookup_dynamic_function(lua, key, type_id);

    function.map(|mut function| {
        lua.create_function_mut(move |lua, args: Variadic<LuaScriptValue>| {
            let world = lua.get_world();
            let out = function.call_script_function(
                args.into_iter().map(Into::into),
                world,
                lua_caller_context(),
            )?;

            Ok(LuaScriptValue::from(out))
        })
    })
}

fn lookup_function_typed<T: 'static + ?Sized>(
    lua: &Lua,
    key: &str,
) -> Option<Result<Function, mlua::Error>> {
    let type_id = TypeId::of::<T>();
    lookup_function(lua, key, type_id)
}

fn lookup_dynamic_function(lua: &Lua, key: &str, type_id: TypeId) -> Option<DynamicScriptFunction> {
    let function_registry = lua
        .get_world()
        .with_resource(|registry: &AppScriptFunctionRegistry| registry.clone());
    let registry = function_registry.read();

    registry
        .get_namespaced_function(key.to_string(), Namespace::OnType(type_id))
        .cloned()
}

fn lookup_dynamic_function_typed<T: 'static + ?Sized>(
    lua: &Lua,
    key: &str,
) -> Option<DynamicScriptFunction> {
    let type_id = TypeId::of::<T>();
    lookup_dynamic_function(lua, key, type_id)
}

fn iter_dynamic_function_overloads(
    lua: &Lua,
    key: &str,
    type_id: TypeId,
) -> impl Iterator<Item = DynamicScriptFunction> {
    let registry = lua
        .get_world()
        .with_resource(|registry: &AppScriptFunctionRegistry| registry.clone());
    let registry = registry.read();

    registry
        .iter_overloads_namespaced(key.to_string(), Namespace::OnType(type_id))
        .cloned()
        .collect::<Vec<_>>()
        .into_iter()
}

fn try_call_overloads(
    lua: &Lua,
    key: &str,
    type_id: TypeId,
    args: Vec<ScriptValue>,
    world: WorldGuard,
) -> Result<LuaScriptValue, InteropError> {
    let overloads = iter_dynamic_function_overloads(lua, key, type_id);
    let mut last_error = None;
    for mut overload in overloads {
        match overload.call_script_function(args.clone(), world.clone(), lua_caller_context()) {
            Ok(out) => return Ok(out.into()),
            Err(e) => last_error = Some(e),
        }
    }

    Err(last_error
        .unwrap_or_else(|| InteropError::missing_function(type_id, key.to_string()).into())
        .into())
}

impl UserData for LuaReflectReference {
    fn add_methods<T: UserDataMethods<Self>>(m: &mut T) {
        m.add_meta_function(
            MetaMethod::Index,
            |lua, (self_, key): (LuaReflectReference, LuaScriptValue)| {
                let world = lua.get_world();
                let self_: ReflectReference = self_.into();
                let type_id = self_.tail_type_id(world.clone())?.or_fake_id();

                let key: ScriptValue = key.into();

                if let ScriptValue::String(ref key) = key {
                    if let Some(func) = lookup_function(lua, key, type_id) {
                        return func?.into_lua(lua);
                    }
                    // try look up the function under the reflect reference namespace as well
                    if let Some(func) = lookup_function_typed::<ReflectReference>(lua, key) {
                        return func?.into_lua(lua);
                    }
                };

                // lookup get index function
                let mut index_func = lookup_dynamic_function_typed::<ReflectReference>(lua, "get")
                    .expect("No 'get' function registered for a ReflectReference");

                // call the function with the key
                let out = index_func.call_script_function(
                    vec![ScriptValue::Reference(self_), key],
                    world.clone(),
                    lua_caller_context(),
                )?;
                LuaScriptValue::from(out).into_lua(lua)
            },
        );

        m.add_meta_function(
            MetaMethod::NewIndex,
            |lua, (self_, key, value): (LuaReflectReference, LuaScriptValue, LuaScriptValue)| {
                let self_: ReflectReference = self_.into();
                let key: ScriptValue = key.into();
                let value: ScriptValue = value.into();

                lookup_dynamic_function_typed::<ReflectReference>(lua, "set")
                    .expect("No 'set' function registered for a ReflectReference")
                    .call_script_function(
                        vec![ScriptValue::Reference(self_), key, value],
                        lua.get_world(),
                        lua_caller_context(),
                    )?;

                Ok(())
            },
        );

        m.add_meta_function(
            MetaMethod::Sub,
            |lua, (self_, other): (LuaReflectReference, LuaScriptValue)| {
                let world = lua.get_world();
                let self_: ReflectReference = self_.into();
                let other: ScriptValue = other.into();
                let target_type_id = self_.tail_type_id(world.clone())?.or_fake_id();
                let args = vec![ScriptValue::Reference(self_), other];
                Ok(try_call_overloads(lua, "sub", target_type_id, args, world)?)
            },
        );

        m.add_meta_function(
            MetaMethod::Add,
            |lua, (self_, other): (LuaReflectReference, LuaScriptValue)| {
                let world = lua.get_world();
                let self_: ReflectReference = self_.into();
                let other: ScriptValue = other.into();
                let target_type_id = self_.tail_type_id(world.clone())?.or_fake_id();
                let args = vec![ScriptValue::Reference(self_), other];
                Ok(try_call_overloads(lua, "add", target_type_id, args, world)?)
            },
        );

        m.add_meta_function(
            MetaMethod::Mul,
            |lua, (self_, other): (LuaReflectReference, LuaScriptValue)| {
                let world = lua.get_world();
                let self_: ReflectReference = self_.into();
                let other: ScriptValue = other.into();
                let target_type_id = self_.tail_type_id(world.clone())?.or_fake_id();
                let args = vec![ScriptValue::Reference(self_), other];
                Ok(try_call_overloads(lua, "mul", target_type_id, args, world)?)
            },
        );

        m.add_meta_function(
            MetaMethod::Div,
            |lua, (self_, other): (LuaReflectReference, LuaScriptValue)| {
                let world = lua.get_world();
                let self_: ReflectReference = self_.into();
                let other: ScriptValue = other.into();
                let target_type_id = self_.tail_type_id(world.clone())?.or_fake_id();
                let args = vec![ScriptValue::Reference(self_), other];
                Ok(try_call_overloads(lua, "div", target_type_id, args, world)?)
            },
        );

        m.add_meta_function(
            MetaMethod::Mod,
            |lua, (self_, other): (LuaReflectReference, LuaScriptValue)| {
                let world = lua.get_world();
                let self_: ReflectReference = self_.into();
                let other: ScriptValue = other.into();
                let target_type_id = self_.tail_type_id(world.clone())?.or_fake_id();
                let args = vec![ScriptValue::Reference(self_), other];
                Ok(try_call_overloads(lua, "rem", target_type_id, args, world)?)
            },
        );

        m.add_meta_function(MetaMethod::Unm, |lua, self_: LuaReflectReference| {
            let world = lua.get_world();
            let self_: ReflectReference = self_.into();
            let target_type_id = self_.tail_type_id(world.clone())?.or_fake_id();
            let args = vec![ScriptValue::Reference(self_)];
            Ok(try_call_overloads(lua, "neg", target_type_id, args, world)?)
        });

        m.add_meta_function(
            MetaMethod::Pow,
            |lua, (self_, other): (LuaReflectReference, LuaScriptValue)| {
                let world = lua.get_world();
                let self_: ReflectReference = self_.into();
                let other: ScriptValue = other.into();
                let target_type_id = self_.tail_type_id(world.clone())?.or_fake_id();
                let args = vec![ScriptValue::Reference(self_), other];
                Ok(try_call_overloads(lua, "pow", target_type_id, args, world)?)
            },
        );

        m.add_meta_function(
            MetaMethod::Eq,
            |lua, (self_, other): (LuaReflectReference, LuaScriptValue)| {
                let world = lua.get_world();
                let self_: ReflectReference = self_.into();
                let other: ScriptValue = other.into();
                let target_type_id = self_.tail_type_id(world.clone())?.or_fake_id();
                let args = vec![ScriptValue::Reference(self_), other];
                Ok(try_call_overloads(lua, "eq", target_type_id, args, world)?)
            },
        );

        m.add_meta_function(
            MetaMethod::Lt,
            |lua, (self_, other): (LuaReflectReference, LuaScriptValue)| {
                let world = lua.get_world();
                let self_: ReflectReference = self_.into();
                let other: ScriptValue = other.into();
                let target_type_id = self_.tail_type_id(world.clone())?.or_fake_id();
                let args = vec![ScriptValue::Reference(self_), other];
                Ok(try_call_overloads(lua, "lt", target_type_id, args, world)?)
            },
        );

        #[cfg(any(
            feature = "lua54",
            feature = "lua53",
            feature = "lua52",
            feature = "luajit52",
        ))]
        m.add_meta_function(MetaMethod::Pairs, |l, s: LuaReflectReference| {
            let iter_func = lookup_function_typed::<ReflectReference>(l, "iter")
                .expect("No iter function registered");

            Ok(iter_func)
        });

        // #[cfg(any(
        //     feature = "lua54",
        //     feature = "lua53",
        //     feature = "lua52",
        //     feature = "luajit52",
        // ))]
        // m.add_meta_function(MetaMethod::Pairs, |l, s: LuaReflectReference| {
        //     bevy::log::debug!("ReflectReference::Pairs with value: {:?}", s);
        //     let mut iterator_base = s.0.into_iter_infinite();
        //     let iterator = TypedFunction::from_rust_mut(
        //         move |l, ()| {
        //             let (next_ref, idx) = iterator_base.next_ref();
        //             bevy::log::debug!("iteration: {:?}", idx);
        //             let next = LuaReflectReference(next_ref).to_lua_proxy(l);
        //             let next = match next {
        //                 Ok(n) => Some(n),
        //                 Err(e) => {
        //                     bevy::log::debug!("Error in iteration: {:?}", e);
        //                     None
        //                 }
        //             };
        //             bevy::log::debug!("next: {:?}", next);
        //             // TODO: we should differentiate between no more values and an actual error
        //             match (next, idx) {
        //                 (None, bevy_mod_scripting_core::bindings::IterationKey::Index(_)) => {
        //                     Ok((Value::Nil, Value::Nil))
        //                 }
        //                 (Some(n), bevy_mod_scripting_core::bindings::IterationKey::Index(i)) => {
        //                     Ok((Value::Integer((i + 1) as i64), n))
        //                 }
        //             }
        //         },
        //         l,
        //     )?;

        //     Ok((iterator, Value::Nil, Value::Nil))
        // });

        m.add_meta_function(MetaMethod::ToString, |lua, self_: LuaReflectReference| {
            let world = lua.get_world();
            Ok(self_.0.display_with_world(world))
        });

        m.add_function("print_value", |lua, self_: LuaReflectReference| {
            let world = lua.get_world();
            Ok(self_.0.display_value_with_world(world))
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
            |lua, (self_, key): (LuaStaticReflectReference, LuaScriptValue)| {
                let type_id = self_.0;

                let key: ScriptValue = key.into();

                if let ScriptValue::String(ref key) = key {
                    if let Some(func) = lookup_function(lua, key, type_id) {
                        return func?.into_lua(lua);
                    }
                };

                Err(InteropError::missing_function(type_id, key.to_string()).into())
            },
        );
    }
}

// #[cfg(test)]
// mod test {

//     use bevy::{
//         app::App,
//         ecs::{reflect::AppTypeRegistry, world::World},
//         reflect::{FromReflect, OffsetAccess, Reflect},
//     };
//     use bevy_mod_scripting_core::{
//         bindings::ReflectAllocator,
//         bindings::{ReflectBase, ReflectBaseType, WorldAccessGuard, WorldCallbackAccess},
//     };
//     use bevy_mod_scripting_derive::LuaProxy;

//     use crate::{bindings::world::LuaWorld, type_data::RegisterLua};

//     use super::*;

//     #[derive(Reflect)]
//     struct TestStruct {
//         value: usize,
//         proxy: TestProxied,
//         proxies: Vec<TestProxied>,
//     }

//     #[derive(Reflect)]
//     struct TestTupleStruct(usize, TestProxied, Vec<TestProxied>);

//     #[derive(Reflect)]
//     enum TestTupleEnum {
//         Value(usize),
//         Proxy(TestProxied),
//         Proxies(Vec<TestProxied>),
//     }

//     #[derive(Reflect, LuaProxy)]
//     #[proxy(bms_core_path = "bevy_mod_scripting_core", bms_lua_path = "crate")]
//     #[reflect(LuaProxied)]
//     pub struct TestProxied;

//     impl PartialEq for LuaTestProxied {
//         fn eq(&self, other: &Self) -> bool {
//             self.0 == other.0
//         }
//     }

//     /// asserts that setting then indexing into a LuaReflectReference of type T with the given expression returns the expected value.
//     /// Provides `t and `world` globals, with t being the LuaReflectReference to the provided value.
//     fn assert_lua_set_get_returns<
//         T: Reflect,
//         F: Fn(ReflectReference) -> O,
//         O: for<'l> FromLua<'l> + for<'l> IntoLua<'l> + PartialEq + std::fmt::Debug,
//     >(
//         mut world: &mut World,
//         val: T,
//         expr: &'static str,
//         expected: F,
//     ) {
//         let lua = Lua::new();
//         let mut allocator = ReflectAllocator::default();
//         let reflect_ref = LuaReflectReference(ReflectReference::new_allocated(val, &mut allocator));
//         world.insert_resource(allocator);

//         WorldCallbackAccess::with_callback_access(world, |access| {
//             let globals = lua.globals();
//             globals.set("test", reflect_ref.clone()).unwrap();
//             globals.set("world", LuaWorld(access.clone())).unwrap();
//             globals
//                 .set("expected", expected(reflect_ref.0.clone()))
//                 .unwrap();

//             let lua_code = format!(
//                 r#"
//                 {expr} = expected
//                 return {expr}
//                 "#
//             );
//             let result = lua
//                 .load(&lua_code)
//                 .into_function()
//                 .unwrap_or_else(|e| panic!("Could not load lua code into function: `{e}`"))
//                 .call(())
//                 .unwrap_or_else(|e| {
//                     panic!("Could not convert expression value to expected type: `{e}`")
//                 });
//             let result: O = result;
//             assert_eq!(result, expected(reflect_ref.0));
//         });
//     }

//     #[test]
//     fn test_index_lua_value() {
//         // so we have the registry and can just do this
//         let mut app = App::new();
//         app.register_lua_value::<usize>();

//         assert_lua_set_get_returns(
//             app.world_mut(),
//             TestStruct {
//                 value: 123,
//                 proxy: TestProxied,
//                 proxies: vec![],
//             },
//             "test.value",
//             |_| 123usize,
//         );

//         let mut app = App::new();
//         app.register_lua_value::<usize>();

//         assert_lua_set_get_returns(
//             app.world_mut(),
//             TestTupleStruct(123, TestProxied, vec![]),
//             "test._1",
//             |_| 123usize,
//         );

//         let mut app = App::new();
//         app.register_lua_value::<usize>();

//         assert_lua_set_get_returns(
//             app.world_mut(),
//             TestTupleEnum::Value(123usize),
//             "test._1",
//             |_| 123usize,
//         );
//     }

//     #[test]
//     fn test_index_lua_proxy() {
//         // so we have the registry and can just do this
//         let mut app = App::new();
//         app.register_lua_proxy::<TestProxied>();

//         assert_lua_set_get_returns(
//             app.world_mut(),
//             TestStruct {
//                 value: 123,
//                 proxy: TestProxied,
//                 proxies: vec![],
//             },
//             "test.proxy",
//             |mut r| {
//                 r.index_path(ParsedPath::parse_static("proxy").unwrap());
//                 LuaTestProxied(r)
//             },
//         );

//         let mut app = App::new();
//         app.register_lua_proxy::<TestProxied>();

//         assert_lua_set_get_returns(
//             app.world_mut(),
//             TestTupleStruct(123, TestProxied, vec![]),
//             "test._2",
//             |mut r| {
//                 r.index_path(ParsedPath::parse_static(".1").unwrap());
//                 LuaTestProxied(r)
//             },
//         );

//         let mut app = App::new();
//         app.register_lua_proxy::<TestProxied>();

//         assert_lua_set_get_returns(
//             app.world_mut(),
//             TestTupleEnum::Proxy(TestProxied),
//             "test._1",
//             |mut r| {
//                 r.index_path(ParsedPath::parse_static(".0").unwrap());
//                 LuaTestProxied(r)
//             },
//         );
//     }

//     #[test]
//     fn test_index_lua_proxy_vec() {
//         // so we have the registry and can just do this
//         let mut app = App::new();
//         app.register_lua_proxy::<TestProxied>();

//         assert_lua_set_get_returns(
//             app.world_mut(),
//             TestStruct {
//                 value: 123,
//                 proxy: TestProxied,
//                 proxies: vec![TestProxied],
//             },
//             "test.proxies[1]",
//             |mut r| {
//                 r.index_path(ParsedPath::parse_static("proxies").unwrap());
//                 r.index_path(ParsedPath::parse_static("[0]").unwrap());
//                 LuaTestProxied(r)
//             },
//         );

//         let mut app = App::new();
//         app.register_lua_proxy::<TestProxied>();

//         assert_lua_set_get_returns(
//             app.world_mut(),
//             TestTupleStruct(123, TestProxied, vec![TestProxied]),
//             "test._3[1]",
//             |mut r| {
//                 r.index_path(ParsedPath::parse_static(".2").unwrap());
//                 r.index_path(ParsedPath::parse_static("[0]").unwrap());
//                 LuaTestProxied(r)
//             },
//         );

//         let mut app = App::new();
//         app.register_lua_proxy::<TestProxied>();

//         assert_lua_set_get_returns(
//             app.world_mut(),
//             TestTupleEnum::Proxies(vec![TestProxied]),
//             "test._1[1]",
//             |mut r| {
//                 r.index_path(ParsedPath::parse_static(".0").unwrap());
//                 r.index_path(ParsedPath::parse_static("[0]").unwrap());
//                 LuaTestProxied(r)
//             },
//         );
//     }
// }
