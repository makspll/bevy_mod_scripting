//! Set of traits used to define how types are turned into and from proxies in Lua.
//! Proxies can either be logical "copies" or owned "direct representations" of the instance, or references to one via the [`bevy_mod_scripting_core::bindings::ReflectReference`] construct.
use std::{
    cell::UnsafeCell,
    marker::PhantomData,
    num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize},
    sync::Arc,
};

use bevy::reflect::{FromReflect, Reflect, TypeRegistry};
use smallvec::SmallVec;

use crate::{
    bindings::ReflectAllocation,
    error::ScriptResult,
    prelude::{ReflectAllocator, ScriptError},
};

use super::{
    world::{WorldAccessGuard, WorldAccessUnit, WorldAccessWrite},
    ReflectReference, DEFAULT_INTERVAL, DEFAULT_TIMEOUT,
};

/// Inverse to [`Unproxy`], packages up a type into a proxy type.
pub trait Proxy: Sized {
    type Input<'a>;

    /// Proxies a type without access to the allocator, types which require access to the allocator will throw an error here
    fn proxy<'a>(input: Self::Input<'a>) -> ScriptResult<Self> {
        Err(ScriptError::new_reflection_error(format!(
            "Cannot unproxy type: `{}` without allocator access. Use proxy_with_allocator instead.",
            std::any::type_name::<Self>(),
        )))
    }

    /// Proxies a type with access to the allocator
    fn proxy_with_allocator<'a>(
        input: Self::Input<'a>,
        _allocator: &mut ReflectAllocator,
    ) -> ScriptResult<Self> {
        Self::proxy(input)
    }
}

/// A mechanism for converting proxy types into their represented types.
/// Note this can be implemented by 'meta-proxy' types which themselves aren't proxies, but wrap other proxies and provide a specific unproxying mechanism.
/// `RefProxy` and `RefMutProxy` are such 'meta-proxy' types.
///
/// the [`Unproxy::Output`] type parameter is the type that this `proxy` will be converted to after unwrapping.
///
pub trait Unproxy {
    type Output<'o>
    where
        Self: 'o;

    fn collect_accesses<'w>(
        &self,
        _guard: &WorldAccessGuard<'w>,
        _accesses: &mut SmallVec<[WorldAccessWrite<'w>; 1]>,
    ) -> ScriptResult<()> {
        Ok(())
    }

    fn accesses_len(&self) -> usize {
        0
    }

    /// Unproxies a proxy type into the represented type without world access
    /// This will fail on proxies which require world access to unproxy (for example those whose proxies are glorified [`ReflectReference`]'s )
    fn unproxy<'o>(&'o mut self) -> ScriptResult<Self::Output<'o>> {
        Err(ScriptError::new_reflection_error(format!(
            "Cannot unproxy type: `{}` without world access. Use unproxy_with_world instead",
            std::any::type_name::<Self>(),
        )))
    }

    /// Unproxies a proxy type into the represented type with world access
    /// # Safety
    /// - The caller must not use the accesses in the accesses list after the unproxy call at all, as implementors assume they have unique access to the accesses.
    unsafe fn unproxy_with_world<'w, 'o>(
        &'o mut self,
        _guard: &WorldAccessGuard<'w>,
        _accesses: &'o [WorldAccessUnit<'w>],
        _type_registry: &TypeRegistry,
        _allocator: &'o ReflectAllocator,
    ) -> ScriptResult<Self::Output<'o>> {
        self.unproxy()
    }
}

/// A wrapper type which when unproxied will return a `T` value.
/// Requires the type to be constructible from a reference to the proxy type.
#[derive(Debug, PartialEq, Eq)]
pub struct ValProxy<T, P>(pub P, PhantomData<T>);

impl<T, P> ValProxy<T, P> {
    pub fn new(v: P) -> Self {
        Self(v, PhantomData)
    }
}

/// A wrapper type which when unproxied will return a `T` value.
/// Assumes that the proxy type contains a [`ReflectReference`] via [`AsRef<ReflectReference>`]
#[derive(PartialEq, Eq, Debug)]
pub struct ReflectValProxy<T, P>(pub P, PhantomData<T>);

impl<T, P> ReflectValProxy<T, P> {
    pub fn new(v: P) -> Self {
        Self(v, PhantomData)
    }
}

/// A proxy type which when unproxied will return a reference to a `T` value.
/// Assumes that the proxy type contains a [`ReflectReference`] via [`AsRef<ReflectReference>`]
pub struct ReflectRefProxy<T, P>(pub P, PhantomData<T>);

impl<T, P> ReflectRefProxy<T, P> {
    pub fn new(v: P) -> Self {
        Self(v, PhantomData)
    }
}

/// A proxy type which when unproxied will return a mutable reference to a `T` value.
/// Assumes that the proxy type contains a [`ReflectReference`] via [`AsRef<ReflectReference>`]
#[derive(Debug)]
pub struct ReflectRefMutProxy<T, P>(pub P, PhantomData<T>);

impl<T, P> ReflectRefMutProxy<T, P> {
    pub fn new(v: P) -> Self {
        Self(v, PhantomData)
    }
}

impl<T, P> Unproxy for ValProxy<T, P>
where
    T: for<'l> From<&'l P>,
{
    type Output<'o> = T where Self: 'o;

    fn unproxy<'o>(&'o mut self) -> ScriptResult<Self::Output<'o>> {
        Ok(T::from(&self.0))
    }
}

impl<T, P> Proxy for ValProxy<T, P>
where
    T: Into<P>,
{
    type Input<'a> = T;

    fn proxy<'a>(input: Self::Input<'a>) -> ScriptResult<Self> {
        Ok(ValProxy::new(input.into()))
    }
}

impl<T, P> Proxy for ReflectValProxy<T, P>
where
    T: Reflect,
    P: From<ReflectReference>,
{
    type Input<'a> = T;

    fn proxy_with_allocator<'a>(
        input: Self::Input<'a>,
        allocator: &mut ReflectAllocator,
    ) -> ScriptResult<Self> {
        Ok(Self::new(
            ReflectReference::new_allocated(input, allocator).into(),
        ))
    }
}

impl<T, P> Unproxy for ReflectValProxy<T, P>
where
    P: AsRef<ReflectReference>,
    T: FromReflect,
{
    type Output<'o> = T where Self: 'o;

    unsafe fn unproxy_with_world<'w, 'o>(
        &'o mut self,
        guard: &WorldAccessGuard<'w>,
        _accesses: &'o [WorldAccessUnit<'w>],
        type_registry: &TypeRegistry,
        allocator: &'o ReflectAllocator,
    ) -> ScriptResult<Self::Output<'o>> {
        let reflect_ref: &ReflectReference = self.0.as_ref();
        let access = reflect_ref.base.base_id.get_reflect_access_id();
        let access = guard
            .get_access_timeout(access, DEFAULT_TIMEOUT, DEFAULT_INTERVAL)
            .ok_or_else(|| {
                ScriptError::new_reflection_error(format!(
                    "Could not unproxy type: `{}`. Aliasing access.",
                    std::any::type_name::<T>()
                ))
            })?;
        let out = reflect_ref.reflect(
            guard.as_unsafe_world_cell(),
            &access,
            type_registry,
            Some(allocator),
        )?;
        let out = T::from_reflect(out).ok_or_else(|| {
            ScriptError::new_reflection_error(format!(
                "FromReflect failed for `{}`.",
                std::any::type_name::<T>()
            ))
        })?;
        guard.release_access(access);
        Ok(out)
    }
}

impl<T, P> Proxy for ReflectRefProxy<T, P>
where
    T: FromReflect,
    P: From<ReflectReference>,
{
    type Input<'a> = &'a T;

    fn proxy_with_allocator<'a>(
        input: Self::Input<'a>,
        allocator: &mut ReflectAllocator,
    ) -> ScriptResult<Self> {
        let inner = T::from_reflect(input).ok_or_else(|| {
            ScriptError::new_reflection_error(format!(
                "FromReflect failed for `{}`.",
                std::any::type_name::<T>()
            ))
        })?;
        Ok(Self::new(
            ReflectReference::new_allocated(inner, allocator).into(),
        ))
    }
}

impl<T, P> Unproxy for ReflectRefProxy<T, P>
where
    P: AsRef<ReflectReference>,
    T: Reflect,
{
    type Output<'o> = &'o T where Self: 'o;

    fn collect_accesses<'w>(
        &self,
        guard: &WorldAccessGuard<'w>,
        accesses: &mut SmallVec<[WorldAccessUnit<'w>; 1]>,
    ) -> ScriptResult<()> {
        let reflect_ref: &ReflectReference = self.0.as_ref();
        let access = reflect_ref.base.base_id.get_reflect_access_id();
        let access = guard
            .get_access_timeout(access, DEFAULT_TIMEOUT, DEFAULT_INTERVAL)
            .ok_or_else(|| {
                ScriptError::new_reflection_error(format!(
                    "Could not unproxy type: `{}`. Aliasing access.",
                    std::any::type_name::<T>()
                ))
            })?;
        accesses.push(access);
        Ok(())
    }

    unsafe fn unproxy_with_world<'w, 'o>(
        &'o mut self,
        guard: &WorldAccessGuard<'w>,
        accesses: &'o [WorldAccessUnit<'w>],
        type_registry: &TypeRegistry,
        allocator: &'o ReflectAllocator,
    ) -> ScriptResult<Self::Output<'o>> {
        let reflect_ref: &ReflectReference = self.0.as_ref();
        let access = accesses.last().ok_or_else(|| {
            ScriptError::new_reflection_error(format!(
                "No required access collected when unproxying type: `{}`.",
                std::any::type_name::<T>()
            ))
        })?;

        let out = reflect_ref.reflect(
            guard.as_unsafe_world_cell(),
            access,
            type_registry,
            Some(allocator),
        )?;
        let out = out.try_downcast_ref().ok_or_else(|| {
            ScriptError::new_reflection_error(format!(
                "Could not downcast value from reflect reference to type: `{}`.",
                std::any::type_name::<T>()
            ))
        })?;
        Ok(out)
    }

    fn accesses_len(&self) -> usize {
        1
    }
}

impl<T, P> Unproxy for ReflectRefMutProxy<T, P>
where
    P: AsRef<ReflectReference>,
    T: Reflect,
{
    type Output<'o> = &'o mut T where Self: 'o;

    fn collect_accesses<'w>(
        &self,
        guard: &WorldAccessGuard<'w>,
        accesses: &mut SmallVec<[WorldAccessUnit<'w>; 1]>,
    ) -> ScriptResult<()> {
        let reflect_ref: &ReflectReference = self.0.as_ref();
        let access = reflect_ref.base.base_id.get_reflect_access_id();
        let access = guard
            .get_access_timeout(access, DEFAULT_TIMEOUT, DEFAULT_INTERVAL)
            .ok_or_else(|| {
                ScriptError::new_reflection_error(format!(
                    "Could not unproxy type: `{}`. Aliasing access.",
                    std::any::type_name::<T>()
                ))
            })?;
        accesses.push(access);
        Ok(())
    }

    unsafe fn unproxy_with_world<'w, 'o>(
        &'o mut self,
        guard: &WorldAccessGuard<'w>,
        accesses: &'o [WorldAccessUnit<'w>],
        type_registry: &TypeRegistry,
        allocator: &'o ReflectAllocator,
    ) -> ScriptResult<Self::Output<'o>> {
        let reflect_ref: &ReflectReference = self.0.as_ref();
        accesses
            .last()
            .ok_or_else(|| {
                ScriptError::new_reflection_error(format!(
                    "No required access collected when unproxying type: `{}`.",
                    std::any::type_name::<T>()
                ))
            })
            .and_then(|access| {
                reflect_ref.expect_write_access(
                    access,
                    type_registry,
                    Some(allocator),
                    guard.as_unsafe_world_cell(),
                )
            })?;

        // Safety:
        // - we verified and we have the right access
        // - the caller promises not to alias it from the root access
        let out = unsafe {
            reflect_ref.reflect_mut_unsafe(
                guard.as_unsafe_world_cell(),
                type_registry,
                Some(allocator),
            )?
        };
        let out = out.try_downcast_mut().ok_or_else(|| {
            ScriptError::new_reflection_error(format!(
                "Could not downcast value from reflect reference to type: `{}`.",
                std::any::type_name::<T>()
            ))
        })?;
        Ok(out)
    }

    fn accesses_len(&self) -> usize {
        1
    }
}

macro_rules! impl_unproxy_via_vec {
    ($type:ty, $out_type:ty, ($($generics:tt)*)) => {
        impl<'c, $($generics)*> Unproxy for $type {
            type Output<'o> = $out_type where Self: 'o;

            fn collect_accesses<'w>(
                &self,
                guard: &WorldAccessGuard<'w>,
                accesses: &mut SmallVec<[WorldAccessUnit<'w>; 1]>,
            ) -> ScriptResult<()> {
                for item in self {
                    item.collect_accesses(guard, accesses)?;
                }
                Ok(())
            }

            fn accesses_len(&self) -> usize {
                self.iter().map(|item| item.accesses_len()).sum()
            }

            fn unproxy(&mut self) -> ScriptResult<Self::Output<'_>> {
                let mut out = Vec::with_capacity(self.len());
                for item in self {
                    let unproxied = item.unproxy()?;
                    out.push(unproxied);
                }
                Ok(out.try_into().map_err(|_| "something went wrong").unwrap())
            }

            unsafe fn unproxy_with_world<'w, 'o>(
                &'o mut self,
                guard: &WorldAccessGuard<'w>,
                accesses: &'o [WorldAccessUnit<'w>],
                type_registry: &TypeRegistry,
                allocator: &'o ReflectAllocator,
            ) -> ScriptResult<Self::Output<'o>> {
                let mut out = Vec::with_capacity(self.len());
                let mut offset = 0;
                for item in self.iter_mut() {
                    let width = item.accesses_len();
                    let unproxied = item.unproxy_with_world(
                        guard,
                        &accesses[offset..offset + width],
                        type_registry,
                        allocator,
                    )?;
                    out.push(unproxied);
                    offset += width;
                }
                Ok(out.try_into().map_err(|_| "something went wrong").unwrap())
            }
        }
    };
}

macro_rules! impl_proxy_via_vec {
    ($type:ty, $item_type:ty, $in_type:ty, ($($generics:tt)*)) => {
        impl<$($generics)*> Proxy for $type {
            type Input<'i> = $in_type;

            fn proxy(input: Self::Input<'_>) -> ScriptResult<Self> {
                let mut out = Vec::with_capacity(input.len());
                for item in input {
                    out.push(<$item_type as Proxy>::proxy(item)?);
                }
                Ok(out.try_into().map_err(|_| "something went wrong").unwrap())
            }

            fn proxy_with_allocator(
                input: Self::Input<'_>,
                _allocator: &mut ReflectAllocator,
            ) -> ScriptResult<Self> {
                let mut out = Vec::with_capacity(input.len());
                for item in input {
                    out.push(<$item_type as Proxy>::proxy_with_allocator(item, _allocator)?);
                }
                Ok(out.try_into().map_err(|_| "something went wrong").unwrap())
            }
        }
    };
}

impl_unproxy_via_vec!(Vec<T>, Vec<T::Output<'o>>, (T: Unproxy));
impl_proxy_via_vec!(Vec<T>, T, Vec<T::Input<'i>>, (T: Proxy));
impl_unproxy_via_vec!([T; C], [T::Output<'o>; C], (T: Unproxy, const C: usize));
impl_proxy_via_vec!([T; C],T,[T::Input<'i>; C], (T: Proxy, const C: usize));
impl_unproxy_via_vec!(SmallVec<[T; C]>, SmallVec<[T::Output<'o>; C]>, (T: Unproxy, const C: usize));
impl_proxy_via_vec!(SmallVec<[T; C]>, T, SmallVec<[T::Input<'i>; C]>, (T: Proxy, const C: usize));

// impl_proxy_unproxy_via_vec!(T, SmallVec, SmallVec<[T; C]>);
// impl<'c, T: 'c> Unproxy for &'c T {
//     type Output<'o> = &'c T where Self: 'o;

//     fn unproxy(&mut self) -> ScriptResult<Self::Output<'_>> {
//         Ok(self)
//     }
// }

// impl<'s, T> Proxy for &'s T {
//     type Input<'b> = &'s T;

//     fn proxy(input: Self::Input<'_>) -> ScriptResult<Self> {
//         Ok(input)
//     }

//     fn proxy_with_allocator(
//         input: Self::Input<'_>,
//         _allocator: &mut ReflectAllocator,
//     ) -> ScriptResult<Self> {
//         Ok(input)
//     }
// }

// impl<T> Unproxy for &mut T {
//     type Output<'o> = &'o mut T where Self: 'o;

//     fn unproxy(&mut self) -> ScriptResult<Self::Output<'_>> {
//         Ok(self)
//     }
// }

// impl<'s, T> Proxy for &'s mut T {
//     type Input<'a> = &'s mut T;

//     fn proxy(input: Self::Input<'_>) -> ScriptResult<Self> {
//         Ok(input)
//     }

//     fn proxy_with_allocator(
//         input: Self::Input<'_>,
//         _allocator: &mut ReflectAllocator,
//     ) -> ScriptResult<Self> {
//         Ok(input)
//     }
// }

impl<T: Unproxy> Unproxy for Option<T> {
    type Output<'o> = Option<T::Output<'o>> where Self: 'o;

    fn unproxy(&mut self) -> ScriptResult<Self::Output<'_>> {
        if let Some(s) = self {
            let inner = s.unproxy()?;
            Ok(Some(inner))
        } else {
            Ok(None)
        }
    }

    unsafe fn unproxy_with_world<'w, 'o>(
        &'o mut self,
        guard: &WorldAccessGuard<'w>,
        accesses: &'o [WorldAccessUnit<'w>],
        type_registry: &TypeRegistry,
        allocator: &'o ReflectAllocator,
    ) -> ScriptResult<Self::Output<'o>> {
        if let Some(s) = self {
            let inner = s.unproxy_with_world(guard, accesses, type_registry, allocator)?;
            Ok(Some(inner))
        } else {
            Ok(None)
        }
    }

    fn collect_accesses<'w>(
        &self,
        guard: &WorldAccessGuard<'w>,
        accesses: &mut SmallVec<[WorldAccessWrite<'w>; 1]>,
    ) -> ScriptResult<()> {
        self.as_ref()
            .map(|s| s.collect_accesses(guard, accesses))
            .unwrap_or_else(|| Ok(()))
    }

    fn accesses_len(&self) -> usize {
        self.as_ref().map_or(0, |s| s.accesses_len())
    }
}

impl<T: Proxy> Proxy for Option<T> {
    type Input<'a> = Option<T::Input<'a>>;

    fn proxy(input: Self::Input<'_>) -> ScriptResult<Self> {
        input.map(T::proxy).transpose()
    }

    fn proxy_with_allocator(
        input: Self::Input<'_>,
        _allocator: &mut ReflectAllocator,
    ) -> ScriptResult<Self> {
        input
            .map(|i| T::proxy_with_allocator(i, _allocator))
            .transpose()
    }
}

impl<T: Proxy, E> Proxy for Result<T, E> {
    type Input<'a> = Result<T::Input<'a>, E>;

    fn proxy(input: Self::Input<'_>) -> ScriptResult<Self> {
        match input {
            Ok(i) => Ok(Ok(T::proxy(i)?)),
            Err(e) => Ok(Err(e)),
        }
    }

    fn proxy_with_allocator(
        input: Self::Input<'_>,
        _allocator: &mut ReflectAllocator,
    ) -> ScriptResult<Self> {
        match input {
            Ok(i) => Ok(Ok(T::proxy_with_allocator(i, _allocator)?)),
            Err(e) => Ok(Err(e)),
        }
    }
}

impl<T: Unproxy, E: Unproxy> Unproxy for Result<T, E> {
    type Output<'o> = Result<T::Output<'o>, E::Output<'o>> where Self: 'o;

    fn unproxy(&mut self) -> ScriptResult<Self::Output<'_>> {
        match self {
            Ok(s) => Ok(Ok(s.unproxy()?)),
            Err(e) => Ok(Err(e.unproxy()?)),
        }
    }

    unsafe fn unproxy_with_world<'w, 'o>(
        &'o mut self,
        guard: &WorldAccessGuard<'w>,
        accesses: &'o [WorldAccessUnit<'w>],
        type_registry: &TypeRegistry,
        allocator: &'o ReflectAllocator,
    ) -> ScriptResult<Self::Output<'o>> {
        match self {
            Ok(s) => Ok(Ok(s.unproxy_with_world(
                guard,
                accesses,
                type_registry,
                allocator,
            )?)),
            Err(e) => Ok(Err(e.unproxy()?)),
        }
    }

    fn collect_accesses<'w>(
        &self,
        guard: &WorldAccessGuard<'w>,
        accesses: &mut SmallVec<[WorldAccessWrite<'w>; 1]>,
    ) -> ScriptResult<()> {
        match self {
            Ok(s) => s.collect_accesses(guard, accesses),
            Err(_) => Ok(()),
        }
    }

    fn accesses_len(&self) -> usize {
        match self {
            Ok(s) => s.accesses_len(),
            Err(_) => 0,
        }
    }
}

macro_rules! impl_unproxy_by_move {
    ($($ty:ty),*) => {
        $(
            impl Unproxy for $ty {
                type Output<'o> = $ty;

                fn unproxy(
                    &mut self
                ) -> ScriptResult<Self::Output<'_>> {
                    Ok(*self)
                }
            }

            impl Unproxy for &$ty {
                type Output<'o> = &'o $ty where Self : 'o;

                fn unproxy(
                    &mut self
                ) -> ScriptResult<Self::Output<'_>> {
                    Ok(self)
                }
            }

            impl Unproxy for &mut $ty {
                type Output<'o> = &'o mut $ty where Self : 'o;

                fn unproxy(
                    &mut self
                ) -> ScriptResult<Self::Output<'_>> {
                    Ok(self)
                }
            }
        )*
    };
}

macro_rules! impl_proxy_by_move {
    ($($ty:ident),*) => {
        $(
            impl Proxy for $ty {
                type Input<'a> = Self;

                fn proxy(input: Self::Input<'_>) -> ScriptResult<Self> {
                    Ok(input)
                }
            }

            impl <'l>Proxy for &'l $ty {
                type Input<'a> = Self;

                fn proxy(input: Self::Input<'_>) -> ScriptResult<Self> {
                    Ok(input)
                }
            }

            impl <'l>Proxy for &'l mut $ty {
                type Input<'a> = Self;

                fn proxy(input: Self::Input<'_>) -> ScriptResult<Self> {
                    Ok(input)
                }
            }
        )*
    }
}

impl_proxy_by_move!(
    usize,
    u8,
    u16,
    u32,
    u64,
    u128,
    isize,
    i8,
    i16,
    i32,
    i64,
    i128,
    f32,
    f64,
    bool,
    NonZeroUsize,
    NonZeroU8,
    NonZeroU16,
    NonZeroU32,
    NonZeroU64,
    NonZeroU128
);

impl_unproxy_by_move!(
    usize,
    u8,
    u16,
    u32,
    u64,
    u128,
    isize,
    i8,
    i16,
    i32,
    i64,
    i128,
    f32,
    f64,
    bool,
    NonZeroUsize,
    NonZeroU8,
    NonZeroU16,
    NonZeroU32,
    NonZeroU64,
    NonZeroU128
);

macro_rules! impl_unproxy_by_clone {
    ($($ty:ty),*) => {
        $(impl Unproxy for $ty {
            type Output<'o> = $ty;

            fn unproxy(&mut self) -> ScriptResult<Self::Output<'_>> {
                Ok(self.clone())
            }
        })*
    };
}

impl_unproxy_by_clone!(String);
impl_proxy_by_move!(String);

macro_rules! impl_tuple_unproxy_proxy {
    ($(($ty:ident, $idx:tt)),*) => {
        impl <$($ty : Proxy,)*> Proxy for ($($ty,)*)
        {
            type Input<'a> = ($($ty::Input<'a>,)*);

            #[allow(clippy::unused_unit)]
            fn proxy(_input: Self::Input<'_>) -> ScriptResult<Self> {
                Ok(($($ty::proxy(_input.$idx)?,)*))
            }

            fn proxy_with_allocator(_input: Self::Input<'_>, _allocator: &mut ReflectAllocator) -> ScriptResult<Self> {
                Ok(($($ty::proxy_with_allocator(_input.$idx, _allocator)?,)*))
            }
        }

        impl<$($ty: Unproxy),*> Unproxy for ($($ty,)*) {
            type Output<'o> = ($($ty::Output<'o>,)*) where Self: 'o;

            fn collect_accesses<'w>(
                &self,
                _guard: &WorldAccessGuard<'w>,
                _accesses: &mut SmallVec<[WorldAccessWrite<'w>; 1]>,
            ) -> ScriptResult<()> {
                $(self.$idx.collect_accesses(_guard, _accesses)?;)*
                Ok(())
            }

            fn accesses_len(&self) -> usize {
                let mut _len = 0;
                $(_len += self.$idx.accesses_len();)*
                _len
            }

            fn unproxy(&mut self) -> ScriptResult<Self::Output<'_>> {
                Ok(($(
                    self.$idx.unproxy()?
                ,)*))
            }

            #[allow(unused_assignments)]
            unsafe fn unproxy_with_world<'w,'o>(
                &'o mut self,
                _guard: &WorldAccessGuard<'w>,
                _accesses: &'o [WorldAccessUnit<'w>],
                _type_registry: &TypeRegistry,
                _allocator: &'o ReflectAllocator,
            ) -> ScriptResult<Self::Output<'o>> {
                let mut _offset = 0;

                Ok(($(
                    {
                        let width = self.$idx.accesses_len();
                        let elem = self.$idx.unproxy_with_world(_guard, &_accesses[_offset.._offset+width], _type_registry, _allocator)?;

                        _offset += width;
                        elem
                    }
                ,)*))

            }
        }
    };
}

impl_tuple_unproxy_proxy!();
#[rustfmt::skip]
impl_tuple_unproxy_proxy!((A, 0));
#[rustfmt::skip]
impl_tuple_unproxy_proxy!((A, 0), (B, 1));
#[rustfmt::skip]
impl_tuple_unproxy_proxy!((A, 0), (B, 1), (C, 2));
#[rustfmt::skip]
impl_tuple_unproxy_proxy!((A, 0), (B, 1), (C, 2), (D, 3));
#[rustfmt::skip]
impl_tuple_unproxy_proxy!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4));
#[rustfmt::skip]
impl_tuple_unproxy_proxy!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5));
#[rustfmt::skip]
impl_tuple_unproxy_proxy!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6));
#[rustfmt::skip]
impl_tuple_unproxy_proxy!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7));
#[rustfmt::skip]
impl_tuple_unproxy_proxy!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8));
#[rustfmt::skip]
impl_tuple_unproxy_proxy!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9));
#[rustfmt::skip]
impl_tuple_unproxy_proxy!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10));
#[rustfmt::skip]
impl_tuple_unproxy_proxy!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11));
#[rustfmt::skip]
impl_tuple_unproxy_proxy!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12));
#[rustfmt::skip]
impl_tuple_unproxy_proxy!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13));
#[rustfmt::skip]
impl_tuple_unproxy_proxy!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14));
#[rustfmt::skip]
impl_tuple_unproxy_proxy!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14), (P, 15));
#[rustfmt::skip]
impl_tuple_unproxy_proxy!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14), (P, 15), (Q, 16));

#[cfg(test)]
mod test {

    use bevy::ecs::{component::Component, world::World};

    use crate::bindings::{ReflectBase, ReflectBaseType};

    use super::*;

    #[derive(Reflect, Component, PartialEq, Eq, Debug, Clone)]
    struct Test(pub &'static str);

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct ValTestProxy(Test);

    impl From<Test> for ValTestProxy {
        fn from(value: Test) -> Self {
            Self(value)
        }
    }

    impl<'a> From<&'a ValTestProxy> for Test {
        fn from(value: &'a ValTestProxy) -> Self {
            value.0.clone()
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct TestProxy(ReflectReference);

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

    macro_rules! assert_proxy_invertible {
        ($original:expr, $($proxy_ty:tt)*) => {
            let mut world = World::new();
            let mut accesses = SmallVec::new();
            let world = WorldAccessGuard::new(&mut world);
            let type_registry = TypeRegistry::default();
            let mut allocator = ReflectAllocator::default();

            // test with world version
            let mut proxy = <$($proxy_ty)* as Proxy>::proxy_with_allocator($original, &mut allocator).unwrap();
            proxy.collect_accesses(&world, &mut accesses).unwrap();

            let unproxied = unsafe {
                proxy
                    .unproxy_with_world(&world, &mut accesses, &type_registry, &allocator)
                    .unwrap()
            };
            assert_eq!(
                unproxied, $original,
                "Proxy and unproxy does not yield original type, expected {:?}, got {:?}",
                $original, unproxied
            );

            let mut proxy = <$($proxy_ty)* as Proxy>::proxy($original).unwrap();

            let unproxied_without_world = proxy.unproxy().unwrap();
            assert_eq!(
                unproxied_without_world, $original,
                "Proxy and unproxy does not yield original type, expected {:?}, got {:?}",
                $original, unproxied_without_world
            );

        };
    }

    #[test]
    pub fn test_non_reflect_val_proxy() {
        assert_proxy_invertible!(Test("test"), ValProxy::<Test, ValTestProxy>);
    }

    #[test]
    pub fn test_complex_types_proxy_is_inverse_of_unproxy() {
        assert_proxy_invertible!([32; 4], [usize; 4]);
        assert_proxy_invertible!(
            core::array::from_fn::<_, 4, _>(|_| vec![Test("test")]),
            [Vec::<ValProxy::<Test, ValTestProxy>>; 4]
        );
        assert_proxy_invertible!(Vec::<usize>::default(), Vec::<usize>);
        assert_proxy_invertible!(Some(Test("test")), Option::<ValProxy::<Test, ValTestProxy>>);
        assert_proxy_invertible!(None::<Test>, Option::<ValProxy::<Test, ValTestProxy>>);
        assert_proxy_invertible!(
            Some(Some(Test("test"))),
            Option::<Option<ValProxy::<Test, ValTestProxy>>>
        );
        assert_proxy_invertible!(
            vec![Some(Test("test")), None, Some(Test("test"))],
            Vec::<Option<ValProxy::<Test, ValTestProxy>>>
        );
        assert_proxy_invertible!(vec![&1, &2, &3], Vec::<&usize>);
        // assert_proxy_invertible!(vec![&(1, 2)], Vec::<&(usize, usize)>);
        assert_proxy_invertible!(vec![vec![1, 2], vec![1, 2, 3]], Vec::<Vec<usize>>);
        assert_proxy_invertible!(
            vec![vec![(1, 2), (3, 4)], vec![(1, 2), (3, 4)]],
            Vec::<Vec<(usize, usize)>>
        );
        assert_proxy_invertible!(Some(1), Option::<usize>);
        assert_proxy_invertible!(Some(Some(1)), Option::<Option<usize>>);
        assert_proxy_invertible!(None::<usize>, Option::<usize>);
        assert_proxy_invertible!(None::<Option<usize>>, Option::<Option<usize>>);
        assert_proxy_invertible!(vec![Some(1), None, Some(2)], Vec::<Option<usize>>);
        assert_proxy_invertible!(
            vec![Some(vec![1, 2, 3]), None, Some(vec![1, 4])],
            Vec::<Option<Vec<usize>>>
        );
    }

    #[test]
    pub fn test_val_proxy() {
        let mut allocator = ReflectAllocator::default();
        let (alloc_id, _) = allocator.allocate(Test("test"));

        let mut proxy = ReflectValProxy::<Test, TestProxy>::new(TestProxy(ReflectReference {
            base: ReflectBaseType {
                type_id: std::any::TypeId::of::<Test>(),
                base_id: ReflectBase::Owned(alloc_id),
            },
            reflect_path: vec![],
        }));
        let mut world = World::new();
        let mut accesses = SmallVec::new();
        let world = WorldAccessGuard::new(&mut world);
        let type_registry = TypeRegistry::default();

        proxy.collect_accesses(&world, &mut accesses).unwrap();
        let unproxied = unsafe {
            proxy
                .unproxy_with_world(&world, &accesses, &type_registry, &allocator)
                .unwrap()
        };
        assert_eq!(unproxied.0, "test");
    }

    #[test]
    pub fn test_proxy_ref() {
        let mut allocator = ReflectAllocator::default();
        let (alloc_id, _) = allocator.allocate(Test("test"));

        let mut proxy = ReflectRefProxy::<Test, TestProxy>::new(TestProxy(ReflectReference {
            base: ReflectBaseType {
                type_id: std::any::TypeId::of::<Test>(),
                base_id: ReflectBase::Owned(alloc_id),
            },
            reflect_path: vec![],
        }));
        let mut world = World::new();
        let mut accesses = SmallVec::new();
        let world = WorldAccessGuard::new(&mut world);
        let type_registry = TypeRegistry::default();

        proxy.collect_accesses(&world, &mut accesses).unwrap();
        let unproxied = unsafe {
            proxy
                .unproxy_with_world(&world, &accesses, &type_registry, &allocator)
                .unwrap()
        };
        assert_eq!(unproxied.0, "test");
    }

    #[test]
    pub fn test_proxy_ref_mut() {
        let mut allocator = ReflectAllocator::default();
        let (alloc_id, _) = allocator.allocate(Test("test"));

        let mut proxy = ReflectRefMutProxy::<Test, TestProxy>::new(TestProxy(ReflectReference {
            base: ReflectBaseType {
                type_id: std::any::TypeId::of::<Test>(),
                base_id: ReflectBase::Owned(alloc_id),
            },
            reflect_path: vec![],
        }));
        let mut world = World::new();
        let mut accesses = SmallVec::new();
        let world = WorldAccessGuard::new(&mut world);
        let type_registry = TypeRegistry::default();

        proxy.collect_accesses(&world, &mut accesses).unwrap();
        let unproxied = unsafe {
            proxy
                .unproxy_with_world(&world, &accesses, &type_registry, &allocator)
                .unwrap()
        };
        assert_eq!(unproxied.0, "test");
    }

    #[test]
    pub fn test_vec_proxy_ref_mut() {
        let mut allocator = ReflectAllocator::default();
        let (alloc_id, _) = allocator.allocate(Test("test"));

        let mut proxy = vec![Some(ReflectRefMutProxy::<Test, TestProxy>::new(TestProxy(
            ReflectReference {
                base: ReflectBaseType {
                    type_id: std::any::TypeId::of::<Test>(),
                    base_id: ReflectBase::Owned(alloc_id),
                },
                reflect_path: vec![],
            },
        )))];
        let mut world = World::new();
        let mut accesses = SmallVec::new();
        let world = WorldAccessGuard::new(&mut world);
        let type_registry = TypeRegistry::default();

        proxy.collect_accesses(&world, &mut accesses).unwrap();
        let unproxied = unsafe {
            proxy
                .unproxy_with_world(&world, &accesses, &type_registry, &allocator)
                .unwrap()
        };
        assert_eq!(unproxied[0].as_ref().unwrap().0, "test");
    }

    #[test]
    pub fn test_invalid_access() {
        let mut allocator = ReflectAllocator::default();

        let (allocation_id, _) = allocator.allocate(Test("test"));

        let reflect_ref = ReflectReference {
            base: ReflectBaseType {
                type_id: std::any::TypeId::of::<Test>(),
                base_id: ReflectBase::Owned(allocation_id),
            },
            reflect_path: vec![],
        };

        // mutable access to the same allocation
        let proxy = vec![
            ReflectRefMutProxy::<Test, TestProxy>::new(TestProxy(reflect_ref.clone())),
            ReflectRefMutProxy::<Test, TestProxy>::new(TestProxy(reflect_ref)),
        ];

        let mut world = World::new();
        let mut accesses = SmallVec::new();
        let world = WorldAccessGuard::new(&mut world);

        let result = proxy.collect_accesses(&world, &mut accesses);
        assert!(matches!(result, Err(..)));
    }
}
