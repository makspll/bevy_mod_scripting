//! This module contains the [`FromScript`] trait and its implemenations.

use crate::function::bound::BoundFields;
use crate::function::script_function::{DynamicScriptFunction, DynamicScriptFunctionMut};
use crate::native::{ArgSource, NativeStorage};
use crate::{
    EnumVariantRef, ReflectReference, ScriptValue, construct::construct_reference,
    error::InteropError, script_value::VariadicTuple,
};
use bevy_mod_scripting_world::{CallScope, WorldGuard};
use bevy_platform::collections::{HashMap, HashSet};
use bevy_reflect::{FromReflect, Reflect};
use nonmax::NonMaxU32;
use std::{
    any::TypeId,
    collections::VecDeque,
    ffi::OsString,
    ops::{Deref, DerefMut},
    path::PathBuf,
};

/// Describes the procedure for constructing a value of type `T` from a [`ScriptValue`].
///
/// The [`FromScript::This`] associated type is used to allow for the implementation of this trait to return
/// a type borrowing from the world for the duration of a [`CallScope`]. In 99% cases you can just use `Self` as the associated type.
pub trait FromScript {
    /// The type that is constructed from the script value.
    type This<'s>;

    /// Construct a value of type `T` from a [`ScriptValue`].
    ///
    /// Borrows from the world can't outlive the `scope`, which releases their accesses when it ends,
    /// see [`bevy_mod_scripting_world::WorldAccessGuard::with_call_scope`].
    fn from_script<'s>(
        value: ScriptValue,
        scope: &'s CallScope<'_>,
    ) -> Result<Self::This<'s>, InteropError>
    where
        Self: Sized;

    /// Whether a value of this kind could be converted, without converting it. `false` means [`Self::from_script`] would fail.
    ///
    /// Unions only try the types which could convert a value, so they don't build an error for each type which can't.
    fn could_convert(_value: &ScriptValue) -> bool
    where
        Self: Sized,
    {
        true
    }

    /// Converts argument `index` of a typed call (see [`crate::native`]), which may be given in its native form.
    ///
    /// The default converts the argument to a [`ScriptValue`] and calls [`Self::from_script`], so every type works in typed calls,
    /// types with a faster way override it.
    fn from_arg<'s>(
        args: &dyn ArgSource,
        index: usize,
        scope: &'s CallScope<'_>,
    ) -> Result<Self::This<'s>, InteropError>
    where
        Self: Sized,
    {
        Self::from_script(args.script_value(index)?, scope)
    }

    /// Whether [`Self::from_arg`] takes argument `index` as it is, without converting it to a [`ScriptValue`], i.e. a native value
    /// of the type. Used by [`Union`] to pick a type without converting.
    fn accepts_arg(_args: &dyn ArgSource, _index: usize) -> bool
    where
        Self: Sized,
    {
        false
    }
}

/// Whether argument `index` is a native value of `T`.
fn is_native_arg<T: 'static>(args: &dyn ArgSource, index: usize) -> bool {
    // Safety: `ArgSource` gives initialised storage, alive until the call returns
    args.native(index).is_some_and(
        |base| unsafe { NativeStorage::header(base) }.vtable.type_id == TypeId::of::<T>(),
    )
}

fn native_arg<T: 'static>(
    args: &dyn ArgSource,
    index: usize,
    scope: &CallScope<'_>,
    exclusive: bool,
) -> Result<Option<(Option<*mut u8>, *mut T)>, InteropError> {
    let Some(base) = args.native(index) else {
        // a plain value (i.e. a LuaJIT cdata), whose bytes scripts can write: its borrows are kept by the scope
        let Some(value) = args.plain(index, TypeId::of::<T>()) else {
            return Ok(None);
        };
        // Safety: `ArgSource` gives a T, alive until the call returns, which outlives the scope
        if !unsafe { scope.borrow_plain(value, std::mem::size_of::<T>(), exclusive) } {
            return Err(InteropError::string(format!(
                "a {} is passed to a function in a way which conflicts with another argument",
                std::any::type_name::<T>()
            )));
        }
        return Ok(Some((None, value as *mut T)));
    };
    // Safety: `ArgSource` gives initialised storage, alive until the call returns, which outlives the scope
    let header = unsafe { NativeStorage::header(base) };
    if header.vtable.type_id != TypeId::of::<T>() {
        return Ok(None);
    }
    // Safety: see above, a place of a T
    if let Some(place) = unsafe { NativeStorage::place(base) } {
        // access claims keep other arguments from aliasing it
        return Ok(Some((
            Some(base),
            unsafe { place.resolve(scope, exclusive) }? as *mut T,
        )));
    }
    // Safety: see above
    if !unsafe { scope.borrow(&header.borrow, exclusive) } {
        return Err(InteropError::string(format!(
            "a {} is passed to a function in a way which conflicts with another argument",
            header.vtable.type_name
        )));
    }
    // Safety: see above, the value is a T
    Ok(Some(
        (Some(base), unsafe { NativeStorage::value(base) } as *mut T)
    ))
}

#[profiling::all_functions]
impl FromScript for ScriptValue {
    type This<'w> = Self;
    fn from_script(value: ScriptValue, _scope: &CallScope) -> Result<Self, InteropError> {
        Ok(value)
    }
}

#[profiling::all_functions]
impl FromScript for () {
    type This<'w> = Self;
    fn from_script(_value: ScriptValue, _scope: &CallScope) -> Result<Self, InteropError> {
        Ok(())
    }
}

#[profiling::all_functions]
impl FromScript for bool {
    type This<'w> = Self;
    #[profiling::function]
    fn from_script<'s>(
        value: ScriptValue,
        scope: &'s CallScope<'_>,
    ) -> Result<Self::This<'s>, InteropError>
    where
        Self: Sized,
    {
        match value {
            ScriptValue::Bool(b) => Ok(b),
            ScriptValue::Reference(r) => r.downcast::<Self>(scope.world()?),
            _ => Err(InteropError::value_mismatch(
                std::any::TypeId::of::<Self>(),
                value,
            )),
        }
    }

    fn could_convert(value: &ScriptValue) -> bool {
        matches!(value, ScriptValue::Bool(_) | ScriptValue::Reference(_))
    }
}

macro_rules! impl_from_with_downcast {
    ($($ty:ty),*) => {
        $(
            #[profiling::all_functions]
            impl FromScript for $ty {
                type This<'w> = Self;
                #[profiling::function]
                fn from_script(value: ScriptValue, scope: &CallScope) -> Result<Self, InteropError> {
                    match value {
                        ScriptValue::Integer(i) => Ok(i as $ty),
                        ScriptValue::Float(i) => Ok(i as $ty),
                        ScriptValue::Reference(r) => r.downcast::<Self>(scope.world()?),
                        ScriptValue::Bool(b) => Ok(b as usize as $ty),
                        _ => Err(InteropError::value_mismatch(std::any::TypeId::of::<Self>(), value)),
                    }
                }

                fn could_convert(value: &ScriptValue) -> bool {
                    matches!(
                        value,
                        ScriptValue::Integer(_) | ScriptValue::Float(_) | ScriptValue::Bool(_) | ScriptValue::Reference(_)
                    )
                }

                fn from_arg(args: &dyn ArgSource, index: usize, scope: &CallScope<'_>) -> Result<Self, InteropError> {
                    if let Some(i) = args.integer(index) {
                        return Ok(i as $ty);
                    }
                    if let Some(n) = args.number(index) {
                        return Ok(n as $ty);
                    }
                    Self::from_script(args.script_value(index)?, scope)
                }

                fn accepts_arg(args: &dyn ArgSource, index: usize) -> bool {
                    args.integer(index).is_some() || args.number(index).is_some()
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
                fn from_script(value: ScriptValue, scope: &CallScope) -> Result<Self, InteropError> {
                    match value {
                        ScriptValue::String(s) => Ok(s.to_string().into()),
                        ScriptValue::Reference(r) => r.downcast::<Self>(scope.world()?),
                        _ => Err(InteropError::value_mismatch(std::any::TypeId::of::<Self>(), value)),
                    }
                }

                fn could_convert(value: &ScriptValue) -> bool {
                    matches!(value, ScriptValue::String(_) | ScriptValue::Reference(_))
                }

                fn from_arg(args: &dyn ArgSource, index: usize, scope: &CallScope<'_>,) -> Result<Self, InteropError> {
                    // read from the language's string, no script value
                    if let Some(s) = args.string(index) {
                        return Ok(s.to_owned().into());
                    }
                    Self::from_script(args.script_value(index)?, scope)
                }
            }
        )*
    };
}

impl_from_stringlike!(String, PathBuf, OsString);

impl FromScript for NonMaxU32 {
    type This<'w> = Self;

    fn from_script<'s>(
        value: ScriptValue,
        _scope: &'s CallScope<'_>,
    ) -> Result<Self::This<'s>, InteropError>
    where
        Self: Sized,
    {
        let converted = match value {
            ScriptValue::Integer(i) => u32::try_from(i).ok(),
            // `as` saturates, so check the range first
            ScriptValue::Float(f) if (0.0..u32::MAX as f64).contains(&f) => Some(f as u32),
            _ => None,
        };
        converted
            .and_then(NonMaxU32::new)
            .ok_or_else(|| InteropError::value_mismatch(std::any::TypeId::of::<Self>(), value))
    }
}

#[profiling::all_functions]
impl FromScript for char {
    type This<'w> = Self;
    #[profiling::function]
    fn from_script<'s>(
        value: ScriptValue,
        scope: &'s CallScope<'_>,
    ) -> Result<Self::This<'s>, InteropError>
    where
        Self: Sized,
    {
        match value {
            ScriptValue::Integer(i) => Ok(i as u8 as char),
            ScriptValue::String(c) => c.chars().next().ok_or_else(|| {
                InteropError::value_mismatch(TypeId::of::<char>(), ScriptValue::String(c))
            }),
            ScriptValue::Reference(r) => r.downcast::<Self>(scope.world()?),
            _ => Err(InteropError::value_mismatch(
                std::any::TypeId::of::<Self>(),
                value,
            )),
        }
    }

    fn could_convert(value: &ScriptValue) -> bool {
        matches!(
            value,
            ScriptValue::Integer(_) | ScriptValue::String(_) | ScriptValue::Reference(_)
        )
    }
}

#[profiling::all_functions]
impl FromScript for ReflectReference {
    type This<'w> = Self;
    #[profiling::function]
    fn from_script(value: ScriptValue, _scope: &CallScope) -> Result<Self, InteropError> {
        match value {
            ScriptValue::Reference(r) => Ok(r),
            _ => Err(InteropError::value_mismatch(
                std::any::TypeId::of::<ReflectReference>(),
                value,
            )),
        }
    }

    fn could_convert(value: &ScriptValue) -> bool {
        matches!(value, ScriptValue::Reference(_))
    }
}

fn variant_as_value<T: 'static>(
    reference: &ReflectReference,
    world: &WorldGuard,
) -> Option<ReflectReference> {
    variant_as_value_of(reference, world, TypeId::of::<T>())
}

/// [`variant_as_value`] for a type known only by its [`TypeId`].
pub(crate) fn variant_as_value_of(
    reference: &ReflectReference,
    world: &WorldGuard,
    type_id: TypeId,
) -> Option<ReflectReference> {
    let variant = reference.downcast::<EnumVariantRef>(world.clone()).ok()?;
    if variant.type_id != type_id {
        return None;
    }
    construct_reference(
        world,
        variant.type_info(world).ok()?,
        Some(variant.variant_name(world).ok()?),
        BoundFields::Positional(VecDeque::new()),
        false,
    )
    .ok()
}

/// A wrapper around a value of type `T`.
///
/// This can be used to retrieve a value out of a [`ScriptValue::Reference`] corresponding to the type `T`.
/// You can also use this to return values from a script function to be allocated directly as a [`ScriptValue::Reference`].
#[derive(Reflect)]
pub struct V<T>(pub T);

#[profiling::all_functions]
impl<T> V<T> {
    /// Create a new `Val` with the given value.
    pub fn new(value: T) -> Self {
        V(value)
    }

    /// Unwrap the value from the `Val`.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for V<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for V<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<T> for V<T> {
    fn from(value: T) -> Self {
        V(value)
    }
}

#[profiling::all_functions]
impl<T: FromReflect> FromScript for V<T> {
    type This<'w> = Self;
    #[profiling::function]
    fn from_script(value: ScriptValue, scope: &CallScope) -> Result<Self, InteropError> {
        let world = scope.world()?;
        match value {
            ScriptValue::Reference(reflect_reference) => {
                let from_reflect = |reference: &ReflectReference| {
                    reference.with_reflect(world.clone(), |r| {
                        T::from_reflect(r).ok_or_else(|| {
                            InteropError::failed_from_reflect(
                                Some(TypeId::of::<T>()),
                                format!(
                                    "Expected '{}' but receievd: {r:?}",
                                    std::any::type_name::<T>()
                                ),
                            )
                        })
                    })?
                };
                match from_reflect(&reflect_reference) {
                    Ok(value) => Ok(V(value)),
                    Err(err) => match variant_as_value::<T>(&reflect_reference, &world) {
                        Some(value) => from_reflect(&value).map(V),
                        None => Err(err),
                    },
                }
            }
            _ => Err(InteropError::value_mismatch(
                std::any::TypeId::of::<T>(),
                value,
            )),
        }
    }

    fn could_convert(value: &ScriptValue) -> bool {
        matches!(value, ScriptValue::Reference(_))
    }

    fn from_arg<'s>(
        args: &dyn ArgSource,
        index: usize,
        scope: &'s CallScope<'_>,
    ) -> Result<Self, InteropError> {
        if let Some((base, src)) = native_arg::<T>(args, index, scope, false)? {
            let Some(base) = base else {
                // Safety: a plain value, of a `Copy` type (see `ArgSource::plain`), borrowed while it's copied
                return Ok(V(unsafe { std::ptr::read(src) }));
            };
            let mut value = std::mem::MaybeUninit::<T>::uninit();
            // Safety: `src` is a T (in the storage, or in the world for a place), borrowed so nothing changes it while it's copied
            unsafe {
                NativeStorage::clone_value(base, src as *const u8, value.as_mut_ptr() as *mut u8);
                return Ok(V(value.assume_init()));
            }
        }
        Self::from_script(args.script_value(index)?, scope)
    }

    fn accepts_arg(args: &dyn ArgSource, index: usize) -> bool {
        is_native_arg::<T>(args, index)
    }
}

/// A wrapper around a reference to a value of type `T`.
///
/// This can be used to retrieve a reference out of a [`ScriptValue::Reference`] corresponding to the type `T`.
/// Before downcasting the reference, it will claim write access to the object to ensure that the reference is valid.
///
/// The access is released when the [`CallScope`] it was converted in ends, see [`M`].
pub struct R<'w, T>(pub &'w T);

impl<T> Deref for R<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[profiling::all_functions]
impl<T: FromReflect> FromScript for R<'_, T> {
    type This<'a> = R<'a, T>;
    #[profiling::function]
    fn from_script<'s>(
        value: ScriptValue,
        scope: &'s CallScope<'_>,
    ) -> Result<Self::This<'s>, InteropError> {
        match value {
            ScriptValue::Reference(reflect_reference) => {
                let world = scope.world()?;
                // the claim keeps an allocated value alive, the borrow outlives `reflect_reference`
                // and is released when `scope` ends, which the borrow can't outlive
                if scope.claim_read_access(&reflect_reference.base.base_id) {
                    // Safety: we just claimed access
                    let ref_ =
                        unsafe { reflect_reference.reflect_unsafe_non_empty(world.clone()) }?;
                    match ref_.try_downcast_ref::<T>() {
                        Some(cast) => Ok(R(cast)),
                        None => match variant_as_value::<T>(&reflect_reference, &world) {
                            Some(value) => Self::from_script(ScriptValue::Reference(value), scope),
                            None => Err(InteropError::type_mismatch(
                                std::any::TypeId::of::<T>(),
                                ref_.get_represented_type_info().map(|i| i.type_id()),
                            )),
                        },
                    }
                } else {
                    Err(InteropError::cannot_claim_access(
                        (&reflect_reference.base.base_id).into(),
                        format!("In conversion to type: R<{}>", std::any::type_name::<T>()),
                    ))
                }
            }
            _ => Err(InteropError::value_mismatch(
                std::any::TypeId::of::<T>(),
                value,
            )),
        }
    }

    fn could_convert(value: &ScriptValue) -> bool {
        matches!(value, ScriptValue::Reference(_))
    }

    fn from_arg<'s>(
        args: &dyn ArgSource,
        index: usize,
        scope: &'s CallScope<'_>,
    ) -> Result<Self::This<'s>, InteropError> {
        if let Some((_, value)) = native_arg::<T>(args, index, scope, false)? {
            // Safety: a T, borrowed shared until the scope ends: in the storage, alive until the call returns, or in the world for a place, kept by the access claimed for it
            return Ok(R(unsafe { &*value }));
        }
        Self::from_script(args.script_value(index)?, scope)
    }

    fn accepts_arg(args: &dyn ArgSource, index: usize) -> bool {
        is_native_arg::<T>(args, index)
    }
}

impl<'a, T> From<&'a T> for R<'a, T> {
    fn from(value: &'a T) -> Self {
        R(value)
    }
}

/// A wrapper around a mutable reference to a value of type `T`.
///
/// This can be used to retrieve a mutable reference out of a [`ScriptValue::Reference`] corresponding to the type `T`.
/// Before downcasting the reference, it will claim write access to the object to ensure that the reference is valid.
///
/// The access is released when the [`CallScope`] it was converted in ends, i.e. when the function call returns,
/// which is why the borrow can't be kept past the call:
///
/// ```
/// # use bevy_mod_scripting_bindings::{conversions::from::M, function::script_function::ScriptFunction};
/// fn double(mut m: M<u32>) { *m.0 *= 2 }
/// let _ = double.into_dynamic_script_function();
/// ```
///
/// ```compile_fail
/// # use bevy_mod_scripting_bindings::{conversions::from::M, function::script_function::ScriptFunction};
/// fn stash(m: M<'static, u32>) {}
/// let _ = stash.into_dynamic_script_function();
/// ```
pub struct M<'w, T>(pub &'w mut T);

impl<T> Deref for M<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<T> DerefMut for M<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

impl<'a, T> From<&'a mut T> for M<'a, T> {
    fn from(value: &'a mut T) -> Self {
        M(value)
    }
}

#[profiling::all_functions]
impl<T: FromReflect> FromScript for M<'_, T> {
    type This<'w> = M<'w, T>;
    #[profiling::function]
    fn from_script<'s>(
        value: ScriptValue,
        scope: &'s CallScope<'_>,
    ) -> Result<Self::This<'s>, InteropError> {
        match value {
            ScriptValue::Reference(reflect_reference) => {
                // a function without a typed entry gets copies of native values, a change would be lost with the copy
                if let crate::ReflectBase::Owned(id) = &reflect_reference.base.base_id
                    && id.is_native_copy()
                {
                    return Err(InteropError::string(format!(
                        "a {} stored natively can't be changed by a function without a typed entry, register it as `Native`",
                        std::any::type_name::<T>()
                    )));
                }
                // the claim keeps an allocated value alive, the borrow outlives `reflect_reference`
                // and is released when `scope` ends, which the borrow can't outlive
                if scope.claim_write_access(&reflect_reference.base.base_id) {
                    // Safety: we just claimed write access
                    let ref_ =
                        unsafe { reflect_reference.reflect_mut_unsafe_non_empty(scope.world()?) }?;
                    let type_id = ref_.get_represented_type_info().map(|i| i.type_id());
                    let cast = ref_.try_downcast_mut::<T>().ok_or_else(|| {
                        InteropError::type_mismatch(std::any::TypeId::of::<T>(), type_id)
                    })?;
                    Ok(M(cast))
                } else {
                    Err(InteropError::cannot_claim_access(
                        (&reflect_reference.base.base_id).into(),
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

    fn could_convert(value: &ScriptValue) -> bool {
        matches!(value, ScriptValue::Reference(_))
    }

    fn from_arg<'s>(
        args: &dyn ArgSource,
        index: usize,
        scope: &'s CallScope<'_>,
    ) -> Result<Self::This<'s>, InteropError> {
        if let Some((_, value)) = native_arg::<T>(args, index, scope, true)? {
            // Safety: a T, borrowed exclusively until the scope ends: in the storage, alive until the call returns, or in the world for a place, kept by the access claimed for it
            return Ok(M(unsafe { &mut *value }));
        }
        Self::from_script(args.script_value(index)?, scope)
    }

    fn accepts_arg(args: &dyn ArgSource, index: usize) -> bool {
        is_native_arg::<T>(args, index)
    }
}

#[profiling::all_functions]
impl<T: FromScript> FromScript for Option<T> {
    type This<'w> = Option<T::This<'w>>;
    #[profiling::function]
    fn from_script<'s>(
        value: ScriptValue,
        scope: &'s CallScope<'_>,
    ) -> Result<Self::This<'s>, InteropError> {
        match value {
            ScriptValue::Unit => Ok(None),
            _ => Ok(Some(T::from_script(value, scope)?)),
        }
    }

    fn could_convert(value: &ScriptValue) -> bool {
        matches!(value, ScriptValue::Unit) || T::could_convert(value)
    }
}

#[profiling::all_functions]
impl<T: FromScript + 'static> FromScript for Vec<T>
where
    for<'w> T::This<'w>: Into<T>,
{
    type This<'w> = Self;
    #[profiling::function]
    fn from_script(value: ScriptValue, scope: &CallScope) -> Result<Self, InteropError> {
        match value {
            ScriptValue::List(list) => {
                let mut vec = Vec::with_capacity(list.len());
                for item in list {
                    vec.push(T::from_script(item, scope)?.into());
                }
                Ok(vec)
            }
            _ => Err(InteropError::value_mismatch(
                std::any::TypeId::of::<Vec<T>>(),
                value,
            )),
        }
    }

    fn could_convert(value: &ScriptValue) -> bool {
        matches!(value, ScriptValue::List(_))
    }

    fn from_arg(
        args: &dyn ArgSource,
        index: usize,
        scope: &CallScope<'_>,
    ) -> Result<Self, InteropError> {
        // the items read as arguments, no script values
        let mut vec = None;
        if args.list(index, &mut |items| {
            vec = Some(
                (0..items.len())
                    .map(|i| T::from_arg(items, i, scope).map(Into::into))
                    .collect(),
            );
        }) && let Some(vec) = vec
        {
            return vec;
        }
        Self::from_script(args.script_value(index)?, scope)
    }
}

#[profiling::all_functions]
impl<T: FromScript + 'static> FromScript for VecDeque<T>
where
    for<'w> T::This<'w>: Into<T>,
{
    type This<'w> = Self;
    #[profiling::function]
    fn from_script(value: ScriptValue, scope: &CallScope) -> Result<Self, InteropError> {
        match value {
            ScriptValue::List(list) => {
                let mut vec = VecDeque::with_capacity(list.len());
                for item in list {
                    vec.push_back(T::from_script(item, scope)?.into());
                }
                Ok(vec)
            }
            _ => Err(InteropError::value_mismatch(
                std::any::TypeId::of::<VecDeque<T>>(),
                value,
            )),
        }
    }

    fn could_convert(value: &ScriptValue) -> bool {
        matches!(value, ScriptValue::List(_))
    }
}

#[profiling::all_functions]
impl<T: FromScript + 'static, const N: usize> FromScript for [T; N]
where
    for<'w> T::This<'w>: Into<T>,
{
    type This<'w> = Self;
    #[profiling::function]
    fn from_script(value: ScriptValue, scope: &CallScope) -> Result<Self, InteropError> {
        match value {
            ScriptValue::List(list) if list.len() == N => {
                let converted_list = list
                    .into_iter()
                    .map(|item| T::from_script(item, scope).map(Into::into))
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

    fn could_convert(value: &ScriptValue) -> bool {
        matches!(value, ScriptValue::List(_))
    }
}

#[profiling::all_functions]
impl FromScript for DynamicScriptFunctionMut {
    type This<'w> = Self;
    #[profiling::function]
    fn from_script<'s>(
        value: ScriptValue,
        _: &'s CallScope<'_>,
    ) -> Result<Self::This<'s>, InteropError>
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

    fn could_convert(value: &ScriptValue) -> bool {
        matches!(value, ScriptValue::FunctionMut(_))
    }
}

#[profiling::all_functions]
impl FromScript for DynamicScriptFunction {
    type This<'w> = Self;
    #[profiling::function]
    fn from_script<'s>(
        value: ScriptValue,
        _: &'s CallScope<'_>,
    ) -> Result<Self::This<'s>, InteropError>
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

    fn could_convert(value: &ScriptValue) -> bool {
        matches!(value, ScriptValue::Function(_))
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
            fn from_script(value: ScriptValue, scope: &CallScope) -> Result<Self, InteropError> {
                match value {
                    ScriptValue::Map(map) => {
                        let mut hashmap = <$hashmap_type>::new();
                        for (key, value) in map {
                            hashmap.insert(key, V::from_script(value, scope)?.into());
                        }
                        Ok(hashmap)
                    }
                    ScriptValue::List(list) => {
                        let mut hashmap = <$hashmap_type>::new();
                        for elem in list {
                            let (key, val) = <(String, V)>::from_script(elem, scope)?;
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

            fn could_convert(value: &ScriptValue) -> bool {
                matches!(value, ScriptValue::Map(_) | ScriptValue::List(_))
            }

            fn from_arg(
                args: &dyn ArgSource,
                index: usize,
                scope: &CallScope<'_>,
            ) -> Result<Self, InteropError> {
                // the entries read as arguments, no script values
                let mut hashmap = <$hashmap_type>::new();
                let read = args.map(index, &mut |entry| {
                    let key = String::from_arg(entry, 0, scope)?;
                    hashmap.insert(key, V::from_arg(entry, 1, scope)?.into());
                    Ok(())
                });
                match read {
                    Some(read) => read.map(|()| hashmap),
                    // not a map with string keys, or no fast way: as before
                    None => Self::from_script(args.script_value(index)?, scope),
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
            fn from_script(value: ScriptValue, scope: &CallScope) -> Result<Self, InteropError> {
                match value {
                    ScriptValue::Map(map) => {
                        let mut hashmap = <$hashset_type>::new();
                        for (_, value) in map {
                            hashmap.insert(V::from_script(value, scope)?.into());
                        }
                        Ok(hashmap)
                    }
                    ScriptValue::List(list) => {
                        let mut hashmap = <$hashset_type>::new();
                        for elem in list {
                            let key = <V>::from_script(elem, scope)?;
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

            fn could_convert(value: &ScriptValue) -> bool {
                matches!(value, ScriptValue::Map(_) | ScriptValue::List(_))
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

    /// Returns true if this union represents the left kind of value
    pub fn is_left(&self) -> bool {
        matches!(self, Union(Ok(_)))
    }

    /// Returns true if this union represents the right kind of value
    pub fn is_right(&self) -> bool {
        matches!(self, Union(Err(_)))
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
impl<T1: FromScript, T2: FromScript> FromScript for Union<T1, T2> {
    type This<'w> = Union<T1::This<'w>, T2::This<'w>>;
    fn from_script<'s>(
        value: ScriptValue,
        scope: &'s CallScope<'_>,
    ) -> Result<Self::This<'s>, InteropError> {
        // only types which could convert the value are tried, and it is only copied to try both
        match (T1::could_convert(&value), T2::could_convert(&value)) {
            (true, true) => {
                if let Ok(v) = T1::from_script(value.clone(), scope) {
                    return Ok(Union(Ok(v)));
                }
                T2::from_script(value, scope).map(|v| Union(Err(v)))
            }
            (true, false) => T1::from_script(value, scope).map(|v| Union(Ok(v))),
            (false, _) => T2::from_script(value, scope).map(|v| Union(Err(v))),
        }
    }

    fn could_convert(value: &ScriptValue) -> bool {
        T1::could_convert(value) || T2::could_convert(value)
    }

    fn from_arg<'s>(
        args: &dyn ArgSource,
        index: usize,
        scope: &'s CallScope<'_>,
    ) -> Result<Self::This<'s>, InteropError> {
        // a type which takes the argument as it is, i.e. a native value or a number, without converting it
        if T1::accepts_arg(args, index) {
            return T1::from_arg(args, index, scope).map(|v| Union(Ok(v)));
        }
        if T2::accepts_arg(args, index) {
            return T2::from_arg(args, index, scope).map(|v| Union(Err(v)));
        }
        Self::from_script(args.script_value(index)?, scope)
    }

    fn accepts_arg(args: &dyn ArgSource, index: usize) -> bool {
        T1::accepts_arg(args, index) || T2::accepts_arg(args, index)
    }
}

impl FromScript for VariadicTuple {
    type This<'w> = Self;

    fn from_script<'s>(
        value: ScriptValue,
        _scope: &'s CallScope<'_>,
    ) -> Result<Self::This<'s>, InteropError>
    where
        Self: Sized,
    {
        match value {
            ScriptValue::List(tuple) | ScriptValue::Tuple(VariadicTuple(tuple)) => Ok(Self(tuple)),
            v => Ok(Self(VecDeque::from_iter([v]))),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{CurrentScriptAttachment, WorldExtensions};
    use bevy_ecs::world::World;
    use bevy_mod_scripting_world::WorldAccessGuard;

    #[test]
    fn non_max_u32_rejects_out_of_range_values() {
        let mut world = World::default();
        let cache = WorldAccessGuard::setup_cache(&world, CurrentScriptAttachment::empty());
        let guard = WorldAccessGuard::new_exclusive(&mut world, cache);
        guard.with_call_scope(|scope| {
            let convert = |v| {
                NonMaxU32::from_script(v, scope)
                    .ok()
                    .map(|v: NonMaxU32| v.get())
            };
            assert_eq!(convert(ScriptValue::Integer(0)), Some(0));
            assert_eq!(convert(ScriptValue::Float(7.0)), Some(7));
            for invalid in [
                ScriptValue::Integer(-1),
                ScriptValue::Integer(u32::MAX as i64),
                ScriptValue::Integer(1 << 40),
                ScriptValue::Float(-1.0),
                ScriptValue::Float(u32::MAX as f64),
                ScriptValue::Float(f64::NAN),
            ] {
                assert_eq!(convert(invalid.clone()), None, "{invalid:?}");
            }
        });
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

            fn from_script(value: ScriptValue, scope: &CallScope) -> Result<Self, InteropError> {
                match value {
                    ScriptValue::List(list) | ScriptValue::Tuple(VariadicTuple(list)) => {
                        let expected_arg_count = $crate::function::script_function::count!( $($ty)* );
                        if list.len() != expected_arg_count {
                            return Err(InteropError::length_mismatch(expected_arg_count, list.len()));
                        }

                        let mut iter = list.into_iter();
                        $(
                            let next_item = iter.next().ok_or_else(|| InteropError::invariant("list has right amount of elements"))?;
                            let $ty = $ty::from_script(next_item, scope)?.into();
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
