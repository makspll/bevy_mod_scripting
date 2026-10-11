//! The static reference to a type, as accessed through a type's global in scripts, i.e. `Vec3` in `Vec3.new{}`.

use crate::ScriptTypeRegistration;
use bevy_reflect::Reflect;

/// A static reference to a type, what scripts see as the type's global.
///
/// Functions registered on this type are available on every type's static reference, i.e. `MyType.function(..)`.
/// When called this way, the first argument is the [`StaticTypeRef`] of the type, and is filled in automatically.
///
/// Functions registered on the type itself take priority over the ones registered here.
#[derive(Clone, Reflect, Debug)]
pub struct StaticTypeRef {
    /// The type this reference points to
    pub registration: ScriptTypeRegistration,
}

impl StaticTypeRef {
    /// Creates a static reference to the type with the given registration
    pub fn new(registration: ScriptTypeRegistration) -> Self {
        Self { registration }
    }
}

/// Finds the registration of the type with the given id.
pub fn get_registration(
    world: &bevy_mod_scripting_world::WorldGuard,
    type_id: std::any::TypeId,
) -> Result<ScriptTypeRegistration, crate::InteropError> {
    let registry = world.type_registry();
    let registry = registry.read();
    registry
        .get(type_id)
        .cloned()
        .map(|registration| ScriptTypeRegistration::new(std::sync::Arc::new(registration)))
        .ok_or_else(|| {
            crate::InteropError::string(format!("type with id {type_id:?} is not registered"))
        })
}
