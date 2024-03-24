use bevy::prelude::*;
use parking_lot::RwLock;
use std::fmt::Debug;
use std::{borrow::Cow, sync::Weak};

use bevy_mod_scripting_core::world::WorldPointer;

use crate::{
    error::ReflectionError,
    sub_reflect::{ReflectBase, ReflectionPath, ReflectionPathElement},
};

/// A reference to a rust type available from some script language.
/// References can be either to rust or script managed values (created either on the bevy or script side).
/// but also to any subfield of those values (All pointed to values must support `reflect`).
/// Each reference holds a reflection path from the root.
///
/// Automatically converts to the most convenient lua representation.
/// See [`ReflectReference::to_lua`]
#[derive(Clone, Debug)]
pub struct ReflectReference {
    /// The reflection path from the root
    pub(crate) path: ReflectionPath,
    pub(crate) world_ptr: WorldPointer,
}

impl ReflectReference {
    /// Safely creates a new base component reference
    pub fn new_component_ref(
        comp: ReflectComponent,
        entity: Entity,
        world_ptr: WorldPointer,
    ) -> Self {
        Self {
            path: ReflectionPath::new(ReflectBase::Component { comp, entity }),
            world_ptr,
        }
    }

    pub fn new_resource_ref(res: ReflectResource, world_ptr: WorldPointer) -> Self {
        Self {
            path: ReflectionPath::new(ReflectBase::Resource { res }),
            world_ptr,
        }
    }

    /// Creates a reference to a script owned value
    pub fn new_script_ref<T: Reflect>(ptr: Weak<RwLock<T>>, world_ptr: WorldPointer) -> Self {
        Self {
            path: ReflectionPath::new(ReflectBase::ScriptOwned { val: ptr }),
            world_ptr,
        }
    }

    /// Creates a new script reference which points to a sub component of the original data,
    /// This also updates the pointer
    pub(crate) fn sub_ref(&self, elem: ReflectionPathElement) -> ReflectReference {
        let path = self.path.new_sub(elem);

        Self {
            path,
            ..self.clone()
        }
    }

    /// Retrieves the underlying `dyn Reflect` reference and applies function which can retrieve a value.
    /// Panics if the reference is invalid or world is already borrowed mutably.
    #[inline(always)]
    pub fn get<O, F>(&self, f: F) -> Result<O, ReflectionError>
    where
        F: FnOnce(&dyn Reflect) -> O,
    {
        self.path.get(self.world_ptr.clone(), f)
    }

    pub fn get_typed<T, O, F>(&self, f: F) -> Result<O, ReflectionError>
    where
        F: FnOnce(&T) -> O,
        T: Reflect,
    {
        self.path.get(self.world_ptr.clone(), |reflect| {
            (f)(reflect.downcast_ref::<T>().unwrap_or_else(|| {
                panic!(
                    "Expected `{}` found `{}`",
                    ::std::any::type_name::<T>(),
                    reflect.get_represented_type_info().unwrap().type_path()
                )
            }))
        })
    }

    /// Retrieves the underlying `dyn Reflect` reference and applies function which can retrieve a value.
    /// If this is a component it is marked as changed.
    /// Panics if the reference is invalid or if the world/value is already borrowed or if r is not a mutable pointer.
    #[inline(always)]
    pub fn get_mut<O, F>(&mut self, f: F) -> Result<O, ReflectionError>
    where
        F: FnOnce(&mut dyn Reflect) -> O,
    {
        self.path.get_mut(self.world_ptr.clone(), f)
    }

    pub fn get_mut_typed<T, O, F>(&mut self, f: F) -> Result<O, ReflectionError>
    where
        F: FnOnce(&mut T) -> O,
        T: Reflect,
    {
        self.path.get_mut(self.world_ptr.clone(), |reflect| {
            (f)(reflect.downcast_mut().unwrap())
        })
    }

    /// applies another [`ReflectReference`] to self by carefuly acquiring locks and cloning if necessary.
    ///
    /// This is semantically equivalent to the [`Reflect::apply`] method.
    /// If you know the type of this value use [`Self::apply_luaref_typed`] since it avoids double cloning and allocating
    pub fn apply(&mut self, other: &ReflectReference) -> Result<(), ReflectionError> {
        // sadly apply already performs a clone for value types, so this incurs
        // a double clone in some cases TODO: is there another way ?
        // can we avoid the box ?
        let cloned = other.get(|s| s.clone_value())?;

        self.get_mut(|s| s.apply(&*cloned))
    }

    /// Unlike apply this method expects the other type to be identical. Does not allocate so is likely to be faster than apply, uses direct assignment.
    /// If you have a concrete value use [`Self::set_val`](TypedReflectReference) unstead
    pub fn set<T>(&mut self, other: &Self) -> Result<(), ReflectionError>
    where
        T: Reflect + Clone,
    {
        let other: T = other.get_typed(|s: &T| s.clone())?;
        self.get_mut_typed(|s| *s = other)
    }

    /// Version of [`Self::set`](TypedReflectReference) which directly accepts a `T` value
    pub fn set_val<T>(&mut self, other: T) -> Result<(), ReflectionError>
    where
        T: Reflect,
    {
        self.get_mut_typed(|s| *s = other)
    }
}

/// A version of index for returning values instead of references
pub trait ValueIndex<Idx> {
    type Output;

    fn index(&self, index: Idx) -> Self::Output;
}

impl ValueIndex<usize> for ReflectReference {
    type Output = Self;

    fn index(&self, index: usize) -> Self::Output {
        self.sub_ref(ReflectionPathElement::IndexAccess(index))
    }
}

impl ValueIndex<Cow<'static, str>> for ReflectReference {
    type Output = Self;

    fn index(&self, index: Cow<'static, str>) -> Self::Output {
        self.sub_ref(ReflectionPathElement::FieldAccess(index))
    }
}

/// A value representing a type which has no special UserData implementation,
/// It exposes the much less convenient reflect interface of the underlying type.
#[derive(Clone, Debug)]
pub struct ReflectedValue {
    pub(crate) ref_: ReflectReference,
}

impl From<ReflectedValue> for ReflectReference {
    fn from(ref_: ReflectedValue) -> Self {
        ref_.ref_
    }
}
