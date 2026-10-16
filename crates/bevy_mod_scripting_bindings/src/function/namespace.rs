//! A module for managing namespaces for functions

use crate::{
    DummyScriptFunctionRegistry, ScriptFunctionRegistryArc,
    docgen::info::GetFunctionInfo,
    function::script_function::{AppScriptFunctionRegistry, ScriptFunction},
};
use ::bevy_reflect::{GetTypeRegistration, Reflect};
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_mod_scripting_derive::DebugWithTypeInfo;
use bevy_mod_scripting_display::{DisplayWithTypeInfo, WithTypeInfo};
use bevy_mod_scripting_world::WorldGuard;
use std::{any::TypeId, borrow::Cow, marker::PhantomData};

use super::type_dependencies::GetFunctionTypeDependencies;

/// A namespace for functions
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Reflect, DebugWithTypeInfo)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
pub enum FunctionScope {
    /// The function is registered in the global namespace, i.e. with no namespace.
    /// In practice functions in this namespace should be callable directly by their name, i.e. `my_function()`
    #[default]
    Global,
    /// The function is registered in the namespace corresponding to the given type.
    /// In practice functions in this namespace should be callable by their qualified name, i.e. `MyType.my_function()`
    OnType(TypeId),
}

/// A type which implements [`IntoFunctionScope`] by always converting to the global namespace
pub struct GlobalNamespace;

/// A type convertible to a [`FunctionScope`]
pub trait IntoFunctionScope {
    /// Converts this type into a [`FunctionScope`]
    fn into_function_scope() -> FunctionScope;
}

impl<T: ?Sized + 'static> IntoFunctionScope for T {
    fn into_function_scope() -> FunctionScope {
        if TypeId::of::<T>() == TypeId::of::<GlobalNamespace>() {
            FunctionScope::Global
        } else {
            FunctionScope::OnType(TypeId::of::<T>())
        }
    }
}

#[profiling::all_functions]
impl FunctionScope {
    /// The scope of functions shared by all references, i.e. `eq` or `display`, registered on [`crate::ReflectReference`].
    ///
    /// Functions on a reference's own type take priority over these.
    pub fn all_references() -> Self {
        FunctionScope::OnType(TypeId::of::<crate::ReflectReference>())
    }

    /// The scope of functions shared by all types, i.e. the `new` constructor, registered on [`crate::StaticTypeRef`].
    ///
    /// They are bound to the type they are called on, see [`crate::DynamicScriptFunction::bind_to_type`].
    pub fn all_types() -> Self {
        FunctionScope::OnType(TypeId::of::<crate::StaticTypeRef>())
    }

    /// The scope of functions shared by all enum variants, registered on [`crate::EnumVariantRef`].
    pub fn all_variants() -> Self {
        FunctionScope::OnType(TypeId::of::<crate::EnumVariantRef>())
    }

    /// Returns the prefix for this namespace
    pub fn prefix(self) -> Cow<'static, str> {
        match self {
            FunctionScope::Global => Cow::Borrowed(""),
            FunctionScope::OnType(type_id) => Cow::Owned(format!("{type_id:?}::")),
        }
    }

    /// Returns the fully qualified name of a function in this namespace
    pub fn function_name<I: Into<Cow<'static, str>>>(self, name: I) -> Cow<'static, str> {
        match self {
            FunctionScope::Global => name.into(),
            FunctionScope::OnType(type_id) => Cow::Owned(format!("{:?}::{}", type_id, name.into())),
        }
    }
}

/// A convenience builder for registering multiple functions in a namespace
pub struct NamespaceBuilder<'a, N> {
    /// If true will use the dummy function registry instead
    registry: ScriptFunctionRegistryArc,
    /// phantom data to reference the namespace type
    namespace: PhantomData<N>,
    /// a cached reference to the world
    pub world: &'a mut World,
}

#[profiling::all_functions]
impl<'a, S: IntoFunctionScope> NamespaceBuilder<'a, S> {
    /// Creates a new `NamespaceBuilder` that will register functions in the namespace corresponding to the given type
    /// It will also register the type itself in the type registry.
    pub fn new(world: &'a mut World) -> Self
    where
        S: GetTypeRegistration,
    {
        {
            let registry = world.get_resource_or_init::<AppTypeRegistry>();
            let mut registry = registry.write();
            registry.register::<S>();
        }
        Self {
            registry: world
                .get_resource_or_init::<AppScriptFunctionRegistry>()
                .0
                .clone(),
            namespace: Default::default(),
            world,
        }
    }

    /// Prefer using the `register` method on the `NamespaceBuilder` instead
    pub fn new_unregistered(world: &'a mut World) -> Self {
        Self {
            registry: world
                .get_resource_or_init::<AppScriptFunctionRegistry>()
                .0
                .clone(),
            namespace: Default::default(),
            world,
        }
    }

    /// Register functions for this namespace on the dummy function registry instead.
    ///
    /// This will appear in documentation but not become callable.
    pub fn with_dummy_registry(mut self) -> Self {
        self.registry = self
            .world
            .get_resource_or_init::<DummyScriptFunctionRegistry>()
            .0
            .clone();
        self
    }

    /// Registers a function in the namespace
    pub fn register<'env, N, F, M>(&mut self, name: N, function: F) -> &mut Self
    where
        N: Into<Cow<'static, str>>,
        F: ScriptFunction<'env, M> + GetFunctionTypeDependencies<M> + GetFunctionInfo<M>,
    {
        self.register_inner(name, function, None, None)
    }

    /// Registers a function in the namespace with a docstring
    pub fn register_documented<'env, N, F, M>(
        &mut self,
        name: N,
        function: F,
        docstring: &'static str,
        arg_names: &'static [&'static str],
    ) -> &mut Self
    where
        N: Into<Cow<'static, str>>,
        F: ScriptFunction<'env, M> + GetFunctionTypeDependencies<M> + GetFunctionInfo<M>,
    {
        self.register_inner(name, function, Some(docstring), Some(arg_names))
    }

    fn register_inner<'env, N, F, M>(
        &mut self,
        name: N,
        function: F,
        docstring: Option<&'static str>,
        arg_names: Option<&'static [&'static str]>,
    ) -> &mut Self
    where
        N: Into<Cow<'static, str>>,
        F: ScriptFunction<'env, M> + GetFunctionTypeDependencies<M> + GetFunctionInfo<M>,
    {
        {
            {
                let mut registry = self.registry.write();
                registry.register_with_arg_names(
                    S::into_function_scope(),
                    name,
                    function,
                    docstring.unwrap_or_default(),
                    arg_names.unwrap_or(&[]),
                );
            }
            {
                let type_registry = self.world.get_resource_or_init::<AppTypeRegistry>();
                let mut type_registry = type_registry.write();
                F::register_type_dependencies(&mut type_registry);
            }
        }
        self
    }
}

impl From<TypeId> for FunctionScope {
    fn from(value: TypeId) -> Self {
        FunctionScope::OnType(value)
    }
}

impl DisplayWithTypeInfo for FunctionScope {
    fn display_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&WorldGuard>,
    ) -> std::fmt::Result {
        match self {
            FunctionScope::Global => f.write_str("Global Namespace"),
            FunctionScope::OnType(type_id) => {
                write!(
                    f,
                    "Namespace for type {}",
                    WithTypeInfo::new_with_opt_info(type_id, type_info_provider)
                )
            }
        }
    }
}
