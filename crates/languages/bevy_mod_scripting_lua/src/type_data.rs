use std::{
    any::{Any, TypeId},
    clone,
    collections::{HashMap, HashSet},
    sync::Arc,
};

use bevy::{
    app::App,
    log::debug,
    prelude::{AppTypeRegistry, Mut},
    reflect::{
        impl_reflect, DynamicEnum, DynamicList, DynamicTuple, DynamicVariant, FromType,
        GetTypeRegistration, List, ParsedPath, PartialReflect, Reflect, ReflectFromReflect,
        ReflectMut, ReflectPathError, TypeInfo, TypePath, TypeRegistration,
    },
};
use bevy_mod_scripting_core::{
    bindings::{DeferredReflection, ReflectAllocator, ReflectReference, ReflectionPathElem},
    error::ScriptError,
    reflection_extensions::PartialReflectExt,
};
use tealr::mlu::mlua::{FromLua, IntoLua, Lua, Table, Value};

use crate::bindings::{proxy::LuaProxied, reference::LuaReflectReference, world::GetWorld};

/// Stores the procedure used to convert a lua value to a reflect value and vice versa, Used for types which are represented in lua via proxies which store
/// a reference to the actual value.
/// This is used for types which are represented in lua with pass by reference semantics
#[derive(Clone)]
pub struct ReflectLuaProxied {
    // TODO: should these be script errors?
    pub into_proxy: Arc<
        dyn for<'l> Fn(ReflectReference, &'l Lua) -> Result<Value<'l>, tealr::mlu::mlua::Error>
            + Send
            + Sync
            + 'static,
    >,
    pub from_proxy: Arc<
        dyn for<'l> Fn(Value<'l>, &'l Lua) -> Result<ReflectReference, tealr::mlu::mlua::Error>
            + Send
            + Sync
            + 'static,
    >,
    /// Optional override for setting behavior, should be respected by all handling types for unproxying to work recurively.
    /// Normally used when [`PartialReflect::apply`] does not do the right thing.
    pub opt_set: Option<
        Arc<
            dyn for<'l> Fn(
                    &mut dyn PartialReflect,
                    Box<dyn PartialReflect>,
                ) -> Result<(), ScriptError>
                + Send
                + Sync
                + 'static,
        >,
    >,
}

impl ReflectLuaProxied {
    /// Generates a type data which can be used on [`Option<T>`] types which are represented in lua with pass by reference semantics
    fn new_for_option(inner_lua_proxied_data: &ReflectLuaProxied, container_type_info: &'static TypeInfo) -> Self {
        let ref_into_option = |mut r: ReflectReference| {
            r.index_path(ParsedPath::parse_static("0").expect("Invalid reflection path"));
            r
        };
        let into_proxy_clone = inner_lua_proxied_data.into_proxy.clone();
        let from_proxy_clone = inner_lua_proxied_data.from_proxy.clone();
        // let from_proxy_clone = inner_lua_proxied_data.from_proxy.clone();
        Self {
            into_proxy: Arc::new(move |reflect_ref, l| {
                // read the value and check if it is None, if so return nil
                // otherwise use the inner type's into_proxy
                let world = l.get_world();
                let is_some = reflect_ref
                    .with_reflect(&world, |s, _, _| s.as_option().map(|r| r.is_some()))??;

                if is_some {
                    (into_proxy_clone)(ref_into_option(reflect_ref), l)
                } else {
                    Ok(Value::Nil)
                }
            }),
            from_proxy: Arc::new(move |v, l| {
                if v.is_nil() {
                    bevy::log::debug!("Option from proxy: Nil");
                    // we need to allocate a new reflect reference since we don't have one existing
                    let mut dynamic_value = DynamicEnum::new("None", DynamicVariant::Unit);
                    dynamic_value.set_represented_type(Some(container_type_info));
                    let world = l.get_world();
                    let reflect_ref =
                        world.with_resource(|_, mut allocator: Mut<ReflectAllocator>| {
                            Ok::<_, tealr::mlu::mlua::Error>(ReflectReference::new_allocated(
                                dynamic_value,
                                &mut allocator,
                            ))
                        })?;

                    Ok(reflect_ref)
                } else {
                    bevy::log::debug!("Option from proxy: {:?}", v);
                    let mut inner_ref = (from_proxy_clone)(v, l)?;
                    inner_ref.reflect_path.pop();
                    Ok(inner_ref)
                }
            }),
            opt_set: None,
        }
    }

    fn dynamic_list_from_value<'lua>(
        v: Table<'lua>,
        lua: &'lua Lua,
        from_proxy: &Arc<
            dyn for<'l> Fn(
                    Value<'l>,
                    &'l Lua,
                )
                    -> Result<Box<dyn PartialReflect>, tealr::mlu::mlua::Error>
                + Send
                + Sync
                + 'static,
        >,
        container_type_info: &'static TypeInfo
    ) -> Result<DynamicList, tealr::mlu::mlua::Error> {
        let lua_values = v.sequence_values().collect::<Result<Vec<Value>, _>>()?;

        let converted_values = lua_values
            .into_iter()
            .map(|v| (from_proxy)(v, lua))
            .collect::<Result<Vec<_>, _>>()?;

        let mut dynamic_type = DynamicList::from_iter(converted_values);
        dynamic_type.set_represented_type(Some(container_type_info));
        Ok(dynamic_type)
    }

    fn dynamic_list_from_proxy<'lua>(
        v: Table<'lua>,
        lua: &'lua Lua,
        from_proxy: &Arc<
            dyn for<'l> Fn(Value<'l>, &'l Lua) -> Result<ReflectReference, tealr::mlu::mlua::Error>
                + Send
                + Sync
                + 'static,
        >,
        container_type_info: &'static TypeInfo,
    ) -> Result<DynamicList, tealr::mlu::mlua::Error> {
        let lua_values = v.sequence_values().collect::<Result<Vec<Value>, _>>()?;

        // TODO: less allocations plz, i can't be bothered to do this right now
        let converted_values = lua_values
            .into_iter()
            .map(|v| (from_proxy)(v, lua))
            .collect::<Result<Vec<_>, _>>()?;
        let world = lua.get_world();

        let boxed_values = converted_values
            .into_iter()
            .map(|v| v.with_reflect(&world, |r, _, _| r.clone_value()))
            .collect::<Result<Vec<_>, _>>()?;

        let mut dynamic_type = DynamicList::from_iter(boxed_values);
        // TODO: what to do with this memory, I think it's fine to leave hanging till exit
        dynamic_type.set_represented_type(Some(container_type_info));
        Ok(dynamic_type)
    }

    fn new_for_listlike_value(
        inner_lua_value_data: &ReflectLuaValue,
        container_type_info: &'static TypeInfo,
    ) -> Self {
        let from_value_clone = inner_lua_value_data.from_value.clone();
        Self {
            into_proxy: Arc::new(|r, l| LuaReflectReference(r).into_lua(l)),
            from_proxy: Arc::new(move |v, l| {
                if let Value::Table(t) = v {
                    let dynamic_table = Self::dynamic_list_from_value(t, l, &from_value_clone, container_type_info)?;
                    let world = l.get_world();
                    let allocated =
                        world.with_resource(|_, mut allocator: Mut<ReflectAllocator>| {
                            ReflectReference::new_allocated(dynamic_table, &mut allocator)
                        });
                    Ok(allocated)
                } else {
                    LuaReflectReference::from_lua(v, l).map(|v| v.0)
                }
            }),
            opt_set: Some(Arc::new(|r, other| {
                r.set_as_list(other, |a, b| {
                    Ok(a.try_apply(b)?)
                })
            })),
        }
    }

    fn new_for_listlike_proxy(inner_lua_proxied_data: &ReflectLuaProxied) -> Self {
        // let from_proxy_clone = inner_lua_proxied_data.from_proxy.clone();
        Self {
            into_proxy: Arc::new(|r, l| LuaReflectReference(r).into_lua(l)),
            // from_proxy: Arc::new(|v, l| {
            //     // if it's a table, set existing elements using underlying from proxy
            //     // if it's a ref, return the ref

            //     if let Value::Table(t) = v {
            //         let ts = t.sequence_values::<Value>();
            //         for e in ts {
            //             let v = e?;
            //             let elem_ref = (from_proxy_clone)(v, l)?;
            //         }
            //     } else {
            //         LuaReflectReference::from_lua(v, l)
            //     }
            // }),
            from_proxy: Arc::new(|v, l| {
                // can either be a table or a direct ref
                // construct dynamic vec either way
                // if let Value::Table(t) = v {
                //     for v in t.sequence_values::<Value>() {
                //         let lua_elem = v?;
                //     }
                // } else {
                //     todo!()
                // }
                todo!()
            }),
            opt_set: todo!(), //     set_from_proxy: todo!(), // set_from_proxy: Arc::new(|r, v, l| {
                              //                              //     let table = if let Value::Table(t) = v {
                              //                              //         t
                              //                              //     } else {
                              //                              //         return Err(tealr::mlu::mlua::Error::external(
                              //                              //             ScriptError::new_runtime_error(format!(
                              //                              //                 "Cannot set value of type `{}` via type: `{}`. Expected table",
                              //                              //                 v.type_name(),
                              //                              //                 "List",
                              //                              //             )),
                              //                              //         ));
                              //                              //     };

                              //                              //     let lua_values = table.sequence_values().collect::<Result<Vec<Value>, _>>()?;

                              //                              //     let converted_values = lua_values
                              //                              //         .into_iter()
                              //                              //         .map(|v| (from_proxy)(v, lua))
                              //                              //         .collect::<Result<Vec<_>, _>>()?;

                              //                              //     let target_list = r.as_list().map_err(tealr::mlu::mlua::Error::external)?;
                              //                              // }),
        }
    }
}

impl<T: LuaProxied + Reflect> FromType<T> for ReflectLuaProxied
where
    T::Proxy: for<'l> IntoLua<'l> + for<'l> FromLua<'l>,
    T::Proxy: From<ReflectReference> + AsRef<ReflectReference>,
{
    fn from_type() -> Self {
        Self {
            into_proxy: Arc::new(|p, l| T::Proxy::from(p).into_lua(l)),
            from_proxy: Arc::new(|v, l| T::Proxy::from_lua(v, l).map(|p| p.as_ref().clone())),
            opt_set: None,
            // set_from_proxy: Arc::new(|r, v, l| {
            //     let proxy = T::Proxy::from_lua(v, l).map(|p| p.as_ref().clone())?;
            //     let world = l.get_world();
            //     proxy.with_reflect(&world, |other, _, _| {
            //         r.apply(other);
            //         Ok::<_, tealr::mlu::mlua::Error>(())
            //     })?;
            //     Ok(())
            // }),
        }
    }
}

/// Stores the procedure used to convert a lua value to a reflect value and vice versa, Used for types which are represented directly in lua with
/// pass by value semantics, These need to implement [`Clone`]
#[derive(Clone)]
pub struct ReflectLuaValue {
    pub into_value: Arc<
        dyn for<'l> Fn(&dyn PartialReflect, &'l Lua) -> Result<Value<'l>, tealr::mlu::mlua::Error>
            + Send
            + Sync
            + 'static,
    >,
    // pub set_value: Arc<
    //     dyn for<'l> Fn(
    //             &mut dyn PartialReflect,
    //             Value<'l>,
    //             &'l Lua,
    //         ) -> Result<(), tealr::mlu::mlua::Error>
    //         + Send
    //         + Sync
    //         + 'static,
    // >,
    pub from_value: Arc<
        dyn for<'l> Fn(
                Value<'l>,
                &'l Lua,
            ) -> Result<Box<dyn PartialReflect>, tealr::mlu::mlua::Error>
            + Send
            + Sync
            + 'static,
    >,
}

impl ReflectLuaValue {
    fn dynamic_option_from_value<'lua>(
        v: Value<'lua>,
        lua: &'lua Lua,
        from_value: &Arc<
            dyn for<'l> Fn(
                    Value<'l>,
                    &'l Lua,
                )
                    -> Result<Box<dyn PartialReflect>, tealr::mlu::mlua::Error>
                + Send
                + Sync
                + 'static,
        >,
        container_type_info: &'static TypeInfo,
    ) -> Result<DynamicEnum, tealr::mlu::mlua::Error> {
        let mut dynamic_enum = if v.is_nil() {
            DynamicEnum::new("None", DynamicVariant::Unit)
        } else {
            let inner = (from_value)(v, lua)?;
            DynamicEnum::new("Some", DynamicVariant::Tuple([inner].into_iter().collect()))
        };

        dynamic_enum.set_represented_type(Some(container_type_info));

        Ok(dynamic_enum)
    }

    /// generates implementation for an inner type wrapped by an option
    ///
    /// the option should check if the value is None if so return nil,
    /// if the value is some use ReflectLuaValue implementation of the inner type.
    ///
    /// If there is a type mismatch at any point will return an error
    pub fn new_for_option(inner_reflect_lua_value: &ReflectLuaValue, container_type_info: &'static TypeInfo) -> Self {
        let into_value_clone = inner_reflect_lua_value.into_value.clone();
        // we have to do this so the closures can be moved into the arc
        let from_value_clone2 = inner_reflect_lua_value.from_value.clone();
        Self {
            into_value: Arc::new(move |r, lua| {
                r.as_option()
                    .map_err(tealr::mlu::mlua::Error::external)?
                    .map(|inner| (into_value_clone)(inner, lua))
                    .unwrap_or_else(|| Ok(Value::Nil))
            }),
            from_value: Arc::new(move |v, l| {
                let dynamic_option = Self::dynamic_option_from_value(v, l, &from_value_clone2, container_type_info)?;

                Ok(Box::new(dynamic_option))
            }),
            // set_value: Arc::new(move |r, v, l| {
            //     let dynamic = Self::dynamic_option_from_value(v, l, &from_value_clone)?;
            //     r.apply(&dynamic);
            //     Ok(())
            // }),
        }
    }

    // fn dynamic_list_from_value<'lua>(
    //     v: Value<'lua>,
    //     lua: &'lua Lua,
    //     from_value: &Arc<
    //         dyn for<'l> Fn(
    //                 Value<'l>,
    //                 &'l Lua,
    //             )
    //                 -> Result<Box<dyn PartialReflect>, tealr::mlu::mlua::Error>
    //             + Send
    //             + Sync
    //             + 'static,
    //     >,
    // ) -> Result<DynamicList, tealr::mlu::mlua::Error> {
    //     let table = if let Value::Table(t) = v {
    //         t
    //     } else {
    //         return Err(tealr::mlu::mlua::Error::external(
    //             ScriptError::new_runtime_error(format!(
    //                 "Cannot set value of type `{}` via type: `{}`. Expected table",
    //                 v.type_name(),
    //                 "List",
    //             )),
    //         ));
    //     };

    //     let lua_values = table.sequence_values().collect::<Result<Vec<Value>, _>>()?;

    //     let converted_values = lua_values
    //         .into_iter()
    //         .map(|v| (from_value)(v, lua))
    //         .collect::<Result<Vec<_>, _>>()?;
    //     let dynamic_type = DynamicList::from_iter(converted_values);
    //     // TODO: set the represented type, need to pass type info
    //     // dynamic_type.set_represented_type(represented_type);
    //     Ok(dynamic_type)
    // }

    // pub fn new_for_list(inner_reflect_lua_value: &ReflectLuaValue) -> Self {
    //     let into_value_clone = inner_reflect_lua_value.into_value.clone();
    //     let from_value_clone = inner_reflect_lua_value.from_value.clone();
    //     let from_value_clone2 = inner_reflect_lua_value.from_value.clone();
    //     Self {
    //         into_value: Arc::new(move |r, l| {
    //             let inner = r.as_list().map_err(tealr::mlu::mlua::Error::external)?;
    //             let inner = inner
    //                 .map(|i| (into_value_clone)(i, l))
    //                 .collect::<Result<Vec<_>, _>>()?;

    //             let t = l.create_table_from(
    //                 inner
    //                     .into_iter()
    //                     .enumerate()
    //                     .map(|(i, v)| (Value::Integer(i as i64 + 1), v)),
    //             )?;

    //             Ok(Value::Table(t))
    //         }),
    //         set_value: Arc::new(move |r, v, l| {
    //             let dynamic = Self::dynamic_list_from_value(v, l, &from_value_clone)?;

    //             // apply will not remove excess elements
    //             if let ReflectMut::List(list) = r.reflect_mut() {
    //                 if dynamic.len() < list.len() {
    //                     (dynamic.len()..list.len()).rev().for_each(|i| {
    //                         list.remove(i);
    //                     });
    //                 }
    //             }
    //             println!("reflect: {:?}", r);
    //             println!("dynamic: {:?}", dynamic);

    //             r.apply(&dynamic);
    //             Ok(())
    //         }),
    //         from_value: Arc::new(move |v, l| {
    //             let dynamic = Self::dynamic_list_from_value(v, l, &from_value_clone2)?;
    //             Ok(Box::new(dynamic))
    //         }),
    //     }
    // }
}

impl<T: Reflect + Clone + for<'l> IntoLua<'l> + for<'l> FromLua<'l>> FromType<T>
    for ReflectLuaValue
{
    fn from_type() -> Self {
        Self {
            into_value: Arc::new(|v, l| {
                bevy::log::debug!("Converting lua value to lua: {:?}", v);
                v.try_downcast_ref::<T>()
                    .unwrap_or_else(|| {
                        panic!(
                            "Expected type: {}, got: {}",
                            std::any::type_name::<T>(),
                            v.reflect_type_path()
                        )
                    })
                    .clone()
                    .into_lua(l)
            }),
            // set_value: Arc::new(|t, v, l| {
            //     bevy::log::debug!("Setting lua value: {:?}, with {:?}", t, v);
            //     let t = t.try_downcast_mut::<T>().unwrap();
            //     *t = T::from_lua(v, l)?;
            //     Ok(())
            // }),
            from_value: Arc::new(|v, l| {
                bevy::log::debug!("Building concrete type from lua value: {:?}", v);
                T::from_lua(v, l).map(|v| Box::new(v) as Box<dyn PartialReflect>)
            }),
        }
    }
}

/// Registers a lua proxy object via the reflection system
pub trait RegisterLua {
    fn register_lua_proxy<T: LuaProxied + Reflect + TypePath + GetTypeRegistration>(
        &mut self,
    ) -> &mut Self
    where
        T::Proxy: for<'l> IntoLua<'l> + for<'l> FromLua<'l>,
        T::Proxy: From<ReflectReference> + AsRef<ReflectReference>;

    fn register_lua_value<T>(&mut self) -> &mut Self
    where
        T: for<'l> IntoLua<'l> + for<'l> FromLua<'l>,
        T: Reflect + Clone + TypePath + GetTypeRegistration;
}

impl RegisterLua for App {
    fn register_lua_proxy<T: LuaProxied + Reflect + TypePath + GetTypeRegistration>(
        &mut self,
    ) -> &mut Self
    where
        T::Proxy: for<'l> IntoLua<'l> + for<'l> FromLua<'l>,
        T::Proxy: From<ReflectReference> + AsRef<ReflectReference>,
    {
        self.register_type::<T>();
        self.register_type_data::<T, ReflectLuaProxied>()
    }

    fn register_lua_value<T>(&mut self) -> &mut Self
    where
        T: for<'l> IntoLua<'l> + for<'l> FromLua<'l>,
        T: Reflect + Clone + TypePath + GetTypeRegistration,
    {
        self.register_type::<T>();
        self.register_type_data::<T, ReflectLuaValue>()
    }
}

/// Checks if the type registration is for a type which matches the pattern `core::option::Option<T>`, and extracts `T`'s typeId as well as the Option's typeId
fn destructure_option_type(reg: &TypeRegistration) -> Option<(TypeId, TypeId)> {
    let type_path_table = reg.type_info().type_path_table();
    let is_core = type_path_table.crate_name().is_some_and(|s| s == "core");
    let is_option = type_path_table.ident().is_some_and(|s| s == "Option");

    if is_core && is_option {
        reg.type_info()
            .generics()
            .get_named("T")
            .map(|t| (reg.type_id(), t.type_id()))
    } else {
        None
    }
}

fn destructure_list_type(reg: &TypeRegistration) -> Option<(TypeId, TypeId)> {
    let type_path_table = reg.type_info().as_list().ok()?;

    let inner_type_id = type_path_table.item_ty().id();
    Some((reg.type_id(), inner_type_id))
}

/// iterates over type data for all types which have registered [`ReflectLuaProxied`] and [`ReflectLuaValue`] implementations,
/// and registers corresponding [`Option<T>`] [`Result<T, E>`] etc type data equivalents
pub fn pre_register_common_containers(type_registry: &mut AppTypeRegistry) {
    let mut type_registry = type_registry.write();

    let mut lua_value_insertions: HashMap<TypeId, ReflectLuaValue> = Default::default();
    let mut lua_proxied_insertions: HashMap<TypeId, ReflectLuaProxied> = Default::default();
    for (option_type_id, inner_type_id) in type_registry.iter().filter_map(destructure_option_type)
    {
        let container_type_info = type_registry.get(option_type_id).expect("invariant").type_info();
        // TODO: reuse the leaked box in both branches when either is true
        if let Some(inner_lua_value_data) =
            type_registry.get_type_data::<ReflectLuaValue>(inner_type_id)
        {
            let option_lua_proxied_data = ReflectLuaValue::new_for_option(inner_lua_value_data, Box::leak(Box::new(container_type_info.clone())));
            lua_value_insertions.insert(option_type_id, option_lua_proxied_data);
        }

        if let Some(inner_lua_proxied_data) =
            type_registry.get_type_data::<ReflectLuaProxied>(inner_type_id)
        {
            let option_lua_proxied_data = ReflectLuaProxied::new_for_option(inner_lua_proxied_data, Box::leak(Box::new(container_type_info.clone())));

            lua_proxied_insertions.insert(option_type_id, option_lua_proxied_data);
        }
    }

    for (vec_type_id, inner_type_id) in type_registry.iter().filter_map(destructure_list_type) {
        let container_type_info = type_registry.get(vec_type_id).expect("invariant").type_info();
        if let Some(inner_lua_value_data) =
            type_registry.get_type_data::<ReflectLuaValue>(inner_type_id)
        {
            let vec_lua_proxied_data =
                ReflectLuaProxied::new_for_listlike_value(inner_lua_value_data, Box::leak(Box::new(container_type_info.clone())));
            lua_proxied_insertions.insert(vec_type_id, vec_lua_proxied_data);
        }
    }

    for (type_id, lua_value_data) in lua_value_insertions {
        type_registry
            .get_mut(type_id)
            .expect("We just found the type id from the type registry")
            .insert(lua_value_data);
    }

    for (type_id, lua_proxied_data) in lua_proxied_insertions {
        type_registry
            .get_mut(type_id)
            .expect("We just found the type id from the type registry")
            .insert(lua_proxied_data);
    }
}

pub fn register_lua_values(app: &mut bevy::prelude::App) {
    app.register_lua_value::<usize>();
    app.register_lua_value::<isize>();
    app.register_lua_value::<f32>();
    app.register_lua_value::<f64>();
    app.register_lua_value::<u128>();
    app.register_lua_value::<u64>();
    app.register_lua_value::<u32>();
    app.register_lua_value::<u16>();
    app.register_lua_value::<u8>();
    app.register_lua_value::<i128>();
    app.register_lua_value::<i64>();
    app.register_lua_value::<i32>();
    app.register_lua_value::<i16>();
    app.register_lua_value::<i8>();
    app.register_lua_value::<String>();
    app.register_lua_value::<bool>();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bindings::{reference::LuaReflectReference, world::LuaWorld};
    use bevy::{
        prelude::World,
        reflect::{TypeRegistry, TypeRegistryArc},
    };
    use bevy_mod_scripting_core::bindings::WorldCallbackAccess;
    use std::sync::Arc;

    #[derive(Reflect, Debug, PartialEq)]
    struct Proxied(usize);

    impl LuaProxied for Proxied {
        type Proxy = LuaReflectReference;
    }

    fn setup_type_registry<T1, T2>() -> AppTypeRegistry
    where
        T1: GetTypeRegistration,
        T2: GetTypeRegistration,
    {
        let mut type_registry = TypeRegistry::default();
        type_registry.register::<T1>();
        type_registry.register::<T2>();

        let type_registry_arc = TypeRegistryArc {
            internal: Arc::new(type_registry.into()),
        };
        AppTypeRegistry(type_registry_arc)
    }

    macro_rules! prepare_type_registry_with_common_containers {
        ($target_type:ty, $container_type:ty, $type_data:ty) => {{
            {
                let mut type_registry = setup_type_registry::<$target_type, $container_type>();
                type_registry
                    .write()
                    .register_type_data::<$target_type, $type_data>();

                pre_register_common_containers(&mut type_registry);
                type_registry
            }
        }};
    }

    macro_rules! assert_transitively_registers {
        ($target_type:ty = $inner_type_data:ty, $container_type:ty = $type_data:ty) => {{
            let type_registry = prepare_type_registry_with_common_containers!(
                $target_type,
                $container_type,
                $inner_type_data
            );

            let type_registry = type_registry.read();
            let container_type_id = std::any::TypeId::of::<$container_type>();
            let container_type_registration = type_registry.get(container_type_id).unwrap();

            let type_data = container_type_registration.contains::<$type_data>();
            assert!(
                type_data,
                "{:?} should have type data {:?}",
                std::any::type_name::<$container_type>(),
                std::any::type_name::<$type_data>()
            );
        }};
    }

    macro_rules! assert_lua_value_into_equals {
        ($target_type:ty, $container_type:ty, $type_data:ty, $val:expr => $exp:expr) => {{
            let type_registry = prepare_type_registry_with_common_containers!(
                $target_type,
                $container_type,
                $type_data
            );

            let type_registry = type_registry.read();
            let container_type_id = std::any::TypeId::of::<$container_type>();
            let container_type_data = type_registry
                .get_type_data::<$type_data>(container_type_id)
                .unwrap();
            let l = Lua::new();
            let out = (container_type_data.into_value)(&$val, &l).unwrap();
            assert_eq!(out, $exp);
        }};
    }

    macro_rules! assert_lua_container_value_from_equals {
        ($target_type:ty, $container_type:ty, $type_data:ty, $val:expr => $exp:expr) => {{
            let type_registry = prepare_type_registry_with_common_containers!(
                $target_type,
                $container_type,
                $type_data
            );

            let type_registry = type_registry.read();
            let container_type_id = std::any::TypeId::of::<$container_type>();
            let container_type_data = type_registry
                .get_type_data::<$type_data>(container_type_id)
                .unwrap();
            let l = Lua::new();
            let out = (container_type_data.from_value)($val, &l).unwrap();
            assert!(out.reflect_partial_eq(&$exp).unwrap());
        }};
    }

    // macro_rules! assert_lua_container_value_after_set_equals {
    //     ($target_type:ty, $container_type:ty, $type_data:ty, $val:expr => $set:expr => $exp:expr) => {{
    //         let type_registry = prepare_type_registry_with_common_containers!(
    //             $target_type,
    //             $container_type,
    //             $type_data
    //         );

    //         let type_registry = type_registry.read();
    //         let container_type_id = std::any::TypeId::of::<$container_type>();
    //         let container_type_data = type_registry
    //             .get_type_data::<$type_data>(container_type_id)
    //             .unwrap();
    //         let l = Lua::new();
    //         let mut target = $val;
    //         (container_type_data.set_value)(&mut target, $set, &l).unwrap();
    //         assert!(
    //             target.reflect_partial_eq(&$exp).unwrap(),
    //             "Expected {:?} got: {:?}",
    //             $exp,
    //             target
    //         );
    //     }};
    // }

    macro_rules! assert_lua_container_proxy_into_proxy {
        (;WITH; $target_type:ty = $inner_type_data:ty, $container_type:ty = $type_data:ty ;FROM; $val:expr ;INTO; $exp:expr) => {{
            let app_type_registry = prepare_type_registry_with_common_containers!(
                $target_type,
                $container_type,
                $inner_type_data
            );
            let mut world = World::new();

            let type_registry = app_type_registry.read();
            let container_type_id = std::any::TypeId::of::<$container_type>();
            let container_type_data = type_registry
                .get_type_data::<$type_data>(container_type_id)
                .unwrap()
                .clone();

            let mut allocator = ReflectAllocator::default();
            let allocated_value = ReflectReference::new_allocated($val, &mut allocator);
            world.insert_resource(allocator);
            drop(type_registry);
            world.insert_resource(app_type_registry);
            let lua = Lua::new();
            WorldCallbackAccess::with_callback_access(&mut world, |world| {
                lua.globals().set("world", LuaWorld(world.clone())).unwrap();

                let gotten_into_value =
                    (container_type_data.into_proxy)(allocated_value, &lua).unwrap();
                let converted_ref = LuaReflectReference::from_lua(gotten_into_value.clone(), &lua)
                    .expect(&format!(
                        "Could not convert to lua reflect reference got: {:?}",
                        gotten_into_value
                    ))
                    .0;
                let world = world.read().unwrap();
                converted_ref.with_reflect(&world, |r, _, _| {
                    assert!(
                        r.reflect_partial_eq(&$exp).unwrap(),
                        "Expected {:?} got {:?}",
                        $exp,
                        r
                    );
                });
            });
        }};
    }

    macro_rules! assert_lua_container_proxy_opt_set {
        ($allocator:ident, $lua:ident ;WITH; $target_type:ty = $inner_type_data:ty, $container_type:ty = $type_data:ty ;SET; $val:expr ;TO; $set:expr ;EXPECT; $exp:expr) => {{
            let app_type_registry = prepare_type_registry_with_common_containers!(
                $target_type,
                $container_type,
                $inner_type_data
            );
            let mut world = World::new();

            let type_registry = app_type_registry.read();
            let container_type_id = std::any::TypeId::of::<$container_type>();
            let container_type_data = type_registry
                .get_type_data::<$type_data>(container_type_id)
                .unwrap()
                .clone();

            let mut $allocator = ReflectAllocator::default();
            drop(type_registry);
            world.insert_resource(app_type_registry);
            let $lua = Lua::new();
            let set_expr = Box::new($set);
            world.insert_resource($allocator);
            WorldCallbackAccess::with_callback_access(&mut world, |world| {
                $lua.globals()
                    .set("world", LuaWorld(world.clone()))
                    .unwrap();
                let mut target = $val;
                (container_type_data.opt_set.unwrap())(&mut target, set_expr).unwrap();
                assert!(target.reflect_partial_eq(&$exp).unwrap());
            });
        }};
    }

    macro_rules! assert_lua_container_proxy_into_proxy_lua_only {
        (;WITH; $target_type:ty = $inner_type_data:ty, $container_type:ty = $type_data:ty ;FROM; $val:expr ;INTO; $exp:expr) => {{
            let app_type_registry = prepare_type_registry_with_common_containers!(
                $target_type,
                $container_type,
                $inner_type_data
            );
            let mut world = World::new();

            let type_registry = app_type_registry.read();
            let container_type_id = std::any::TypeId::of::<$container_type>();
            let container_type_data = type_registry
                .get_type_data::<$type_data>(container_type_id)
                .unwrap()
                .clone();

            let mut allocator = ReflectAllocator::default();
            let allocated_value = ReflectReference::new_allocated($val, &mut allocator);
            world.insert_resource(allocator);
            drop(type_registry);
            world.insert_resource(app_type_registry);
            let lua = Lua::new();
            WorldCallbackAccess::with_callback_access(&mut world, |world| {
                lua.globals().set("world", LuaWorld(world.clone())).unwrap();

                let gotten_into_value =
                    (container_type_data.into_proxy)(allocated_value, &lua).unwrap();
                assert_eq!(gotten_into_value, $exp);
            });
        }};
    }

    #[test]
    fn test_pre_register_common_containers_lua_value() {
        assert_transitively_registers!(usize = ReflectLuaValue, Option<usize> = ReflectLuaValue);
        assert_transitively_registers!(usize = ReflectLuaValue, Vec<usize> = ReflectLuaProxied);
    }

    #[test]
    fn test_pre_register_common_containers_lua_proxy() {
        assert_transitively_registers!(Proxied = ReflectLuaProxied, Option<Proxied> = ReflectLuaProxied );
    }

    #[test]
    fn test_option_container_impls() {
        // Inner Value
        // into
        assert_lua_value_into_equals!(
            usize,
            Option<usize>,
            ReflectLuaValue,
            Some(2usize) => Value::Integer(2)
        );
        assert_lua_value_into_equals!(
            usize,
            Option<usize>,
            ReflectLuaValue,
            None::<usize> => Value::Nil
        );

        // from
        assert_lua_container_value_from_equals!(
            usize,
            Option<usize>,
            ReflectLuaValue,
            Value::Integer(2) => Some(2usize)
        );
        assert_lua_container_value_from_equals!(
            usize,Option<usize>,
            ReflectLuaValue,
            Value::Nil => None::<usize>
        );

        // set
        // assert_lua_container_value_after_set_equals!(
        //     usize, Option<usize>,
        //     ReflectLuaValue,
        //     Some(2usize) => Value::Nil => None::<usize>
        // );
        // assert_lua_container_value_after_set_equals!(
        //     usize, Option<usize>,
        //     ReflectLuaValue,
        //     None::<usize> => Value::Integer(2) => Some(2usize)
        // );
        // assert_lua_container_value_after_set_equals!(
        //     usize, Option<usize>,
        //     ReflectLuaValue,
        //     None::<usize> => Value::Nil => None::<usize>
        // );
        // assert_lua_container_value_after_set_equals!(
        //     usize, Option<usize>,
        //     ReflectLuaValue,
        //     Some(2usize) => Value::Integer(3) => Some(3usize)
        // );

        // Inner Proxy
        // into
        assert_lua_container_proxy_into_proxy!(
            ;WITH; Proxied = ReflectLuaProxied, Option<Proxied> = ReflectLuaProxied 
            ;FROM; Some(Proxied(2))
            ;INTO; Proxied(2usize));
        assert_lua_container_proxy_into_proxy_lua_only!(
            ;WITH; Proxied = ReflectLuaProxied, Option<Proxied> = ReflectLuaProxied
            ;FROM; None::<Proxied>
            ;INTO; Value::Nil
        );

        // set from
        // assert_lua_container_proxy_opt_set!(alloc, lua, Proxied, Option<Proxied>, ReflectLuaProxied,
        //     Some(Proxied(2))
        //     => None::<Proxied>
        //     => None::<Proxied>);
        // assert_lua_container_proxy_opt_set!(alloc, lua, Proxied, Option<Proxied>, ReflectLuaProxied,
        //     None::<Proxied>
        //     => Proxied(2)// LuaReflectReference(ReflectReference::new_allocated(Proxied(2), &mut alloc))
        //     => Some(Proxied(2)));
        // assert_lua_container_proxy_opt_set!(alloc, lua, Proxied, Option<Proxied>, ReflectLuaProxied,
        //     Some(Proxied(2))
        //     =>  Proxied(3) // LuaReflectReference(ReflectReference::new_allocated(Proxied(3), &mut alloc))
        //     => Some(Proxied(3)));
        // assert_lua_container_proxy_opt_set!(alloc, lua, Proxied, Option<Proxied>, ReflectLuaProxied,
        //     None::<Proxied>
        //     => None::<Proxied>
        //     => None::<Proxied>);
    }

    #[test]
    fn test_listlike_container_impls() {
        // Inner Value
        // into
        assert_lua_container_proxy_into_proxy!(
            ;WITH; usize = ReflectLuaValue, Vec<usize> = ReflectLuaProxied
            ;FROM; vec![Proxied(2), Proxied(3)]
            ;INTO; vec![Proxied(2), Proxied(3)]
        );
        assert_lua_container_proxy_into_proxy!(
            ;WITH; usize = ReflectLuaValue, Vec<usize> = ReflectLuaProxied
            ;FROM; Vec::<Proxied>::default()
            ;INTO; Vec::<Proxied>::default()
        );

        assert_lua_container_proxy_opt_set!(alloc, lua
            ;WITH; usize = ReflectLuaValue, Vec<usize> = ReflectLuaProxied
            ;SET; Vec::<Proxied>::default()
            ;TO; Vec::<Proxied>::default()
            ;EXPECT; Vec::<Proxied>::default()
        );

        assert_lua_container_proxy_opt_set!(alloc, lua
            ;WITH; usize = ReflectLuaValue, Vec<usize> = ReflectLuaProxied
            ;SET; Vec::<Proxied>::default()
            ;TO; vec![Proxied(2)]
            ;EXPECT; vec![Proxied(2)]
        );
    }

    #[test]
    fn test_get_inner_type_id_option() {
        let app_type_registry = setup_type_registry::<usize, Option<usize>>();

        let type_registry = app_type_registry.read();
        let option_type_id = std::any::TypeId::of::<Option<usize>>();
        let inner_type_id = std::any::TypeId::of::<usize>();
        let option_type_registration = type_registry.get(option_type_id).unwrap();

        let (gotten_option_type_id, gotten_inner_type_id) =
            destructure_option_type(option_type_registration).unwrap();
        assert_eq!(
            gotten_inner_type_id, inner_type_id,
            "Option<usize> should have inner type usize"
        );
        assert_eq!(
            gotten_option_type_id, option_type_id,
            "Option<usize> should have type id Option<usize>"
        );
    }
}
