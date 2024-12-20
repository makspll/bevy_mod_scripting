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
        function::CallScriptFunction,
        pretty_print::{DisplayWithWorld, ReflectReferencePrinter},
        script_value::ScriptValue,
        ReflectAllocator, ReflectRefIter, ReflectReference, ReflectionPathExt, TypeIdSource,
        WorldCallbackAccess,
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
use crate::bindings::world::LuaWorld;

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

    function.map(|function| {
        lua.create_function(move |lua, args: Variadic<LuaScriptValue>| {
            let world = lua.get_world();
            let out = function.call_script_function(args.into_iter().map(Into::into), world)?;

            Ok(LuaScriptValue::from(out))
        })
    })
}

fn lookup_dynamic_function<'lua>(
    lua: &'lua Lua,
    key: &str,
    type_id: TypeId,
) -> Option<DynamicFunction<'static>> {
    let function_registry = lua
        .get_world()
        .with_resource(|registry: &AppFunctionRegistry| registry.clone());
    let registry = function_registry.read();

    registry
        .get_namespaced_function(key.to_string(), Namespace::OnType(type_id))
        .cloned()
}

fn lookup_dynamic_function_typed<'lua, T: 'static + ?Sized>(
    lua: &'lua Lua,
    key: &str,
) -> Option<Result<DynamicFunction<'static>, mlua::Error>> {
    let type_id = TypeId::of::<T>();
    let function = lookup_dynamic_function(lua, key, type_id);

    function.map(Ok)
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
                };

                // lookup get function
                let index_func =
                    lookup_dynamic_function_typed::<ReflectReference>(lua, "get_1_indexed")
                        .expect("No 'get' function registered for a ReflectReference")?;

                // call the function with the key
                let out = index_func.call_script_function(
                    vec![ScriptValue::Reference(self_), key],
                    world.clone(),
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

                lookup_dynamic_function_typed::<ReflectReference>(lua, "set_1_indexed")
                    .expect("No 'set' function registered for a ReflectReference")?
                    .call_script_function(
                        vec![ScriptValue::Reference(self_), key, value],
                        lua.get_world(),
                    )?;

                Ok(())
            },
        );

        // m.add_meta_function(
        //     MetaMethod::Index,
        //     |l, (mut self_, key): (LuaReflectReference, Value)| {
        //         bevy::log::debug!(
        //             "ReflectReference::Index with key: {:?} and value: {:?}",
        //             key,
        //             self_
        //         );
        //         // catchall, parse the path
        //         let mut elem = Self::parse_value_index(key)?;
        //         Self::to_host_index(&mut elem);
        //         self_.0.index_path(elem);
        //         bevy::log::debug!("Target reflect reference after indexing key: {:?}", self_.0);
        //         self_.to_lua_proxy(l)
        //     },
        // );
        // m.add_meta_function(
        //     MetaMethod::NewIndex,
        //     |l, (mut self_, key, value): (LuaReflectReference, Value, Value)| {
        //         bevy::log::debug!(
        //             "ReflectReference::NewIndex with key: {:?} and value: {:?}",
        //             key,
        //             value
        //         );

        //         let mut elem = Self::parse_value_index(key)?;
        //         Self::to_host_index(&mut elem);
        //         self_.0.index_path(elem);
        //         bevy::log::debug!("Target reflect reference after indexing key: {:?}", self_.0);
        //         self_.set_with_lua_proxy(l, value)
        //     },
        // );

        // m.add_function_mut(
        //     "insert",
        //     |l, (self_, key, value): (LuaReflectReference, Value, Value)| {
        //         let world = l.get_world();
        //         bevy::log::debug!(
        //             "ReflectReference::insert with key: {:?} and value: {:?}",
        //             key,
        //             value
        //         );
        //         let key = self_
        //             .clone()
        //             .concrete_from_value(key, l, TypeIdSource::Key)?;
        //         bevy::log::debug!("Key: {:?}", key);
        //         let value = self_
        //             .clone()
        //             .concrete_from_value(value, l, TypeIdSource::Element)?;
        //         bevy::log::debug!("Value: {:?}", value);
        //         self_
        //             .0
        //             .with_reflect_mut(&world, |r| r.try_insert_boxed(key, value))??;
        //         Ok(())
        //     },
        // );

        // m.add_function_mut("push", |l, (self_, value): (LuaReflectReference, Value)| {
        //     let world = l.get_world();
        //     bevy::log::debug!("ReflectReference::push with value: {:?}", value);
        //     let value = self_
        //         .clone()
        //         .concrete_from_value(value, l, TypeIdSource::Element)?;
        //     self_
        //         .0
        //         .with_reflect_mut(&world, |r| r.try_push_boxed(value))??;
        //     Ok(())
        // });

        // m.add_function_mut("pop", |l, self_: LuaReflectReference| {
        //     let world = l.get_world();
        //     bevy::log::debug!("ReflectReference::pop");
        //     let ref_ = self_.0.with_reflect_mut(&world, |r| {
        //         let last_elem = r.try_pop_boxed()?;
        //         let allocator = world.allocator();
        //         let mut allocator = allocator.write();
        //         let reflect_ref = LuaReflectReference(ReflectReference::new_allocated_boxed(
        //             last_elem,
        //             &mut allocator,
        //         ));
        //         Ok::<_, ScriptError>(reflect_ref)
        //     })??;

        //     Ok(ref_)
        // });

        // m.add_function("clear", |l, self_: LuaReflectReference| {
        //     let world = l.get_world();
        //     bevy::log::debug!("ReflectReference::clear");
        //     self_.0.with_reflect_mut(&world, |r| r.try_clear())??;
        //     Ok(())
        // });

        // m.add_meta_function(MetaMethod::Len, |l, self_: LuaReflectReference| {
        //     self_.len(l)
        // });

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
