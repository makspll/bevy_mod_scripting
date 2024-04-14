//! Set of traits used to define how types are turned into and from proxies in Lua.
//! Proxies can either be logical "copies" or owned "direct representations" of the instance, or references to one via the [`bevy_mod_scripting_core::bindings::ReflectReference`] construct.
use std::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};

use bevy::{
    reflect::{FromReflect, Reflect, TypeRegistry},
    utils::smallvec::SmallVec,
};

use crate::{
    bindings::{ReflectReference, WorldAccessGuard, WorldAccessUnit, WorldAccessWrite},
    prelude::{ReflectAllocator, ReflectionError},
};

pub trait Proxied: Sized {
    type Proxy: Clone
        + AsRef<ReflectReference>
        + Into<ReflectReference>
        + From<ReflectReference>
        + std::fmt::Debug;
}

pub struct Proxy<T: Proxied>(pub T::Proxy);

pub struct RefProxy<T: Proxied>(pub T::Proxy);

pub struct RefMutProxy<T: Proxied>(pub T::Proxy);

pub trait Unproxy<'w, 'c> {
    type Output: 'c;

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

    /// # Safety
    /// - The caller must not use the accesses in the accesses list after the unproxy call to create a mutable reference, as this call might borrow mutably from the same access
    unsafe fn unproxy(
        &'c mut self,
        guard: &WorldAccessGuard<'w>,
        accesses: &'c [WorldAccessUnit<'w>],
        type_registry: &TypeRegistry,
        allocator: &'c ReflectAllocator,
    ) -> Result<Self::Output, ReflectionError>;
}

impl<'w, 'c, T: Proxied + FromReflect> Unproxy<'w, 'c> for Proxy<T> {
    type Output = T;

    unsafe fn unproxy(
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

impl<'w, 'c, T: Proxied + Reflect + 'w> Unproxy<'w, 'c> for RefProxy<T> {
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

    unsafe fn unproxy(
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

impl<'w, 'c, T: Proxied + Reflect + 'w> Unproxy<'w, 'c> for RefMutProxy<T> {
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

    unsafe fn unproxy(
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

    unsafe fn unproxy(
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
            let unproxied = item.unproxy(
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

impl<'w, 'c, T: Unproxy<'w, 'c> + 'c> Unproxy<'w, 'c> for &T
where
    T::Output: Copy,
{
    type Output = &'c T;

    unsafe fn unproxy(
        &'c mut self,
        _guard: &WorldAccessGuard<'w>,
        _accesses: &'c [WorldAccessUnit<'w>],
        _type_registry: &TypeRegistry,
        _allocator: &'c ReflectAllocator,
    ) -> Result<Self::Output, ReflectionError> {
        Ok(self)
    }
}

impl<'w, 'c, T: Unproxy<'w, 'c> + 'c> Unproxy<'w, 'c> for &mut T
where
    T::Output: Copy,
{
    type Output = &'c mut T;

    unsafe fn unproxy(
        &'c mut self,
        _guard: &WorldAccessGuard<'w>,
        _accesses: &'c [WorldAccessUnit<'w>],
        _type_registry: &TypeRegistry,
        _allocator: &'c ReflectAllocator,
    ) -> Result<Self::Output, ReflectionError> {
        Ok(*self)
    }
}

macro_rules! impl_by_move {
    ($($ty:ty),*) => {
        $(impl<'w, 'c> Unproxy<'w, 'c> for $ty {
            type Output = $ty;

            unsafe fn unproxy(
                &'c mut self,
                _guard: &WorldAccessGuard<'w>,
                _accesses: &'c [WorldAccessUnit<'w>],
                _type_registry: &TypeRegistry,
                _allocator: &'c ReflectAllocator,
            ) -> Result<Self::Output, ReflectionError> {
                Ok(*self)
            }
        })*
    };
}

impl_by_move!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64, bool);
impl_by_move!(
    NonZeroUsize,
    NonZeroU8,
    NonZeroU16,
    NonZeroU32,
    NonZeroU64,
    NonZeroU128
);

macro_rules! impl_by_clone {
    ($($ty:ty),*) => {
        $(impl<'w, 'c> Unproxy<'w, 'c> for $ty {
            type Output = $ty;

            unsafe fn unproxy(
                &'c mut self,
                _guard: &WorldAccessGuard<'w>,
                _accesses: &'c [WorldAccessUnit<'w>],
                _type_registry: &TypeRegistry,
                _allocator: &'c ReflectAllocator,
            ) -> Result<Self::Output, ReflectionError> {
                Ok(self.clone())
            }
        })*
    };
}

impl_by_clone!(String);

macro_rules! impl_tuple_unproxy {
    ($(($ty:ident, $idx:tt)),*) => {
        impl<'w, 'c, $($ty: Unproxy<'w, 'c>),*> Unproxy<'w, 'c> for ($($ty,)*) {
            type Output = ($($ty::Output,)*);

            fn collect_accesses(
                &self,
                guard: &WorldAccessGuard<'w>,
                accesses: &mut SmallVec<[WorldAccessWrite<'w>; 1]>,
            ) -> Result<(), ReflectionError> {
                $(self.$idx.collect_accesses(guard, accesses)?;)*
                Ok(())
            }

            fn accesses_len(&self) -> usize {
                let mut len = 0;
                $(len += self.$idx.accesses_len();)*
                len
            }

            #[allow(unused_assignments)]
            unsafe fn unproxy(
                &'c mut self,
                guard: &WorldAccessGuard<'w>,
                accesses: &'c [WorldAccessUnit<'w>],
                type_registry: &TypeRegistry,
                allocator: &'c ReflectAllocator,
            ) -> Result<Self::Output, ReflectionError> {
                let mut offset = 0;

                Ok(($(
                    {
                        let width = self.$idx.accesses_len();
                        let elem = self.$idx.unproxy(guard, &accesses[offset..offset+width], type_registry, allocator)?;

                        offset += width;
                        elem
                    }
                ,)*))

            }
        }
    };
}

#[rustfmt::skip]
impl_tuple_unproxy!((A, 0));
#[rustfmt::skip]
impl_tuple_unproxy!((A, 0), (B, 1));
#[rustfmt::skip]
impl_tuple_unproxy!((A, 0), (B, 1), (C, 2));
#[rustfmt::skip]
impl_tuple_unproxy!((A, 0), (B, 1), (C, 2), (D, 3));
#[rustfmt::skip]
impl_tuple_unproxy!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4));
#[rustfmt::skip]
impl_tuple_unproxy!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5));
#[rustfmt::skip]
impl_tuple_unproxy!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6));
#[rustfmt::skip]
impl_tuple_unproxy!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7));
#[rustfmt::skip]
impl_tuple_unproxy!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8));
#[rustfmt::skip]
impl_tuple_unproxy!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9));
#[rustfmt::skip]
impl_tuple_unproxy!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10));
#[rustfmt::skip]
impl_tuple_unproxy!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11));
#[rustfmt::skip]
impl_tuple_unproxy!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12));
#[rustfmt::skip]
impl_tuple_unproxy!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13));
#[rustfmt::skip]
impl_tuple_unproxy!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14));
#[rustfmt::skip]
impl_tuple_unproxy!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14), (P, 15));
#[rustfmt::skip]
impl_tuple_unproxy!((A, 0), (B, 1), (C, 2), (D, 3), (E, 4), (F, 5), (G, 6), (H, 7), (I, 8), (J, 9), (K, 10), (L, 11), (M, 12), (N, 13), (O, 14), (P, 15), (Q, 16));

#[cfg(test)]
mod test {
    use std::{cell::UnsafeCell, sync::Arc};

    use bevy::ecs::{component::Component, world::World};

    use crate::{
        allocator::ReflectAllocation,
        bindings::{ReflectBase, ReflectBaseType},
    };

    use super::*;

    #[derive(Reflect, Component)]
    struct Test(pub &'static str);

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

    #[test]
    pub fn test_proxy() {
        let mut allocator = ReflectAllocator::default();
        let alloc_id = allocator.allocate(ReflectAllocation::new(Arc::new(UnsafeCell::new(Test(
            "test",
        )))));

        let mut proxy = Proxy::<Test>(TestProxy(ReflectReference {
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
                .unproxy(&world, &accesses, &type_registry, &allocator)
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

        let mut proxy = RefProxy::<Test>(TestProxy(ReflectReference {
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
                .unproxy(&world, &accesses, &type_registry, &allocator)
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

        let mut proxy = RefMutProxy::<Test>(TestProxy(ReflectReference {
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
                .unproxy(&world, &accesses, &type_registry, &allocator)
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

        let mut proxy = vec![RefMutProxy::<Test>(TestProxy(ReflectReference {
            base: ReflectBaseType {
                type_id: std::any::TypeId::of::<Test>(),
                base_id: ReflectBase::Owned(alloc_id),
            },
            reflect_path: vec![],
        }))];
        let mut world = World::new();
        let mut accesses = SmallVec::new();
        let world = WorldAccessGuard::new(&mut world);
        let type_registry = TypeRegistry::default();

        proxy.collect_accesses(&world, &mut accesses).unwrap();
        let unproxied = unsafe {
            proxy
                .unproxy(&world, &accesses, &type_registry, &allocator)
                .unwrap()
        };
        assert_eq!(unproxied[0].0, "test");
    }

    #[test]
    pub fn test_vec_usize_ref() {
        let allocator = ReflectAllocator::default();

        let mut proxy = vec![&1];
        let mut world = World::new();
        let mut accesses = SmallVec::new();
        let world = WorldAccessGuard::new(&mut world);
        let type_registry = TypeRegistry::default();

        proxy.collect_accesses(&world, &mut accesses).unwrap();
        let unproxied = unsafe {
            proxy
                .unproxy(&world, &accesses, &type_registry, &allocator)
                .unwrap()
        };
        assert_eq!(unproxied[0], &42);
    }

    #[test]
    pub fn test_tuple_refs() {
        let allocator = ReflectAllocator::default();

        let mut proxy = (vec![&1], vec![&2, &4], vec![&3]);
        let mut world = World::new();
        let mut accesses = SmallVec::new();
        let world = WorldAccessGuard::new(&mut world);
        let type_registry = TypeRegistry::default();

        proxy.collect_accesses(&world, &mut accesses).unwrap();
        let unproxied = unsafe {
            proxy
                .unproxy(&world, &accesses, &type_registry, &allocator)
                .unwrap()
        };
        assert_eq!(unproxied, (vec![&1], vec![&2, &4], vec![&3]));
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
            RefMutProxy::<Test>(TestProxy(reflect_ref.clone())),
            RefMutProxy::<Test>(TestProxy(reflect_ref)),
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
