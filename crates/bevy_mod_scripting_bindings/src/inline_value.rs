//! Small values passed to scripts without allocating, see [`InlineValue`].
//!
//! Values returned to scripts by value ([`crate::V`]) are owned by the script. Instead of allocating them,
//! small values without drop glue are stored inline in the [`crate::ScriptValue`], decided at compile time by [`is_inline`],
//! so no type registry lookups are needed.
//!
//! This is only a storage optimization, scripts can't tell inline values from allocated ones:
//! language implementations hold the inline value in their wrapper and promote it in place to an allocation
//! ([`InlineValue::into_reference`]) before any operation which needs a place, i.e. accessing or assigning a field
//! or calling a function which mutates it. Reads of fields and elements always return references into the place.
use std::{
    any::TypeId,
    mem::{MaybeUninit, align_of, needs_drop, size_of},
};

use bevy_mod_scripting_display::DebugWithTypeInfo;
use bevy_mod_scripting_world::WorldAccessGuard;
use bevy_reflect::{PartialReflect, Reflect};

use crate::ReflectReference;

/// The maximum size of values stored inline, in bytes.
pub const INLINE_CAPACITY: usize = 16;

/// Whether values of the type are stored inline when passed to scripts by value, see [`InlineValue`].
///
/// Types with drop glue are never inline: copies of the bytes would be dropped more than once.
pub const fn is_inline<T>() -> bool {
    size_of::<T>() <= INLINE_CAPACITY && align_of::<T>() <= align_of::<u64>() && !needs_drop::<T>()
}

/// The operations on an inline value of a given type, monomorphized when the value is created.
struct InlineVTable {
    type_id: fn() -> TypeId,
    type_name: fn() -> &'static str,
    as_reflect: unsafe fn(*const ()) -> *const dyn Reflect,
    into_reference: unsafe fn(*const ()) -> ReflectReference,
}

unsafe fn as_reflect<T: Reflect>(data: *const ()) -> *const dyn Reflect {
    data as *const T as *const dyn Reflect
}

unsafe fn into_reference<T: Reflect>(data: *const ()) -> ReflectReference {
    // Safety: the caller ensures the data holds a `T`, which has no drop glue, so reading a copy is fine
    ReflectReference::new_allocated(unsafe { (data as *const T).read() })
}

trait InlineVTableOf: Reflect + Sized {
    const VTABLE: InlineVTable = InlineVTable {
        type_id: TypeId::of::<Self>,
        type_name: std::any::type_name::<Self>,
        as_reflect: as_reflect::<Self>,
        into_reference: into_reference::<Self>,
    };
}

impl<T: Reflect> InlineVTableOf for T {}

/// A small value without drop glue stored inline, with the operations on its type, see the [module docs](self).
#[derive(Clone, Copy)]
pub struct InlineValue {
    data: MaybeUninit<[u64; 2]>,
    vtable: &'static InlineVTable,
}

// Safety: only `Reflect` types are stored, which are `Send` and `Sync`
unsafe impl Send for InlineValue {}
// Safety: only `Reflect` types are stored, which are `Send` and `Sync`
unsafe impl Sync for InlineValue {}

impl InlineValue {
    /// Stores the value inline, or returns it if the type isn't [inline](is_inline).
    ///
    /// The check is a constant, the branch not taken is removed at compile time.
    #[inline]
    pub fn new<T: Reflect>(value: T) -> Result<Self, T> {
        if !is_inline::<T>() {
            return Err(value);
        }
        let mut data = MaybeUninit::<[u64; 2]>::uninit();
        // Safety: the type fits in and is at most as aligned as the data, checked above
        unsafe { (data.as_mut_ptr() as *mut T).write(value) };
        Ok(Self {
            data,
            vtable: &<T as InlineVTableOf>::VTABLE,
        })
    }

    /// The type of the value
    pub fn type_id(&self) -> TypeId {
        (self.vtable.type_id)()
    }

    /// The name of the type of the value
    pub fn type_name(&self) -> &'static str {
        (self.vtable.type_name)()
    }

    /// The value
    pub fn as_reflect(&self) -> &dyn Reflect {
        // Safety: the vtable belongs to the type of the data
        unsafe { &*(self.vtable.as_reflect)(self.data.as_ptr() as *const ()) }
    }

    /// The value as a partial reflect
    pub fn as_partial_reflect(&self) -> &dyn PartialReflect {
        self.as_reflect().as_partial_reflect()
    }

    /// A copy of the value if it's a `T`
    pub fn downcast<T: Reflect>(&self) -> Option<T> {
        // Safety: the data holds a `T`, which has no drop glue, so reading a copy is fine
        (self.type_id() == TypeId::of::<T>())
            .then(|| unsafe { (self.data.as_ptr() as *const T).read() })
    }

    /// Moves the value into an allocation, giving it a place, see the [module docs](self).
    pub fn into_reference(self) -> ReflectReference {
        // Safety: the vtable belongs to the type of the data
        unsafe { (self.vtable.into_reference)(self.data.as_ptr() as *const ()) }
    }
}

impl std::fmt::Debug for InlineValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_partial_reflect().debug(f)
    }
}

impl DebugWithTypeInfo for InlineValue {
    fn to_string_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _type_info_provider: Option<&WorldAccessGuard>,
    ) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl PartialEq for InlineValue {
    fn eq(&self, other: &Self) -> bool {
        self.type_id() == other.type_id()
            && self
                .as_partial_reflect()
                .reflect_partial_eq(other.as_partial_reflect())
                .unwrap_or(false)
    }
}

#[cfg(test)]
mod test {
    use bevy_ecs::entity::Entity;

    use super::*;

    #[derive(Reflect, Clone)]
    struct Big([u64; 3]);

    #[test]
    fn small_types_without_drop_glue_are_inline() {
        assert!(is_inline::<Entity>());
        assert!(is_inline::<[f32; 4]>());
        assert!(!is_inline::<Big>());
        assert!(!is_inline::<String>());
        assert!(InlineValue::new(Big([0; 3])).is_err());
    }

    #[test]
    fn inline_values_keep_their_value_and_type() {
        let entity = Entity::from_raw_u32(7).unwrap();
        let value = InlineValue::new(entity).unwrap_or_else(|_| panic!("entities are inline"));
        assert_eq!(value.type_id(), TypeId::of::<Entity>());
        assert_eq!(value.downcast::<Entity>(), Some(entity));
        assert_eq!(value.downcast::<u64>(), None);
        assert_eq!(value, value);
        assert_eq!(
            value.as_reflect().downcast_ref::<Entity>().copied(),
            Some(entity)
        );
        let reference = value.into_reference();
        assert_eq!(reference.base.type_id, TypeId::of::<Entity>());
    }
}
