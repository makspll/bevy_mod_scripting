//! The kinds of types functions can be shared by, i.e. component types, as the generic types documentation uses for them.
//!
//! A function taking `ComponentType<A>` is shared by all component types, and called on one of them, i.e. `Transform.get_on(entity)`.

/// The generic type of any type, i.e. `import_namespace("math").Vec3`
pub const TYPE: &str = "Type";
/// The generic type of component types
pub const COMPONENT_TYPE: &str = "ComponentType";
/// The generic type of resource types
pub const RESOURCE_TYPE: &str = "ResourceType";
/// The generic type of bundle types
pub const BUNDLE_TYPE: &str = "BundleType";
/// The generic type of asset types
pub const ASSET_TYPE: &str = "AssetType";

/// Every kind, the broadest first.
pub const ALL: [&str; 5] = [TYPE, COMPONENT_TYPE, RESOURCE_TYPE, BUNDLE_TYPE, ASSET_TYPE];
