//! References to a variant of an enum, such as `KeyCode.KeyA` in scripts.

use crate::{InteropError, ScriptTypeRegistration};
use bevy_reflect::{PartialReflect, Reflect, ReflectRef, TypeInfo, enums::VariantInfo};

/// A reference to a variant of an enum, independent of any value of that enum.
///
/// This is what scripts see when accessing `MyEnum.Variant`, it can be compared to
/// the result of `value:variant()` and used to construct values of the variant.
#[derive(Clone, Reflect, Debug)]
pub struct EnumVariantRef {
    /// The enum this is a variant of
    pub registration: ScriptTypeRegistration,
    /// The index of the variant in the enum
    pub variant_index: usize,
}

impl PartialEq for EnumVariantRef {
    fn eq(&self, other: &Self) -> bool {
        self.registration.type_id() == other.registration.type_id()
            && self.variant_index == other.variant_index
    }
}

impl EnumVariantRef {
    /// Finds the variant with the given name, if the type is an enum with such a variant.
    pub fn from_name(registration: ScriptTypeRegistration, name: &str) -> Option<Self> {
        let variant_index = match registration.registration.type_info() {
            TypeInfo::Enum(info) => info.index_of(name)?,
            _ => return None,
        };
        Some(Self {
            registration,
            variant_index,
        })
    }

    /// The variant a value is currently in, if the value is an enum.
    pub fn from_value(
        registration: ScriptTypeRegistration,
        value: &dyn PartialReflect,
    ) -> Option<Self> {
        match value.reflect_ref() {
            ReflectRef::Enum(e) => Some(Self {
                registration,
                variant_index: e.variant_index(),
            }),
            _ => None,
        }
    }

    /// Information about the variant
    pub fn info(&self) -> Result<&'static VariantInfo, InteropError> {
        match self.registration.registration.type_info() {
            TypeInfo::Enum(info) => info.variant_at(self.variant_index),
            _ => None,
        }
        .ok_or_else(|| {
            InteropError::invariant("enum variant reference points to a missing variant")
        })
    }

    /// The name of the variant
    pub fn variant_name(&self) -> Result<&'static str, InteropError> {
        self.info().map(|info| info.name())
    }
}
