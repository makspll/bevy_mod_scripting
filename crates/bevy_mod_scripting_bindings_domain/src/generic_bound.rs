use bevy_mod_scripting_derive::DebugWithTypeInfo;

/// A bound on the generic parameter of a script function, i.e. `A: Component` for `Transform.get_on(entity)`.
///
/// Functions shared by all types are only available on the types satisfying their bounds.
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize, DebugWithTypeInfo,
)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
#[serde(rename_all = "camelCase")]
pub enum GenericBound {
    /// A component type, with `ReflectComponent` type data
    Component,
    /// A resource type, with `ReflectResource` type data
    Resource,
    /// A bundle type, with `ReflectBundle` type data, or a component type: every component is a bundle
    Bundle,
    /// An asset type, with `ReflectAsset` type data
    Asset,
}
