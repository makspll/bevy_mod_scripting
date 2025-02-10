//! Information about functions and their arguments.

use bevy::reflect::Reflect;

use crate::bindings::function::from::{Mut, Val};
use crate::bindings::function::namespace::Namespace;
use crate::bindings::function::{arg_meta::ArgMeta, from::Ref};
use std::{any::TypeId, borrow::Cow};

/// for things you can call and provide some introspection capability.
pub trait GetFunctionInfo<Marker> {
    /// Get the function info for the function.
    fn get_function_info(&self, name: Cow<'static, str>, namespace: Namespace) -> FunctionInfo;
}

#[derive(Debug, Clone, PartialEq, Reflect)]
/// Information about a function.
pub struct FunctionInfo {
    /// The name of the function.
    pub name: Cow<'static, str>,
    /// The namespace of the function.
    pub namespace: Namespace,
    /// Information about the arguments of the function.
    pub arg_info: Vec<FunctionArgInfo>,
    /// Information about the return value of the function.
    pub return_info: Option<FunctionReturnInfo>,
    /// Documentation for the function.
    pub docs: Option<Cow<'static, str>>,
}

impl Default for FunctionInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl FunctionInfo {
    /// Create a new function info with default values.
    pub fn new() -> Self {
        Self {
            name: Cow::Borrowed(""),
            namespace: Namespace::Global,
            arg_info: Vec::new(),
            return_info: None,
            docs: None,
        }
    }

    /// Create a new function info with a name and namespace.
    pub fn new_for(name: Cow<'static, str>, namespace: Namespace) -> Self {
        Self {
            name,
            namespace,
            arg_info: Vec::new(),
            return_info: None,
            docs: None,
        }
    }

    /// Add an argument to the function info.
    pub fn add_arg<T: ArgMeta + 'static>(mut self, name: Option<Cow<'static, str>>) -> Self {
        self.arg_info.push(FunctionArgInfo {
            name,
            arg_index: self.arg_info.len(),
            type_id: TypeId::of::<T>(),
            type_kind: TypeKind::new_for::<T>(),
        });
        self
    }

    /// Add a return value to the function info.
    pub fn add_return<T: 'static>(mut self) -> Self {
        self.return_info = Some(FunctionReturnInfo::new_for::<T>());
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

/// The kind of type being used as a return value or argument in a script function.
#[derive(Debug, Clone, PartialEq, Reflect)]
pub enum TypeKind {
    /// A `Ref` wrapped type
    Ref,
    /// A `Mut` wrapped type
    Mut,
    /// A `Val` wrapped type
    Val,
    /// Any other type, such as a primtive or something which implements `IntoScript` or `FromScript` directly
    Primitive,
}

impl TypeKind {
    /// Create a new type kind for a specific type.
    pub fn new_for<T: 'static>() -> Self {
        let type_id = TypeId::of::<T>();
        if std::any::TypeId::of::<Ref<T>>() == type_id {
            Self::Ref
        } else if std::any::TypeId::of::<Mut<T>>() == type_id {
            Self::Mut
        } else if std::any::TypeId::of::<Val<T>>() == type_id {
            Self::Val
        } else {
            Self::Primitive
        }
    }
}

#[derive(Debug, Clone, PartialEq, Reflect)]
/// Information about a function argument.
pub struct FunctionArgInfo {
    /// The name of the argument.
    pub name: Option<Cow<'static, str>>,
    /// The index of the argument.
    pub arg_index: usize,
    /// The type of the argument.
    pub type_id: TypeId,
    /// The kind of type being used as an argument.
    pub type_kind: TypeKind,
}

impl FunctionArgInfo {
    /// Create a new function argument info with a name.
    pub fn with_name(mut self, name: Cow<'static, str>) -> Self {
        self.name = Some(name);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Reflect)]
/// Information about a function return value.
pub struct FunctionReturnInfo {
    /// The type of the return value.
    pub type_id: TypeId,
    /// The kind of type being used as a return value.
    pub type_kind: TypeKind,
}

impl FunctionReturnInfo {
    /// Create a new function return info for a specific type.
    pub fn new_for<T: 'static>() -> Self {
        Self {
            type_id: TypeId::of::<T>(),
            type_kind: TypeKind::new_for::<T>(),
        }
    }
}

macro_rules! impl_documentable {
    ($( $param:ident ),*) => {
        impl<$($param,)* F, O> GetFunctionInfo<fn($($param),*) -> O> for F
            where
            F: Fn($($param),*) -> O,
            $($param: ArgMeta + 'static,)*
            O: 'static
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

bevy::utils::all_tuples!(impl_documentable, 0, 13, T);

#[cfg(test)]
mod test {
    use crate::bindings::function::from::{Mut, Ref, Val};

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
        assert_eq!(info.return_info.unwrap().type_id, TypeId::of::<f64>());

        assert_eq!(info.arg_info[0].type_id, TypeId::of::<i32>());
        assert_eq!(info.arg_info[1].type_id, TypeId::of::<f32>());
    }

    #[test]
    fn test_get_function_info_references() {
        let fn_ = |_: Ref<i32>, _: Mut<f32>| -> Val<f64> { Val::new(0.0) };

        let info = fn_.get_function_info(Cow::Borrowed("test_fn"), Namespace::Global);
        assert_eq!(info.name, "test_fn");
        assert_eq!(info.namespace, Namespace::Global);
        assert_eq!(info.arg_info.len(), 2);
        assert_eq!(info.return_info.unwrap().type_id, TypeId::of::<Val<f64>>());

        assert_eq!(info.arg_info[0].type_id, TypeId::of::<Ref<'static, i32>>());
        assert_eq!(info.arg_info[1].type_id, TypeId::of::<Mut<'static, f32>>());
    }
}
