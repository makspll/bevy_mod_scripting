use std::any::TypeId;

use bevy::reflect::PartialReflect;

use crate::{
    bindings::{ReflectReference, WorldGuard},
    error::InteropError,
    prelude::ScriptValue,
};

/// Converts from a [`ScriptValue`] to a value equivalent to the given [`TypeId`].
///
/// Type Erased version of [`super::from::FromScript`].
pub trait FromScriptRef {
    fn from_script_ref(
        target: TypeId,
        value: ScriptValue,
        world: WorldGuard,
    ) -> Result<Self, InteropError>
    where
        Self: Sized;
}

impl FromScriptRef for Box<dyn PartialReflect> {
    fn from_script_ref(
        target: TypeId,
        value: ScriptValue,
        world: WorldGuard,
    ) -> Result<Self, InteropError>
    where
        Self: Sized,
    {
        todo!()
    }
}
