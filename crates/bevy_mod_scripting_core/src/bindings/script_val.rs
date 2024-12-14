use std::{
    any::{type_name, TypeId},
    borrow::Cow,
    ffi::{CStr, CString, OsStr, OsString},
    path::{Path, PathBuf},
};

use bevy::reflect::{
    Access, DynamicEnum, DynamicTuple, DynamicVariant, OffsetAccess, ParsedPath, PartialReflect,
};

use crate::{
    error::{ScriptError, ScriptResult, ValueConversionError},
    reflection_extensions::{PartialReflectExt, TypeInfoExtensions},
};

use super::{
    pretty_print::DisplayWithWorld, ReflectReference, ReflectionPathElem, WorldAccessGuard,
    WorldGuard,
};

/// An abstraction of values that can be passed to and from scripts.
/// This allows us to re-use logic between scripting languages.
#[derive(Debug, Clone, PartialEq)]
pub enum ScriptValue {
    Unit,
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(Cow<'static, str>),
    Reference(ReflectReference),
    World,
}

impl TryFrom<ScriptValue> for ReflectionPathElem {
    type Error = ScriptError;
    fn try_from(value: ScriptValue) -> Result<Self, Self::Error> {
        Ok(match value {
            ScriptValue::Integer(i) => {
                ReflectionPathElem::Reflection(ParsedPath::from(vec![OffsetAccess {
                    access: bevy::reflect::Access::ListIndex(i as usize),
                    offset: Some(1),
                }]))
            }
            ScriptValue::Float(v) => {
                return Err(ValueConversionError::InvalidIndex {
                    index: v.to_string().into(),
                    base: None,
                    reason: Some("floating point numbers cannot be used as indices".into()),
                })?
            }
            ScriptValue::String(cow) => {
                if let Some(tuple_struct_index) = cow.strip_prefix("_") {
                    if let Ok(index) = tuple_struct_index.parse::<usize>() {
                        let parsed_path = ParsedPath::from(vec![OffsetAccess {
                            access: bevy::reflect::Access::TupleIndex(index),
                            offset: Some(1),
                        }]);
                        return Ok(ReflectionPathElem::Reflection(parsed_path));
                    }
                }

                let path = match cow {
                    Cow::Borrowed(v) => ParsedPath::parse_static(v).map_err(|e| {
                        ValueConversionError::InvalidIndex {
                            index: v.into(),
                            base: None,
                            reason: Some(e.to_string().into()),
                        }
                    })?,
                    Cow::Owned(o) => {
                        ParsedPath::parse(&o).map_err(|e| ValueConversionError::InvalidIndex {
                            index: o.clone().into(),
                            base: None,
                            reason: Some(e.to_string().into()),
                        })?
                    }
                };

                ReflectionPathElem::new_reflection(path)
            }
            ScriptValue::Reference(reflect_reference) => {
                return Err(ValueConversionError::InvalidIndex {
                    index: format!("{:?}", reflect_reference).into(),
                    base: None,
                    reason: Some("References cannot be used as indices".into()),
                })?
            }
            _ => ReflectionPathElem::Identity,
        })
    }
}

/// A trait for converting a value into a [`ScriptVal`].
pub trait IntoScriptValue {
    fn into_script_value(self, guard: WorldGuard) -> ScriptResult<ScriptValue>;
}

/// Targeted conversion from a [`ScriptValue`] to a specific type. Can create dynamic types as well as concrete types depending on the implementation.
pub trait FromScriptValue {
    fn from_script_value(
        value: ScriptValue,
        guard: WorldGuard,
        target_type: TypeId,
    ) -> ScriptResult<Box<dyn PartialReflect>>;
}

impl IntoScriptValue for ReflectReference {
    /// For ReflectReferences we do a bit more logic here.
    ///
    /// We don't want to output references when they are pointing to primitives for example.
    ///
    /// There are a few rules:
    /// - If we are pointing to a type that is better represented as another variant other than [`ScriptValue::Reference`], we will convert it to that variant.
    /// - If we are pointing to a type that is better represented as a [`ScriptValue::Reference`], we will keep it as is.
    fn into_script_value(self, world: WorldGuard) -> ScriptResult<ScriptValue> {
        self.clone().with_reflect(world.clone(), move |r| {
            // for primitives we want to convert to the primitive type
            // we do not need to retain the reference
            if let Some(prim) = r.as_primitive(world.clone()) {
                return Ok(prim);
            }

            // for options we want to convert to
            // - the inner type if it's some
            // - nil if it's none
            // to retain the reference we need to return a reference pointing into the inner type
            if let Ok(inner) = r.as_option() {
                if inner.is_some() {
                    let mut inner_ref = self;
                    inner_ref.index_path(ParsedPath::parse_static(".0").expect("invariant"));
                    // hint for the from impl that we want to trim the reference
                    inner_ref.index_path(ReflectionPathElem::Identity);
                    // cascade the conversion
                    return inner_ref.into_script_value(world);
                } else {
                    return Ok(ScriptValue::Unit);
                }
            }

            // as a last resort we just retain the original reference with no conversion
            Ok(ScriptValue::Reference(self))
        })?
    }
}

impl FromScriptValue for ScriptValue {
    fn from_script_value(
        value: ScriptValue,
        world: WorldGuard,
        target_type: TypeId,
    ) -> ScriptResult<Box<dyn PartialReflect>> {
        // is the target type an option?
        let type_registry = world.type_registry();
        let type_registry = type_registry.read();
        let type_info = type_registry.get_type_info(target_type).ok_or({
            ValueConversionError::MissingTypeInformation {
                type_id: target_type,
            }
        })?;

        // for primitives
        if let Some(value) =
            <dyn PartialReflect>::from_primitive(value.clone(), world.clone(), target_type)
        {
            return value;
        }

        if type_info.is_option() {
            // in this case we can expect a few things
            // - the value we're pointing to will:
            //    - either point to 'inside' the option
            //    - or to the whole option
            // - the Value might be either a Unit or a Reference
            let inner_type_id = type_info.option_inner_type().expect("invariant");

            match value {
                ScriptValue::Unit => {
                    // if the value is a unit it's easy, we just construct a dynamic none variant
                    let mut dynamic_none = DynamicEnum::new("None", DynamicVariant::Unit);
                    dynamic_none.set_represented_type(Some(type_info));
                    return Ok(Box::new(dynamic_none));
                }
                ScriptValue::Reference(mut reference) => {
                    // if the value is a reference we need to check if it's pointing to the inner value
                    // that will only be the case if the reference was created using into_script_value
                    // meaning it will end with a ".0" followed by Identity index.

                    if reference.reflect_path.last() == Some(&ReflectionPathElem::Identity)
                        && reference.reflect_path.len() > 2
                    {
                        let _ = reference
                            .reflect_path
                            .drain(reference.reflect_path.len() - 2..);
                    }

                    // construct dynamic variant
                    let out = reference.clone_value(world)?;
                    // out.set_represented_type(Some(type_info));
                    return Ok(out);
                }
                value => {
                    let inner =
                        ScriptValue::from_script_value(value, world.clone(), inner_type_id)?;

                    let mut dynamic_some = DynamicEnum::new(
                        "Some",
                        DynamicVariant::Tuple(DynamicTuple::from_iter(vec![inner])),
                    );
                    dynamic_some.set_represented_type(Some(type_info));
                    return Ok(Box::new(dynamic_some));
                }
            };
        }

        // if not primitive or option, we expect a reference
        if let ScriptValue::Reference(reflect_reference) = value {
            reflect_reference.clone_value(world)
        } else {
            return Err(ValueConversionError::TypeMismatch {
                expected_type: Cow::Owned(target_type.display_with_world(world.clone())),
                actual_type: Some(Cow::Owned(value.display_with_world(world))),
            })?;
        }
    }
}

impl IntoScriptValue for () {
    fn into_script_value(self, _: WorldGuard) -> ScriptResult<ScriptValue> {
        Ok(ScriptValue::Unit)
    }
}

impl IntoScriptValue for &'static str {
    fn into_script_value(self, _: WorldGuard) -> ScriptResult<ScriptValue> {
        Ok(ScriptValue::String(self.into()))
    }
}

impl IntoScriptValue for &'static CStr {
    fn into_script_value(self, _: WorldGuard) -> ScriptResult<ScriptValue> {
        Ok(ScriptValue::String(self.to_string_lossy()))
    }
}

impl IntoScriptValue for &'static OsStr {
    fn into_script_value(self, _: WorldGuard) -> ScriptResult<ScriptValue> {
        Ok(ScriptValue::String(self.to_string_lossy()))
    }
}

impl IntoScriptValue for &'static Path {
    fn into_script_value(self, _: WorldGuard) -> ScriptResult<ScriptValue> {
        Ok(ScriptValue::String(self.to_string_lossy()))
    }
}

impl IntoScriptValue for Cow<'static, str> {
    fn into_script_value(self, _: WorldGuard) -> ScriptResult<ScriptValue> {
        Ok(ScriptValue::String(self.into_owned().into()))
    }
}

impl IntoScriptValue for Cow<'static, CStr> {
    fn into_script_value(self, _: WorldGuard) -> ScriptResult<ScriptValue> {
        Ok(ScriptValue::String(
            self.to_string_lossy().into_owned().into(),
        ))
    }
}

impl IntoScriptValue for bool {
    fn into_script_value(self, _: WorldGuard) -> ScriptResult<ScriptValue> {
        Ok(ScriptValue::Bool(self))
    }
}

impl IntoScriptValue for f32 {
    fn into_script_value(self, _: WorldGuard) -> ScriptResult<ScriptValue> {
        Ok(ScriptValue::Float(self as f64))
    }
}

impl IntoScriptValue for f64 {
    fn into_script_value(self, _: WorldGuard) -> ScriptResult<ScriptValue> {
        Ok(ScriptValue::Float(self))
    }
}

impl IntoScriptValue for i8 {
    fn into_script_value(self, _: WorldGuard) -> ScriptResult<ScriptValue> {
        Ok(ScriptValue::Integer(self as i64))
    }
}

impl IntoScriptValue for i16 {
    fn into_script_value(self, _: WorldGuard) -> ScriptResult<ScriptValue> {
        Ok(ScriptValue::Integer(self as i64))
    }
}

impl IntoScriptValue for i32 {
    fn into_script_value(self, _: WorldGuard) -> ScriptResult<ScriptValue> {
        Ok(ScriptValue::Integer(self as i64))
    }
}

impl IntoScriptValue for i64 {
    fn into_script_value(self, _: WorldGuard) -> ScriptResult<ScriptValue> {
        Ok(ScriptValue::Integer(self))
    }
}

impl IntoScriptValue for i128 {
    fn into_script_value(self, _: WorldGuard) -> ScriptResult<ScriptValue> {
        Ok(ScriptValue::Integer(self as i64))
    }
}

impl IntoScriptValue for isize {
    fn into_script_value(self, _: WorldGuard) -> ScriptResult<ScriptValue> {
        Ok(ScriptValue::Integer(self as i64))
    }
}

impl IntoScriptValue for u8 {
    fn into_script_value(self, _: WorldGuard) -> ScriptResult<ScriptValue> {
        Ok(ScriptValue::Integer(self as i64))
    }
}

impl IntoScriptValue for u16 {
    fn into_script_value(self, _: WorldGuard) -> ScriptResult<ScriptValue> {
        Ok(ScriptValue::Integer(self as i64))
    }
}

impl IntoScriptValue for u32 {
    fn into_script_value(self, _: WorldGuard) -> ScriptResult<ScriptValue> {
        Ok(ScriptValue::Integer(self as i64))
    }
}

impl IntoScriptValue for u64 {
    fn into_script_value(self, _: WorldGuard) -> ScriptResult<ScriptValue> {
        Ok(ScriptValue::Integer(self as i64))
    }
}

impl IntoScriptValue for u128 {
    fn into_script_value(self, _: WorldGuard) -> ScriptResult<ScriptValue> {
        Ok(ScriptValue::Integer(self as i64))
    }
}

impl IntoScriptValue for usize {
    fn into_script_value(self, _: WorldGuard) -> ScriptResult<ScriptValue> {
        Ok(ScriptValue::Integer(self as i64))
    }
}

impl IntoScriptValue for Box<str> {
    fn into_script_value(self, _: WorldGuard) -> ScriptResult<ScriptValue> {
        Ok(ScriptValue::String(self.to_string().into()))
    }
}

impl IntoScriptValue for CString {
    fn into_script_value(self, _: WorldGuard) -> ScriptResult<ScriptValue> {
        Ok(ScriptValue::String(
            self.to_string_lossy().into_owned().into(),
        ))
    }
}

impl IntoScriptValue for String {
    fn into_script_value(self, _: WorldGuard) -> ScriptResult<ScriptValue> {
        Ok(ScriptValue::String(self.into()))
    }
}

impl IntoScriptValue for OsString {
    fn into_script_value(self, _: WorldGuard) -> ScriptResult<ScriptValue> {
        Ok(ScriptValue::String(
            self.to_string_lossy().into_owned().into(),
        ))
    }
}

impl IntoScriptValue for PathBuf {
    fn into_script_value(self, _: WorldGuard) -> ScriptResult<ScriptValue> {
        Ok(ScriptValue::String(
            self.to_string_lossy().into_owned().into(),
        ))
    }
}

impl FromScriptValue for () {
    fn from_script_value(
        value: ScriptValue,
        world: WorldGuard,
        target_type: TypeId,
    ) -> ScriptResult<Box<dyn PartialReflect>> {
        if target_type == TypeId::of::<()>() {
            match value {
                ScriptValue::Unit => Ok(Box::new(())),
                ScriptValue::Reference(ref_) => Ok(Box::new(ref_.downcast::<()>(world)?)),
                _ => Err(ValueConversionError::TypeMismatch {
                    expected_type: Cow::Owned(target_type.display_with_world(world.clone())),
                    actual_type: Some(Cow::Owned(value.display_with_world(world))),
                }
                .into()),
            }
        } else {
            Err(ValueConversionError::TypeMismatch {
                expected_type: Cow::Owned(target_type.display_with_world(world)),
                actual_type: Some(Cow::Borrowed(type_name::<()>())),
            }
            .into())
        }
    }
}

impl FromScriptValue for bool {
    fn from_script_value(
        value: ScriptValue,
        world: WorldGuard,
        target_type: TypeId,
    ) -> ScriptResult<Box<dyn PartialReflect>> {
        if target_type == TypeId::of::<bool>() {
            match value {
                ScriptValue::Bool(v) => Ok(Box::new(v)),
                ScriptValue::Reference(ref_) => Ok(Box::new(ref_.downcast::<bool>(world)?)),
                _ => Err(ValueConversionError::TypeMismatch {
                    expected_type: Cow::Owned(target_type.display_with_world(world.clone())),
                    actual_type: Some(Cow::Owned(value.display_with_world(world))),
                }
                .into()),
            }
        } else {
            Err(ValueConversionError::TypeMismatch {
                expected_type: Cow::Owned(target_type.display_with_world(world)),
                actual_type: Some(Cow::Borrowed(type_name::<bool>())),
            }
            .into())
        }
    }
}

impl FromScriptValue for &'static str {
    fn from_script_value(
        value: ScriptValue,
        world: WorldGuard,
        target_type: TypeId,
    ) -> ScriptResult<Box<dyn PartialReflect>> {
        if target_type == TypeId::of::<&'static str>() {
            match value {
                ScriptValue::String(Cow::Borrowed(s)) => Ok(Box::new(s)),
                ScriptValue::Reference(ref_) => Ok(Box::new(ref_.downcast::<&'static str>(world)?)),
                _ => Err(ValueConversionError::TypeMismatch {
                    expected_type: Cow::Owned(target_type.display_with_world(world.clone())),
                    actual_type: Some(Cow::Owned(value.display_with_world(world))),
                }
                .into()),
            }
        } else {
            Err(ValueConversionError::TypeMismatch {
                expected_type: Cow::Owned(target_type.display_with_world(world)),
                actual_type: Some(Cow::Borrowed(type_name::<&'static str>())),
            }
            .into())
        }
    }
}

impl FromScriptValue for Cow<'static, str> {
    fn from_script_value(
        value: ScriptValue,
        world: WorldGuard,
        target_type: TypeId,
    ) -> ScriptResult<Box<dyn PartialReflect>> {
        if target_type == TypeId::of::<Cow<'static, str>>() {
            match value {
                ScriptValue::String(s) => Ok(Box::new(s)),
                ScriptValue::Reference(ref_) => {
                    Ok(Box::new(ref_.downcast::<Cow<'static, str>>(world)?))
                }
                _ => Err(ValueConversionError::TypeMismatch {
                    expected_type: Cow::Owned(target_type.display_with_world(world.clone())),
                    actual_type: Some(Cow::Owned(value.display_with_world(world))),
                }
                .into()),
            }
        } else {
            Err(ValueConversionError::TypeMismatch {
                expected_type: Cow::Owned(target_type.display_with_world(world)),
                actual_type: Some(Cow::Borrowed(type_name::<Cow<'static, str>>())),
            }
            .into())
        }
    }
}

impl FromScriptValue for String {
    fn from_script_value(
        value: ScriptValue,
        world: WorldGuard,
        target_type: TypeId,
    ) -> ScriptResult<Box<dyn PartialReflect>> {
        if target_type == TypeId::of::<String>() {
            match value {
                ScriptValue::String(s) => Ok(Box::new(s.into_owned())),
                ScriptValue::Reference(ref_) => Ok(Box::new(ref_.downcast::<String>(world)?)),
                _ => Err(ValueConversionError::TypeMismatch {
                    expected_type: Cow::Owned(target_type.display_with_world(world.clone())),
                    actual_type: Some(Cow::Owned(value.display_with_world(world))),
                }
                .into()),
            }
        } else {
            Err(ValueConversionError::TypeMismatch {
                expected_type: Cow::Owned(target_type.display_with_world(world)),
                actual_type: Some(Cow::Borrowed(type_name::<String>())),
            }
            .into())
        }
    }
}

impl FromScriptValue for f32 {
    fn from_script_value(
        value: ScriptValue,
        world: WorldGuard,
        target_type: TypeId,
    ) -> ScriptResult<Box<dyn PartialReflect>> {
        if target_type == TypeId::of::<f32>() {
            match value {
                ScriptValue::Float(v) => Ok(Box::new(v as f32)),
                ScriptValue::Integer(v) => Ok(Box::new(v as f32)),
                ScriptValue::Reference(ref_) => Ok(Box::new(ref_.downcast::<f32>(world)?)),
                _ => Err(ValueConversionError::TypeMismatch {
                    expected_type: Cow::Owned(target_type.display_with_world(world.clone())),
                    actual_type: Some(Cow::Owned(value.display_with_world(world))),
                }
                .into()),
            }
        } else {
            Err(ValueConversionError::TypeMismatch {
                expected_type: Cow::Owned(target_type.display_with_world(world)),
                actual_type: Some(Cow::Borrowed(type_name::<f32>())),
            }
            .into())
        }
    }
}

impl FromScriptValue for f64 {
    fn from_script_value(
        value: ScriptValue,
        world: WorldGuard,
        target_type: TypeId,
    ) -> ScriptResult<Box<dyn PartialReflect>> {
        if target_type == TypeId::of::<f64>() {
            match value {
                ScriptValue::Float(v) => Ok(Box::new(v)),
                ScriptValue::Integer(v) => Ok(Box::new(v as f64)),
                ScriptValue::Reference(ref_) => Ok(Box::new(ref_.downcast::<f64>(world)?)),
                _ => Err(ValueConversionError::TypeMismatch {
                    expected_type: Cow::Owned(target_type.display_with_world(world.clone())),
                    actual_type: Some(Cow::Owned(value.display_with_world(world))),
                }
                .into()),
            }
        } else {
            Err(ValueConversionError::TypeMismatch {
                expected_type: Cow::Owned(target_type.display_with_world(world)),
                actual_type: Some(Cow::Borrowed(type_name::<f64>())),
            }
            .into())
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
                ) -> ScriptResult<Box<dyn PartialReflect>> {
                    if target_type == TypeId::of::<$t>() {
                        match value {
                            ScriptValue::Integer(v) => Ok(Box::new(v as $t)),
                            ScriptValue::Float(v) => Ok(Box::new(v as $t)),
                            ScriptValue::Reference(ref_) => Ok(Box::new(ref_.downcast::<$t>(world)?)),
                            _ => Err(ValueConversionError::TypeMismatch {
                                expected_type: Cow::Owned(target_type.display_with_world(world.clone())),
                                actual_type: Some(Cow::Owned(value.display_with_world(world))),
                            }
                            .into()),
                        }
                    } else {
                        Err(ValueConversionError::TypeMismatch {
                            expected_type: Cow::Owned(target_type.display_with_world(world)),
                            actual_type: Some(Cow::Borrowed(type_name::<$t>())),
                        }
                        .into())
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
    ) -> ScriptResult<Box<dyn PartialReflect>> {
        if target_type == TypeId::of::<&'static Path>() {
            match value {
                ScriptValue::String(Cow::Borrowed(s)) => Ok(Box::new(Path::new(s))),
                _ => Err(ValueConversionError::TypeMismatch {
                    expected_type: Cow::Owned(target_type.display_with_world(world.clone())),
                    actual_type: Some(Cow::Owned(value.display_with_world(world))),
                }
                .into()),
            }
        } else {
            Err(ValueConversionError::TypeMismatch {
                expected_type: Cow::Owned(target_type.display_with_world(world)),
                actual_type: Some(Cow::Borrowed(type_name::<&'static Path>())),
            }
            .into())
        }
    }
}

impl FromScriptValue for PathBuf {
    fn from_script_value(
        value: ScriptValue,
        world: WorldGuard,
        target_type: TypeId,
    ) -> ScriptResult<Box<dyn PartialReflect>> {
        if target_type == TypeId::of::<PathBuf>() {
            match value {
                ScriptValue::String(s) => Ok(Box::new(PathBuf::from(s.into_owned()))),
                ScriptValue::Reference(ref_) => {
                    Ok(Box::new(ref_.downcast::<&'static Path>(world)?))
                }
                _ => Err(ValueConversionError::TypeMismatch {
                    expected_type: Cow::Owned(target_type.display_with_world(world.clone())),
                    actual_type: Some(Cow::Owned(value.display_with_world(world))),
                }
                .into()),
            }
        } else {
            Err(ValueConversionError::TypeMismatch {
                expected_type: Cow::Owned(target_type.display_with_world(world)),
                actual_type: Some(Cow::Borrowed(type_name::<PathBuf>())),
            }
            .into())
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
//                 _ => Err(ValueConversionError::TypeMismatch {
//                     expected_type: Cow::Owned(target_type.display_with_world(world)),
//                     actual_type: Some(Cow::Owned(value.display_with_world(world))),
//                 }
//                 .into()),
//             }
//         } else {
//             Err(ValueConversionError::TypeMismatch {
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
//                         ValueConversionError::TypeMismatch {
//                             expected_type: Cow::Owned(target_type.display_with_world(world)),
//                             actual_type: Some(Cow::Owned(e.to_string())),
//                         }
//                         .into()
//                     }),
//                 _ => Err(ValueConversionError::TypeMismatch {
//                     expected_type: Cow::Owned(target_type.display_with_world(world)),
//                     actual_type: Some(Cow::Owned(value.display_with_world(world))),
//                 }
//                 .into()),
//             }
//         } else {
//             Err(ValueConversionError::TypeMismatch {
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
    ) -> ScriptResult<Box<dyn PartialReflect>> {
        if target_type == TypeId::of::<OsString>() {
            match value {
                ScriptValue::String(s) => Ok(Box::new(OsString::from(s.into_owned()))),
                ScriptValue::Reference(ref_) => Ok(Box::new(ref_.downcast::<OsString>(world)?)),
                _ => Err(ValueConversionError::TypeMismatch {
                    expected_type: Cow::Owned(target_type.display_with_world(world.clone())),
                    actual_type: Some(Cow::Owned(value.display_with_world(world))),
                }
                .into()),
            }
        } else {
            Err(ValueConversionError::TypeMismatch {
                expected_type: Cow::Owned(target_type.display_with_world(world)),
                actual_type: Some(Cow::Borrowed(type_name::<OsString>())),
            }
            .into())
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
//                 _ => Err(ValueConversionError::TypeMismatch {
//                     expected_type: Cow::Owned(target_type.display_with_world(world)),
//                     actual_type: Some(Cow::Owned(value.display_with_world(world))),
//                 }
//                 .into()),
//             }
//         } else {
//             Err(ValueConversionError::TypeMismatch {
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
//                             ValueConversionError::TypeMismatch {
//                                 expected_type: Cow::Owned(target_type.display_with_world(world)),
//                                 actual_type: Some(Cow::Owned(e.to_string())),
//                             }
//                             .into()
//                         })
//                 }
//                 _ => Err(ValueConversionError::TypeMismatch {
//                     expected_type: Cow::Owned(target_type.display_with_world(world)),
//                     actual_type: Some(Cow::Owned(value.display_with_world(world))),
//                 }
//                 .into()),
//             }
//         } else {
//             Err(ValueConversionError::TypeMismatch {
//                 expected_type: Cow::Owned(target_type.display_with_world(world)),
//                 actual_type: Some(Cow::Borrowed(type_name::<&'static CStr>())),
//             }
//             .into())
//         }
//     }
// }

#[cfg(test)]
mod test {
    use std::any::Any;

    use bevy::{
        prelude::{AppTypeRegistry, World},
        utils::HashMap,
    };

    use crate::prelude::AppReflectAllocator;

    use super::*;

    #[test]
    fn test_basic_into_conversions() {
        let mut world = World::new();
        world.insert_resource(AppReflectAllocator::default());
        world.insert_resource(AppTypeRegistry::default());
        let guard = WorldAccessGuard::new(&mut world);
        let guard = WorldGuard::new(guard);
        assert_eq!(
            ().into_script_value(guard.clone()).unwrap(),
            ScriptValue::Unit
        );
        assert_eq!(
            true.into_script_value(guard.clone()).unwrap(),
            ScriptValue::Bool(true)
        );
        assert_eq!(
            false.into_script_value(guard.clone()).unwrap(),
            ScriptValue::Bool(false)
        );
        assert_eq!(
            0i64.into_script_value(guard.clone()).unwrap(),
            ScriptValue::Integer(0)
        );
        assert_eq!(
            0.0f64.into_script_value(guard.clone()).unwrap(),
            ScriptValue::Float(0.0)
        );
        assert_eq!(
            "".into_script_value(guard.clone()).unwrap(),
            ScriptValue::String("".into())
        );
        assert_eq!(
            "hello".into_script_value(guard.clone()).unwrap(),
            ScriptValue::String("hello".into())
        );

        assert_eq!(
            CString::new("hello")
                .unwrap()
                .into_script_value(guard.clone())
                .unwrap(),
            ScriptValue::String("hello".into())
        );

        assert_eq!(
            OsStr::new("hello")
                .into_script_value(guard.clone())
                .unwrap(),
            ScriptValue::String("hello".into())
        );

        assert_eq!(
            Path::new("hello").into_script_value(guard.clone()).unwrap(),
            ScriptValue::String("hello".into())
        );

        assert_eq!(
            Cow::Borrowed("hello")
                .into_script_value(guard.clone())
                .unwrap(),
            ScriptValue::String("hello".into())
        );

        assert_eq!(
            Cow::<str>::Owned("hello".to_string())
                .into_script_value(guard)
                .unwrap(),
            ScriptValue::String("hello".into())
        );
    }

    #[test]
    fn test_reference_into_conversions() {
        let mut world = World::new();
        world.insert_resource(AppReflectAllocator::default());
        world.insert_resource(AppTypeRegistry::default());
        let world = WorldAccessGuard::new(&mut world);

        let allocator = world.allocator();
        let mut allocator = allocator.write();
        let usize_reference = ReflectReference::new_allocated(2usize, &mut allocator);
        let string_reference = ReflectReference::new_allocated("hello", &mut allocator);
        let option_reference = ReflectReference::new_allocated(Some(2usize), &mut allocator);
        let none_reference = ReflectReference::new_allocated(None::<usize>, &mut allocator);
        let nested_option_reference =
            ReflectReference::new_allocated(Some(Some(2usize)), &mut allocator);
        let nested_none_reference =
            ReflectReference::new_allocated(Some(None::<usize>), &mut allocator);

        let vec_reference = ReflectReference::new_allocated(vec![1, 2, 3], &mut allocator);
        let map_reference = ReflectReference::new_allocated(
            HashMap::from_iter(vec![(1, 2), (3, 4)]),
            &mut allocator,
        );

        drop(allocator);

        let world = WorldGuard::new(world);

        assert_eq!(
            usize_reference.clone().into_script_value(world.clone()),
            Ok(ScriptValue::Integer(2))
        );

        assert_eq!(
            string_reference.clone().into_script_value(world.clone()),
            Ok(ScriptValue::String("hello".into()))
        );

        assert_eq!(
            option_reference.clone().into_script_value(world.clone()),
            Ok(ScriptValue::Integer(2))
        );

        assert_eq!(
            none_reference.clone().into_script_value(world.clone()),
            Ok(ScriptValue::Unit)
        );

        assert_eq!(
            nested_option_reference
                .clone()
                .into_script_value(world.clone()),
            Ok(ScriptValue::Integer(2))
        );

        assert_eq!(
            nested_none_reference
                .clone()
                .into_script_value(world.clone()),
            Ok(ScriptValue::Unit)
        );

        assert_eq!(
            vec_reference.clone().into_script_value(world.clone()),
            Ok(ScriptValue::Reference(vec_reference))
        );

        assert_eq!(
            map_reference.clone().into_script_value(world.clone()),
            Ok(ScriptValue::Reference(map_reference))
        );
    }

    #[test]
    fn test_basic_from_conversions() {
        let mut world = World::new();
        world.insert_resource(AppReflectAllocator::default());
        world.insert_resource(AppTypeRegistry::default());
        let guard = WorldAccessGuard::new(&mut world);
        let guard = WorldGuard::new(guard);

        assert!(
            <()>::from_script_value(ScriptValue::Unit, guard.clone(), TypeId::of::<()>())
                .unwrap()
                .reflect_partial_eq(&())
                .unwrap()
        );

        assert!(<bool>::from_script_value(
            ScriptValue::Bool(true),
            guard.clone(),
            TypeId::of::<bool>()
        )
        .unwrap()
        .reflect_partial_eq(&true)
        .unwrap());

        assert!(<&'static str>::from_script_value(
            ScriptValue::String("hello".into()),
            guard.clone(),
            TypeId::of::<&'static str>()
        )
        .unwrap()
        .reflect_partial_eq(&"hello")
        .unwrap());

        assert!(<Cow<'static, str>>::from_script_value(
            ScriptValue::String("hello".into()),
            guard.clone(),
            TypeId::of::<Cow<'static, str>>()
        )
        .unwrap()
        .reflect_partial_eq(&Cow::Borrowed("hello"))
        .unwrap());

        assert!(<String>::from_script_value(
            ScriptValue::String("hello".into()),
            guard.clone(),
            TypeId::of::<String>()
        )
        .unwrap()
        .reflect_partial_eq(&"hello".to_string())
        .unwrap());

        assert!(<f32>::from_script_value(
            ScriptValue::Float(0.0),
            guard.clone(),
            TypeId::of::<f32>()
        )
        .unwrap()
        .reflect_partial_eq(&0.0f32)
        .unwrap());

        assert!(<f64>::from_script_value(
            ScriptValue::Float(0.0),
            guard.clone(),
            TypeId::of::<f64>()
        )
        .unwrap()
        .reflect_partial_eq(&0.0f64)
        .unwrap());

        assert!(<i64>::from_script_value(
            ScriptValue::Integer(0),
            guard.clone(),
            TypeId::of::<i64>()
        )
        .unwrap()
        .reflect_partial_eq(&0i64)
        .unwrap());

        assert!(<i8>::from_script_value(
            ScriptValue::Integer(0),
            guard.clone(),
            TypeId::of::<i8>()
        )
        .unwrap()
        .reflect_partial_eq(&0i8)
        .unwrap());

        assert!(<i16>::from_script_value(
            ScriptValue::Integer(0),
            guard.clone(),
            TypeId::of::<i16>()
        )
        .unwrap()
        .reflect_partial_eq(&0i16)
        .unwrap());

        assert!(<i32>::from_script_value(
            ScriptValue::Integer(0),
            guard.clone(),
            TypeId::of::<i32>()
        )
        .unwrap()
        .reflect_partial_eq(&0i32)
        .unwrap());

        assert!(<i128>::from_script_value(
            ScriptValue::Integer(0),
            guard.clone(),
            TypeId::of::<i128>()
        )
        .unwrap()
        .reflect_partial_eq(&0i128)
        .unwrap());

        assert!(<isize>::from_script_value(
            ScriptValue::Integer(0),
            guard.clone(),
            TypeId::of::<isize>()
        )
        .unwrap()
        .reflect_partial_eq(&0isize)
        .unwrap());

        assert!(<u8>::from_script_value(
            ScriptValue::Integer(0),
            guard.clone(),
            TypeId::of::<u8>()
        )
        .unwrap()
        .reflect_partial_eq(&0u8)
        .unwrap());

        assert!(<u16>::from_script_value(
            ScriptValue::Integer(0),
            guard.clone(),
            TypeId::of::<u16>()
        )
        .unwrap()
        .reflect_partial_eq(&0u16)
        .unwrap());

        assert!(<u32>::from_script_value(
            ScriptValue::Integer(0),
            guard.clone(),
            TypeId::of::<u32>()
        )
        .unwrap()
        .reflect_partial_eq(&0u32)
        .unwrap());

        assert!(<u64>::from_script_value(
            ScriptValue::Integer(0),
            guard.clone(),
            TypeId::of::<u64>()
        )
        .unwrap()
        .reflect_partial_eq(&0u64)
        .unwrap());

        assert!(<u128>::from_script_value(
            ScriptValue::Integer(0),
            guard.clone(),
            TypeId::of::<u128>()
        )
        .unwrap()
        .reflect_partial_eq(&0u128)
        .unwrap());

        assert!(<usize>::from_script_value(
            ScriptValue::Integer(0),
            guard.clone(),
            TypeId::of::<usize>()
        )
        .unwrap()
        .reflect_partial_eq(&0usize)
        .unwrap());

        assert!(<&'static Path>::from_script_value(
            ScriptValue::String("hello".into()),
            guard.clone(),
            TypeId::of::<&'static Path>()
        )
        .unwrap()
        .reflect_partial_eq(&Path::new("hello"))
        .unwrap());

        assert!(<PathBuf>::from_script_value(
            ScriptValue::String("hello".into()),
            guard.clone(),
            TypeId::of::<PathBuf>()
        )
        .unwrap()
        .reflect_partial_eq(&PathBuf::from("hello"))
        .unwrap());

        assert!(<OsString>::from_script_value(
            ScriptValue::String("hello".into()),
            guard.clone(),
            TypeId::of::<OsString>()
        )
        .unwrap()
        .reflect_partial_eq(&OsString::from("hello"))
        .unwrap());

        assert!(<OsString>::from_script_value(
            ScriptValue::String("hello".into()),
            guard.clone(),
            TypeId::of::<OsString>()
        )
        .unwrap()
        .reflect_partial_eq(&OsString::from("hello"))
        .unwrap());
    }

    #[test]
    fn test_script_value_reference_from_conversions() {
        let mut world = World::new();
        world.insert_resource(AppReflectAllocator::default());
        world.insert_resource(AppTypeRegistry::default());
        let guard = WorldAccessGuard::new(&mut world);

        let allocator = guard.allocator();
        let mut allocator = allocator.write();
        let usize_reference = ReflectReference::new_allocated(2usize, &mut allocator);
        let string_reference = ReflectReference::new_allocated("hello", &mut allocator);
        let option_reference = ReflectReference::new_allocated(Some(2usize), &mut allocator);
        let none_reference = ReflectReference::new_allocated(None::<usize>, &mut allocator);
        let nested_option_reference =
            ReflectReference::new_allocated(Some(Some(2usize)), &mut allocator);
        let nested_none_reference =
            ReflectReference::new_allocated(Some(None::<usize>), &mut allocator);

        let vec_option_reference =
            ReflectReference::new_allocated(vec![Some(1usize), None, Some(3usize)], &mut allocator);

        let type_registry = guard.type_registry();
        let mut type_registry = type_registry.write();

        type_registry.register::<&'static str>();
        type_registry.register::<Option<usize>>();
        type_registry.register::<Option<Option<usize>>>();
        type_registry.register::<Vec<Option<usize>>>();

        drop(type_registry);
        drop(allocator);
        let guard = WorldGuard::new(guard);

        assert!(<ScriptValue>::from_script_value(
            ScriptValue::Reference(usize_reference.clone()),
            guard.clone(),
            TypeId::of::<usize>()
        )
        .unwrap()
        .reflect_partial_eq(&2usize)
        .unwrap());

        assert!(<ScriptValue>::from_script_value(
            ScriptValue::Reference(string_reference.clone()),
            guard.clone(),
            TypeId::of::<&'static str>()
        )
        .unwrap()
        .reflect_partial_eq(&"hello")
        .unwrap());

        println!(
            "{:?}",
            <ScriptValue>::from_script_value(
                ScriptValue::Reference(option_reference.clone()),
                guard.clone(),
                TypeId::of::<Option<usize>>()
            )
        );
        assert!(<ScriptValue>::from_script_value(
            ScriptValue::Reference(option_reference.clone()),
            guard.clone(),
            TypeId::of::<Option<usize>>()
        )
        .unwrap()
        .reflect_partial_eq(&Some(2usize))
        .unwrap());

        assert!(<ScriptValue>::from_script_value(
            ScriptValue::Reference(none_reference.clone()),
            guard.clone(),
            TypeId::of::<Option<usize>>()
        )
        .unwrap()
        .reflect_partial_eq(&None::<usize>)
        .unwrap());

        assert!(<ScriptValue>::from_script_value(
            ScriptValue::Reference(nested_option_reference.clone()),
            guard.clone(),
            TypeId::of::<Option<Option<usize>>>()
        )
        .unwrap()
        .reflect_partial_eq(&Some(Some(2usize)))
        .unwrap());

        assert!(<ScriptValue>::from_script_value(
            ScriptValue::Reference(nested_none_reference.clone()),
            guard.clone(),
            TypeId::of::<Option<Option<usize>>>()
        )
        .unwrap()
        .reflect_partial_eq(&Some(None::<usize>))
        .unwrap());

        assert!(<ScriptValue>::from_script_value(
            ScriptValue::Reference(vec_option_reference.clone()),
            guard.clone(),
            TypeId::of::<Vec<Option<usize>>>()
        )
        .unwrap()
        .reflect_partial_eq(&vec![Some(1usize), None, Some(3usize)])
        .unwrap());
    }

    #[test]
    pub fn test_script_value_other_from_conversions() {
        let mut world = World::new();
        world.insert_resource(AppReflectAllocator::default());
        world.insert_resource(AppTypeRegistry::default());
        let guard = WorldAccessGuard::new(&mut world);

        let type_registry = guard.type_registry();
        let mut type_registry = type_registry.write();
        type_registry.register::<Option<String>>();
        type_registry.register::<Option<Option<String>>>();

        drop(type_registry);
        let guard = WorldGuard::new(guard);

        assert!(<ScriptValue>::from_script_value(
            ScriptValue::String("hello".into()),
            guard.clone(),
            TypeId::of::<Option<String>>()
        )
        .unwrap()
        .reflect_partial_eq(&Some("hello".to_string()))
        .unwrap());

        println!(
            "{:?}",
            <ScriptValue>::from_script_value(
                ScriptValue::Unit,
                guard.clone(),
                TypeId::of::<Option<String>>()
            )
        );
        assert!(<ScriptValue>::from_script_value(
            ScriptValue::Unit,
            guard.clone(),
            TypeId::of::<Option<String>>()
        )
        .unwrap()
        .reflect_partial_eq(&None::<String>)
        .unwrap());

        assert!(<ScriptValue>::from_script_value(
            ScriptValue::Unit,
            guard.clone(),
            TypeId::of::<Option<Option<String>>>()
        )
        .unwrap()
        .reflect_partial_eq(&None::<String>)
        .unwrap());

        assert!(<ScriptValue>::from_script_value(
            ScriptValue::String("hello".into()),
            guard.clone(),
            TypeId::of::<Option<Option<String>>>()
        )
        .unwrap()
        .reflect_partial_eq(&Some(Some("hello".to_string())))
        .unwrap());
    }
}
