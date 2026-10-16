//! Argument and return types for functions shared between many types, whose signature depends on the type they are bound to.
//!
//! These are documented with [`BoundPlaceholder`]s, which are replaced with the actual type once the function is bound,
//! see [`crate::docgen::info::FunctionInfo::bind_to_type`].

use std::{any::TypeId, collections::VecDeque};

use bevy_mod_scripting_world::WorldGuard;
use bevy_platform::collections::HashMap;
use bevy_reflect::TypeRegistry;

use super::{
    arg_meta::ArgMeta, from::FromScript, into::IntoScript, type_dependencies::GetTypeDependencies,
};
use crate::{
    ReflectReference, ScriptValue,
    docgen::typed_through::{BoundPlaceholder, ThroughTypeInfo, TypedThrough},
    error::InteropError,
};

/// A value of the type a shared function is bound to.
pub struct BoundValue(pub ReflectReference);

impl IntoScript for BoundValue {
    fn into_script(self, world: WorldGuard) -> Result<ScriptValue, InteropError> {
        self.0.into_script(world)
    }
}

impl TypedThrough for BoundValue {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::Bound(BoundPlaceholder::Value)
    }
}

impl FromScript for BoundValue {
    type This<'w> = Self;

    fn from_script(value: ScriptValue, world: WorldGuard) -> Result<Self, InteropError> {
        ReflectReference::from_script(value, world).map(BoundValue)
    }
}

impl ArgMeta for BoundValue {}

impl GetTypeDependencies for BoundValue {
    type Underlying = ReflectReference;
    fn register_type_dependencies(_registry: &mut TypeRegistry) {}
}

/// A handle to an asset of the type a shared function is bound to, i.e. `Handle<Image>` for `Image`.
pub struct BoundHandle(pub ReflectReference);

impl IntoScript for BoundHandle {
    fn into_script(self, world: WorldGuard) -> Result<ScriptValue, InteropError> {
        self.0.into_script(world)
    }
}

impl FromScript for BoundHandle {
    type This<'w> = Self;

    fn from_script(value: ScriptValue, world: WorldGuard) -> Result<Self, InteropError> {
        ReflectReference::from_script(value, world).map(BoundHandle)
    }
}

impl ArgMeta for BoundHandle {}

impl TypedThrough for BoundHandle {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::Bound(BoundPlaceholder::Handle)
    }
}

impl GetTypeDependencies for BoundHandle {
    type Underlying = ReflectReference;
    fn register_type_dependencies(_registry: &mut TypeRegistry) {}
}

/// Whether the type is registered as an asset, i.e. with `App::register_asset_reflect`.
pub fn is_asset_type(registry: &TypeRegistry, type_id: TypeId) -> bool {
    registry
        .get_type_data::<bevy_asset::ReflectAsset>(type_id)
        .is_some()
}

/// The registered `Handle<T>` type for the asset type `T`, if `T` is registered as an asset.
pub fn asset_handle_type_info(
    registry: &TypeRegistry,
    asset_type_id: TypeId,
) -> Option<&'static bevy_reflect::TypeInfo> {
    registry
        .get_type_data::<bevy_asset::ReflectAsset>(asset_type_id)
        .and_then(|asset| registry.get(asset.handle_type_id()))
        .map(|registration| registration.type_info())
}

/// A table of the fields of the type a shared function is bound to.
///
/// Fields are keyed by name, or for tuple-like types by their index.
/// A list is taken to be the fields in order, i.e. `{ 1, "a" }` is the same as `{ ["1"] = 1, ["2"] = "a" }` in Lua.
pub enum BoundFields {
    /// Fields keyed by name or index
    Keyed(HashMap<String, ScriptValue>),
    /// Fields in order
    Positional(VecDeque<ScriptValue>),
}

impl BoundFields {
    /// Converts the fields into a map keyed by name or index, with indices following the given convention.
    pub fn into_map(self, one_indexed: bool) -> HashMap<String, ScriptValue> {
        match self {
            BoundFields::Keyed(map) => map,
            BoundFields::Positional(list) => list
                .into_iter()
                .enumerate()
                .map(|(idx, value)| ((idx + one_indexed as usize).to_string(), value))
                .collect(),
        }
    }
}

impl FromScript for BoundFields {
    type This<'w> = Self;

    fn from_script(value: ScriptValue, _world: WorldGuard) -> Result<Self, InteropError> {
        match value {
            ScriptValue::Map(map) => Ok(BoundFields::Keyed(map)),
            ScriptValue::List(list) => Ok(BoundFields::Positional(list)),
            value => Err(InteropError::value_mismatch(
                TypeId::of::<HashMap<String, ScriptValue>>(),
                value,
            )),
        }
    }
}

impl ArgMeta for BoundFields {}

impl TypedThrough for BoundFields {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::Bound(BoundPlaceholder::Fields)
    }
}

impl GetTypeDependencies for BoundFields {
    type Underlying = Self;
    fn register_type_dependencies(_registry: &mut TypeRegistry) {}
}
