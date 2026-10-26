//! Information about functions and their arguments.

use crate::function::arg_meta::ArgMeta;
use crate::function::namespace::FunctionScope;
use crate::function::script_function::FunctionCallContext;
use bevy_mod_scripting_derive::DebugWithTypeInfo;
use bevy_mod_scripting_display::{DisplayWithTypeInfo, WithTypeInfo};
use bevy_mod_scripting_world::WorldGuard;
use bevy_reflect::{Reflect, TypeInfo};
use std::{any::TypeId, borrow::Cow};

use super::typed_through::{ThroughTypeInfo, TypedThrough};
use bevy_mod_scripting_bindings_domain::GenericBound;

/// for things you can call and provide some introspection capability.
#[diagnostic::on_unimplemented(
    message = "This function contains types in its signature which don't implement [`ArgMeta`] or [`TypedThrough`].",
    note = "These types are necessary to build [`FunctionInfo`] structs for each registered function."
)]
pub trait GetFunctionInfo<Marker> {
    /// Get the function info for the function.
    fn get_function_info(&self, name: Cow<'static, str>, namespace: FunctionScope) -> FunctionInfo;
}

#[derive(Clone, Reflect, DebugWithTypeInfo)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
/// Information about a function.
pub struct FunctionInfo {
    /// The name of the function.
    pub name: Cow<'static, str>,
    /// The namespace of the function.
    pub namespace: FunctionScope,
    /// Information about the arguments of the function.
    pub arg_info: Vec<FunctionArgInfo>,
    /// Information about the return value of the function.
    pub return_info: FunctionReturnInfo,
    /// Documentation for the function.
    pub docs: Option<Cow<'static, str>>,
}

impl Default for FunctionInfo {
    fn default() -> Self {
        Self::new()
    }
}

#[profiling::all_functions]
impl FunctionInfo {
    /// Create a new function info with default values.
    pub fn new() -> Self {
        Self {
            name: Cow::Borrowed(""),
            namespace: FunctionScope::Member,
            arg_info: Vec::new(),
            return_info: FunctionReturnInfo::default(),
            docs: None,
        }
    }

    /// Create a new function info with a name and namespace.
    pub fn new_for(name: Cow<'static, str>, namespace: FunctionScope) -> Self {
        Self {
            name,
            namespace,
            arg_info: Vec::new(),
            return_info: FunctionReturnInfo::default(),
            docs: None,
        }
    }

    /// Set the name of the function info.
    pub fn with_name(mut self, name: impl Into<Cow<'static, str>>) -> Self {
        self.name = name.into();
        self
    }

    /// Set the namespace of the function info.
    pub fn with_namespace(mut self, namespace: FunctionScope) -> Self {
        self.namespace = namespace;
        self
    }

    /// Add an argument to the function info.
    pub fn add_arg<T: ArgMeta + TypedThrough + 'static>(
        mut self,
        name: Option<Cow<'static, str>>,
    ) -> Self {
        self.arg_info
            .push(FunctionArgInfo::for_type::<T>(name, self.arg_info.len()));
        self
    }

    /// Add a return value to the function info.
    pub fn add_return<T: TypedThrough + 'static>(mut self) -> Self {
        self.return_info = FunctionReturnInfo::new_for::<T>();
        self
    }

    /// Add documentation to the function info.
    pub fn with_docs(mut self, docs: impl Into<Cow<'static, str>>) -> Self {
        self.docs = Some(docs.into());
        self
    }

    /// Replaces the generic parameters in the signature with the given type, and for fields, the variant.
    ///
    /// See [`ThroughTypeInfo::bind_generics`].
    pub fn bind_generics(
        mut self,
        type_info: &'static TypeInfo,
        variant: Option<&'static str>,
    ) -> Self {
        for arg in &mut self.arg_info {
            arg.type_info = arg
                .type_info
                .as_ref()
                .map(|t| t.bind_generics(type_info, variant));
        }
        self.return_info.type_info = self
            .return_info
            .type_info
            .as_ref()
            .map(|t| t.bind_generics(type_info, variant));
        self
    }

    /// Whether the signature mentions generic parameters, i.e. depends on what it is bound to.
    pub fn has_generics(&self) -> bool {
        self.arg_info
            .iter()
            .map(|arg| &arg.type_info)
            .chain(std::iter::once(&self.return_info.type_info))
            .any(|t| t.as_ref().is_some_and(ThroughTypeInfo::has_generics))
    }

    /// The bounds on the generic parameter of the signature, i.e. `Component` for a function taking [`crate::function::generics::ComponentOf`].
    pub fn generic_bounds(&self) -> Vec<GenericBound> {
        let mut bounds = vec![];
        for type_info in self
            .arg_info
            .iter()
            .map(|arg| &arg.type_info)
            .chain(std::iter::once(&self.return_info.type_info))
            .flatten()
        {
            type_info.generic_bounds(&mut bounds);
        }
        bounds
    }

    /// The signature of a function shared between all types, as seen on the given type.
    ///
    /// The first argument passed by scripts is the type itself, which is bound automatically and so removed,
    /// the function is moved into the type's namespace, and its generic parameters are bound to the type.
    pub fn bind_to_type(&self, type_info: &'static TypeInfo) -> Self {
        let mut info = self.clone();
        if let Some(receiver) = info
            .arg_info
            .iter()
            .position(|arg| arg.type_id != TypeId::of::<FunctionCallContext>())
        {
            info.arg_info.remove(receiver);
        }
        for (idx, arg) in info.arg_info.iter_mut().enumerate() {
            arg.arg_index = idx;
        }
        info.namespace = FunctionScope::OnType(type_info.type_id());
        info.bind_generics(type_info, None)
    }

    /// Add argument names to the function info.
    ///
    /// If the number of argument names is less than the number of arguments, the remaining arguments will be unnamed.
    /// If the number of argument names is greater than the number of arguments, the extra argument names will be ignored.
    pub fn with_arg_names(mut self, arg_names: &[&'static str]) -> Self {
        self.arg_info
            .iter_mut()
            .zip(arg_names.iter())
            .for_each(|(arg, name)| {
                arg.name = Some(Cow::Borrowed(*name));
            });
        self
    }
}

#[derive(Clone, Reflect, DebugWithTypeInfo)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
/// Information about a function argument.
pub struct FunctionArgInfo {
    /// The name of the argument.
    pub name: Option<Cow<'static, str>>,
    /// The index of the argument.
    pub arg_index: usize,
    /// The type of the argument.
    pub type_id: TypeId,
    /// The type information of the argument.
    #[reflect(ignore)]
    pub type_info: Option<ThroughTypeInfo>,
}

impl DisplayWithTypeInfo for FunctionArgInfo {
    fn display_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&WorldGuard>,
    ) -> std::fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "{name}: ")?;
        }
        let type_id = self.type_id;
        WithTypeInfo::new_with_opt_info(&type_id, type_info_provider)
            .display_with_type_info(f, type_info_provider)?;
        Ok(())
    }
}

#[profiling::all_functions]
impl FunctionArgInfo {
    /// Create a new function argument info with a name.
    pub fn with_name(mut self, name: Cow<'static, str>) -> Self {
        self.name = Some(name);
        self
    }

    /// Create a new function argument info for a specific type.
    pub fn for_type<T: TypedThrough + 'static>(
        name: Option<impl Into<Cow<'static, str>>>,
        arg_index: usize,
    ) -> Self {
        Self {
            name: name.map(Into::into),
            arg_index,
            type_id: TypeId::of::<T>(),
            type_info: Some(T::through_type_info()),
        }
    }
}

#[derive(Clone, Reflect, DebugWithTypeInfo)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
/// Information about a function return value.
pub struct FunctionReturnInfo {
    /// The type of the return value.
    pub type_id: TypeId,
    /// The type information of the return value.
    #[reflect(ignore)]
    pub type_info: Option<ThroughTypeInfo>,
}

impl Default for FunctionReturnInfo {
    fn default() -> Self {
        Self::new_for::<()>()
    }
}

impl DisplayWithTypeInfo for FunctionReturnInfo {
    fn display_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&WorldGuard>,
    ) -> std::fmt::Result {
        let type_id = self.type_id;
        WithTypeInfo::new_with_opt_info(&type_id, type_info_provider)
            .display_with_type_info(f, type_info_provider)?;
        Ok(())
    }
}

#[profiling::all_functions]
impl FunctionReturnInfo {
    /// Create a new function return info for a specific type.
    pub fn new_for<T: TypedThrough + 'static>() -> Self {
        Self {
            type_id: TypeId::of::<T>(),
            type_info: Some(T::through_type_info()),
        }
    }
}

macro_rules! impl_documentable {
    ($( $param:ident ),*) => {
        #[profiling::all_functions]
        impl<$($param,)* F, O> GetFunctionInfo<fn($($param),*) -> O> for F
            where
            F: Fn($($param),*) -> O,
            $($param: ArgMeta + TypedThrough + 'static,)*
            O: TypedThrough + 'static
        {
            fn get_function_info(&self, name: Cow<'static, str>, namespace: FunctionScope) -> FunctionInfo {
                #[allow(unused_mut)]
                let mut info = FunctionInfo::new_for(name, namespace);
                $(
                    info = info.add_arg::<$param>(None);
                )*
                info.add_return::<O>()
            }
        }
    };
}

variadics_please::all_tuples!(impl_documentable, 0, 13, T);

#[cfg(test)]
mod test {
    use bevy_mod_scripting_bindings_domain::ReflectionPrimitiveKind;

    use crate::function::from::{M, R, V};

    use super::*;

    #[test]
    fn test_get_function_info() {
        fn test_fn(a: i32, b: f32) -> f64 {
            (a as f64) + (b as f64)
        }

        let info = test_fn.get_function_info(Cow::Borrowed("test_fn"), FunctionScope::Member);
        assert_eq!(info.name, "test_fn");
        assert_eq!(info.namespace, FunctionScope::Member);
        assert_eq!(info.arg_info.len(), 2);
        assert_eq!(info.return_info.type_id, TypeId::of::<f64>());

        assert_eq!(info.arg_info[0].type_id, TypeId::of::<i32>());
        assert_eq!(info.arg_info[1].type_id, TypeId::of::<f32>());

        match info.arg_info[0].type_info.as_ref().unwrap() {
            ThroughTypeInfo::Primitive(prim) => {
                assert_eq!(*prim, ReflectionPrimitiveKind::I32);
            }
            _ => panic!("Expected TypeInfo"),
        }

        match info.arg_info[1].type_info.as_ref().unwrap() {
            ThroughTypeInfo::Primitive(prim) => {
                assert_eq!(*prim, ReflectionPrimitiveKind::F32);
            }
            _ => panic!("Expected TypeInfo"),
        }
    }

    #[test]
    fn test_get_function_info_references() {
        let fn_ = |_: R<i32>, _: M<f32>| -> V<f64> { V::new(0.0) };

        let info = fn_.get_function_info(Cow::Borrowed("test_fn"), FunctionScope::Member);
        assert_eq!(info.name, "test_fn");
        assert_eq!(info.namespace, FunctionScope::Member);
        assert_eq!(info.arg_info.len(), 2);
        assert_eq!(info.return_info.type_id, TypeId::of::<V<f64>>());

        assert_eq!(info.arg_info[0].type_id, TypeId::of::<R<'static, i32>>());
        assert_eq!(info.arg_info[1].type_id, TypeId::of::<M<'static, f32>>());

        match &info.arg_info[0].type_info {
            Some(ThroughTypeInfo::Ref(through_type)) => {
                assert_eq!(through_type.type_id(), TypeId::of::<i32>());
            }
            _ => panic!("Expected Ref"),
        }

        match &info.arg_info[1].type_info {
            Some(ThroughTypeInfo::Mut(through_type)) => {
                assert_eq!(through_type.type_id(), TypeId::of::<f32>());
            }
            _ => panic!("Expected Mut"),
        }
    }
}
