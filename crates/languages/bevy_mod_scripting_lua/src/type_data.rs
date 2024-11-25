use std::{
    any::TypeId,
    collections::{HashMap, HashSet},
    sync::Arc,
};

use bevy::{
    app::App,
    log::debug,
    prelude::{AppTypeRegistry, Mut},
    reflect::{
        impl_reflect, DynamicEnum, DynamicList, DynamicTuple, DynamicVariant, FromType,
        GetTypeRegistration, ParsedPath, PartialReflect, Reflect, ReflectFromReflect,
        ReflectPathError, TypePath, TypeRegistration,
    },
};
use bevy_mod_scripting_core::{
    bindings::{DeferredReflection, ReflectAllocator, ReflectReference, ReflectionPathElem},
    error::ScriptError,
    reflection_extensions::PartialReflectExt,
};
use tealr::mlu::mlua::{FromLua, IntoLua, Lua, Value};

use crate::bindings::{proxy::LuaProxied, world::GetWorld};

/// Stores the procedure used to convert a lua value to a reflect value and vice versa, Used for types which are represented in lua via proxies which store
/// a reference to the actual value.
/// This is used for types which are represented in lua with pass by reference semantics
#[derive(Clone)]
pub struct ReflectLuaProxied {
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
}

impl ReflectLuaProxied {
    /// Generates a type data which can be used on [`Option<T>`] types which are represented in lua with pass by reference semantics
    fn new_for_option(
        inner_lua_proxied_data: &ReflectLuaProxied,
        option_from_reflect: &ReflectFromReflect,
    ) -> Self {
        let ref_into_option = |mut r: ReflectReference| {
            r.index_path(ParsedPath::parse_static("0").expect("Invalid reflection path"));
            r
        };
        let into_proxy_clone = inner_lua_proxied_data.into_proxy.clone();
        let from_proxy_clone = inner_lua_proxied_data.from_proxy.clone();
        let option_from_reflect = option_from_reflect.clone();
        Self {
            into_proxy: Arc::new(move |reflect_ref, l| {
                // read the value and check if it is None, if so return nil
                // otherwise use the inner type's into_proxy
                let world = l.get_world()?;
                let is_some = world
                    .with_allocator_and_type_registry(|_, type_registry, allocator| {
                        let type_registry = type_registry.read();
                        reflect_ref.with_reflect(&world, &type_registry, Some(&allocator), |s| {
                            s.as_option().map(|r| r.is_some())
                        })
                    })
                    .map_err(tealr::mlu::mlua::Error::external)?;

                if is_some {
                    (into_proxy_clone)(ref_into_option(reflect_ref), l)
                } else {
                    Ok(Value::Nil)
                }
            }),
            from_proxy: Arc::new(move |v, l| {
                if v.is_nil() {
                    // we need to allocate a new reflect reference since we don't have one existing
                    let dynamic_value = DynamicEnum::new("None", DynamicVariant::Unit);

                    let world = l.get_world()?;
                    let reflect_ref =
                        world.with_resource(|w, mut allocator: Mut<ReflectAllocator>| {
                            let value = option_from_reflect
                                .from_reflect(&dynamic_value)
                                .ok_or_else(|| {
                                    tealr::mlu::mlua::Error::external(
                                        ScriptError::new_reflection_error(""),
                                    )
                                })?
                                .into_partial_reflect();
                            Ok::<_, tealr::mlu::mlua::Error>(ReflectReference::new_allocated_boxed(
                                value,
                                &mut allocator,
                            ))
                        })?;

                    Ok(reflect_ref)
                } else {
                    let mut inner_ref = (from_proxy_clone)(v, l)?;
                    inner_ref.reflect_path.pop();
                    Ok(inner_ref)
                }
            }),
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
    pub set_value: Arc<
        dyn for<'l> Fn(
                &mut dyn PartialReflect,
                Value<'l>,
                &'l Lua,
            ) -> Result<(), tealr::mlu::mlua::Error>
            + Send
            + Sync
            + 'static,
    >,
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
    ) -> Result<DynamicEnum, tealr::mlu::mlua::Error> {
        if v.is_nil() {
            Ok(DynamicEnum::new("None", DynamicVariant::Unit))
        } else {
            let inner = (from_value)(v, lua)?;
            Ok(DynamicEnum::new(
                "Some",
                DynamicVariant::Tuple([inner].into_iter().collect()),
            ))
        }
    }

    /// generates implementation for an inner type wrapped by an option
    ///
    /// the option should check if the value is None if so return nil,
    /// if the value is some use ReflectLuaValue implementation of the inner type.
    ///
    /// If there is a type mismatch at any point will return an error
    pub fn new_for_option(
        inner_reflect_lua_value: &ReflectLuaValue,
        option_from_reflect: &ReflectFromReflect,
    ) -> Self {
        let into_value_clone = inner_reflect_lua_value.into_value.clone();
        // we have to do this so the closures can be moved into the arc
        let from_value_clone = inner_reflect_lua_value.from_value.clone();
        let from_value_clone2 = inner_reflect_lua_value.from_value.clone();
        let from_reflect_clone = option_from_reflect.clone();
        Self {
            into_value: Arc::new(move |r, lua| {
                r.as_option()
                    .map_err(tealr::mlu::mlua::Error::external)?
                    .map(|inner| (into_value_clone)(inner, lua))
                    .unwrap_or_else(|| Ok(Value::Nil))
            }),
            set_value: Arc::new(move |r, v, l| {
                let dynamic = Self::dynamic_option_from_value(v, l, &from_value_clone)?;
                r.apply(&dynamic);

                Ok(())
            }),
            from_value: Arc::new(move |v, l| {
                let dynamic_option = Self::dynamic_option_from_value(v, l, &from_value_clone2)?;

                from_reflect_clone
                    .from_reflect(&dynamic_option)
                    .ok_or_else(|| {
                        tealr::mlu::mlua::Error::external(ScriptError::new_runtime_error(
                            "Failed to convert to option",
                        ))
                    })
                    .map(<dyn Reflect>::into_partial_reflect)
            }),
        }
    }

    fn dynamic_list_from_value<'lua>(
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
    ) -> Result<DynamicList, tealr::mlu::mlua::Error> {
        let table = if let Value::Table(t) = v {
            t
        } else {
            return Err(tealr::mlu::mlua::Error::external(
                ScriptError::new_runtime_error(format!(
                    "Cannot set value of type `{}` via type: `{}`. Expected table",
                    v.type_name(),
                    "List",
                )),
            ));
        };

        let lua_values = table.sequence_values().collect::<Result<Vec<Value>, _>>()?;

        let converted_values = lua_values
            .into_iter()
            .map(|v| (from_value)(v, lua))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(DynamicList::from_iter(converted_values))
    }

    pub fn new_for_list(inner_reflect_lua_value: &ReflectLuaValue) -> Self {
        let into_value_clone = inner_reflect_lua_value.into_value.clone();
        let from_value_clone = inner_reflect_lua_value.from_value.clone();
        let from_value_clone2 = inner_reflect_lua_value.from_value.clone();
        // let vec_from_reflect = vec_from_reflect.clone();
        Self {
            into_value: Arc::new(move |r, l| {
                let inner = r.as_list().map_err(tealr::mlu::mlua::Error::external)?;
                let inner = inner
                    .map(|i| (into_value_clone)(i, l))
                    .collect::<Result<Vec<_>, _>>()?;

                l.create_table_from(
                    inner
                        .into_iter()
                        .enumerate()
                        .map(|(i, v)| (Value::Integer(i as i64 + 1), v)),
                )
                .map(Value::Table)
            }),
            set_value: Arc::new(move |r, v, l| {
                let dynamic = Self::dynamic_list_from_value(v, l, &from_value_clone)?;
                r.apply(&dynamic);
                Ok(())
            }),
            from_value: Arc::new(move |v, l| {
                let dynamic = Self::dynamic_list_from_value(v, l, &from_value_clone2)?;

                // vec_from_reflect
                //     .from_reflect(&dynamic)
                //     .ok_or_else(|| {
                //         tealr::mlu::mlua::Error::external(ScriptError::new_runtime_error(
                //             "Failed to convert to option",
                //         ))
                //     })
                //     .map(<dyn Reflect>::into_partial_reflect)
                // TODO: testing this out, not returning a concrete type could be weird
                // would anything but this impl care about this?
                // Ok(Box::new(dynamic))
                Ok(Box::new(dynamic))
            }),
        }
    }
}

impl<T: Reflect + Clone + for<'l> IntoLua<'l> + for<'l> FromLua<'l>> FromType<T>
    for ReflectLuaValue
{
    fn from_type() -> Self {
        Self {
            into_value: Arc::new(|v, l| v.try_downcast_ref::<T>().unwrap().clone().into_lua(l)),
            set_value: Arc::new(|t, v, l| {
                let t = t.try_downcast_mut::<T>().unwrap();
                *t = T::from_lua(v, l)?;
                Ok(())
            }),
            from_value: Arc::new(|v, l| {
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

fn destructure_vec_type(reg: &TypeRegistration) -> Option<(TypeId, TypeId)> {
    let type_path_table = reg.type_info().type_path_table();
    let is_core = type_path_table.crate_name().is_some_and(|s| s == "alloc");
    let is_vec = type_path_table.ident().is_some_and(|s| s == "Vec");

    if is_core && is_vec {
        reg.type_info()
            .generics()
            .get_named("T")
            .map(|t| (reg.type_id(), t.type_id()))
    } else {
        None
    }
}

/// iterates over type data for all types which have registered [`ReflectLuaProxied`] and [`ReflectLuaValue`] implementations,
/// and registers corresponding [`Option<T>`] [`Result<T, E>`] etc type data equivalents
pub fn pre_register_common_containers(type_registry: &mut AppTypeRegistry) {
    let mut type_registry = type_registry.write();

    let mut lua_value_insertions: HashMap<TypeId, ReflectLuaValue> = Default::default();
    let mut lua_proxied_insertions: HashMap<TypeId, ReflectLuaProxied> = Default::default();
    for (option_type_id, inner_type_id) in type_registry.iter().filter_map(destructure_option_type)
    {
        if let Some(inner_lua_value_data) =
            type_registry.get_type_data::<ReflectLuaValue>(inner_type_id)
        {
            if let Some(option_from_reflect) =
                type_registry.get_type_data::<ReflectFromReflect>(option_type_id)
            {
                let option_lua_proxied_data =
                    ReflectLuaValue::new_for_option(inner_lua_value_data, option_from_reflect);

                lua_value_insertions.insert(option_type_id, option_lua_proxied_data);
            }
        }

        if let Some(inner_lua_proxied_data) =
            type_registry.get_type_data::<ReflectLuaProxied>(inner_type_id)
        {
            if let Some(option_from_reflect) =
                type_registry.get_type_data::<ReflectFromReflect>(option_type_id)
            {
                let option_lua_proxied_data =
                    ReflectLuaProxied::new_for_option(inner_lua_proxied_data, option_from_reflect);

                lua_proxied_insertions.insert(option_type_id, option_lua_proxied_data);
            }
        }
    }

    for (vec_type_id, inner_type_id) in type_registry.iter().filter_map(destructure_vec_type) {
        if let Some(inner_lua_value_data) =
            type_registry.get_type_data::<ReflectLuaValue>(inner_type_id)
        {
            // if let Some(vec_from_reflect) =
            //     type_registry.get_type_data::<ReflectFromReflect>(vec_type_id)
            // {
            let vec_lua_value_data = ReflectLuaValue::new_for_list(inner_lua_value_data);

            lua_value_insertions.insert(vec_type_id, vec_lua_value_data);
            // }
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

    #[derive(Reflect)]
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

    macro_rules! assert_transitively_registers {
        ($target_type:ty, $container_type:ty, $type_data:ty) => {{
            let mut app_type_registry = setup_type_registry::<$target_type, $container_type>();
            app_type_registry
                .write()
                .register_type_data::<$target_type, $type_data>();

            pre_register_common_containers(&mut app_type_registry);

            let type_registry = app_type_registry.read();
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

    #[test]
    fn test_pre_register_common_containers_lua_value() {
        assert_transitively_registers!(usize, Option<usize>, ReflectLuaValue);
        assert_transitively_registers!(usize, Vec<usize>, ReflectLuaValue);
    }

    #[test]
    fn test_pre_register_common_containers_lua_proxy() {
        assert_transitively_registers!(Proxied, Option<Proxied>, ReflectLuaProxied);
    }

    #[test]
    fn test_option_lua_proxy_impl_into_proxy() {
        let mut app_type_registry = setup_type_registry::<Proxied, Option<Proxied>>();
        app_type_registry
            .write()
            .register_type_data::<Proxied, ReflectLuaProxied>();
        let mut world = World::default();

        pre_register_common_containers(&mut app_type_registry);

        let type_registry = app_type_registry.read();
        let option_type_data = type_registry
            .get_type_data::<ReflectLuaProxied>(std::any::TypeId::of::<Option<Proxied>>())
            .unwrap()
            .clone();
        let inner_type_data = type_registry
            .get_type_data::<ReflectLuaProxied>(std::any::TypeId::of::<Proxied>())
            .unwrap()
            .clone();

        let mut allocator = ReflectAllocator::default();
        let inner_value = ReflectReference::new_allocated(Proxied(4), &mut allocator);
        let some_value = ReflectReference::new_allocated(Some(Proxied(4)), &mut allocator);
        let none_value = ReflectReference::new_allocated(None::<Proxied>, &mut allocator);
        world.insert_resource(allocator);
        drop(type_registry);
        world.insert_resource(app_type_registry);

        let lua = Lua::new();
        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            lua.globals().set("world", LuaWorld(world.clone())).unwrap();

            // option::some into_proxy should be equivalent result to inner_type into_proxy apart from the reflect path
            let expected_into_value = (inner_type_data.into_proxy)(inner_value, &lua).unwrap();
            let gotten_into_value = (option_type_data.into_proxy)(some_value, &lua).unwrap();

            let converted_into_value = LuaReflectReference::from_lua(expected_into_value, &lua)
                .unwrap()
                .0
                .reflect_path;

            let mut converted_gotten_into_value =
                LuaReflectReference::from_lua(gotten_into_value, &lua)
                    .unwrap()
                    .0
                    .reflect_path;
            converted_gotten_into_value.pop();

            assert_eq!(
                converted_into_value, converted_gotten_into_value,
                "Option<Proxied> into_proxy should be equivalent to Proxied into_proxy for Some variant apart from the last element in the path"
            );

            // the none variant should be equivalent to Value::Nil
            let expected_into_value = Value::Nil;
            let gotten_into_value = (option_type_data.into_proxy)(none_value, &lua).unwrap();

            assert_eq!(
                expected_into_value, gotten_into_value,
                "Option<Proxied> into_proxy should be equivalent to Value::Nil for None variant"
            );
        })
    }

    #[test]
    fn test_option_lua_proxy_impl_from_proxy() {
        let mut app_type_registry = setup_type_registry::<Proxied, Option<Proxied>>();
        app_type_registry
            .write()
            .register_type_data::<Proxied, ReflectLuaProxied>();
        let mut world = World::default();

        pre_register_common_containers(&mut app_type_registry);

        let type_registry = app_type_registry.read();
        let option_type_data = type_registry
            .get_type_data::<ReflectLuaProxied>(std::any::TypeId::of::<Option<Proxied>>())
            .unwrap()
            .clone();

        let mut allocator = ReflectAllocator::default();
        let some_value = ReflectReference::new_allocated(Some(Proxied(4)), &mut allocator);
        let none_value = ReflectReference::new_allocated(None::<Proxied>, &mut allocator);
        world.insert_resource(allocator);
        drop(type_registry);
        world.insert_resource(app_type_registry);

        let lua = Lua::new();
        WorldCallbackAccess::with_callback_access(&mut world, |world| {
            lua.globals().set("world", LuaWorld(world.clone())).unwrap();

            let some_lua_value = LuaReflectReference(some_value.clone())
                .into_lua(&lua)
                .unwrap();
            let gotten_value = (option_type_data.from_proxy)(some_lua_value, &lua).unwrap();
            assert_eq!(gotten_value.reflect_path, some_value.reflect_path);

            let gotten_value = (option_type_data.from_proxy)(Value::Nil, &lua).unwrap();
            assert_eq!(gotten_value.reflect_path, none_value.reflect_path);
        })
    }

    #[test]
    fn test_option_lua_value_impl_into_value() {
        let mut app_type_registry = setup_type_registry::<usize, Option<usize>>();
        app_type_registry
            .write()
            .register_type_data::<usize, ReflectLuaValue>();

        pre_register_common_containers(&mut app_type_registry);

        let type_registry = app_type_registry.read();
        let option_type_data = type_registry
            .get_type_data::<ReflectLuaValue>(std::any::TypeId::of::<Option<usize>>())
            .unwrap();
        let inner_type_data = type_registry
            .get_type_data::<ReflectLuaValue>(std::any::TypeId::of::<usize>())
            .unwrap();

        // option::some into_value should be equivalent result to inner_type into_value
        let lua = Lua::new();
        let value: usize = 5;
        let option_value = Some(value);
        let expected_into_value = (inner_type_data.into_value)(&value, &lua).unwrap();
        let gotten_into_value = (option_type_data.into_value)(&option_value, &lua).unwrap();

        assert_eq!(
            expected_into_value, gotten_into_value,
            "Option<usize> into_value should be equivalent to usize into_value for Some variant"
        );

        // the none variant should be equivalent to Value::Nil
        let option_value: Option<usize> = None;
        let expected_into_value = Value::Nil;
        let gotten_into_value = (option_type_data.into_value)(&option_value, &lua).unwrap();

        assert_eq!(
            expected_into_value, gotten_into_value,
            "Option<usize> into_value should be equivalent to Value::Nil for None variant"
        );
    }

    #[test]
    fn test_vec_lua_value_impl_into_value() {
        let mut app_type_registry = setup_type_registry::<usize, Vec<usize>>();
        app_type_registry
            .write()
            .register_type_data::<usize, ReflectLuaValue>();

        pre_register_common_containers(&mut app_type_registry);

        let type_registry = app_type_registry.read();
        let option_type_data = type_registry
            .get_type_data::<ReflectLuaValue>(std::any::TypeId::of::<Vec<usize>>())
            .unwrap();
        let inner_type_data = type_registry
            .get_type_data::<ReflectLuaValue>(std::any::TypeId::of::<usize>())
            .unwrap();

        // option::some into_value should be a table with the inner value converted to Lua
        let lua = Lua::new();
        let value: usize = 5;
        let option_value = vec![value];
        let expected_into_value = (inner_type_data.into_value)(&value, &lua).unwrap();
        let gotten_into_value = (option_type_data.into_value)(&option_value, &lua).unwrap();

        assert_eq!(
            expected_into_value,
            gotten_into_value.as_table().unwrap().pop().unwrap(),
        );

        // an empty vec should be equivalent to an empty table
        let vec_value: Vec<usize> = vec![];
        let gotten_into_value = (option_type_data.into_value)(&vec_value, &lua).unwrap();

        assert!(gotten_into_value.as_table().unwrap().is_empty());
    }

    #[test]
    fn test_option_lua_value_set_value() {
        let mut app_type_registry = setup_type_registry::<usize, Option<usize>>();
        app_type_registry
            .write()
            .register_type_data::<usize, ReflectLuaValue>();

        pre_register_common_containers(&mut app_type_registry);

        let type_registry = app_type_registry.read();
        let option_type_data = type_registry
            .get_type_data::<ReflectLuaValue>(std::any::TypeId::of::<Option<usize>>())
            .unwrap();

        // setting an existing Some variant works correctly
        let lua = Lua::new();
        let mut target_option = Some(0usize);
        (option_type_data.set_value)(&mut target_option, 5.into_lua(&lua).unwrap(), &lua).unwrap();
        assert_eq!(
            target_option,
            Some(5),
            "Option<usize> set_value should set the value to Some(5)"
        );

        // setting an existing Some variant to nil should set the value to None
        (option_type_data.set_value)(&mut target_option, Value::Nil, &lua).unwrap();
        assert_eq!(
            target_option, None,
            "Option<usize> set_value should set the value to None"
        );

        // setting a none variant should set the Some variant correctly
        let mut target_option: Option<usize> = None;
        (option_type_data.set_value)(&mut target_option, 5usize.into_lua(&lua).unwrap(), &lua)
            .unwrap();
        assert_eq!(
            target_option,
            Some(5),
            "Option<usize> set_value should set the value to None"
        );

        // setting a none variant to nil should stay as None
        (option_type_data.set_value)(&mut target_option, Value::Nil, &lua).unwrap();
        assert_eq!(
            target_option, None,
            "Option<usize> set_value should set the value to None"
        );
    }

    #[test]
    fn test_vec_lua_value_set_value() {
        let mut app_type_registry = setup_type_registry::<usize, Vec<usize>>();
        app_type_registry
            .write()
            .register_type_data::<usize, ReflectLuaValue>();

        pre_register_common_containers(&mut app_type_registry);

        let type_registry = app_type_registry.read();
        let option_type_data = type_registry
            .get_type_data::<ReflectLuaValue>(std::any::TypeId::of::<Vec<usize>>())
            .unwrap();

        // setting an existing vec
        let lua = Lua::new();
        let new_vec = Value::Table(
            lua.create_table_from(vec![(1, 5.into_lua(&lua).unwrap())])
                .unwrap(),
        );
        let mut target_vec = vec![0usize];
        (option_type_data.set_value)(&mut target_vec, new_vec.clone(), &lua).unwrap();
        assert_eq!(target_vec, vec![5],);

        // setting an existing vec to an empty table should create a new empty vec
        (option_type_data.set_value)(
            &mut target_vec,
            Value::Table(
                lua.create_table_from(Vec::<(isize, usize)>::default())
                    .unwrap(),
            ),
            &lua,
        )
        .unwrap();
        assert_eq!(target_vec, Vec::<usize>::default(),);

        // setting an empty vec to a table with a value should set the vec to the value
        (option_type_data.set_value)(&mut target_vec, new_vec, &lua).unwrap();
        assert_eq!(target_vec, vec![5],);
    }

    #[test]
    fn test_option_lua_value_from_value() {
        let mut app_type_registry = setup_type_registry::<usize, Option<usize>>();
        app_type_registry
            .write()
            .register_type_data::<usize, ReflectLuaValue>();

        pre_register_common_containers(&mut app_type_registry);

        let type_registry = app_type_registry.read();
        let option_type_data = type_registry
            .get_type_data::<ReflectLuaValue>(std::any::TypeId::of::<Option<usize>>())
            .unwrap();

        // from_value should correctly work for concrete values
        let lua = Lua::new();
        let value = 5usize;
        let gotten_value =
            (option_type_data.from_value)(value.into_lua(&lua).unwrap(), &lua).unwrap();

        assert_eq!(
            Some(value),
            *gotten_value.try_downcast::<Option<usize>>().unwrap(),
            "Option<usize> from_value should correctly convert a value"
        );

        // from_value should correctly work for nil values
        let nil_lua = Value::Nil;
        let gotten_value = (option_type_data.from_value)(nil_lua, &lua).unwrap();
        assert_eq!(
            None::<usize>,
            *gotten_value.try_downcast::<Option<usize>>().unwrap(),
            "Option<usize> from_value should correctly convert a nil value"
        );
    }

    #[test]
    fn test_vec_lua_value_from_value() {
        let mut app_type_registry = setup_type_registry::<usize, Vec<usize>>();
        app_type_registry
            .write()
            .register_type_data::<usize, ReflectLuaValue>();

        pre_register_common_containers(&mut app_type_registry);

        let type_registry = app_type_registry.read();
        let option_type_data = type_registry
            .get_type_data::<ReflectLuaValue>(std::any::TypeId::of::<Vec<usize>>())
            .unwrap();

        // from_value should correctly work for concrete values
        let lua = Lua::new();
        let value = vec![5usize];
        let gotten_value =
            (option_type_data.from_value)(value.into_lua(&lua).unwrap(), &lua).unwrap();

        assert_eq!(
            gotten_value
                .as_list()
                .unwrap()
                .map(|v| *v.try_downcast_ref::<usize>().unwrap())
                .collect::<Vec<_>>()
                .pop(),
            Some(5usize)
        );

        // from_value should correctly work for empty lists
        let nil_lua = Value::Table(
            lua.create_table_from(Vec::<(isize, usize)>::default())
                .unwrap(),
        );
        let gotten_value = (option_type_data.from_value)(nil_lua, &lua).unwrap();
        assert!(gotten_value.as_list().unwrap().count() == 0);
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
