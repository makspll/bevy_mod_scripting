//! Value types: small reflected types which scripts own copies of, rather than references to, see [`ReflectScriptValueType`].

use std::any::TypeId;

use bevy_mod_scripting_display::{DebugWithTypeInfo, DisplayWithTypeInfo};
use bevy_mod_scripting_world::{WorldAccessGuard, WorldGuard};
use bevy_reflect::{CreateTypeData, FromReflect, PartialReflect, Reflect};

use crate::{InteropError, ReflectReference};

/// Type data marking a type as a value type.
///
/// Values of it made for scripts, i.e. returned [`crate::V`]s, are copies owned by the script ([`crate::ScriptValue::Owned`]),
/// rather than references. Their fields and methods are reached without the world's access map.
///
/// Fields of components and resources stay references, i.e. `transform.translation.x = 1` still sets the component.
#[derive(Clone, Copy)]
pub struct ReflectScriptValueType {
    clone: fn(&dyn Reflect) -> Box<dyn Reflect>,
}

impl<T: Reflect + Clone> CreateTypeData<T> for ReflectScriptValueType {
    fn create_type_data(_input: ()) -> Self {
        Self {
            clone: clone_value::<T>,
        }
    }
}

fn clone_value<T: Reflect + Clone>(value: &dyn Reflect) -> Box<dyn Reflect> {
    match value.downcast_ref::<T>() {
        Some(value) => Box::new(value.clone()),
        None => unreachable!("only made for values of the type the type data was made for"),
    }
}

impl ReflectScriptValueType {
    /// The value type data of a type, if it is a value type.
    pub fn of(world: &WorldGuard, type_id: TypeId) -> Option<Self> {
        world
            .type_registry()
            .read()
            .get_type_data::<Self>(type_id)
            .copied()
    }

    /// Copies a value of this type.
    ///
    /// The value must be of the type this type data was made for.
    pub fn copy(&self, value: &dyn Reflect) -> OwnedValue {
        self.own((self.clone)(value))
    }

    /// Owns a value of this type.
    ///
    /// The value must be of the type this type data was made for.
    pub fn own(&self, value: Box<dyn Reflect>) -> OwnedValue {
        OwnedValue {
            value,
            clone: self.clone,
        }
    }
}

/// A value of a value type owned by a script, see [`ReflectScriptValueType`].
///
/// Cloning it copies the value.
pub struct OwnedValue {
    value: Box<dyn Reflect>,
    clone: fn(&dyn Reflect) -> Box<dyn Reflect>,
}

impl OwnedValue {
    /// Owns a value, which doesn't have to be registered as a value type.
    pub fn new<T: Reflect + Clone>(value: T) -> Self {
        Self {
            value: Box::new(value),
            clone: clone_value::<T>,
        }
    }

    /// The type of the value.
    pub fn type_id(&self) -> TypeId {
        self.value.as_any().type_id()
    }

    /// The value.
    pub fn as_reflect(&self) -> &dyn Reflect {
        &*self.value
    }

    /// The value, mutably.
    pub fn as_reflect_mut(&mut self) -> &mut dyn Reflect {
        &mut *self.value
    }

    /// The value as a `T`, converted with [`FromReflect`] if it is of another type.
    pub fn into_typed<T: FromReflect>(self) -> Result<Box<T>, InteropError> {
        match self.value.downcast::<T>() {
            Ok(value) => Ok(value),
            Err(value) => T::from_reflect(value.as_partial_reflect())
                .map(Box::new)
                .ok_or_else(|| {
                    InteropError::type_mismatch(TypeId::of::<T>(), Some(value.as_any().type_id()))
                }),
        }
    }

    /// The value as a `T`, if it is one.
    pub fn downcast<T: Reflect>(self) -> Result<Box<T>, InteropError> {
        self.value.downcast::<T>().map_err(|value| {
            InteropError::type_mismatch(TypeId::of::<T>(), Some(value.as_any().type_id()))
        })
    }

    /// The boxed value.
    pub fn into_partial_reflect(self) -> Box<dyn PartialReflect> {
        self.value.into_partial_reflect()
    }

    /// A reference to an allocation holding the value, for operations which only work on references.
    pub fn into_reference(self) -> ReflectReference {
        ReflectReference::new_allocated_boxed(self.value)
    }
}

impl Clone for OwnedValue {
    fn clone(&self) -> Self {
        Self {
            value: (self.clone)(&*self.value),
            clone: self.clone,
        }
    }
}

impl std::fmt::Debug for OwnedValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl DebugWithTypeInfo for OwnedValue {
    fn to_string_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _type_info_provider: Option<&WorldAccessGuard>,
    ) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl DisplayWithTypeInfo for OwnedValue {
    fn display_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _type_info_provider: Option<&WorldAccessGuard>,
    ) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
