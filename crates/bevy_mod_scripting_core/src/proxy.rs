//! Set of traits used to define how types are turned into and from proxies in Lua.
//! Proxies can either be logical "copies" or owned "direct representations" of the instance, or references to one via the [`bevy_mod_scripting_core::bindings::ReflectReference`] construct.
use std::{
    cell::UnsafeCell,
    marker::PhantomData,
    num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize},
    sync::Arc,
};

use bevy::{
    reflect::{FromReflect, Reflect, TypeRegistry},
    utils::smallvec::SmallVec,
};

use crate::{
    allocator::ReflectAllocation,
    bindings::{
        ReflectBaseType, ReflectReference, WorldAccessGuard, WorldAccessUnit, WorldAccessWrite,
    },
    prelude::{ReflectAllocator, ReflectionError},
};

/// Inverse to [`Unproxy`], packages up a type into a proxy type.
pub trait Proxy<'a>: Sized {
    type Input;

    /// Proxies a type without access to the allocator, types which require access to the allocator will throw an error here
    fn proxy(input: Self::Input) -> Result<Self, ReflectionError> {
        Err(ReflectionError::InsufficientAccess {
            base: std::any::type_name::<Self>().to_owned(),
            reason: "Attempted to proxy a type that requires an allocator without providing it"
                .to_owned(),
        })
    }

    /// Proxies a type with access to the allocator
    fn proxy_with_allocator(
        input: Self::Input,
        _allocator: &mut ReflectAllocator,
    ) -> Result<Self, ReflectionError> {
        Self::proxy(input)
    }
}

/// A mechanism for converting proxy types into their represented types.
/// Note this can be implemented by 'meta-proxy' types which themselves aren't proxies, but wrap other proxies and provide a specific unproxying mechanism.
/// `RefProxy` and `RefMutProxy` are such 'meta-proxy' types.
///
/// the [`Unproxy::Output`] type parameter is the type that this `proxy` will be converted to after unwrapping.
pub trait Unproxy<'w, 'o> {
    type Output: 'o;

    fn collect_accesses(
        &self,
        _guard: &WorldAccessGuard<'w>,
        _accesses: &mut SmallVec<[WorldAccessWrite<'w>; 1]>,
    ) -> Result<(), ReflectionError> {
        Ok(())
    }

    fn accesses_len(&self) -> usize {
        0
    }

    /// Unproxies a proxy type into the represented type without world access
    /// This will fail on proxies which require world access to unproxy (for example those whose proxies are glorified [`ReflectReference`]'s )
    fn unproxy(&'o mut self) -> Result<Self::Output, ReflectionError> {
        Err(ReflectionError::InsufficientAccess {
            base: std::any::type_name::<Self::Output>().to_owned(),
            reason: "Attempted to unproxy a type that requires world access without providing it"
                .to_owned(),
        })
    }

    /// Unproxies a proxy type into the represented type with world access
    /// # Safety
    /// - The caller must not use the accesses in the accesses list after the unproxy call at all, as implementors assume they have unique access to the accesses.
    unsafe fn unproxy_with_world(
        &'o mut self,
        _guard: &WorldAccessGuard<'w>,
        _accesses: &'o [WorldAccessUnit<'w>],
        _type_registry: &TypeRegistry,
        _allocator: &'o ReflectAllocator,
    ) -> Result<Self::Output, ReflectionError> {
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
pub struct ReflectRefMutProxy<T, P>(pub P, PhantomData<T>);

impl<T, P> ReflectRefMutProxy<T, P> {
    pub fn new(v: P) -> Self {
        Self(v, PhantomData)
    }
}

impl<'w, 'c, T: 'c, P: 'c> Unproxy<'w, 'c> for ValProxy<T, P>
where
    T: From<&'c P>,
{
    type Output = T;

    fn unproxy(&'c mut self) -> Result<Self::Output, ReflectionError> {
        Ok(T::from(&self.0))
    }
}

impl<T, P> Proxy<'_> for ValProxy<T, P>
where
    T: Into<P>,
{
    type Input = T;

    fn proxy(input: Self::Input) -> Result<Self, ReflectionError> {
        Ok(ValProxy::new(input.into()))
    }
}

impl<T, P> Proxy<'_> for ReflectValProxy<T, P>
where
    T: Reflect,
    P: From<ReflectReference>,
{
    type Input = T;

    fn proxy_with_allocator(
        input: Self::Input,
        allocator: &mut ReflectAllocator,
    ) -> Result<Self, ReflectionError> {
        Ok(Self::new(
            ReflectReference::new_allocated(input, allocator).into(),
        ))
    }
}

impl<'w, 'c, T, P> Unproxy<'w, 'c> for ReflectValProxy<T, P>
where
    P: AsRef<ReflectReference>,
    T: FromReflect,
{
    type Output = T;

    unsafe fn unproxy_with_world(
        &'c mut self,
        guard: &WorldAccessGuard<'w>,
        _accesses: &'c [WorldAccessUnit<'w>],
        type_registry: &TypeRegistry,
        allocator: &'c ReflectAllocator,
    ) -> Result<Self::Output, ReflectionError> {
        let reflect_ref: &ReflectReference = self.0.as_ref();
        let access = reflect_ref.base.base_id.get_reflect_access_id();
        let access =
            guard
                .get_access(access)
                .ok_or_else(|| ReflectionError::InsufficientAccess {
                    base: std::any::type_name::<T>().to_owned(),
                    reason: "Attempted to access the same component/resource/allocation in one Unproxying operation".to_owned(),
                })?;
        let out = reflect_ref.reflect(
            guard.as_unsafe_world_cell(),
            &access,
            type_registry,
            Some(allocator),
        )?;
        let out = T::from_reflect(out).ok_or_else(|| ReflectionError::CannotDowncast {
            reference: reflect_ref.clone(),
            to: std::any::type_name::<T>().to_string(),
        })?;
        guard.release_access(access);
        Ok(out)
    }
}

impl<'a, T, P> Proxy<'a> for ReflectRefProxy<T, P>
where
    T: FromReflect,
    P: From<ReflectReference>,
{
    type Input = &'a T;

    fn proxy_with_allocator(
        input: Self::Input,
        allocator: &mut ReflectAllocator,
    ) -> Result<Self, ReflectionError> {
        let inner = T::from_reflect(input).ok_or_else(|| ReflectionError::FromReflectFailure {
            ref_: input.reflect_type_path().to_owned(),
        })?;
        Ok(Self::new(
            ReflectReference::new_allocated(inner, allocator).into(),
        ))
    }
}

impl<'w, 'c, T, P> Unproxy<'w, 'c> for ReflectRefProxy<T, P>
where
    P: AsRef<ReflectReference>,
    T: Reflect,
{
    type Output = &'c T;

    fn collect_accesses(
        &self,
        guard: &WorldAccessGuard<'w>,
        accesses: &mut SmallVec<[WorldAccessUnit<'w>; 1]>,
    ) -> Result<(), ReflectionError> {
        let reflect_ref: &ReflectReference = self.0.as_ref();
        let access = reflect_ref.base.base_id.get_reflect_access_id();
        let access =
            guard.get_access(access)
                .ok_or_else(|| ReflectionError::InsufficientAccess {
                    base: std::any::type_name::<T>().to_owned(),
                    reason: "Attempted to access the same component/resource/allocation in one Unproxying operation".to_owned(),
                })?;
        accesses.push(access);
        Ok(())
    }

    unsafe fn unproxy_with_world(
        &'c mut self,
        guard: &WorldAccessGuard<'w>,
        accesses: &'c [WorldAccessUnit<'w>],
        type_registry: &TypeRegistry,
        allocator: &'c ReflectAllocator,
    ) -> Result<Self::Output, ReflectionError> {
        let reflect_ref: &ReflectReference = self.0.as_ref();
        let access = accesses
            .last()
            .ok_or_else(|| ReflectionError::InsufficientAccess {
                base: std::any::type_name::<T>().to_owned(),
                reason: "No access collected when unproxying".to_owned(),
            })?;

        let out = reflect_ref.reflect(
            guard.as_unsafe_world_cell(),
            access,
            type_registry,
            Some(allocator),
        )?;
        let out = out
            .downcast_ref()
            .ok_or_else(|| ReflectionError::CannotDowncast {
                reference: reflect_ref.clone(),
                to: std::any::type_name::<T>().to_string(),
            })?;
        Ok(out)
    }

    fn accesses_len(&self) -> usize {
        1
    }
}

impl<'w, 'c, T, P> Unproxy<'w, 'c> for ReflectRefMutProxy<T, P>
where
    P: AsRef<ReflectReference>,
    T: Reflect,
{
    type Output = &'c mut T;

    fn collect_accesses(
        &self,
        guard: &WorldAccessGuard<'w>,
        accesses: &mut SmallVec<[WorldAccessUnit<'w>; 1]>,
    ) -> Result<(), ReflectionError> {
        let reflect_ref: &ReflectReference = self.0.as_ref();
        let access = reflect_ref.base.base_id.get_reflect_access_id();
        let access =
            guard.get_access(access)
                .ok_or_else(|| ReflectionError::InsufficientAccess {
                    base: std::any::type_name::<T>().to_owned(),
                    reason: "Attempted to access the same component/resource/allocation in one Unproxying operation".to_owned(),
                })?;
        accesses.push(access);
        Ok(())
    }

    unsafe fn unproxy_with_world(
        &'c mut self,
        guard: &WorldAccessGuard<'w>,
        accesses: &'c [WorldAccessUnit<'w>],
        type_registry: &TypeRegistry,
        allocator: &'c ReflectAllocator,
    ) -> Result<Self::Output, ReflectionError> {
        let reflect_ref: &ReflectReference = self.0.as_ref();
        accesses
            .last()
            .ok_or_else(|| ReflectionError::InsufficientAccess {
                base: std::any::type_name::<T>().to_owned(),
                reason: "No access collected when unproxying".to_owned(),
            })
            .and_then(|access| {
                reflect_ref.expect_write_access(access, type_registry, guard.as_unsafe_world_cell())
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
        let out = out
            .downcast_mut()
            .ok_or_else(|| ReflectionError::CannotDowncast {
                reference: reflect_ref.clone(),
                to: std::any::type_name::<T>().to_string(),
            })?;
        Ok(out)
    }

    fn accesses_len(&self) -> usize {
        1
    }
}

impl<'w, 'c, T: Unproxy<'w, 'c>> Unproxy<'w, 'c> for Vec<T> {
    type Output = Vec<T::Output>;

    fn collect_accesses(
        &self,
        guard: &WorldAccessGuard<'w>,
        accesses: &mut SmallVec<[WorldAccessUnit<'w>; 1]>,
    ) -> Result<(), ReflectionError> {
        for item in self {
            item.collect_accesses(guard, accesses)?;
        }
        Ok(())
    }

    fn accesses_len(&self) -> usize {
        self.iter().map(|item| item.accesses_len()).sum()
    }

    fn unproxy(&'c mut self) -> Result<Self::Output, ReflectionError> {
        let mut out = Vec::with_capacity(self.len());
        for item in self {
            let unproxied = item.unproxy()?;
            out.push(unproxied);
        }
        Ok(out)
    }

    unsafe fn unproxy_with_world(
        &'c mut self,
        guard: &WorldAccessGuard<'w>,
        accesses: &'c [WorldAccessUnit<'w>],
        type_registry: &TypeRegistry,
        allocator: &'c ReflectAllocator,
    ) -> Result<Self::Output, ReflectionError> {
        let mut out = Vec::with_capacity(self.len());
        let mut offset = 0;
        for item in self {
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
        Ok(out)
    }
}

impl<'a, T: Proxy<'a>> Proxy<'a> for Vec<T> {
    type Input = Vec<T::Input>;

    fn proxy(input: Self::Input) -> Result<Self, ReflectionError> {
        input.into_iter().map(T::proxy).collect()
    }

    fn proxy_with_allocator(
        input: Self::Input,
        _allocator: &mut ReflectAllocator,
    ) -> Result<Self, ReflectionError> {
        input
            .into_iter()
            .map(|i| T::proxy_with_allocator(i, _allocator))
            .collect()
    }
}

impl<'w, 'c, T: Unproxy<'w, 'c> + 'c> Unproxy<'w, 'c> for &T {
    type Output = &'c T;

    fn unproxy(&'c mut self) -> Result<Self::Output, ReflectionError> {
        Ok(self)
    }
}

impl<'a, T: Proxy<'a>> Proxy<'a> for &'a T {
    type Input = &'a T;

    fn proxy(input: Self::Input) -> Result<Self, ReflectionError> {
        Ok(input)
    }

    fn proxy_with_allocator(
        input: Self::Input,
        _allocator: &mut ReflectAllocator,
    ) -> Result<Self, ReflectionError> {
        Ok(input)
    }
}

impl<'w, 'c, T: Unproxy<'w, 'c> + 'c> Unproxy<'w, 'c> for &mut T {
    type Output = &'c mut T;

    fn unproxy(&'c mut self) -> Result<Self::Output, ReflectionError> {
        Ok(self)
    }
}

impl<'a, T: Proxy<'a>> Proxy<'a> for &'a mut T {
    type Input = &'a mut T;

    fn proxy(input: Self::Input) -> Result<Self, ReflectionError> {
        Ok(input)
    }

    fn proxy_with_allocator(
        input: Self::Input,
        _allocator: &mut ReflectAllocator,
    ) -> Result<Self, ReflectionError> {
        Ok(input)
    }
}

impl<'w, 'c, T: Unproxy<'w, 'c> + 'c> Unproxy<'w, 'c> for Option<T> {
    type Output = Option<T::Output>;

    fn unproxy(&'c mut self) -> Result<Self::Output, ReflectionError> {
        if let Some(s) = self {
            let inner = s.unproxy()?;
            Ok(Some(inner))
        } else {
            Ok(None)
        }
    }

    unsafe fn unproxy_with_world(
        &'c mut self,
        guard: &WorldAccessGuard<'w>,
        accesses: &'c [WorldAccessUnit<'w>],
        type_registry: &TypeRegistry,
        allocator: &'c ReflectAllocator,
    ) -> Result<Self::Output, ReflectionError> {
        if let Some(s) = self {
            let inner = s.unproxy_with_world(guard, accesses, type_registry, allocator)?;
            Ok(Some(inner))
        } else {
            Ok(None)
        }
    }

    fn collect_accesses(
        &self,
        guard: &WorldAccessGuard<'w>,
        accesses: &mut SmallVec<[WorldAccessWrite<'w>; 1]>,
    ) -> Result<(), ReflectionError> {
        self.as_ref()
            .map(|s| s.collect_accesses(guard, accesses))
            .unwrap_or_else(|| Ok(()))
    }

    fn accesses_len(&self) -> usize {
        self.as_ref().map_or(0, |s| s.accesses_len())
    }
}

impl<'a, T: Proxy<'a>> Proxy<'a> for Option<T> {
    type Input = Option<T::Input>;

    fn proxy(input: Self::Input) -> Result<Self, ReflectionError> {
        input.map(T::proxy).transpose()
    }

    fn proxy_with_allocator(
        input: Self::Input,
        _allocator: &mut ReflectAllocator,
    ) -> Result<Self, ReflectionError> {
        input
            .map(|i| T::proxy_with_allocator(i, _allocator))
            .transpose()
    }
}

macro_rules! impl_unproxy_by_move {
    ($($ty:ty),*) => {
        $(impl<'w, 'c> Unproxy<'w, 'c> for $ty {
            type Output = $ty;

            fn unproxy(
                &'c mut self
            ) -> Result<Self::Output, ReflectionError> {
                Ok(*self)
            }
        })*
    };
}

macro_rules! impl_proxy_by_move {
    ($($ty:ident),*) => {
        $(
            impl Proxy<'_> for $ty {
                type Input = Self;

                fn proxy(input: Self::Input) -> Result<Self, ReflectionError> {
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
        $(impl<'w, 'c> Unproxy<'w, 'c> for $ty {
            type Output = $ty;

            fn unproxy(&'c mut self) -> Result<Self::Output, ReflectionError> {
                Ok(self.clone())
            }
        })*
    };
}

impl_unproxy_by_clone!(String);
impl_proxy_by_move!(String);

macro_rules! impl_tuple_unproxy_proxy {
    ($(($ty:ident, $idx:tt)),*) => {
        impl <'a,$($ty : Proxy<'a>,)*> Proxy<'a> for ($($ty,)*)
        {
            type Input = ($($ty::Input,)*);

            #[allow(clippy::unused_unit)]
            fn proxy(_input: Self::Input) -> Result<Self, ReflectionError> {
                Ok(($($ty::proxy(_input.$idx)?,)*))
            }

            fn proxy_with_allocator(_input: Self::Input, _allocator: &mut ReflectAllocator) -> Result<Self, ReflectionError> {
                Ok(($($ty::proxy_with_allocator(_input.$idx, _allocator)?,)*))
            }
        }

        impl<'w, 'c, $($ty: Unproxy<'w, 'c>),*> Unproxy<'w, 'c> for ($($ty,)*) {
            type Output = ($($ty::Output,)*);

            fn collect_accesses(
                &self,
                _guard: &WorldAccessGuard<'w>,
                _accesses: &mut SmallVec<[WorldAccessWrite<'w>; 1]>,
            ) -> Result<(), ReflectionError> {
                $(self.$idx.collect_accesses(_guard, _accesses)?;)*
                Ok(())
            }

            fn accesses_len(&self) -> usize {
                let mut _len = 0;
                $(_len += self.$idx.accesses_len();)*
                _len
            }

            fn unproxy(&'c mut self) -> Result<Self::Output, ReflectionError> {
                Ok(($(
                    self.$idx.unproxy()?
                ,)*))
            }

            #[allow(unused_assignments)]
            unsafe fn unproxy_with_world(
                &'c mut self,
                _guard: &WorldAccessGuard<'w>,
                _accesses: &'c [WorldAccessUnit<'w>],
                _type_registry: &TypeRegistry,
                _allocator: &'c ReflectAllocator,
            ) -> Result<Self::Output, ReflectionError> {
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
    use std::{cell::UnsafeCell, sync::Arc};

    use bevy::ecs::{component::Component, world::World};

    use crate::{
        allocator::ReflectAllocation,
        bindings::{ReflectBase, ReflectBaseType},
    };

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

            // test allocator version works as well
            $($proxy_ty)*::proxy_with_allocator($original, &mut allocator).unwrap();
            // test proxying works
            let mut proxy = $($proxy_ty)*::proxy($original).unwrap();
            proxy.collect_accesses(&world, &mut accesses).unwrap();

            // test both unproxy methods work
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
        assert_proxy_invertible!(vec![&(1, 2)], Vec::<&(usize, usize)>);
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
        let alloc_id = allocator.allocate(ReflectAllocation::new(Arc::new(UnsafeCell::new(Test(
            "test",
        )))));

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
        let alloc_id = allocator.allocate(ReflectAllocation::new(Arc::new(UnsafeCell::new(Test(
            "test",
        )))));

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
        let alloc_id = allocator.allocate(ReflectAllocation::new(Arc::new(UnsafeCell::new(Test(
            "test",
        )))));

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
        let alloc_id = allocator.allocate(ReflectAllocation::new(Arc::new(UnsafeCell::new(Test(
            "test",
        )))));

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

        let allocation_id = allocator.allocate(ReflectAllocation::new(Arc::new(UnsafeCell::new(
            Test("test"),
        ))));

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
        assert!(matches!(
            result,
            Err(ReflectionError::InsufficientAccess { .. })
        ));
    }
}
