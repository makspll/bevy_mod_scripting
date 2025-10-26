//! This module contains the [`FromScript`] trait and its implemenations.

use crate::{
    ReflectReference, ScriptValue, WorldGuard, access_map::ReflectAccessId, error::InteropError,
};
use bevy_platform::collections::{HashMap, HashSet};
use bevy_reflect::{FromReflect, Reflect};
use std::{
    any::TypeId,
    ffi::OsString,
    ops::{Deref, DerefMut},
    path::PathBuf,
};

use super::script_function::{DynamicScriptFunction, DynamicScriptFunctionMut};

/// Describes the procedure for constructing a value of type `T` from a [`ScriptValue`].
///
/// The [`FromScript::This`] associated type is used to allow for the implementation of this trait to return
/// a type with the lifetime of the world guard. In 99% cases you can just use `Self` as the associated type.
pub trait FromScript {
    /// The type that is constructed from the script value.
    type This<'w>;

    /// Construct a value of type `T` from a [`ScriptValue`].
    fn from_script(
        value: ScriptValue,
        world: WorldGuard<'_>,
    ) -> Result<Self::This<'_>, InteropError>
    where
        Self: Sized;
}

#[profiling::all_functions]
impl FromScript for ScriptValue {
    type This<'w> = Self;
    fn from_script(value: ScriptValue, _world: WorldGuard) -> Result<Self, InteropError> {
        Ok(value)
    }
}

#[profiling::all_functions]
impl FromScript for () {
    type This<'w> = Self;
    fn from_script(_value: ScriptValue, _world: WorldGuard) -> Result<Self, InteropError> {
        Ok(())
    }
}

#[profiling::all_functions]
impl FromScript for bool {
    type This<'w> = Self;
    #[profiling::function]
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
            #[profiling::all_functions]
            impl FromScript for $ty {
                type This<'w> = Self;
                #[profiling::function]
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

impl_from_with_downcast!(
    i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, f32, f64, usize, isize
);

macro_rules! impl_from_stringlike {
    ($($ty:ty),*) => {
        $(
            #[profiling::all_functions]
            impl FromScript for $ty {
                type This<'w> = Self;
                #[profiling::function]
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

#[profiling::all_functions]
impl FromScript for char {
    type This<'w> = Self;
    #[profiling::function]
    fn from_script(
        value: ScriptValue,
        world: WorldGuard<'_>,
    ) -> Result<Self::This<'_>, InteropError>
    where
        Self: Sized,
    {
        match value {
            ScriptValue::Integer(i) => Ok(i as u8 as char),
            ScriptValue::String(c) => c.chars().next().ok_or_else(|| {
                InteropError::value_mismatch(TypeId::of::<char>(), ScriptValue::String(c))
            }),
            ScriptValue::Reference(r) => r.downcast::<Self>(world),
            _ => Err(InteropError::value_mismatch(
                std::any::TypeId::of::<Self>(),
                value,
            )),
        }
    }
}

#[profiling::all_functions]
impl FromScript for ReflectReference {
    type This<'w> = Self;
    #[profiling::function]
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

#[profiling::all_functions]
impl<T> Val<T> {
    /// Create a new `Val` with the given value.
    pub fn new(value: T) -> Self {
        Val(value)
    }

    /// Unwrap the value from the `Val`.
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

#[profiling::all_functions]
impl<T: FromReflect> FromScript for Val<T> {
    type This<'w> = Self;
    #[profiling::function]
    fn from_script(value: ScriptValue, world: WorldGuard) -> Result<Self, InteropError> {
        match value {
            ScriptValue::Reference(reflect_reference) => Ok(Val(reflect_reference.with_reflect(
                world.clone(),
                |r| {
                    T::from_reflect(r).ok_or_else(|| {
                        InteropError::failed_from_reflect(
                            Some(TypeId::of::<T>()),
                            format!(
                                "Expected '{}' but receievd: {r:?}",
                                std::any::type_name::<T>()
                            ),
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
/// The script function calling mechanism will take care of releasing all accesses claimed during the function call.
pub struct Ref<'w, T>(pub &'w T);

impl<T> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[profiling::all_functions]
impl<T: FromReflect> FromScript for Ref<'_, T> {
    type This<'a> = Ref<'a, T>;
    #[profiling::function]
    fn from_script(
        value: ScriptValue,
        world: WorldGuard<'_>,
    ) -> Result<Self::This<'_>, InteropError> {
        match value {
            ScriptValue::Reference(reflect_reference) => {
                let raid = ReflectAccessId::for_reference(reflect_reference.base.base_id.clone());

                if world.claim_read_access(raid) {
                    // Safety: we just claimed access
                    let ref_ = unsafe { reflect_reference.reflect_unsafe_non_empty(world) }?;
                    let cast = ref_.try_downcast_ref::<T>().ok_or_else(|| {
                        InteropError::type_mismatch(
                            std::any::TypeId::of::<T>(),
                            ref_.get_represented_type_info().map(|i| i.type_id()),
                        )
                    })?;
                    Ok(Ref(cast))
                } else {
                    Err(InteropError::cannot_claim_access(
                        raid,
                        world.get_access_location(raid),
                        format!("In conversion to type: Ref<{}>", std::any::type_name::<T>()),
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

#[profiling::all_functions]
impl<T: FromReflect> FromScript for Mut<'_, T> {
    type This<'w> = Mut<'w, T>;
    #[profiling::function]
    fn from_script(
        value: ScriptValue,
        world: WorldGuard<'_>,
    ) -> Result<Self::This<'_>, InteropError> {
        match value {
            ScriptValue::Reference(reflect_reference) => {
                let raid = ReflectAccessId::for_reference(reflect_reference.base.base_id.clone());

                if world.claim_write_access(raid) {
                    // Safety: we just claimed write access
                    let ref_ = unsafe { reflect_reference.reflect_mut_unsafe_non_empty(world) }?;
                    let type_id = ref_.get_represented_type_info().map(|i| i.type_id());
                    let cast = ref_.try_downcast_mut::<T>().ok_or_else(|| {
                        InteropError::type_mismatch(std::any::TypeId::of::<T>(), type_id)
                    })?;
                    Ok(Mut(cast))
                } else {
                    Err(InteropError::cannot_claim_access(
                        raid,
                        world.get_access_location(raid),
                        format!("In conversion to type: Mut<{}>", std::any::type_name::<T>()),
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

#[profiling::all_functions]
impl<T: FromScript> FromScript for Option<T>
where
    for<'w> T::This<'w>: Into<T>,
{
    type This<'w> = Self;
    #[profiling::function]
    fn from_script(value: ScriptValue, world: WorldGuard) -> Result<Self, InteropError> {
        match value {
            ScriptValue::Unit => Ok(None),
            _ => Ok(Some(T::from_script(value, world)?.into())),
        }
    }
}

#[profiling::all_functions]
impl<T: FromScript + 'static> FromScript for Vec<T>
where
    for<'w> T::This<'w>: Into<T>,
{
    type This<'w> = Self;
    #[profiling::function]
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

#[profiling::all_functions]
impl<T: FromScript + 'static, const N: usize> FromScript for [T; N]
where
    for<'w> T::This<'w>: Into<T>,
{
    type This<'w> = Self;
    #[profiling::function]
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

#[profiling::all_functions]
impl FromScript for DynamicScriptFunctionMut {
    type This<'w> = Self;
    #[profiling::function]
    fn from_script(value: ScriptValue, _: WorldGuard<'_>) -> Result<Self::This<'_>, InteropError>
    where
        Self: Sized,
    {
        match value {
            ScriptValue::FunctionMut(f) => Ok(f),
            _ => Err(InteropError::value_mismatch(
                std::any::TypeId::of::<Self>(),
                value,
            )),
        }
    }
}

#[profiling::all_functions]
impl FromScript for DynamicScriptFunction {
    type This<'w> = Self;
    #[profiling::function]
    fn from_script(value: ScriptValue, _: WorldGuard<'_>) -> Result<Self::This<'_>, InteropError>
    where
        Self: Sized,
    {
        match value {
            ScriptValue::Function(f) => Ok(f),
            _ => Err(InteropError::value_mismatch(
                std::any::TypeId::of::<Self>(),
                value,
            )),
        }
    }
}

macro_rules! impl_from_script_hashmap {
    ($hashmap_type:path) => {
        #[profiling::all_functions]
        impl<V> FromScript for $hashmap_type
        where
            V: FromScript + 'static,
            for<'w> V::This<'w>: Into<V>,
        {
            type This<'w> = Self;
            #[profiling::function]
            fn from_script(value: ScriptValue, world: WorldGuard) -> Result<Self, InteropError> {
                match value {
                    ScriptValue::Map(map) => {
                        let mut hashmap = <$hashmap_type>::new();
                        for (key, value) in map {
                            hashmap.insert(key, V::from_script(value, world.clone())?.into());
                        }
                        Ok(hashmap)
                    }
                    ScriptValue::List(list) => {
                        let mut hashmap = <$hashmap_type>::new();
                        for elem in list {
                            let (key, val) = <(String, V)>::from_script(elem, world.clone())?;
                            hashmap.insert(key, val);
                        }
                        Ok(hashmap)
                    }
                    _ => Err(InteropError::value_mismatch(
                        std::any::TypeId::of::<$hashmap_type>(),
                        value,
                    )),
                }
            }
        }
    };
}

impl_from_script_hashmap!(HashMap<String, V>);
impl_from_script_hashmap!(std::collections::HashMap<String, V>);

macro_rules! impl_from_script_hashset {
    ($hashset_type:path) => {
        #[profiling::all_functions]
        impl<V> FromScript for $hashset_type
        where
            V: FromScript + Eq + std::hash::Hash + 'static,
            for<'w> V::This<'w>: Into<V>,
        {
            type This<'w> = Self;
            #[profiling::function]
            fn from_script(value: ScriptValue, world: WorldGuard) -> Result<Self, InteropError> {
                match value {
                    ScriptValue::Map(map) => {
                        let mut hashmap = <$hashset_type>::new();
                        for (_, value) in map {
                            hashmap.insert(V::from_script(value, world.clone())?.into());
                        }
                        Ok(hashmap)
                    }
                    ScriptValue::List(list) => {
                        let mut hashmap = <$hashset_type>::new();
                        for elem in list {
                            let key = <V>::from_script(elem, world.clone())?;
                            hashmap.insert(key.into());
                        }
                        Ok(hashmap)
                    }
                    _ => Err(InteropError::value_mismatch(
                        std::any::TypeId::of::<$hashset_type>(),
                        value,
                    )),
                }
            }
        }
    };
}

impl_from_script_hashset![HashSet<V>];

/// A union of two or more (by nesting unions) types.
pub struct Union<T1, T2>(Result<T1, T2>);

#[profiling::all_functions]
impl<T1, T2> Union<T1, T2> {
    /// Create a new union with the left value.
    pub fn new_left(value: T1) -> Self {
        Union(Ok(value))
    }

    /// Create a new union with the right value.
    pub fn new_right(value: T2) -> Self {
        Union(Err(value))
    }

    /// Try interpret the union as the left type
    pub fn into_left(self) -> Result<T1, T2> {
        match self.0 {
            Ok(r) => Ok(r),
            Err(l) => Err(l),
        }
    }

    /// Try interpret the union as the right type
    pub fn into_right(self) -> Result<T2, T1> {
        match self.0 {
            Err(r) => Ok(r),
            Ok(l) => Err(l),
        }
    }

    /// Map the union to another type
    pub fn map_both<U1, U2, F: Fn(T1) -> U1, G: Fn(T2) -> U2>(self, f: F, g: G) -> Union<U1, U2> {
        match self.0 {
            Ok(t) => Union(Ok(f(t))),
            Err(t) => Union(Err(g(t))),
        }
    }
}

#[profiling::all_functions]
impl<T1: FromScript, T2: FromScript> FromScript for Union<T1, T2>
where
    for<'a> T1::This<'a>: Into<T1>,
    for<'a> T2::This<'a>: Into<T2>,
{
    type This<'w> = Self;
    fn from_script(
        value: ScriptValue,
        world: WorldGuard<'_>,
    ) -> Result<Self::This<'_>, InteropError> {
        let _ = match T1::from_script(value.clone(), world.clone()) {
            Ok(v) => return Ok(Union(Ok(v.into()))),
            Err(e) => e,
        };

        match T2::from_script(value, world) {
            Ok(v) => Ok(Union(Err(v.into()))),
            Err(e) => Err(e),
        }
    }
}

macro_rules! impl_from_script_tuple {
    ($($ty:ident),*) => {
        #[allow(non_snake_case)]
        #[profiling::all_functions]
        impl<$($ty: FromScript),*> FromScript for ($($ty,)*)
        where
            Self: 'static,
            $(
                for<'w> $ty::This<'w>: Into<$ty>,
            )*
        {
            type This<'w> = Self;

            fn from_script(value: ScriptValue, world: WorldGuard<'_>) -> Result<Self, InteropError> {
                match value {
                    ScriptValue::List(list) => {
                        let expected_arg_count = $crate::function::script_function::count!( $($ty)* );
                        if list.len() != expected_arg_count {
                            return Err(InteropError::length_mismatch(expected_arg_count, list.len()));
                        }

                        let mut iter = list.into_iter();
                        $(
                            let next_item = iter.next().ok_or_else(|| InteropError::invariant("list has right amount of elements"))?;
                            let $ty = $ty::from_script(next_item, world.clone())?.into();
                        )*


                        Ok(($($ty,)*))
                    }
                    _ => Err(InteropError::value_mismatch(
                        std::any::TypeId::of::<Self>(),
                        value,
                    )),
                }
            }
        }
    };
}

variadics_please::all_tuples!(impl_from_script_tuple, 1, 14, T);
