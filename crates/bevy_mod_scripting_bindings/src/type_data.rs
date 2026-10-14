//! Contains various `Reflect` type data we use in BMS.

use bevy_reflect::CreateTypeData;

/// A marker type to indicate that a type is generated.
///
/// To mark a type as generated use:
/// ```rust,ignore
/// registry.register_type_data::<T, MarkAsGenerated>();
/// ```
#[derive(Clone, Copy)]
pub struct MarkAsGenerated;

impl<T> CreateTypeData<T> for MarkAsGenerated {
    fn create_type_data(_input: ()) -> Self {
        Self
    }
}

/// A marker type to indicate that a type is significant.
///
/// Significant types appear "before" core types in documentation
#[derive(Clone, Copy)]
pub struct MarkAsSignificant;

impl<T> CreateTypeData<T> for MarkAsSignificant {
    fn create_type_data(_input: ()) -> Self {
        Self
    }
}

/// A marker type to indicate that a type is core to BMS.
///
/// core types appear before insignificant types in documentation.
#[derive(Clone, Copy)]
pub struct MarkAsCore;

impl<T> CreateTypeData<T> for MarkAsCore {
    fn create_type_data(_input: ()) -> Self {
        Self
    }
}

/// The namespace a type lives in, as seen by scripts.
///
/// Types without this type data live in the namespace named after the crate that defines them
/// (see [`ScriptTypeRegistration::namespace`](crate::ScriptTypeRegistration::namespace)).
/// Use this to override that, i.e. for types which are re-exported from a different crate than they are defined in:
/// ```rust,ignore
/// registry.register_type_data_with::<T, ScriptNamespace, _>("my_namespace");
/// ```
/// or [`ScriptNamespaceAppExt::override_script_namespace`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScriptNamespace(pub std::borrow::Cow<'static, str>);

impl<T> CreateTypeData<T, &'static str> for ScriptNamespace {
    fn create_type_data(input: &'static str) -> Self {
        Self(input.into())
    }
}

impl<T> CreateTypeData<T, String> for ScriptNamespace {
    fn create_type_data(input: String) -> Self {
        Self(input.into())
    }
}

/// Puts `T` in the given script namespace, unless it was already put in a namespace.
///
/// If `T` is already in a different namespace a warning is logged, and the first namespace wins.
/// `T` must be registered. This is what the `namespace` argument of `script_bindings` uses.
pub fn set_script_namespace_if_unset<T: bevy_reflect::Reflect + bevy_reflect::TypePath>(
    registry: &mut bevy_reflect::TypeRegistry,
    namespace: &'static str,
) {
    if let Some(existing) = registry
        .get(std::any::TypeId::of::<T>())
        .and_then(|r| r.data::<ScriptNamespace>())
    {
        if existing.0 != namespace {
            bevy_log::warn!(
                "type `{}` is already in the script namespace `{}`, ignoring attempt to put it in `{namespace}`",
                T::type_path(),
                existing.0
            );
        }
        return;
    }
    registry.register_type_data_with::<T, ScriptNamespace, _>(namespace);
}

/// Extension for setting the script namespace of types.
pub trait ScriptNamespaceAppExt {
    /// Registers `T` if it is not registered yet, and puts it in the given script namespace.
    ///
    /// This always wins: it replaces any namespace set before, and `script_bindings(namespace = ...)` running afterwards keeps it (with a warning).
    fn override_script_namespace<
        T: bevy_reflect::GetTypeRegistration + bevy_reflect::Reflect + bevy_reflect::TypePath,
    >(
        &mut self,
        namespace: &'static str,
    ) -> &mut Self;
}

impl ScriptNamespaceAppExt for bevy_app::App {
    fn override_script_namespace<
        T: bevy_reflect::GetTypeRegistration + bevy_reflect::Reflect + bevy_reflect::TypePath,
    >(
        &mut self,
        namespace: &'static str,
    ) -> &mut Self {
        let registry = self
            .world_mut()
            .get_resource_or_init::<bevy_ecs::reflect::AppTypeRegistry>()
            .clone();
        let mut registry = registry.write();
        registry.register::<T>();
        registry.register_type_data_with::<T, ScriptNamespace, _>(namespace);
        drop(registry);
        self
    }
}
