use crate::{
    bindings::{access_map::ReflectAccessId, ReflectReference, WorldGuard},
    error::InteropError,
    prelude::ScriptValue,
};
use bevy::reflect::{FromReflect, GetTypeRegistration, Reflect};
use std::{
    any::TypeId,
    ffi::OsString,
    ops::{Deref, DerefMut},
    path::PathBuf,
};

/// Describes the procedure for constructing a value of type `T` from a [`ScriptValue`].
///
/// The [`FromScript::This`] associated type is used to allow for the implementation of this trait to return
/// a type with the lifetime of the world guard. In 99% cases you can just use `Self` as the associated type.
pub trait FromScript {
    type This<'w>;
    fn from_script(
        value: ScriptValue,
        world: WorldGuard<'_>,
    ) -> Result<Self::This<'_>, InteropError>
    where
        Self: Sized;
}

impl FromScript for ScriptValue {
    type This<'w> = Self;
    fn from_script(value: ScriptValue, _world: WorldGuard) -> Result<Self, InteropError> {
        Ok(value)
    }
}

impl FromScript for () {
    type This<'w> = Self;
    fn from_script(value: ScriptValue, _world: WorldGuard) -> Result<Self, InteropError> {
        Ok(())
    }
}

impl FromScript for bool {
    type This<'w> = Self;

    fn from_script(
        value: ScriptValue,
        world: WorldGuard<'_>,
    ) -> Result<Self::This<'_>, InteropError>
    where
        Self: Sized,
    {
        match value {
            ScriptValue::Bool(b) => Ok(b),
            ScriptValue::Reference(r) => r.downcast::<Self>(world),
            _ => Err(InteropError::value_mismatch(
                std::any::TypeId::of::<Self>(),
                value,
            )),
        }
    }
}

macro_rules! impl_from_with_downcast {
    ($($ty:ty),*) => {
        $(
            impl FromScript for $ty {
                type This<'w> = Self;
                fn from_script(value: ScriptValue, world: WorldGuard) -> Result<Self, InteropError> {
                    match value {
                        ScriptValue::Integer(i) => Ok(i as $ty),
                        ScriptValue::Float(i) => Ok(i as $ty),
                        ScriptValue::Reference(r) => r.downcast::<Self>(world),
                        ScriptValue::Bool(b) => Ok(b as usize as $ty),
                        _ => Err(InteropError::value_mismatch(std::any::TypeId::of::<Self>(), value)),
                    }
                }
            }
        )*
    };
}

impl_from_with_downcast!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64, usize, isize);

macro_rules! impl_from_stringlike {
    ($($ty:ty),*) => {
        $(
            impl FromScript for $ty {
                type This<'w> = Self;
                fn from_script(value: ScriptValue, world: WorldGuard) -> Result<Self, InteropError> {
                    match value {
                        ScriptValue::String(s) => Ok(s.to_string().into()),
                        ScriptValue::Reference(r) => r.downcast::<Self>(world),
                        _ => Err(InteropError::value_mismatch(std::any::TypeId::of::<Self>(), value)),
                    }
                }
            }
        )*
    };
}

impl_from_stringlike!(String, PathBuf, OsString);

impl FromScript for char {
    type This<'w> = Self;

    fn from_script(
        value: ScriptValue,
        world: WorldGuard<'_>,
    ) -> Result<Self::This<'_>, InteropError>
    where
        Self: Sized,
    {
        match value {
            ScriptValue::Integer(i) => Ok(i as u8 as char),
            ScriptValue::String(c) if c.len() == 1 => Ok(c.chars().next().expect("invariant")),
            ScriptValue::Reference(r) => r.downcast::<Self>(world),
            _ => Err(InteropError::value_mismatch(
                std::any::TypeId::of::<Self>(),
                value,
            )),
        }
    }
}

impl FromScript for ReflectReference {
    type This<'w> = Self;
    fn from_script(value: ScriptValue, _world: WorldGuard) -> Result<Self, InteropError> {
        match value {
            ScriptValue::Reference(r) => Ok(r),
            _ => Err(InteropError::value_mismatch(
                std::any::TypeId::of::<ReflectReference>(),
                value,
            )),
        }
    }
}

/// A wrapper around a value of type `T`.
///
/// This can be used to retrieve a value out of a [`ScriptValue::Reference`] corresponding to the type `T`.
/// You can also use this to return values from a script function to be allocated directly as a [`ScriptValue::Reference`].
#[derive(Reflect)]
pub struct Val<T>(pub T);

impl<T> Val<T> {
    pub fn new(value: T) -> Self {
        Val(value)
    }

    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for Val<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Val<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for Val<T> {
    fn from(value: T) -> Self {
        Val(value)
    }
}

impl<T: FromReflect> FromScript for Val<T> {
    type This<'w> = Self;
    fn from_script(value: ScriptValue, world: WorldGuard) -> Result<Self, InteropError> {
        match value {
            ScriptValue::Reference(reflect_reference) => Ok(Val(reflect_reference.with_reflect(
                world.clone(),
                |r| {
                    T::from_reflect(r).ok_or_else(|| {
                        InteropError::failed_from_reflect(
                            Some(TypeId::of::<T>()),
                            format!("from reflect failed to produce output when converting to Val<T> from: {r:?}")
                                .to_owned(),
                        )
                    })
                },
            )??)),
            _ => Err(InteropError::value_mismatch(
                std::any::TypeId::of::<T>(),
                value,
            )),
        }
    }
}

/// A wrapper around a reference to a value of type `T`.
///
/// This can be used to retrieve a reference out of a [`ScriptValue::Reference`] corresponding to the type `T`.
/// Before downcasting the reference, it will claim write access to the object to ensure that the reference is valid.
///
/// However, the access is NOT released when the `Mut` is dropped. This is not unsafe but can lead to deadlocks if not released later.
/// The [`ScriptFunction`] calling mechanism will take care of releasing all accesses claimed during the function call.
pub struct Ref<'w, T>(pub &'w T);

impl<T> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<T: FromReflect> FromScript for Ref<'_, T> {
    type This<'a> = Ref<'a, T>;

    fn from_script(
        value: ScriptValue,
        world: WorldGuard<'_>,
    ) -> Result<Self::This<'_>, InteropError> {
        match value {
            ScriptValue::Reference(reflect_reference) => {
                let raid = ReflectAccessId::for_reference(reflect_reference.base.base_id.clone())
                    .ok_or_else(|| {
                    InteropError::unregistered_base(reflect_reference.base.clone())
                })?;

                if world.claim_read_access(raid) {
                    // Safety: we just claimed access
                    let ref_ = unsafe { reflect_reference.reflect_unsafe(world) }?;
                    let cast = ref_.try_downcast_ref::<T>().ok_or_else(|| {
                        InteropError::type_mismatch(
                            std::any::TypeId::of::<T>(),
                            ref_.get_represented_type_info().map(|i| i.type_id()),
                        )
                    })?;
                    Ok(Ref(cast))
                } else {
                    Err(InteropError::cannot_claim_access(
                        reflect_reference.base,
                        world.get_access_location(raid),
                    ))
                }
            }
            _ => Err(InteropError::value_mismatch(
                std::any::TypeId::of::<T>(),
                value,
            )),
        }
    }
}

impl<'a, T> From<&'a T> for Ref<'a, T> {
    fn from(value: &'a T) -> Self {
        Ref(value)
    }
}

/// A wrapper around a mutable reference to a value of type `T`.
///
/// This can be used to retrieve a mutable reference out of a [`ScriptValue::Reference`] corresponding to the type `T`.
/// Before downcasting the reference, it will claim write access to the object to ensure that the reference is valid.
///
/// However, the access is NOT released when the `Mut` is dropped. This is not unsafe but can lead to deadlocks if not released later.
/// The [`ScriptFunction`] calling mechanism will take care of releasing all accesses claimed during the function call.
pub struct Mut<'w, T>(pub &'w mut T);

impl<T> Deref for Mut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<T> DerefMut for Mut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

impl<'a, T> From<&'a mut T> for Mut<'a, T> {
    fn from(value: &'a mut T) -> Self {
        Mut(value)
    }
}

impl<T: FromReflect> FromScript for Mut<'_, T> {
    type This<'w> = Mut<'w, T>;

    fn from_script(
        value: ScriptValue,
        world: WorldGuard<'_>,
    ) -> Result<Self::This<'_>, InteropError> {
        match value {
            ScriptValue::Reference(reflect_reference) => {
                let raid = ReflectAccessId::for_reference(reflect_reference.base.base_id.clone())
                    .ok_or_else(|| {
                    InteropError::unregistered_base(reflect_reference.base.clone())
                })?;

                if world.claim_write_access(raid) {
                    // Safety: we just claimed write access
                    let ref_ = unsafe { reflect_reference.reflect_mut_unsafe(world) }?;
                    let type_id = ref_.get_represented_type_info().map(|i| i.type_id());
                    let cast = ref_.try_downcast_mut::<T>().ok_or_else(|| {
                        InteropError::type_mismatch(std::any::TypeId::of::<T>(), type_id)
                    })?;
                    Ok(Mut(cast))
                } else {
                    Err(InteropError::cannot_claim_access(
                        reflect_reference.base,
                        world.get_access_location(raid),
                    ))
                }
            }
            _ => Err(InteropError::value_mismatch(
                std::any::TypeId::of::<T>(),
                value,
            )),
        }
    }
}

impl<T: FromScript> FromScript for Option<T>
where
    for<'w> T::This<'w>: Into<T>,
{
    type This<'w> = Self;

    fn from_script(value: ScriptValue, world: WorldGuard) -> Result<Self, InteropError> {
        match value {
            ScriptValue::Unit => Ok(None),
            _ => Ok(Some(T::from_script(value, world)?.into())),
        }
    }
}

impl<T: FromScript + 'static> FromScript for Vec<T>
where
    for<'w> T::This<'w>: Into<T>,
{
    type This<'w> = Self;

    fn from_script(value: ScriptValue, world: WorldGuard) -> Result<Self, InteropError> {
        match value {
            ScriptValue::List(list) => {
                let mut vec = Vec::with_capacity(list.len());
                for item in list {
                    vec.push(T::from_script(item, world.clone())?.into());
                }
                Ok(vec)
            }
            _ => Err(InteropError::value_mismatch(
                std::any::TypeId::of::<Vec<T>>(),
                value,
            )),
        }
    }
}

impl<T: FromScript + 'static, const N: usize> FromScript for [T; N]
where
    for<'w> T::This<'w>: Into<T>,
{
    type This<'w> = Self;

    fn from_script(value: ScriptValue, world: WorldGuard) -> Result<Self, InteropError> {
        match value {
            ScriptValue::List(list) if list.len() == N => {
                let converted_list = list
                    .into_iter()
                    .map(|item| T::from_script(item, world.clone()).map(Into::into))
                    .collect::<Result<Vec<T>, _>>()?
                    .try_into()
                    .map_err(|list: Vec<T>| InteropError::length_mismatch(N, list.len()))?;
                Ok(converted_list)
            }
            _ => Err(InteropError::value_mismatch(
                std::any::TypeId::of::<[T; N]>(),
                value,
            )),
        }
    }
}
