//! Set of traits used to define how types are turned into and from proxies in Lua.
//! Proxies can either be logical "copies" or owned "direct representations" of the instance, or references to one via the [`bevy_mod_scripting_core::bindings::ReflectReference`] construct.

use bevy::reflect::{FromReflect, Reflect, TypeRegistry};
use bevy_mod_scripting_core::{
    bindings::{
        Proxy, ReflectAllocator, ReflectRefMutProxy, ReflectRefProxy, ReflectReference,
        ReflectValProxy, Unproxy, ValProxy, WorldAccessGuard, WorldAccessUnit, WorldAccessWrite,
    },
    error::{ScriptError, ScriptResult},
};
use tealr::{
    mlu::mlua::{Error, FromLua, IntoLua, IntoLuaMulti, Lua, Value},
    ToTypename,
};

use super::world::Nil;

/// Think of this as a Local trait alias for the [`Proxy`] trait. Specifies the proxy type for a given type.
pub trait LuaProxied {
    type Proxy;
}

/// Proxy which acts exactly like `T` when converting to Lua, but provides a `ToTypename` implementation based on another type `N`.
/// Used internally basically only to support Result types in Lua directly.
/// Will be unnecessary once I get rid of tealr.    
pub(crate) struct TypenameProxy<T, N: ToTypename>(T, std::marker::PhantomData<N>);

impl<T, N: ToTypename> TypenameProxy<T, N> {
    pub fn new(value: T) -> Self {
        Self(value, std::marker::PhantomData)
    }
}

impl<T, N: ToTypename> ToTypename for TypenameProxy<T, N> {
    fn to_typename() -> tealr::Type {
        N::to_typename()
    }
}

impl<'a, T: IntoLuaMulti<'a>, N: ToTypename> IntoLua<'a> for TypenameProxy<T, N> {
    fn into_lua(self, lua: &'a Lua) -> tealr::mlu::mlua::Result<Value<'a>> {
        self.0
            .into_lua_multi(lua)
            .map(|mut v| v.pop_front().unwrap())
    }
}

/// Proxy which converts to lua by throwing the error. Can be used inside a [`Result`] to proxy into a result which will throw the error if it's an [`Err`] when converting to Lua.
pub struct ErrorProxy<E>(E);

impl<'a, E: Into<Box<dyn std::error::Error + Send + Sync + 'static>>> IntoLua<'a>
    for ErrorProxy<E>
{
    fn into_lua(self, _: &'a Lua) -> tealr::mlu::mlua::prelude::LuaResult<Value<'a>> {
        Err(Error::external(self.0))
    }
}

/// This should never really need to be used, but it's here so we can use the type in Lua.
impl<E> ToTypename for ErrorProxy<E> {
    fn to_typename() -> tealr::Type {
        tealr::Type::new_single("Error", tealr::KindOfType::External)
    }
}

impl<E: Into<Box<dyn std::error::Error + Send + Sync + 'static>>> Proxy for ErrorProxy<E> {
    type Input<'i> = E;
    fn proxy<'i>(value: Self::Input<'i>) -> ScriptResult<Self> {
        Ok(Self(value))
    }
}

/// Convenience for proxying a type into lua via itself without implementing [`Proxy`] on it.
/// Converts to Lua via T's implementation of IntoLua directly
pub struct LuaIdentityProxy<T>(pub Option<T>);

impl<T> Proxy for LuaIdentityProxy<T> {
    type Input<'i> = T;
    fn proxy<'i>(value: Self::Input<'i>) -> ScriptResult<Self> {
        Ok(Self(Some(value)))
    }
}

impl<T> Unproxy for LuaIdentityProxy<T> {
    type Output<'o> = T where
        Self: 'o;

    fn unproxy<'o>(&'o mut self) -> ScriptResult<Self::Output<'o>> {
        Ok(self
            .0
            .take()
            .expect("IdentityProxy was already unproxied before"))
    }
}

impl<T: ToTypename> ToTypename for LuaIdentityProxy<T> {
    fn to_typename() -> tealr::Type {
        T::to_typename()
    }
}

impl<'a, T: IntoLua<'a>> IntoLua<'a> for LuaIdentityProxy<T> {
    fn into_lua(self, lua: &'a Lua) -> tealr::mlu::mlua::prelude::LuaResult<Value<'a>> {
        self.0.into_lua(lua)
    }
}

impl<'a, T: FromLua<'a>> FromLua<'a> for LuaIdentityProxy<T> {
    fn from_lua(value: Value<'a>, lua: &'a Lua) -> Result<Self, Error> {
        Ok(Self(Some(T::from_lua(value, lua)?)))
    }
}

/// Proxy which uses [`ValProxy`] to represent the type in Lua. Requires that the type implements [`LuaProxied`] and that the proxy implements [`From`] for the type.
///
/// Used for types which are copied into lua rather than references to originals in the world.
/// Use when your type does not implement Reflect or if it's a simple type that can be copied into lua.
pub struct LuaValProxy<T: LuaProxied>(pub ValProxy<T, T::Proxy>);

/// Proxy which uses [`ReflectValProxy`] to represent the type in Lua. Requires that the type implements [`LuaProxied`] and [`FromReflect`] and that the proxy implements [`AsRef<ReflectReference>`].
/// Think of the proxy as just a container for a [`ReflectReference`].
///
/// Semantically equivalent to `T`, use it where you would use the `T` type.
pub struct LuaReflectValProxy<T: LuaProxied>(pub ReflectValProxy<T, T::Proxy>);

/// Proxy which uses [`ReflectRefProxy`] to represent the type in Lua. Requires that the type implements [`LuaProxied`] and [`Reflect`] and that the proxy implements [`AsRef<ReflectReference>`].
/// Think of the proxy as just a container for a [`ReflectReference`].
///
/// Semantically equivalent to `&T`, use it where you would use the `&T` type.
pub struct LuaReflectRefProxy<T: LuaProxied>(pub ReflectRefProxy<T, T::Proxy>);

/// Proxy which uses [`ReflectRefMutProxy`] to represent the type in Lua. Requires that the type implements [`LuaProxied`] and [`Reflect`] and that the proxy implements [`AsRef<ReflectReference>`].
/// Think of the proxy as just a container for a [`ReflectReference`].
///
/// Semantically equivalent to `&mut T`, use it where you would use the `&mut T` type.
pub struct LuaReflectRefMutProxy<T: LuaProxied>(pub ReflectRefMutProxy<T, T::Proxy>);

macro_rules! impl_lua_unproxy {
    ($ty:ident as $as:ident ($generic:tt) $($bound_var:path : ($($bounds:tt)+),)*) => {
        impl <$generic> Unproxy for $ty<$generic>
        where
            $($bound_var : $($bounds)+),*
        {
            type Output<'b> = <$as<$generic,$generic::Proxy> as Unproxy>::Output<'b> where Self: 'b;

            fn collect_accesses<'w>(
                &self,
                guard: &WorldAccessGuard<'w>,
                accesses: &mut smallvec::SmallVec<[WorldAccessWrite<'w>; 1]>,
            ) -> ScriptResult<()> {
                self.0.collect_accesses(guard, accesses)
            }

            fn unproxy(&mut self) -> ScriptResult<Self::Output<'_>> {
                self.0.unproxy()
            }

            unsafe fn unproxy_with_world<'w,'o>(
                &'o mut self,
                guard: &WorldAccessGuard<'w>,
                accesses: &'o [WorldAccessUnit<'w>],
                type_registry: &TypeRegistry,
                allocator: &'o ReflectAllocator,
            ) -> ScriptResult<Self::Output<'o>> {
                self.0
                    .unproxy_with_world(guard, accesses, type_registry, allocator)
            }

            fn accesses_len(&self) -> usize {
                self.0.accesses_len()
            }
        }

        impl<'lua, $generic: LuaProxied> FromLua<'lua> for $ty<$generic>
        where
            $generic::Proxy: FromLua<'lua>,
        {
            fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> Result<Self, Error> {
                let inner: $generic::Proxy = $generic::Proxy::from_lua(value, lua)?;
                let inner = $as::<$generic,$generic::Proxy>::new(inner);
                Ok(Self(inner))
            }
        }

        impl<'lua, $generic: LuaProxied> IntoLua<'lua> for $ty<$generic>
        where
            $generic::Proxy: IntoLua<'lua>,
        {
            fn into_lua(self, lua: &'lua Lua) -> tealr::mlu::mlua::prelude::LuaResult<Value<'lua>> {
                self.0.0.into_lua(lua)
            }
        }

        impl<T: LuaProxied> ToTypename for $ty<T> where T::Proxy: ToTypename {
            fn to_typename() -> tealr::Type {
                T::Proxy::to_typename()
            }
        }
    };
}

macro_rules! impl_lua_proxy {
    ($ty:ident as $as:ident => $generic:tt : $($bounds:path),* $(| T::Proxy: $($proxy_bounds:tt)*)?) => {
        impl<$generic> bevy_mod_scripting_core::bindings::Proxy for $ty<$generic>
        where
            T::Proxy: $($($proxy_bounds)*)?,
            T: $($bounds+)*,
        {
            type Input<'i>=<$as<$generic, $generic::Proxy> as bevy_mod_scripting_core::bindings::Proxy>::Input<'i>;
            fn proxy<'i>(value: Self::Input<'i>) -> ScriptResult<Self> {
                Ok(Self($as::<$generic,$generic::Proxy>::proxy(value)?))
            }
        }


    };
}

impl_lua_proxy!(LuaValProxy as ValProxy => T : LuaProxied | T::Proxy: From<T>);
impl_lua_proxy!(LuaReflectValProxy as ReflectValProxy => T : LuaProxied,Reflect | T::Proxy: From<ReflectReference> );

impl_lua_unproxy!(LuaValProxy as ValProxy (T)
    T: (LuaProxied),
    T: (for <'l> From<&'l T::Proxy>),
);
impl_lua_unproxy!(LuaReflectValProxy as ReflectValProxy (T)
    T: (FromReflect),
    T: (LuaProxied),
    T::Proxy: (AsRef<ReflectReference>),
);
impl_lua_unproxy!(LuaReflectRefProxy as ReflectRefProxy (T)
    T: (LuaProxied),
    T: (Reflect),
    T::Proxy: (AsRef<ReflectReference>),
);
impl_lua_unproxy!(LuaReflectRefMutProxy as ReflectRefMutProxy (T)
    T: (LuaProxied),
    T: (Reflect),
    T::Proxy: (AsRef<ReflectReference>),
);
