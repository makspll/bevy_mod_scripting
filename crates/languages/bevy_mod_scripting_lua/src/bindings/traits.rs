//! Set of traits used to define how types are turned into and from proxies in Lua.
//! Proxies can either be logical "copies" or owned "direct representations" of the instance, or references to one via the [`bevy_mod_scripting_core::bindings::ReflectReference`] construct.

use bevy::reflect::{FromReflect, TypeRegistry};
use bevy_mod_scripting_core::{
    allocator::ReflectAllocator,
    bindings::{WorldAccessGuard, WorldAccessUnit, WorldAccessWrite},
    error::ReflectionError,
    proxy::{Proxied, Proxy, RefMutProxy, RefProxy, Unproxy},
};
use tealr::mlu::mlua::{Error, FromLua, IntoLua, Lua, Value};

pub struct LuaProxy<T: Proxied + FromReflect>(pub Proxy<T>);
pub struct LuaRefProxy<T: Proxied + FromReflect>(pub RefProxy<T>);
pub struct LuaRefMutProxy<T: Proxied + FromReflect>(pub RefMutProxy<T>);

macro_rules! impl_lua_proxy {
    ($ty:ident as $as:ident => $generic:tt) => {
        impl<'w, 'c, $generic: Proxied + FromReflect + 'c> Unproxy<'w, 'c> for $ty<$generic> {
            type Output = <$as<$generic> as Unproxy<'w, 'c>>::Output;

            fn collect_accesses(
                &self,
                guard: &WorldAccessGuard<'w>,
                accesses: &mut bevy::utils::smallvec::SmallVec<[WorldAccessWrite<'w>; 1]>,
            ) -> Result<(), ReflectionError> {
                self.0.collect_accesses(guard, accesses)
            }

            unsafe fn unproxy(
                &'c mut self,
                guard: &WorldAccessGuard<'w>,
                accesses: &'c [WorldAccessUnit<'w>],
                type_registry: &TypeRegistry,
                allocator: &'c ReflectAllocator,
            ) -> Result<Self::Output, ReflectionError> {
                self.0.unproxy(guard, accesses, type_registry, allocator)
            }

            fn accesses_len(&self) -> usize {
                self.0.accesses_len()
            }
        }

        impl<'lua, $generic: Proxied + FromReflect> IntoLua<'lua> for $ty<$generic>
        where
            $generic::Proxy: IntoLua<'lua>,
        {
            fn into_lua(self, lua: &'lua Lua) -> Result<Value<'lua>, Error> {
                self.0 .0.into_lua(lua)
            }
        }

        impl<'lua, $generic: Proxied + FromReflect> FromLua<'lua> for $ty<$generic>
        where
            $generic::Proxy: FromLua<'lua>,
        {
            fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> Result<Self, Error> {
                let inner: $generic::Proxy = $generic::Proxy::from_lua(value, lua)?;
                let inner: $as<$generic> = $as::<$generic>(inner);
                Ok(Self(inner))
            }
        }
    };
}

impl_lua_proxy!(LuaProxy as Proxy => T);
impl_lua_proxy!(LuaRefProxy as RefProxy => T);
impl_lua_proxy!(LuaRefMutProxy as RefMutProxy => T);

#[cfg(test)]
mod test {

    use std::{cell::UnsafeCell, sync::Arc};

    use bevy::{ecs::component::Component, reflect::Reflect};
    use bevy_mod_scripting_core::{
        allocator::ReflectAllocation,
        bindings::{ReflectBase, ReflectBaseType, ReflectReference},
    };
    use tealr::mlu::mlua::{UserData, UserDataMethods};

    use super::*;

    #[derive(Reflect, Component)]
    struct Test(pub String);

    impl Test {
        fn _set(&mut self, value: &Test) {
            self.0 = value.0.clone();
        }
    }

    impl Proxied for Test {
        type Proxy = TestProxy;
    }

    #[derive(Debug, Clone)]
    struct TestProxy(ReflectReference);

    impl From<TestProxy> for ReflectReference {
        fn from(value: TestProxy) -> Self {
            value.0
        }
    }

    impl From<ReflectReference> for TestProxy {
        fn from(value: ReflectReference) -> Self {
            TestProxy(value)
        }
    }

    impl AsRef<ReflectReference> for TestProxy {
        fn as_ref(&self) -> &ReflectReference {
            &self.0
        }
    }
    impl<'lua> FromLua<'lua> for TestProxy {
        fn from_lua(
            value: Value<'lua>,
            _lua: &'lua Lua,
        ) -> tealr::mlu::mlua::prelude::LuaResult<Self> {
            match value {
                Value::UserData(ud) => {
                    if let Ok(s) = ud.borrow::<Self>() {
                        Ok(s.clone())
                    } else {
                        panic!()
                    }
                }
                _ => panic!(),
            }
        }
    }

    impl UserData for TestProxy {
        fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
            methods.add_method("set", |_lua, _self_, _val: LuaRefProxy<Test>| Ok(()))
        }
    }

    #[test]
    pub fn test_call_set() {
        let lua = Lua::new();
        let globals = lua.globals();
        let test = Test("test".to_string());
        let mut allocator = ReflectAllocator::default();
        let allocation_id =
            allocator.allocate(ReflectAllocation::new(Arc::new(UnsafeCell::new(test))));
        let reflect_ref = ReflectReference {
            base: ReflectBaseType {
                type_id: std::any::TypeId::of::<Test>(),
                base_id: ReflectBase::Owned(allocation_id),
            },
            reflect_path: vec![],
        };
        let proxy = TestProxy(reflect_ref);
        globals.set("test", proxy).unwrap();
        lua.load(
            r#"
            test:set(test)
        "#,
        )
        .exec()
        .unwrap();
    }
}
