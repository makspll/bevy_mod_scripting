//! Generic parameters of script function signatures.
//!
//! Reflection can't see the generic parameters of Rust functions, so signatures name them with marker types instead:
//! [`A`] is a value of the type the parameter stands for ([`ThroughTypeInfo::Param`]), [`TypeOf`] and its kind-bounded
//! variants ([`ComponentOf`], ...) the type itself, as scripts get it from `import_namespace("math").Vec3`,
//! documented as the generic type of its kind, i.e. `ComponentType<A>` (see [`type_kinds`]). For example
//! ```rust,ignore
//! fn get_on(ty: ComponentOf<A>, entity: V<Entity>) -> Result<Option<A>, InteropError>
//! ```
//! returns a value of the component type it is given. Backends document such functions as generic functions,
//! or with the parameters replaced by the type the function is bound to, see [`ThroughTypeInfo::bind_generics`].

use std::{any::TypeId, collections::VecDeque, marker::PhantomData, ops::Deref};

pub use bevy_mod_scripting_bindings_domain::type_kinds;
use bevy_mod_scripting_world::WorldGuard;
use bevy_platform::collections::HashMap;
use bevy_reflect::TypeRegistry;

use super::{
    arg_meta::ArgMeta, from::FromScript, into::IntoScript, type_dependencies::GetTypeDependencies,
};
use crate::{
    ReflectReference, ScriptValue, StaticTypeRef,
    docgen::typed_through::{ThroughTypeInfo, TypedThrough},
    error::InteropError,
};

/// A generic parameter of a script function signature, named in the documentation by [`Self::NAME`].
pub trait GenericParam: 'static {
    /// The name of the parameter, i.e. `"A"`
    const NAME: &'static str;
}

/// A value of the type the generic parameter `A` stands for.
pub struct A(pub ReflectReference);

impl GenericParam for A {
    const NAME: &'static str = "A";
}

impl IntoScript for A {
    fn into_script(self, world: WorldGuard) -> Result<ScriptValue, InteropError> {
        self.0.into_script(world)
    }
}

impl FromScript for A {
    type This<'w> = Self;

    fn from_script(value: ScriptValue, world: WorldGuard) -> Result<Self, InteropError> {
        ReflectReference::from_script(value, world).map(A)
    }
}

impl TypedThrough for A {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::Param(Self::NAME)
    }
}

impl ArgMeta for A {}

impl GetTypeDependencies for A {
    type Underlying = ReflectReference;
    fn register_type_dependencies(_registry: &mut TypeRegistry) {}
}

/// A handle to an asset of the type the generic parameter `P` stands for, i.e. `Handle<Image>` for `Image`.
pub struct HandleOf<P>(pub ReflectReference, PhantomData<P>);

impl<P> HandleOf<P> {
    /// Wraps a reference to a handle
    pub fn new(handle: ReflectReference) -> Self {
        Self(handle, PhantomData)
    }
}

impl<P> IntoScript for HandleOf<P> {
    fn into_script(self, world: WorldGuard) -> Result<ScriptValue, InteropError> {
        self.0.into_script(world)
    }
}

impl<P> FromScript for HandleOf<P> {
    type This<'w> = Self;

    fn from_script(value: ScriptValue, world: WorldGuard) -> Result<Self, InteropError> {
        ReflectReference::from_script(value, world).map(Self::new)
    }
}

impl<P: GenericParam> TypedThrough for HandleOf<P> {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::Generic {
            path: ASSET_HANDLE_PATH,
            args: vec![ThroughTypeInfo::Param(P::NAME)],
        }
    }
}

impl<P> ArgMeta for HandleOf<P> {}

impl<P> GetTypeDependencies for HandleOf<P> {
    type Underlying = ReflectReference;
    fn register_type_dependencies(_registry: &mut TypeRegistry) {}
}

/// The path of the generic asset handle type, without its parameter.
pub const ASSET_HANDLE_PATH: &str = "bevy_asset::handle::Handle";

/// Whether the type is registered as an asset, i.e. with `App::register_asset_reflect`.
pub fn is_asset_type(registry: &TypeRegistry, type_id: TypeId) -> bool {
    registry
        .get_type_data::<bevy_asset::ReflectAsset>(type_id)
        .is_some()
}

#[cfg(test)]
#[test]
fn asset_handle_path_is_the_path_of_the_handle_type() {
    use bevy_reflect::TypePath;
    let path = bevy_asset::Handle::<bevy_asset::LoadedFolder>::type_path();
    assert!(path.starts_with(&format!("{ASSET_HANDLE_PATH}<")), "{path}");
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

/// A table of the fields of a type, the payload of a constructor.
///
/// Fields are keyed by name, or for tuple-like types by their index.
/// A list is taken to be the fields in order, i.e. `{ 1, "a" }` is the same as `{ ["1"] = 1, ["2"] = "a" }` in Lua.
pub enum FieldsPayload {
    /// Fields keyed by name or index
    Keyed(HashMap<String, ScriptValue>),
    /// Fields in order
    Positional(VecDeque<ScriptValue>),
}

impl FieldsPayload {
    /// Converts the fields into a map keyed by name or index, with indices following the given convention.
    pub fn into_map(self, one_indexed: bool) -> HashMap<String, ScriptValue> {
        match self {
            FieldsPayload::Keyed(map) => map,
            FieldsPayload::Positional(list) => list
                .into_iter()
                .enumerate()
                .map(|(idx, value)| ((idx + one_indexed as usize).to_string(), value))
                .collect(),
        }
    }
}

impl FromScript for FieldsPayload {
    type This<'w> = Self;

    fn from_script(value: ScriptValue, _world: WorldGuard) -> Result<Self, InteropError> {
        match value {
            ScriptValue::Map(map) => Ok(FieldsPayload::Keyed(map)),
            ScriptValue::List(list) => Ok(FieldsPayload::Positional(list)),
            value => Err(InteropError::value_mismatch(
                TypeId::of::<HashMap<String, ScriptValue>>(),
                value,
            )),
        }
    }
}

/// The fields of the type the generic parameter `P` stands for, see [`FieldsPayload`].
pub struct FieldsOf<P>(pub FieldsPayload, PhantomData<P>);

impl<P> FieldsOf<P> {
    /// The fields
    pub fn into_inner(self) -> FieldsPayload {
        self.0
    }
}

impl<P> FromScript for FieldsOf<P> {
    type This<'w> = Self;

    fn from_script(value: ScriptValue, world: WorldGuard) -> Result<Self, InteropError> {
        FieldsPayload::from_script(value, world).map(|fields| Self(fields, PhantomData))
    }
}

impl<P> ArgMeta for FieldsOf<P> {}

impl<P: GenericParam> TypedThrough for FieldsOf<P> {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::Fields {
            of: Box::new(ThroughTypeInfo::Param(P::NAME)),
            variant: None,
        }
    }
}

impl<P> GetTypeDependencies for FieldsOf<P> {
    type Underlying = FieldsPayload;
    fn register_type_dependencies(_registry: &mut TypeRegistry) {}
}

/// Declares a marker for the type a generic parameter stands for, bounded to a kind of type.
macro_rules! type_of_marker {
    ($(#[$doc:meta])* $name:ident, $kind:ident) => {
        $(#[$doc])*
        ///
        /// `C` is what is passed at runtime: the static type reference for functions shared by all types,
        /// or a registration for functions which also take types looked up at runtime, i.e. with `get_type_by_name`.
        pub struct $name<P, C = StaticTypeRef>(pub C, PhantomData<P>);

        impl<P, C> $name<P, C> {
            /// What was passed at runtime
            pub fn into_inner(self) -> C {
                self.0
            }
        }

        impl<P, C> Deref for $name<P, C> {
            type Target = C;
            fn deref(&self) -> &C {
                &self.0
            }
        }

        impl<P, C: for<'w> FromScript<This<'w> = C>> FromScript for $name<P, C> {
            type This<'w> = Self;

            fn from_script(value: ScriptValue, world: WorldGuard) -> Result<Self, InteropError> {
                C::from_script(value, world).map(|carrier| Self(carrier, PhantomData))
            }
        }

        impl<P, C> ArgMeta for $name<P, C> {}

        /// Documented as the generic type of its kind applied to the parameter, i.e. `ComponentType<A>`,
        /// or what is passed at runtime, for types looked up at runtime.
        impl<P: GenericParam, C: TypedThrough> TypedThrough for $name<P, C> {
            fn through_type_info() -> ThroughTypeInfo {
                ThroughTypeInfo::Union(vec![
                    ThroughTypeInfo::Generic {
                        path: type_kinds::$kind,
                        args: vec![ThroughTypeInfo::Param(P::NAME)],
                    },
                    C::through_type_info(),
                ])
            }
        }

        impl<P, C: GetTypeDependencies> GetTypeDependencies for $name<P, C> {
            type Underlying = C::Underlying;
            fn register_type_dependencies(registry: &mut TypeRegistry) {
                C::register_type_dependencies(registry)
            }
        }
    };
}

type_of_marker!(
    /// Any type, standing for the generic parameter `P`.
    TypeOf,
    TYPE
);
type_of_marker!(
    /// A component type, standing for the generic parameter `P`.
    ComponentOf,
    COMPONENT_TYPE
);
type_of_marker!(
    /// A resource type, standing for the generic parameter `P`.
    ResourceOf,
    RESOURCE_TYPE
);
type_of_marker!(
    /// A bundle type, standing for the generic parameter `P`.
    BundleOf,
    BUNDLE_TYPE
);
type_of_marker!(
    /// An asset type, standing for the generic parameter `P`.
    AssetOf,
    ASSET_TYPE
);
