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
    reflect::{OffsetAccess, ParsedPath, PartialReflect, ReflectFromReflect},
};
use bevy_mod_scripting_core::{
    bindings::{
        function::CallableWithAccess,
        pretty_print::{DisplayWithWorld, ReflectReferencePrinter},
        script_val::{FromScriptValue, IntoScriptValue, ScriptValue},
        DeferredReflection, ReflectAllocator, ReflectRefIter, ReflectReference, ReflectionPathElem,
        TypeIdSource, WorldCallbackAccess,
    },
    error::{FunctionError, ScriptError, ScriptResult, ValueConversionError},
    new_deferred_reflection,
    reflection_extensions::{PartialReflectExt, TypeIdExtensions},
    Either,
};
use bevy_mod_scripting_functions::namespaced_register::{GetNamespacedFunction, Namespace};
use mlua::{IntoLua, Lua, MetaMethod, UserData, UserDataMethods, Value, Variadic};

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

impl LuaReflectReference {
    pub fn len(&self, lua: &Lua) -> Result<Option<usize>, mlua::Error> {
        self.0.len(lua.get_world()).map_err(mlua::Error::external)
    }

    // pub fn concrete_from_value(
    //     self,
    //     value: Value,
    //     lua: &Lua,
    //     type_id_source: TypeIdSource,
    // ) -> Result<Box<dyn PartialReflect>, mlua::Error> {
    //     let world = lua.get_world();

    //     let type_id = self.0.type_id_of(type_id_source, &world)?;

    //     let o = ReflectReference::map_type_data(
    //         type_id,
    //         &world.clone(),
    //         move |type_data: Option<Either<ReflectLuaValue, ReflectLuaProxied>>| {
    //             let val = match type_data {
    //                 Some(Either::Left(value_data)) => {
    //                     bevy::log::debug!("Converting using ReflectLuaValue");
    //                     (value_data.from_value)(value, lua)?
    //                 }
    //                 Some(Either::Right(proxy_data)) => {
    //                     bevy::log::debug!("Converting using ReflectLuaProxied");
    //                     let other = (proxy_data.from_proxy)(value, lua)?;
    //                     other.with_reflect(&world, |r, _, _| r.clone_value())?
    //                 }
    //                 None => {
    //                     bevy::log::debug!("No conversion type data found");
    //                     return Err(ScriptError::new_runtime_error(format!(
    //                         "Tried to convert lua value: '{value:?}', to {}: '{}'. but this type does not support conversion from lua.",
    //                         match type_id_source {
    //                             TypeIdSource::Key => "key type of",
    //                             TypeIdSource::Element => "element type of",
    //                             TypeIdSource::Tail => "",
    //                         },
    //                         ReflectReferencePrinter::new(self.0).pretty_print(&world),
    //                     )));
    //                 }
    //             };
    //             Ok(val)
    //         },
    //     )??;
    //     Ok(o)
    // }

    // /// Queries the reflection system for a proxy registration for the underlying type.
    // /// If found will convert to lua using this proxy
    // /// If not found will use <Self as [`IntoLua`]>::into_lua to convert to lua
    // pub fn to_lua_proxy(self, lua: &Lua) -> Result<Value<'_>, mlua::Error> {
    //     // note we do not need to refer to LuaWorld here, it does not matter what the proxy is, that's pretty neat,
    //     let world = lua.get_world();

    //     let type_id = self.0.type_id_of(TypeIdSource::Tail, &world)?;

    //     ReflectReference::map_type_data(
    //         type_id,
    //         &world,
    //         |type_data: Option<Either<ReflectLuaValue, ReflectLuaProxied>>| match type_data {
    //             Some(Either::Left(value_data)) => self
    //                 .0
    //                 .with_reflect(&world, |r| (value_data.into_value)(r, lua))?,
    //             Some(Either::Right(proxy_data)) => Ok((proxy_data.into_proxy)(self.0, lua)?),
    //             None => Ok(LuaReflectReference(self.0).into_lua(lua)?),
    //         },
    //     )?
    // }

    // pub fn set_with_lua_proxy(self, lua: &Lua, value: Value) -> Result<(), mlua::Error> {
    //     bevy::log::debug!("Setting lua reflect reference with value: {:?}", value);

    //     let world = lua.get_world();

    //     let type_id = self.0.type_id_of(TypeIdSource::Tail, &world)?;

    //     ReflectReference::map_type_data(
    //         type_id,
    //         &world.clone(),
    //         move |type_data: Option<Either<ReflectLuaValue, ReflectLuaProxied>>| {
    //             // let world = world.clone(); // move copy into closure
    //             match type_data {
    //                 Some(Either::Left(value_data)) => {
    //                     let other = (value_data.from_value)(value, lua)?;
    //                     self.0.with_reflect_mut(&world, |r| {
    //                         r.try_apply(other.as_partial_reflect())
    //                             .map_err(ScriptError::new_reflection_error)
    //                     })?
    //                 }
    //                 Some(Either::Right(proxy_data)) => {
    //                     let other = (proxy_data.from_proxy)(value, lua)?;
    //                     let other = other.with_reflect(&world, |r, _, _| r.clone_value())?;
    //                     // now we can set it
    //                     self.0.with_reflect_mut(&world, |r| {
    //                         if let Some(set) = proxy_data.opt_set {
    //                             set(r, other)
    //                         } else {
    //                             r.try_apply(other.as_partial_reflect())
    //                                 .map_err(ScriptError::new_reflection_error)?;
    //                             Ok(())
    //                         }
    //                     })?
    //                 }
    //                 None => {
    //                     Err(ScriptError::new_runtime_error(format!(
    //                             "Invalid assignment `{}` = `{value:?}`. The left hand side does not support conversion from lua.",
    //                             ReflectReferencePrinter::new(self.0).pretty_print(&world),
    //                         )))
    //                 }
    //             }
    //         },
    //     )??;
    //     Ok(())
    // }

    /// Adjusts all the numeric accesses in the path from 1-indexed to 0-indexed
    pub fn to_host_index(path: &mut ParsedPath) {
        path.0.iter_mut().for_each(|a| match a.access {
            bevy::reflect::Access::FieldIndex(ref mut i) => *i -= 1,
            bevy::reflect::Access::TupleIndex(ref mut i) => *i -= 1,
            bevy::reflect::Access::ListIndex(ref mut i) => *i -= 1,
            _ => {}
        });
    }

    /// Adjusts all the numeric accesses in the path from 0-indexed to 1-indexed
    pub fn from_host_index(path: &mut ParsedPath) {
        path.0.iter_mut().for_each(|a| match a.access {
            bevy::reflect::Access::FieldIndex(ref mut i) => *i += 1,
            bevy::reflect::Access::TupleIndex(ref mut i) => *i += 1,
            bevy::reflect::Access::ListIndex(ref mut i) => *i += 1,
            _ => {}
        });
    }

    pub fn parse_value_index(value: Value) -> Result<ParsedPath, mlua::Error> {
        if let Some(num) = value.as_usize() {
            Ok(vec![OffsetAccess {
                access: bevy::reflect::Access::ListIndex(num),
                offset: Some(1),
            }]
            .into())
        } else if let Some(key) = value.as_str() {
            if let Some(tuple_struct_index) = key.strip_prefix("_") {
                if let Ok(index) = tuple_struct_index.parse::<usize>() {
                    return Ok(vec![OffsetAccess {
                        access: bevy::reflect::Access::TupleIndex(index),
                        offset: Some(1),
                    }]
                    .into());
                }
            }

            ParsedPath::parse(key).map_err(|e| mlua::Error::external(e.to_string()))
        } else {
            Err(mlua::Error::external("Invalid index"))
        }
    }
}

// impl<'lua> IntoLua<'lua> for LuaReflectReference {
//     fn into_lua(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
//         // if we are a primitive type, we can convert to lua directly
//         let world = lua.get_world();

//         let value = self.0.with_reflect(&world, |r| {
//             let tail_type_id = r
//                 .get_represented_type_info()
//                 .map(|i| i.type_id())
//                 .type_id_or_fake_id();

//             if let Ok(v) = r.as_option() {
//                 if let Some(v) = v {
//                     return Ok(v);
//                 }
//             }

//             into_lua_cases!(
//                 lua,
//                 world,
//                 self,
//                 r,
//                 [
//                     // TODO: after update to mlua 10, we can use the new `IntoLua` impls for these types
//                     &'static str,
//                     &'static CStr,
//                     // &'static OsStr,
//                     // &'static Path,
//                     Cow<'static, str>,
//                     Cow<'static, CStr>,
//                     bool,
//                     f32,
//                     f64,
//                     i8,
//                     i16,
//                     i32,
//                     i64,
//                     i128,
//                     isize,
//                     u8,
//                     u16,
//                     u32,
//                     u64,
//                     u128,
//                     usize,
//                     Box<str>,
//                     CString,
//                     String
//                     // OsString,
//                     // PathBuf
//                 ]
//             );
//         });
//     }
// }

// impl_userdata_from_lua!(LuaReflectReference);

// impl LuaProxied for ReflectReference {
//     type Proxy = LuaReflectReference;
// }

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
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(m: &mut T) {
        m.add_meta_function(
            MetaMethod::Index,
            |lua, (self_, key): (LuaReflectReference, LuaScriptValue)| {
                let world = lua.get_world();
                let mut self_: ReflectReference = self_.into();
                let type_id = self_.tail_type_id(world.clone())?.type_id_or_fake_id();

                let key: ScriptValue = key.into();

                if let ScriptValue::String(ref key) = key {
                    let function_registry =
                        world.with_resource(|registry: &AppFunctionRegistry| registry.clone());
                    let registry = function_registry.read();

                    if let Some(func) =
                        registry.get_namespaced_function(key.clone(), Namespace::OnType(type_id))
                    {
                        let func = func.clone();
                        let func =
                            lua.create_function(move |_, args: Variadic<LuaScriptValue>| {
                                let out = func.dynamic_call(
                                    args.into_iter().map(Into::into),
                                    world.clone(),
                                )?;

                                Ok(LuaScriptValue::from(out))
                            })?;

                        return func.into_lua(lua);
                    }
                };

                let mut reflect_path: ReflectionPathElem = key.try_into()?;
                reflect_path.convert_to_0_indexed();
                self_.index_path(reflect_path);
                let script_value = self_.with_reflect(world.clone(), |r| {
                    <&dyn PartialReflect>::into_script_value(r, world.clone())
                })??;
                LuaScriptValue::from(script_value).into_lua(lua)
            },
        );

        m.add_meta_function(
            MetaMethod::NewIndex,
            |lua, (self_, key, value): (LuaReflectReference, LuaScriptValue, LuaScriptValue)| {
                let mut self_: ReflectReference = self_.into();
                let key: ScriptValue = key.into();
                let value: ScriptValue = value.into();
                let mut reflect_path: ReflectionPathElem = key.try_into()?;
                reflect_path.convert_to_0_indexed();

                let world = lua.get_world();

                self_.index_path(reflect_path);
                self_.with_reflect_mut(world.clone(), |r| {
                    let target_type_id = r
                        .get_represented_type_info()
                        .map(|i| i.type_id())
                        .type_id_or_fake_id();
                    let other = <dyn PartialReflect>::from_script_value(
                        value,
                        world.clone(),
                        target_type_id,
                    )
                    .transpose()?
                    .ok_or_else(|| ValueConversionError::TypeMismatch {
                        expected_type: target_type_id.display_with_world(world.clone()).into(),
                        actual_type: Some(
                            r.get_represented_type_info()
                                .map(|i| i.type_id())
                                .type_id_or_fake_id()
                                .display_with_world(world.clone())
                                .into(),
                        ),
                    })?;

                    r.try_apply(other.as_partial_reflect())?;
                    Ok::<_, ScriptError>(())
                })??;

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
