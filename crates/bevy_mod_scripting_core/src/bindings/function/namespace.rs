//! A module for managing namespaces for functions

use crate::{
    bindings::function::script_function::{AppScriptFunctionRegistry, ScriptFunction},
    docgen::info::GetFunctionInfo,
};
use bevy::{
    prelude::{AppTypeRegistry, World},
    reflect::{GetTypeRegistration, Reflect},
};
use std::{any::TypeId, borrow::Cow, marker::PhantomData};

use super::type_dependencies::GetFunctionTypeDependencies;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Reflect)]
/// A namespace for functions
pub enum Namespace {
    /// The function is registered in the global namespace, i.e. with no namespace.
    /// In practice functions in this namespace should be callable directly by their name, i.e. `my_function()`
    #[default]
    Global,
    /// The function is registered in the namespace corresponding to the given type.
    /// In practice functions in this namespace should be callable by their qualified name, i.e. `MyType.my_function()`
    OnType(TypeId),
}

/// A type which implements [`IntoNamespace`] by always converting to the global namespace
pub struct GlobalNamespace;

/// A type convertible to a [`Namespace`]
pub trait IntoNamespace {
    /// Converts this type into a [`Namespace`]
    fn into_namespace() -> Namespace;
}

impl<T: ?Sized + 'static> IntoNamespace for T {
    fn into_namespace() -> Namespace {
        if TypeId::of::<T>() == TypeId::of::<GlobalNamespace>() {
            Namespace::Global
        } else {
            Namespace::OnType(TypeId::of::<T>())
        }
    }
}

/// A type which implements [`IntoNamespace`] by always converting to the global namespace
impl Namespace {
    /// Returns the prefix for this namespace
    pub fn prefix(self) -> Cow<'static, str> {
        match self {
            Namespace::Global => Cow::Borrowed(""),
            Namespace::OnType(type_id) => Cow::Owned(format!("{:?}::", type_id)),
        }
    }

    /// Returns the fully qualified name of a function in this namespace
    pub fn function_name<I: Into<Cow<'static, str>>>(self, name: I) -> Cow<'static, str> {
        match self {
            Namespace::Global => name.into(),
            Namespace::OnType(type_id) => Cow::Owned(format!("{:?}::{}", type_id, name.into())),
        }
    }
}

/// A convenience builder for registering multiple functions in a namespace
pub struct NamespaceBuilder<'a, N> {
    /// phantom data to reference the namespace type
    namespace: PhantomData<N>,
    /// a cached reference to the world
    pub world: &'a mut World,
}

impl<'a, S: IntoNamespace> NamespaceBuilder<'a, S> {
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
            namespace: Default::default(),
            world,
        }
    }

    /// Prefer using the `register` method on the `NamespaceBuilder` instead
    pub fn new_unregistered(world: &'a mut World) -> Self {
        Self {
            namespace: Default::default(),
            world,
        }
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
                let mut registry = self
                    .world
                    .get_resource_or_init::<AppScriptFunctionRegistry>();
                let mut registry = registry.write();
                registry.register_with_arg_names(
                    S::into_namespace(),
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
