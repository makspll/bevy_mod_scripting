use crate::bindings::function::script_function::{
    AppScriptFunctionRegistry, DynamicScriptFunction, GetFunctionTypeDependencies, ScriptFunction,
};
use bevy::{
    prelude::{AppTypeRegistry, World},
    reflect::GetTypeRegistration,
};
use std::{any::TypeId, borrow::Cow, marker::PhantomData};

pub trait RegisterNamespacedFunction {
    fn register_namespaced_function<S, N, F, M>(&mut self, name: N, function: F)
    where
        N: Into<Cow<'static, str>>,
        S: IntoNamespace,
        F: ScriptFunction<'static, M>;
}

pub trait GetNamespacedFunction {
    fn iter_overloads_namespaced<N>(
        &self,
        name: N,
        namespace: Namespace,
    ) -> impl Iterator<Item = &DynamicScriptFunction>
    where
        N: Into<Cow<'static, str>>;
    fn get_namespaced_function<N>(
        &self,
        name: N,
        namespace: Namespace,
    ) -> Option<&DynamicScriptFunction>
    where
        N: Into<Cow<'static, str>>;

    fn get_namespaced_function_typed<N, NS>(&self, name: N) -> Option<&DynamicScriptFunction>
    where
        N: Into<Cow<'static, str>>,
        NS: IntoNamespace,
    {
        Self::get_namespaced_function(self, name, NS::into_namespace())
    }

    fn has_namespaced_function<N>(&self, name: N, namespace: Namespace) -> bool
    where
        N: Into<Cow<'static, str>>;

    fn has_namespaced_function_typed<N, NS>(&self, name: N) -> bool
    where
        N: Into<Cow<'static, str>>,
        NS: IntoNamespace,
    {
        Self::has_namespaced_function(self, name, NS::into_namespace())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
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

pub trait IntoNamespace {
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

impl Namespace {
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

// impl RegisterNamespacedFunction for ScriptFunctionRegistry {
//     fn register_namespaced_function<S, N, F, M>(&mut self, name: N, function: F)
//     where
//         N: Into<Cow<'static, str>>,
//         S: IntoNamespace,
//         F: ScriptFunction<'static, M>,
//     {
//         self.register(S::into_namespace(), name, function);
//     }
// }

// impl GetNamespacedFunction for ScriptFunctionRegistry {
//     fn iter_overloads_namespaced<N>(
//         &self,
//         name: N,
//         namespace: Namespace,
//     ) -> impl Iterator<Item = &DynamicScriptFunction>
//     where
//         N: Into<Cow<'static, str>>,
//     {
//         let cow: Cow<'static, str> = name.into();
//         let function_name = namespace.function_name(cow);
//         self.iter_overloads(function_name)
//     }

//     fn get_namespaced_function<N>(
//         &self,
//         name: N,
//         namespace: Namespace,
//     ) -> Option<&DynamicScriptFunction>
//     where
//         N: Into<Cow<'static, str>>,
//     {
//         let cow: Cow<'static, str> = name.into();
//         let function_name = namespace.function_name(cow);
//         self.get_first(&function_name)
//     }

//     fn has_namespaced_function<N>(&self, name: N, namespace: Namespace) -> bool
//     where
//         N: Into<Cow<'static, str>>,
//     {
//         let cow: Cow<'static, str> = name.into();
//         let function_name = namespace.function_name(cow);
//         self.contains(&function_name)
//     }
// }

pub struct NamespaceBuilder<'a, N> {
    namespace: PhantomData<N>,
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

    pub fn register<N, F, M>(&mut self, name: N, function: F) -> &mut Self
    where
        N: Into<Cow<'static, str>>,
        F: ScriptFunction<'static, M> + GetFunctionTypeDependencies<M>,
    {
        {
            {
                let mut registry = self
                    .world
                    .get_resource_or_init::<AppScriptFunctionRegistry>();
                let mut registry = registry.write();
                registry.register(S::into_namespace(), name, function);
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
