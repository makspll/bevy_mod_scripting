//! A module for managing namespaces for functions

use crate::{
    AppScriptNamespaceMembers, NamespaceMembers, ScriptFunctionRegistryArc,
    docgen::info::GetFunctionInfo,
    function::script_function::{AppScriptFunctionRegistry, ScriptFunction, documented_function},
};
use ::bevy_reflect::{GetTypeRegistration, Reflect};
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_mod_scripting_derive::DebugWithTypeInfo;
use bevy_mod_scripting_display::{DisplayWithTypeInfo, WithTypeInfo};
use bevy_mod_scripting_world::WorldGuard;
use std::{any::TypeId, borrow::Cow, marker::PhantomData};

use super::type_dependencies::GetFunctionTypeDependencies;

/// Where a function lives, for its documentation and errors
#[derive(Clone, Copy, PartialEq, Eq, Hash, Reflect, DebugWithTypeInfo)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
pub enum FunctionScope {
    /// On a type, i.e. `MyType.my_function()`, in the [`crate::ScriptFunctionRegistry`]
    OnType(TypeId),
    /// A member of a script namespace, i.e. `import_namespace("bms").log_info`, see [`crate::NamespaceMembers`]
    Member,
    /// One of the few functions every script has as a global, see [`crate::builtins`]
    Builtin,
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
}

/// A convenience builder for registering functions on a type, or as members of a namespace (see [`NamespaceBuilder::in_namespace`])
pub struct NamespaceBuilder<'a, N> {
    registry: ScriptFunctionRegistryArc,
    /// phantom data to reference the type
    namespace: PhantomData<N>,
    /// The script namespace functions are registered into as members, instead of on the type, see [`NamespaceMembers`]
    members_of: Option<Cow<'static, str>>,
    /// a cached reference to the world
    pub world: &'a mut World,
}

impl<'a> NamespaceBuilder<'a, NamespaceMembers> {
    /// Creates a builder registering functions as members of the given script namespace, i.e. `import_namespace("bms").log_info`.
    pub fn in_namespace(world: &'a mut World, namespace: impl Into<Cow<'static, str>>) -> Self {
        let mut builder = Self::new_unregistered(world);
        builder.members_of = Some(namespace.into());
        builder
    }
}

#[profiling::all_functions]
impl<'a, S: 'static> NamespaceBuilder<'a, S> {
    /// Creates a new `NamespaceBuilder` that will register functions on the given type
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
        Self::new_unregistered(world)
    }

    /// Prefer using the `register` method on the `NamespaceBuilder` instead
    pub fn new_unregistered(world: &'a mut World) -> Self {
        Self {
            registry: world
                .get_resource_or_init::<AppScriptFunctionRegistry>()
                .0
                .clone(),
            namespace: Default::default(),
            members_of: None,
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
        if let Some(namespace) = &self.members_of {
            let name = name.into();
            let function = documented_function(
                FunctionScope::Member,
                name.clone(),
                function,
                docstring,
                arg_names,
            );
            let members = self
                .world
                .get_resource_or_init::<AppScriptNamespaceMembers>();
            members
                .write()
                .register_function(namespace.clone(), name, function);
        } else {
            let mut registry = self.registry.write();
            registry.register_with_arg_names(
                TypeId::of::<S>(),
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
            FunctionScope::Member => f.write_str("Namespace member"),
            FunctionScope::Builtin => f.write_str("Builtin"),
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
