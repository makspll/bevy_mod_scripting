//! The static reference to a type, as accessed through a type's global in scripts, i.e. `Vec3` in `Vec3.new{}`.

use std::{any::TypeId, sync::Arc};

use crate::{FromScript, InteropError, IntoScript, ScriptTypeRegistration, ScriptValue};
use bevy_mod_scripting_world::WorldGuard;
use bevy_reflect::{Reflect, TypeInfo};

/// A static reference to a type, what scripts see as the type's global.
///
/// Functions registered on this type are available on every type's static reference, i.e. `MyType.function(..)`.
/// When called this way, the first argument is the [`StaticTypeRef`] of the type, and is filled in automatically.
///
/// Functions registered on the type itself take priority over the ones registered here.
#[derive(Clone, Copy, Reflect, Debug, PartialEq)]
pub struct StaticTypeRef {
    /// The type this reference points to
    pub type_id: TypeId,
}

impl StaticTypeRef {
    /// Creates a static reference to the type with the given id
    pub fn new(type_id: TypeId) -> Self {
        Self { type_id }
    }

    /// The type information of the referenced type
    pub fn type_info(&self, world: &WorldGuard) -> Result<&'static TypeInfo, InteropError> {
        registered_type_info(world, self.type_id)
    }

    /// The registration of the referenced type
    pub fn registration(&self, world: &WorldGuard) -> Result<ScriptTypeRegistration, InteropError> {
        let registry = world.type_registry();
        let registry = registry.read();
        registry
            .get(self.type_id)
            .map(|registration| ScriptTypeRegistration::new(Arc::new(registration.clone())))
            .ok_or_else(|| {
                InteropError::string(format!("type with id {:?} is not registered", self.type_id))
            })
    }
}

impl FromScript for StaticTypeRef {
    type This<'w> = Self;

    fn from_script(value: ScriptValue, _world: WorldGuard) -> Result<Self, InteropError> {
        match value {
            ScriptValue::Type(type_id) => Ok(Self::new(type_id)),
            value => Err(InteropError::value_mismatch(
                TypeId::of::<StaticTypeRef>(),
                value,
            )),
        }
    }
}

impl IntoScript for StaticTypeRef {
    fn into_script(self, _world: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(ScriptValue::Type(self.type_id))
    }
}

/// Finds the type information of the registered type with the given id.
pub fn registered_type_info(
    world: &WorldGuard,
    type_id: TypeId,
) -> Result<&'static TypeInfo, InteropError> {
    let registry = world.type_registry();
    let registry = registry.read();
    registry
        .get_type_info(type_id)
        .ok_or_else(|| InteropError::string(format!("type with id {type_id:?} is not registered")))
}
