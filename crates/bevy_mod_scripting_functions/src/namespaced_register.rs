use std::{any::TypeId, borrow::Cow, marker::PhantomData};

use bevy::{
    prelude::{AppFunctionRegistry, IntoFunction, World},
    reflect::func::{DynamicFunction, FunctionRegistrationError, FunctionRegistry},
};

pub trait RegisterNamespacedFunction {
    fn register_namespaced_function<S, N, F, M>(
        &mut self,
        name: N,
        function: F,
    ) -> Result<(), FunctionRegistrationError>
    where
        N: Into<Cow<'static, str>>,
        S: IntoNamespace,
        F: IntoFunction<'static, M> + 'static;

    fn overwrite_namespaced_function<S, N, F, M>(
        &mut self,
        name: N,
        function: F,
    ) -> Option<DynamicFunction<'static>>
    where
        N: Into<Cow<'static, str>>,
        S: IntoNamespace,
        F: IntoFunction<'static, M> + 'static;
}

pub trait GetNamespacedFunction {
    fn get_namespaced_function<N>(
        &self,
        name: N,
        namespace: Namespace,
    ) -> Option<&DynamicFunction<'static>>
    where
        N: Into<Cow<'static, str>>;

    fn has_namespaced_function<N>(&self, name: N, namespace: Namespace) -> bool
    where
        N: Into<Cow<'static, str>>;
}

pub enum Namespace {
    /// The function is registered in the global namespace, i.e. with no namespace
    Global,
    /// The function is registered in the namespace corresponding to the given type
    OnType(TypeId),
}

pub trait IntoNamespace {
    fn into_namespace() -> Namespace;
}

impl<T: ?Sized + 'static> IntoNamespace for T {
    fn into_namespace() -> Namespace {
        Namespace::OnType(TypeId::of::<T>())
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
    pub fn function_name(self, name: Cow<'static, str>) -> Cow<'static, str> {
        match self {
            Namespace::Global => name,
            Namespace::OnType(type_id) => Cow::Owned(format!("{:?}::{}", type_id, name)),
        }
    }
}

impl RegisterNamespacedFunction for FunctionRegistry {
    fn register_namespaced_function<S, N, F, M>(
        &mut self,
        name: N,
        function: F,
    ) -> Result<(), FunctionRegistrationError>
    where
        N: Into<Cow<'static, str>>,
        S: IntoNamespace,
        F: IntoFunction<'static, M> + 'static,
    {
        let cow: Cow<'static, str> = name.into();
        let function_name = S::into_namespace().function_name(cow);
        self.register_with_name(function_name, function)?;
        Ok(())
    }

    fn overwrite_namespaced_function<S, N, F, M>(
        &mut self,
        name: N,
        function: F,
    ) -> Option<DynamicFunction<'static>>
    where
        N: Into<Cow<'static, str>>,
        S: IntoNamespace,
        F: IntoFunction<'static, M> + 'static,
    {
        let cow: Cow<'static, str> = name.into();
        let function_name = S::into_namespace().function_name(cow);
        self.overwrite_registration_with_name(function_name, function)
    }
}

impl GetNamespacedFunction for FunctionRegistry {
    fn get_namespaced_function<N>(
        &self,
        name: N,
        namespace: Namespace,
    ) -> Option<&DynamicFunction<'static>>
    where
        N: Into<Cow<'static, str>>,
    {
        let cow: Cow<'static, str> = name.into();
        let function_name = namespace.function_name(cow);
        self.get(&function_name)
    }

    fn has_namespaced_function<N>(&self, name: N, namespace: Namespace) -> bool
    where
        N: Into<Cow<'static, str>>,
    {
        let cow: Cow<'static, str> = name.into();
        let function_name = namespace.function_name(cow);
        self.contains(&function_name)
    }
}

pub struct NamespaceBuilder<'a, N> {
    namespace: PhantomData<N>,
    pub world: &'a mut World,
}

impl<'a, S: IntoNamespace> NamespaceBuilder<'a, S> {
    pub fn new(world: &'a mut World) -> Self {
        Self {
            namespace: Default::default(),
            world,
        }
    }

    pub fn register<N, F, M>(
        &mut self,
        name: N,
        function: F,
    ) -> Result<&mut Self, FunctionRegistrationError>
    where
        N: Into<Cow<'static, str>>,
        F: IntoFunction<'static, M> + 'static,
    {
        {
            let registry = self.world.get_resource_or_init::<AppFunctionRegistry>();
            let mut registry = registry.write();
            registry.register_namespaced_function::<S, _, F, M>(name, function)?;
        }
        Ok(self)
    }

    pub fn overwrite<N, F, M>(&mut self, name: N, function: F) -> &mut Self
    where
        N: Into<Cow<'static, str>>,
        F: IntoFunction<'static, M> + 'static,
    {
        {
            let registry = self.world.get_resource_or_init::<AppFunctionRegistry>();

            let mut registry = registry.write();
            registry.overwrite_namespaced_function::<S, _, F, M>(name, function);
        }
        self
    }
}
