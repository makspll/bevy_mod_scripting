//! References to a variant of an enum, such as `KeyCode.KeyA` in scripts.

use std::any::TypeId;

use crate::{InteropError, static_type::registered_type_info};
use bevy_mod_scripting_world::WorldGuard;
use bevy_reflect::{PartialReflect, Reflect, ReflectRef, TypeInfo, enums::VariantInfo};

/// A reference to a variant of an enum, independent of any value of that enum.
///
/// This is what scripts see when accessing `MyEnum.Variant`, it can be compared to
/// the result of `value:variant()` and used to construct values of the variant.
#[derive(Clone, Copy, Reflect, Debug, PartialEq)]
pub struct EnumVariantRef {
    /// The enum this is a variant of
    pub type_id: TypeId,
    /// The index of the variant in the enum
    pub variant_index: usize,
}

impl EnumVariantRef {
    /// Finds the variant with the given name, if the type is an enum with such a variant.
    pub fn from_name(type_info: &'static TypeInfo, name: &str) -> Option<Self> {
        let variant_index = match type_info {
            TypeInfo::Enum(info) => info.index_of(name)?,
            _ => return None,
        };
        Some(Self {
            type_id: type_info.type_id(),
            variant_index,
        })
    }

    /// The variant a value is currently in, if the value is an enum.
    pub fn from_value(type_id: TypeId, value: &dyn PartialReflect) -> Option<Self> {
        match value.reflect_ref() {
            ReflectRef::Enum(e) => Some(Self {
                type_id,
                variant_index: e.variant_index(),
            }),
            _ => None,
        }
    }

    /// The type information of the enum
    pub fn type_info(&self, world: &WorldGuard) -> Result<&'static TypeInfo, InteropError> {
        registered_type_info(world, self.type_id)
    }

    /// Information about the variant
    pub fn info(&self, world: &WorldGuard) -> Result<&'static VariantInfo, InteropError> {
        match self.type_info(world)? {
            TypeInfo::Enum(info) => info.variant_at(self.variant_index),
            _ => None,
        }
        .ok_or_else(|| {
            InteropError::invariant("enum variant reference points to a missing variant")
        })
    }

    /// The name of the variant
    pub fn variant_name(&self, world: &WorldGuard) -> Result<&'static str, InteropError> {
        self.info(world).map(|info| info.name())
    }
}
