//! Information about functions and their arguments.

use crate::function::arg_meta::ArgMeta;
use crate::function::namespace::Namespace;
use bevy_mod_scripting_derive::DebugWithTypeInfo;
use bevy_mod_scripting_display::{DisplayWithTypeInfo, GetTypeInfo, WithTypeInfo};
use bevy_reflect::Reflect;
use std::{any::TypeId, borrow::Cow};

use super::typed_through::{ThroughTypeInfo, TypedThrough};

/// for things you can call and provide some introspection capability.
pub trait GetFunctionInfo<Marker> {
    /// Get the function info for the function.
    fn get_function_info(&self, name: Cow<'static, str>, namespace: Namespace) -> FunctionInfo;
}

#[derive(Clone, Reflect, DebugWithTypeInfo)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
/// Information about a function.
pub struct FunctionInfo {
    /// The name of the function.
    pub name: Cow<'static, str>,
    /// The namespace of the function.
    pub namespace: Namespace,
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
            namespace: Namespace::Global,
            arg_info: Vec::new(),
            return_info: FunctionReturnInfo::default(),
            docs: None,
        }
    }

    /// Create a new function info with a name and namespace.
    pub fn new_for(name: Cow<'static, str>, namespace: Namespace) -> Self {
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
    pub fn with_namespace(mut self, namespace: Namespace) -> Self {
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
        type_info_provider: Option<&dyn GetTypeInfo>,
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
        type_info_provider: Option<&dyn GetTypeInfo>,
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
            fn get_function_info(&self, name: Cow<'static, str>, namespace: Namespace) -> FunctionInfo {
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
    use crate::{
        docgen::typed_through::UntypedWrapperKind,
        function::from::{Mut, Ref, Val},
    };

    use super::*;

    #[test]
    fn test_get_function_info() {
        fn test_fn(a: i32, b: f32) -> f64 {
            (a as f64) + (b as f64)
        }

        let info = test_fn.get_function_info(Cow::Borrowed("test_fn"), Namespace::Global);
        assert_eq!(info.name, "test_fn");
        assert_eq!(info.namespace, Namespace::Global);
        assert_eq!(info.arg_info.len(), 2);
        assert_eq!(info.return_info.type_id, TypeId::of::<f64>());

        assert_eq!(info.arg_info[0].type_id, TypeId::of::<i32>());
        assert_eq!(info.arg_info[1].type_id, TypeId::of::<f32>());

        match info.arg_info[0].type_info.as_ref().unwrap() {
            ThroughTypeInfo::TypeInfo(type_info) => {
                assert_eq!(type_info.type_id(), TypeId::of::<i32>());
            }
            _ => panic!("Expected TypeInfo"),
        }

        match info.arg_info[1].type_info.as_ref().unwrap() {
            ThroughTypeInfo::TypeInfo(type_info) => {
                assert_eq!(type_info.type_id(), TypeId::of::<f32>());
            }
            _ => panic!("Expected TypeInfo"),
        }
    }

    #[test]
    fn test_get_function_info_references() {
        let fn_ = |_: Ref<i32>, _: Mut<f32>| -> Val<f64> { Val::new(0.0) };

        let info = fn_.get_function_info(Cow::Borrowed("test_fn"), Namespace::Global);
        assert_eq!(info.name, "test_fn");
        assert_eq!(info.namespace, Namespace::Global);
        assert_eq!(info.arg_info.len(), 2);
        assert_eq!(info.return_info.type_id, TypeId::of::<Val<f64>>());

        assert_eq!(info.arg_info[0].type_id, TypeId::of::<Ref<'static, i32>>());
        assert_eq!(info.arg_info[1].type_id, TypeId::of::<Mut<'static, f32>>());

        match &info.arg_info[0].type_info {
            Some(ThroughTypeInfo::UntypedWrapper {
                through_type,
                wrapper_kind,
            }) => {
                assert_eq!(through_type.type_id(), TypeId::of::<i32>());
                assert_eq!(*wrapper_kind, UntypedWrapperKind::Ref);
            }
            _ => panic!("Expected UntypedWrapper"),
        }

        match &info.arg_info[1].type_info {
            Some(ThroughTypeInfo::UntypedWrapper {
                through_type,
                wrapper_kind,
            }) => {
                assert_eq!(through_type.type_id(), TypeId::of::<f32>());
                assert_eq!(*wrapper_kind, UntypedWrapperKind::Mut);
            }
            _ => panic!("Expected UntypedWrapper"),
        }
    }
}
