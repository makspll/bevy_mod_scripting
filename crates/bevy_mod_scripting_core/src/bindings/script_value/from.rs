use std::{
    any::TypeId,
    borrow::Cow,
    ffi::OsString,
    path::{Path, PathBuf},
};

use bevy::reflect::{
    DynamicEnum, DynamicList, DynamicTuple, DynamicVariant, PartialReflect, ReflectFromReflect,
};

use crate::{
    bindings::{ReflectReference, WorldGuard},
    error::InteropError,
    reflection_extensions::TypeInfoExtensions,
};

use super::{FromScriptValue, ScriptValue};

impl FromScriptValue for dyn PartialReflect {
    fn from_script_value(
        value: ScriptValue,
        world: WorldGuard,
        target_type_id: TypeId,
    ) -> Option<Result<Box<dyn PartialReflect>, InteropError>> {
        match target_type_id {
            // TODO: if these types ever support reflect, we can uncomment these lines
            // For some of these we specifically require the borrowed static variant, this will never let you use a dynamically created string from the script
            // we should instead allocate and leak perhaps. then garbage collect later

            // support for arbitrary arg types
            t if t == TypeId::of::<ScriptValue>() => return Some(Ok(Box::new(value))),

            t if t == TypeId::of::<()>() => {
                return <()>::from_script_value(value, world, target_type_id)
            }
            t if t == TypeId::of::<bool>() => {
                return bool::from_script_value(value, world, target_type_id)
            }
            t if t == TypeId::of::<str>() => {
                return <&str>::from_script_value(value, world, target_type_id)
            }
            // t if t == TypeId::of::<CStr>() => return <CStr>::from_script_value(value, world, target_type_id),
            // t if t == TypeId::of::<OsStr>() => return <OsStr>::from_script_value(value, world, target_type_id),
            t if t == TypeId::of::<Path>() => {
                return <&Path>::from_script_value(value, world, target_type_id)
            }
            t if t == TypeId::of::<Cow<'static, str>>() => {
                return <Cow<'static, str>>::from_script_value(value, world, target_type_id)
            }
            // t if t == TypeId::of::<Cow<'static, CStr>>() => return <Cow<'static, CStr>>::from_script_value(value, world, target_type_id),
            t if t == TypeId::of::<f32>() => {
                return f32::from_script_value(value, world, target_type_id)
            }
            t if t == TypeId::of::<f64>() => {
                return f64::from_script_value(value, world, target_type_id)
            }
            t if t == TypeId::of::<i8>() => {
                return i8::from_script_value(value, world, target_type_id)
            }
            t if t == TypeId::of::<i16>() => {
                return i16::from_script_value(value, world, target_type_id)
            }
            t if t == TypeId::of::<i32>() => {
                return i32::from_script_value(value, world, target_type_id)
            }
            t if t == TypeId::of::<i64>() => {
                return i64::from_script_value(value, world, target_type_id)
            }
            t if t == TypeId::of::<i128>() => {
                return i128::from_script_value(value, world, target_type_id)
            }
            t if t == TypeId::of::<isize>() => {
                return isize::from_script_value(value, world, target_type_id)
            }
            t if t == TypeId::of::<u8>() => {
                return u8::from_script_value(value, world, target_type_id)
            }
            t if t == TypeId::of::<u16>() => {
                return u16::from_script_value(value, world, target_type_id)
            }
            t if t == TypeId::of::<u32>() => {
                return u32::from_script_value(value, world, target_type_id)
            }
            t if t == TypeId::of::<u64>() => {
                return u64::from_script_value(value, world, target_type_id)
            }
            t if t == TypeId::of::<u128>() => {
                return u128::from_script_value(value, world, target_type_id)
            }
            t if t == TypeId::of::<usize>() => {
                return usize::from_script_value(value, world, target_type_id);
            }
            // t if t == TypeId::of::<Box<str>>() => return <Box<str>>::from_script_value(value, world, target_type_id),
            // t if t == TypeId::of::<CString>() => return <CString>::from_script_value(value, world, target_type_id),
            t if t == TypeId::of::<String>() => {
                return <String>::from_script_value(value, world, target_type_id)
            }
            t if t == TypeId::of::<OsString>() => {
                return <OsString>::from_script_value(value, world, target_type_id)
            }
            t if t == TypeId::of::<PathBuf>() => {
                return <PathBuf>::from_script_value(value, world, target_type_id)
            }
            _ => {}
        };

        if let Some(opt) = <Option<&dyn PartialReflect>>::from_script_value(
            value.clone(),
            world.clone(),
            target_type_id,
        ) {
            Some(opt)
        } else if let Some(vec) = <Vec<&dyn PartialReflect>>::from_script_value(
            value.clone(),
            world.clone(),
            target_type_id,
        ) {
            Some(vec)
        } else {
            ReflectReference::from_script_value(value, world.clone(), target_type_id)
        }
    }
}

impl FromScriptValue for Option<&dyn PartialReflect> {
    fn from_script_value(
        value: ScriptValue,
        world: WorldGuard,
        target_type_id: TypeId,
    ) -> Option<Result<Box<dyn PartialReflect>, InteropError>> {
        let type_registry = world.type_registry();
        let type_registry = type_registry.read();
        let type_info = type_registry.get_type_info(target_type_id)?;

        if !type_info.is_option() {
            return None;
        };

        let inner_type_id = type_info.option_inner_type().expect("invariant");

        let dynamic = match value {
            ScriptValue::Unit => {
                let mut dynamic_none = DynamicEnum::new("None", DynamicVariant::Unit);
                dynamic_none.set_represented_type(Some(type_info));
                Box::new(dynamic_none)
            }
            v => {
                let inner = match <dyn PartialReflect>::from_script_value(
                    v,
                    world.clone(),
                    inner_type_id,
                ) {
                    Some(Ok(inner)) => inner,
                    Some(Err(e)) => return Some(Err(e)),
                    None => return None,
                };

                let mut dynamic_some = DynamicEnum::new(
                    "Some",
                    DynamicVariant::Tuple(DynamicTuple::from_iter(vec![inner])),
                );
                dynamic_some.set_represented_type(Some(type_info));
                Box::new(dynamic_some)
            }
        };

        match type_registry.get_type_data::<ReflectFromReflect>(target_type_id) {
            Some(from_reflect) => from_reflect
                .from_reflect(dynamic.as_partial_reflect())
                .map(|v| Ok(v.into_partial_reflect())),
            None => Some(Err(InteropError::missing_type_data(
                target_type_id,
                "ReflectFromReflect".to_owned(),
            ))),
        }
    }
}

impl FromScriptValue for Vec<&dyn PartialReflect> {
    fn from_script_value(
        value: ScriptValue,
        world: WorldGuard,
        target_type_id: TypeId,
    ) -> Option<Result<Box<dyn PartialReflect>, InteropError>> {
        let type_registry = world.type_registry();
        let type_registry = type_registry.read();
        let type_info = type_registry.get_type_info(target_type_id)?;

        if !type_info.is_list() {
            return None;
        };

        let inner_type_id = type_info.list_inner_type().expect("invariant");

        let dynamic = match value {
            ScriptValue::List(vec) => {
                let mut dynamic_list = DynamicList::default();
                dynamic_list.set_represented_type(Some(type_info));

                for v in vec.into_iter() {
                    let inner = match <dyn PartialReflect>::from_script_value(
                        v,
                        world.clone(),
                        inner_type_id,
                    ) {
                        Some(Ok(inner)) => inner,
                        Some(Err(e)) => return Some(Err(e)),
                        None => return None,
                    };

                    dynamic_list.push_box(inner);
                }
                Box::new(dynamic_list)
            }
            ScriptValue::Reference(reflect_reference) => {
                // for references we assume they point to a list already, we can safely
                return Some(reflect_reference.to_owned_value(world));
            }
            _ => return Some(Err(InteropError::value_mismatch(target_type_id, value))),
        };

        match type_registry.get_type_data::<ReflectFromReflect>(target_type_id) {
            Some(from_reflect) => from_reflect
                .from_reflect(dynamic.as_partial_reflect())
                .map(|v| Ok(v.into_partial_reflect())),
            None => Some(Err(InteropError::missing_type_data(
                target_type_id,
                "ReflectFromReflect".to_owned(),
            ))),
        }
    }
}

impl FromScriptValue for ReflectReference {
    fn from_script_value(
        value: ScriptValue,
        world: WorldGuard,
        target_type: TypeId,
    ) -> Option<Result<Box<dyn PartialReflect>, InteropError>> {
        match value {
            ScriptValue::Reference(ref_) => Some(ref_.to_owned_value(world)),
            _ => None,
        }
    }
}

impl FromScriptValue for bool {
    fn from_script_value(
        value: ScriptValue,
        world: WorldGuard,
        target_type: TypeId,
    ) -> Option<Result<Box<dyn PartialReflect>, InteropError>> {
        if target_type == TypeId::of::<bool>() {
            Some(match value {
                ScriptValue::Bool(v) => Ok(Box::new(v)),
                ScriptValue::Reference(ref_) => ref_
                    .downcast::<bool>(world)
                    .map(|v| Box::new(v) as Box<dyn PartialReflect>),
                _ => Err(InteropError::value_mismatch(target_type, value)),
            })
        } else {
            None
        }
    }
}

impl FromScriptValue for &'static str {
    fn from_script_value(
        value: ScriptValue,
        world: WorldGuard,
        target_type: TypeId,
    ) -> Option<Result<Box<dyn PartialReflect>, InteropError>> {
        if target_type == TypeId::of::<&'static str>() {
            Some(match value {
                ScriptValue::String(Cow::Borrowed(s)) => Ok(Box::new(s)),
                ScriptValue::Reference(ref_) => ref_
                    .downcast::<&'static str>(world)
                    .map(|v| Box::new(v) as Box<dyn PartialReflect>),
                _ => Err(InteropError::value_mismatch(target_type, value)).into(),
            })
        } else {
            None
        }
    }
}

impl FromScriptValue for Cow<'static, str> {
    fn from_script_value(
        value: ScriptValue,
        world: WorldGuard,
        target_type: TypeId,
    ) -> Option<Result<Box<dyn PartialReflect>, InteropError>> {
        if target_type == TypeId::of::<Cow<'static, str>>() {
            Some(match value {
                ScriptValue::String(s) => Ok(Box::new(s)),
                ScriptValue::Reference(ref_) => ref_
                    .downcast::<Cow<'static, str>>(world)
                    .map(|v| Box::new(v) as Box<dyn PartialReflect>),
                _ => Err(InteropError::value_mismatch(target_type, value)).into(),
            })
        } else {
            None
        }
    }
}

impl FromScriptValue for String {
    fn from_script_value(
        value: ScriptValue,
        world: WorldGuard,
        target_type: TypeId,
    ) -> Option<Result<Box<dyn PartialReflect>, InteropError>> {
        if target_type == TypeId::of::<String>() {
            Some(match value {
                ScriptValue::String(s) => Ok(Box::new(s.into_owned())),
                ScriptValue::Reference(ref_) => ref_
                    .downcast::<String>(world)
                    .map(|v| Box::new(v) as Box<dyn PartialReflect>),
                _ => Err(InteropError::value_mismatch(target_type, value)).into(),
            })
        } else {
            None
        }
    }
}

impl FromScriptValue for f32 {
    fn from_script_value(
        value: ScriptValue,
        world: WorldGuard,
        target_type: TypeId,
    ) -> Option<Result<Box<dyn PartialReflect>, InteropError>> {
        if target_type == TypeId::of::<f32>() {
            Some(match value {
                ScriptValue::Float(v) => Ok(Box::new(v as f32)),
                ScriptValue::Integer(v) => Ok(Box::new(v as f32)),
                ScriptValue::Reference(ref_) => ref_
                    .downcast::<f32>(world)
                    .map(|v| Box::new(v) as Box<dyn PartialReflect>),
                _ => Err(InteropError::value_mismatch(target_type, value)).into(),
            })
        } else {
            None
        }
    }
}

impl FromScriptValue for f64 {
    fn from_script_value(
        value: ScriptValue,
        world: WorldGuard,
        target_type: TypeId,
    ) -> Option<Result<Box<dyn PartialReflect>, InteropError>> {
        if target_type == TypeId::of::<f64>() {
            Some(match value {
                ScriptValue::Float(v) => Ok(Box::new(v)),
                ScriptValue::Integer(v) => Ok(Box::new(v as f64)),
                ScriptValue::Reference(ref_) => ref_
                    .downcast::<f64>(world)
                    .map(|v| Box::new(v) as Box<dyn PartialReflect>),
                _ => Err(InteropError::value_mismatch(target_type, value)).into(),
            })
        } else {
            None
        }
    }
}

// Macro to implement FromScriptValue for integer types
macro_rules! impl_from_script_value_integer {
    ($($t:ty),*) => {
        $(
            impl FromScriptValue for $t {
                fn from_script_value(
                    value: ScriptValue,
                    world: WorldGuard,
                    target_type: TypeId,
                ) -> Option<Result<Box<dyn PartialReflect>, InteropError>> {
                    if target_type == TypeId::of::<$t>() {
                        Some(match value {
                            ScriptValue::Integer(v) => Ok(Box::new(v as $t)),
                            ScriptValue::Float(v) => Ok(Box::new(v as $t)),
                            ScriptValue::Reference(ref_) => ref_
                                .downcast::<$t>(world)
                                .map(|v| Box::new(v) as Box<dyn PartialReflect>),
                            _ => Err(InteropError::value_mismatch(
                                target_type,
                                value,
                            )),
                        })
                    } else {
                        None
                    }
                }
            }
        )*
    };
}

impl_from_script_value_integer!(i8, i16, i32, i64, i128, isize);
impl_from_script_value_integer!(u8, u16, u32, u64, u128, usize);

impl FromScriptValue for &'static Path {
    fn from_script_value(
        value: ScriptValue,
        world: WorldGuard,
        target_type: TypeId,
    ) -> Option<Result<Box<dyn PartialReflect>, InteropError>> {
        if target_type == TypeId::of::<&'static Path>() {
            Some(match value {
                ScriptValue::String(Cow::Borrowed(s)) => Ok(Box::new(Path::new(s))),
                _ => Err(InteropError::value_mismatch(target_type, value)),
            })
        } else {
            None
        }
    }
}

impl FromScriptValue for PathBuf {
    fn from_script_value(
        value: ScriptValue,
        world: WorldGuard,
        target_type: TypeId,
    ) -> Option<Result<Box<dyn PartialReflect>, InteropError>> {
        if target_type == TypeId::of::<PathBuf>() {
            Some(match value {
                ScriptValue::String(s) => Ok(Box::new(PathBuf::from(s.into_owned()))),
                ScriptValue::Reference(ref_) => ref_
                    .downcast::<PathBuf>(world)
                    .map(|v| Box::new(v) as Box<dyn PartialReflect>),
                _ => Err(InteropError::value_mismatch(target_type, value)),
            })
        } else {
            None
        }
    }
}

// Implementations for additional types

// impl FromScriptValue for Box<str> {
//     fn from_script_value(
//         target_type: TypeId,
//         value: ScriptValue,
//         world: WorldGuard,
//     ) -> ScriptResult<Box<dyn PartialReflect>> {
//         if target_type == TypeId::of::<Box<str>>() {
//             match value {
//                 ScriptValue::String(s) => Ok(Box::new(s.into_owned().into_boxed_str())),
//                 _ => Err(InteropError::TypeMismatch {
//                     expected_type: Cow::Owned(target_type.display_with_world(world)),
//                     actual_type: Some(Cow::Owned(value.display_with_world(world))),
//                 }
//                 .into()),
//             }
//         } else {
//             Err(InteropError::TypeMismatch {
//                 expected_type: Cow::Owned(target_type.display_with_world(world)),
//                 actual_type: Some(Cow::Borrowed(type_name::<Box<str>>())),
//             }
//             .into())
//         }
//     }
// }

// impl FromScriptValue for CString {
//     fn from_script_value(
//         target_type: TypeId,
//         value: ScriptValue,
//         world: WorldGuard,
//     ) -> ScriptResult<Box<dyn PartialReflect>> {
//         if target_type == TypeId::of::<CString>() {
//             match value {
//                 ScriptValue::String(s) => CString::new(s.into_owned())
//                     .map(|cstr| Box::new(cstr) as Box<dyn PartialReflect>)
//                     .map_err(|e| {
//                         InteropError::TypeMismatch {
//                             expected_type: Cow::Owned(target_type.display_with_world(world)),
//                             actual_type: Some(Cow::Owned(e.to_string())),
//                         }
//                         .into()
//                     }),
//                 _ => Err(InteropError::TypeMismatch {
//                     expected_type: Cow::Owned(target_type.display_with_world(world)),
//                     actual_type: Some(Cow::Owned(value.display_with_world(world))),
//                 }
//                 .into()),
//             }
//         } else {
//             Err(InteropError::TypeMismatch {
//                 expected_type: Cow::Owned(target_type.display_with_world(world)),
//                 actual_type: Some(Cow::Borrowed(type_name::<CString>())),
//             }
//             .into())
//         }
//     }
// }

impl FromScriptValue for OsString {
    fn from_script_value(
        value: ScriptValue,
        world: WorldGuard,
        target_type: TypeId,
    ) -> Option<Result<Box<dyn PartialReflect>, InteropError>> {
        if target_type == TypeId::of::<OsString>() {
            Some(match value {
                ScriptValue::String(s) => Ok(Box::new(OsString::from(s.into_owned()))),
                ScriptValue::Reference(ref_) => ref_
                    .downcast::<OsString>(world)
                    .map(|v| Box::new(v) as Box<dyn PartialReflect>),
                _ => Err(InteropError::value_mismatch(target_type, value)).into(),
            })
        } else {
            None
        }
    }
}

// impl FromScriptValue for &'static OsStr {
//     fn from_script_value(
//         target_type: TypeId,
//         value: ScriptValue,
//         world: WorldGuard,
//     ) -> ScriptResult<Box<dyn PartialReflect>> {
//         if target_type == TypeId::of::<&'static OsStr>() {
//             match value {
//                 ScriptValue::String(Cow::Borrowed(s)) => Ok(Box::new(OsStr::new(s))),
//                 _ => Err(InteropError::TypeMismatch {
//                     expected_type: Cow::Owned(target_type.display_with_world(world)),
//                     actual_type: Some(Cow::Owned(value.display_with_world(world))),
//                 }
//                 .into()),
//             }
//         } else {
//             Err(InteropError::TypeMismatch {
//                 expected_type: Cow::Owned(target_type.display_with_world(world)),
//                 actual_type: Some(Cow::Borrowed(type_name::<&'static OsStr>())),
//             }
//             .into())
//         }
//     }
// }

// impl FromScriptValue for &'static CStr {
//     fn from_script_value(
//         target_type: TypeId,
//         value: ScriptValue,
//         world: WorldGuard,
//     ) -> ScriptResult<Box<dyn PartialReflect>> {
//         if target_type == TypeId::of::<&'static CStr>() {
//             match value {
//                 ScriptValue::String(Cow::Borrowed(s)) => {
//                     let bytes = s.as_bytes();
//                     CStr::from_bytes_with_nul(bytes)
//                         .map(|cstr| Box::new(cstr) as Box<dyn PartialReflect>)
//                         .map_err(|e| {
//                             InteropError::TypeMismatch {
//                                 expected_type: Cow::Owned(target_type.display_with_world(world)),
//                                 actual_type: Some(Cow::Owned(e.to_string())),
//                             }
//                             .into()
//                         })
//                 }
//                 _ => Err(InteropError::TypeMismatch {
//                     expected_type: Cow::Owned(target_type.display_with_world(world)),
//                     actual_type: Some(Cow::Owned(value.display_with_world(world))),
//                 }
//                 .into()),
//             }
//         } else {
//             Err(InteropError::TypeMismatch {
//                 expected_type: Cow::Owned(target_type.display_with_world(world)),
//                 actual_type: Some(Cow::Borrowed(type_name::<&'static CStr>())),
//             }
//             .into())
//         }
//     }
// }
