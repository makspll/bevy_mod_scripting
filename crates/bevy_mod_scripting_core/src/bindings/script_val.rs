use std::{
    any::{type_name, TypeId},
    borrow::Cow,
    ffi::{CStr, CString, OsStr, OsString},
    path::{Path, PathBuf},
};

use bevy::reflect::{
    Access, DynamicEnum, DynamicList, DynamicTuple, DynamicVariant, OffsetAccess, ParsedPath,
    PartialReflect, Reflect, ReflectFromReflect, TypeData,
};

use crate::{
    error::{InteropError, ScriptError, ScriptResult},
    reflection_extensions::{PartialReflectExt, TypeIdExtensions, TypeInfoExtensions},
};

use super::{pretty_print::DisplayWithWorld, ReflectReference, WorldGuard};

/// An abstraction of values that can be passed to and from scripts.
/// This allows us to re-use logic between scripting languages.
#[derive(Debug, Clone, PartialEq, Reflect)]
#[reflect(opaque)]
pub enum ScriptValue {
    /// Represents the absence of a value.
    Unit,
    /// Represents a boolean value.
    Bool(bool),
    /// Represents an integer value with at most 64 bits.
    Integer(i64),
    /// Represents a floating point value with at most 64 bits.
    Float(f64),
    /// Represents a string value.
    String(Cow<'static, str>),
    /// Represents a list of other things passed by value
    List(Vec<ScriptValue>),
    /// Represents a reference to a value.
    Reference(ReflectReference),
    /// Represents any error, will be thrown when returned to a script
    Error(ScriptError),
    /// A placeholder for a [`crate::bindings::WorldCallbackAccess`] value.
    World,
}

// impl Into<ScriptValue> for ScriptResult<>

impl From<bool> for ScriptValue {
    fn from(value: bool) -> Self {
        ScriptValue::Bool(value)
    }
}

impl From<i64> for ScriptValue {
    fn from(value: i64) -> Self {
        ScriptValue::Integer(value)
    }
}

impl From<f64> for ScriptValue {
    fn from(value: f64) -> Self {
        ScriptValue::Float(value)
    }
}

impl From<&'static str> for ScriptValue {
    fn from(value: &'static str) -> Self {
        ScriptValue::String(value.into())
    }
}

impl From<String> for ScriptValue {
    fn from(value: String) -> Self {
        ScriptValue::String(value.into())
    }
}

impl From<Cow<'static, str>> for ScriptValue {
    fn from(value: Cow<'static, str>) -> Self {
        ScriptValue::String(value)
    }
}

impl From<Vec<ScriptValue>> for ScriptValue {
    fn from(value: Vec<ScriptValue>) -> Self {
        ScriptValue::List(value)
    }
}

impl From<ReflectReference> for ScriptValue {
    fn from(value: ReflectReference) -> Self {
        ScriptValue::Reference(value)
    }
}

impl From<ScriptError> for ScriptValue {
    fn from(value: ScriptError) -> Self {
        ScriptValue::Error(value)
    }
}

impl From<InteropError> for ScriptValue {
    fn from(value: InteropError) -> Self {
        ScriptValue::Error(ScriptError::new(value))
    }
}

impl<T: Into<ScriptValue>> From<Option<T>> for ScriptValue {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(v) => v.into(),
            None => ScriptValue::Unit,
        }
    }
}

impl<T: Into<ScriptValue>, E: Into<ScriptError>> From<Result<T, E>> for ScriptValue {
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(v) => v.into(),
            Err(e) => ScriptValue::Error(e.into()),
        }
    }
}

impl TryFrom<ScriptValue> for ParsedPath {
    type Error = InteropError;
    fn try_from(value: ScriptValue) -> Result<Self, Self::Error> {
        Ok(match value {
            ScriptValue::Integer(i) => ParsedPath::from(vec![OffsetAccess {
                access: bevy::reflect::Access::ListIndex(i as usize),
                offset: Some(1),
            }]),
            ScriptValue::Float(v) => {
                return Err(InteropError::invalid_index(
                    value,
                    "Floating point numbers cannot be used to index into reflected values"
                        .to_owned(),
                ))
            }
            ScriptValue::String(cow) => {
                if let Some(tuple_struct_index) = cow.strip_prefix("_") {
                    if let Ok(index) = tuple_struct_index.parse::<usize>() {
                        let parsed_path = ParsedPath::from(vec![OffsetAccess {
                            access: bevy::reflect::Access::TupleIndex(index),
                            offset: Some(1),
                        }]);
                        return Ok(parsed_path);
                    }
                }

                match cow {
                    Cow::Borrowed(v) => ParsedPath::parse_static(v)
                        .map_err(|e| InteropError::reflection_path_error(e.to_string(), None))?,
                    Cow::Owned(o) => ParsedPath::parse(&o)
                        .map_err(|e| InteropError::reflection_path_error(e.to_string(), None))?,
                }
            }
            ScriptValue::Reference(reflect_reference) => {
                return Err(InteropError::invalid_index(
                    ScriptValue::Reference(reflect_reference),
                    "References cannot be used to index into reflected values".to_owned(),
                ))
            }
            _ => ParsedPath(vec![]),
        })
    }
}

/// A trait for converting a value into a [`ScriptVal`].
pub trait IntoScriptValue {
    fn into_script_value(self, world: WorldGuard) -> Result<ScriptValue, InteropError>;
}

/// Targeted conversion from a [`ScriptValue`] to a specific type. Can create dynamic types as well as concrete types depending on the implementation.
pub trait FromScriptValue {
    /// Returning None is saying that the conversion is not possible.
    /// Returning Some means that the conversion was possible and the result is the converted value or a failure.
    fn from_script_value(
        value: ScriptValue,
        world: WorldGuard,
        target_type_id: TypeId,
    ) -> Option<Result<Box<dyn PartialReflect>, InteropError>>;
}

macro_rules! into_script_value_downcast {
    ($self_:ident, $ty:ty, $world:ident $(, $($exp:tt)*)?) => {{
        $self_
            .try_downcast_ref::<$ty>()
            .ok_or_else(|| InteropError::type_mismatch(
                std::any::TypeId::of::<$ty>(),
                $self_
                        .get_represented_type_info()
                        .map(|ti| ti.type_id()),
            ))?
            $($($exp)*)?
            .into_script_value($world.clone())
    }};
}

impl IntoScriptValue for &dyn PartialReflect {
    fn into_script_value(self, world: WorldGuard) -> Result<ScriptValue, InteropError> {
        let target_type_id = self
            .get_represented_type_info()
            .map(|ti| ti.type_id())
            .type_id_or_fake_id();

        match target_type_id {
            // for arbitrary result types we support ScriptValue returns
            _ if TypeId::of::<ScriptValue>() == target_type_id => {
                match self.try_downcast_ref::<ScriptValue>() {
                    Some(script_val) => return Ok(script_val.clone()),
                    None => {
                        return Err(InteropError::type_mismatch(
                            TypeId::of::<ScriptValue>(),
                            Some(target_type_id),
                        ))
                    }
                }
            }
            _ if TypeId::of::<()>() == target_type_id => {
                return into_script_value_downcast!(self, (), world);
            }
            _ if TypeId::of::<bool>() == target_type_id => {
                return into_script_value_downcast!(self, bool, world);
            }
            _ if TypeId::of::<&'static str>() == target_type_id => {
                return into_script_value_downcast!(self, &'static str, world);
            }
            _ if TypeId::of::<&'static CStr>() == target_type_id => {
                return into_script_value_downcast!(self, &'static CStr, world);
            }
            _ if TypeId::of::<&'static OsStr>() == target_type_id => {
                return into_script_value_downcast!(self, &'static OsStr, world);
            }
            _ if TypeId::of::<&'static Path>() == target_type_id => {
                return into_script_value_downcast!(self, &'static Path, world);
            }
            _ if TypeId::of::<Cow<'static, str>>() == target_type_id => {
                return into_script_value_downcast!(self, Cow<'static, str>, world, .clone());
            }
            _ if TypeId::of::<Cow<'static, CStr>>() == target_type_id => {
                return into_script_value_downcast!(self, Cow<'static, CStr>, world, .clone());
            }
            _ if TypeId::of::<f32>() == target_type_id => {
                return into_script_value_downcast!(self, f32, world);
            }
            _ if TypeId::of::<f64>() == target_type_id => {
                return into_script_value_downcast!(self, f64, world);
            }
            _ if TypeId::of::<i8>() == target_type_id => {
                return into_script_value_downcast!(self, i8, world);
            }
            _ if TypeId::of::<i16>() == target_type_id => {
                return into_script_value_downcast!(self, i16, world);
            }
            _ if TypeId::of::<i32>() == target_type_id => {
                return into_script_value_downcast!(self, i32, world);
            }
            _ if TypeId::of::<i64>() == target_type_id => {
                return into_script_value_downcast!(self, i64, world);
            }
            _ if TypeId::of::<i128>() == target_type_id => {
                return into_script_value_downcast!(self, i128, world);
            }
            _ if TypeId::of::<isize>() == target_type_id => {
                return into_script_value_downcast!(self, isize, world);
            }
            _ if TypeId::of::<u8>() == target_type_id => {
                return into_script_value_downcast!(self, u8, world);
            }
            _ if TypeId::of::<u16>() == target_type_id => {
                return into_script_value_downcast!(self, u16, world);
            }
            _ if TypeId::of::<u32>() == target_type_id => {
                return into_script_value_downcast!(self, u32, world);
            }
            _ if TypeId::of::<u64>() == target_type_id => {
                return into_script_value_downcast!(self, u64, world);
            }
            _ if TypeId::of::<u128>() == target_type_id => {
                return into_script_value_downcast!(self, u128, world);
            }
            _ if TypeId::of::<usize>() == target_type_id => {
                return into_script_value_downcast!(self, usize, world);
            }
            _ if TypeId::of::<Box<str>>() == target_type_id => {
                return into_script_value_downcast!(self, Box<str>, world, .clone());
            }
            _ if TypeId::of::<CString>() == target_type_id => {
                return into_script_value_downcast!(self, CString, world, .clone());
            }
            _ if TypeId::of::<String>() == target_type_id => {
                return into_script_value_downcast!(self, String, world, .clone());
            }
            _ if TypeId::of::<OsString>() == target_type_id => {
                return into_script_value_downcast!(self, OsString, world, .clone());
            }
            _ if TypeId::of::<PathBuf>() == target_type_id => {
                return into_script_value_downcast!(self, PathBuf, world, .clone());
            }
            _ => {}
        };

        // for options we want to convert to
        // - the inner type if it's some
        // - nil if it's none
        // to retain the reference we need to return a reference pointing into the inner type
        if let Ok(inner) = self.as_option() {
            return inner.into_script_value(world);
        }

        if let Ok(list) = self.as_list() {
            let list: Vec<_> = list.collect();
            return list.into_script_value(world);
        }

        // as a last resort we just allocate the value and return a reference to it
        let reflect_reference = self.allocate_cloned(world.clone());
        ReflectReference::into_script_value(reflect_reference, world)
    }
}

impl IntoScriptValue for Option<&dyn PartialReflect> {
    fn into_script_value(self, world: WorldGuard) -> Result<ScriptValue, InteropError> {
        match self {
            Some(inner) => inner.into_script_value(world),
            None => Ok(ScriptValue::Unit),
        }
    }
}

impl IntoScriptValue for Vec<&dyn PartialReflect> {
    fn into_script_value(mut self, world: WorldGuard) -> Result<ScriptValue, InteropError> {
        let mut vec = Vec::with_capacity(self.len());
        for v in self.iter_mut() {
            vec.push(v.into_script_value(world.clone())?);
        }
        Ok(ScriptValue::List(vec))
    }
}

impl IntoScriptValue for ReflectReference {
    fn into_script_value(self, _world: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(ScriptValue::Reference(self))
    }
}

// macro_rules

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

impl IntoScriptValue for () {
    fn into_script_value(self, _: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(ScriptValue::Unit)
    }
}

impl IntoScriptValue for &'static str {
    fn into_script_value(self, _: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(ScriptValue::String(self.into()))
    }
}

impl IntoScriptValue for &'static CStr {
    fn into_script_value(self, _: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(ScriptValue::String(self.to_string_lossy()))
    }
}

impl IntoScriptValue for &'static OsStr {
    fn into_script_value(self, _: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(ScriptValue::String(self.to_string_lossy()))
    }
}

impl IntoScriptValue for &'static Path {
    fn into_script_value(self, _: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(ScriptValue::String(self.to_string_lossy()))
    }
}

impl IntoScriptValue for Cow<'static, str> {
    fn into_script_value(self, _: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(ScriptValue::String(self.into_owned().into()))
    }
}

impl IntoScriptValue for Cow<'static, CStr> {
    fn into_script_value(self, _: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(ScriptValue::String(
            self.to_string_lossy().into_owned().into(),
        ))
    }
}

impl IntoScriptValue for bool {
    fn into_script_value(self, _: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(ScriptValue::Bool(self))
    }
}

impl IntoScriptValue for f32 {
    fn into_script_value(self, _: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(ScriptValue::Float(self as f64))
    }
}

impl IntoScriptValue for f64 {
    fn into_script_value(self, _: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(ScriptValue::Float(self))
    }
}

macro_rules! into_script_value_integers {
    ($($ty:ty),*) => {
        $(
            impl IntoScriptValue for $ty {
                fn into_script_value(self, _: WorldGuard) -> Result<ScriptValue, InteropError> {
                    Ok(ScriptValue::Integer(self as i64))
                }
            }
        )*
    };
}

into_script_value_integers!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

impl IntoScriptValue for Box<str> {
    fn into_script_value(self, _: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(ScriptValue::String(self.to_string().into()))
    }
}

impl IntoScriptValue for CString {
    fn into_script_value(self, _: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(ScriptValue::String(
            self.to_string_lossy().into_owned().into(),
        ))
    }
}

impl IntoScriptValue for String {
    fn into_script_value(self, _: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(ScriptValue::String(self.into()))
    }
}

impl IntoScriptValue for OsString {
    fn into_script_value(self, _: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(ScriptValue::String(
            self.to_string_lossy().into_owned().into(),
        ))
    }
}

impl IntoScriptValue for PathBuf {
    fn into_script_value(self, _: WorldGuard) -> Result<ScriptValue, InteropError> {
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
    ) -> Option<Result<Box<dyn PartialReflect>, InteropError>> {
        if target_type == TypeId::of::<()>() {
            Some(match value {
                ScriptValue::Unit => Ok(Box::new(())),
                ScriptValue::Reference(ref_) => ref_
                    .downcast::<()>(world)
                    .map(|v| Box::new(v) as Box<dyn PartialReflect>),
                _ => Err(InteropError::value_mismatch(target_type, value)),
            })
        } else {
            None
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

#[cfg(test)]
mod test {
    use std::any::Any;

    use bevy::{
        prelude::{AppTypeRegistry, World},
        utils::HashMap,
    };

    use crate::{bindings::WorldAccessGuard, prelude::AppReflectAllocator};

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
            usize_reference
                .clone()
                .into_script_value(world.clone())
                .unwrap(),
            ScriptValue::Integer(2)
        );

        assert_eq!(
            string_reference
                .clone()
                .into_script_value(world.clone())
                .unwrap(),
            ScriptValue::String("hello".into())
        );

        assert_eq!(
            option_reference
                .clone()
                .into_script_value(world.clone())
                .unwrap(),
            ScriptValue::Integer(2)
        );

        assert_eq!(
            none_reference
                .clone()
                .into_script_value(world.clone())
                .unwrap(),
            ScriptValue::Unit
        );

        assert_eq!(
            nested_option_reference
                .clone()
                .into_script_value(world.clone())
                .unwrap(),
            ScriptValue::Integer(2)
        );

        assert_eq!(
            nested_none_reference
                .clone()
                .into_script_value(world.clone())
                .unwrap(),
            ScriptValue::Unit
        );

        assert_eq!(
            vec_reference
                .clone()
                .into_script_value(world.clone())
                .unwrap(),
            ScriptValue::Reference(vec_reference)
        );

        assert_eq!(
            map_reference
                .clone()
                .into_script_value(world.clone())
                .unwrap(),
            ScriptValue::Reference(map_reference)
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
        .unwrap()
        .reflect_partial_eq(&true)
        .unwrap());

        assert!(<&'static str>::from_script_value(
            ScriptValue::String("hello".into()),
            guard.clone(),
            TypeId::of::<&'static str>()
        )
        .unwrap()
        .unwrap()
        .reflect_partial_eq(&"hello")
        .unwrap());

        assert!(<Cow<'static, str>>::from_script_value(
            ScriptValue::String("hello".into()),
            guard.clone(),
            TypeId::of::<Cow<'static, str>>()
        )
        .unwrap()
        .unwrap()
        .reflect_partial_eq(&Cow::Borrowed("hello"))
        .unwrap());

        assert!(<String>::from_script_value(
            ScriptValue::String("hello".into()),
            guard.clone(),
            TypeId::of::<String>()
        )
        .unwrap()
        .unwrap()
        .reflect_partial_eq(&"hello".to_string())
        .unwrap());

        assert!(<f32>::from_script_value(
            ScriptValue::Float(0.0),
            guard.clone(),
            TypeId::of::<f32>()
        )
        .unwrap()
        .unwrap()
        .reflect_partial_eq(&0.0f32)
        .unwrap());

        assert!(<f64>::from_script_value(
            ScriptValue::Float(0.0),
            guard.clone(),
            TypeId::of::<f64>()
        )
        .unwrap()
        .unwrap()
        .reflect_partial_eq(&0.0f64)
        .unwrap());

        assert!(<i64>::from_script_value(
            ScriptValue::Integer(0),
            guard.clone(),
            TypeId::of::<i64>()
        )
        .unwrap()
        .unwrap()
        .reflect_partial_eq(&0i64)
        .unwrap());

        assert!(<i8>::from_script_value(
            ScriptValue::Integer(0),
            guard.clone(),
            TypeId::of::<i8>()
        )
        .unwrap()
        .unwrap()
        .reflect_partial_eq(&0i8)
        .unwrap());

        assert!(<i16>::from_script_value(
            ScriptValue::Integer(0),
            guard.clone(),
            TypeId::of::<i16>()
        )
        .unwrap()
        .unwrap()
        .reflect_partial_eq(&0i16)
        .unwrap());

        assert!(<i32>::from_script_value(
            ScriptValue::Integer(0),
            guard.clone(),
            TypeId::of::<i32>()
        )
        .unwrap()
        .unwrap()
        .reflect_partial_eq(&0i32)
        .unwrap());

        assert!(<i128>::from_script_value(
            ScriptValue::Integer(0),
            guard.clone(),
            TypeId::of::<i128>()
        )
        .unwrap()
        .unwrap()
        .reflect_partial_eq(&0i128)
        .unwrap());

        assert!(<isize>::from_script_value(
            ScriptValue::Integer(0),
            guard.clone(),
            TypeId::of::<isize>()
        )
        .unwrap()
        .unwrap()
        .reflect_partial_eq(&0isize)
        .unwrap());

        assert!(<u8>::from_script_value(
            ScriptValue::Integer(0),
            guard.clone(),
            TypeId::of::<u8>()
        )
        .unwrap()
        .unwrap()
        .reflect_partial_eq(&0u8)
        .unwrap());

        assert!(<u16>::from_script_value(
            ScriptValue::Integer(0),
            guard.clone(),
            TypeId::of::<u16>()
        )
        .unwrap()
        .unwrap()
        .reflect_partial_eq(&0u16)
        .unwrap());

        assert!(<u32>::from_script_value(
            ScriptValue::Integer(0),
            guard.clone(),
            TypeId::of::<u32>()
        )
        .unwrap()
        .unwrap()
        .reflect_partial_eq(&0u32)
        .unwrap());

        assert!(<u64>::from_script_value(
            ScriptValue::Integer(0),
            guard.clone(),
            TypeId::of::<u64>()
        )
        .unwrap()
        .unwrap()
        .reflect_partial_eq(&0u64)
        .unwrap());

        assert!(<u128>::from_script_value(
            ScriptValue::Integer(0),
            guard.clone(),
            TypeId::of::<u128>()
        )
        .unwrap()
        .unwrap()
        .reflect_partial_eq(&0u128)
        .unwrap());

        assert!(<usize>::from_script_value(
            ScriptValue::Integer(0),
            guard.clone(),
            TypeId::of::<usize>()
        )
        .unwrap()
        .unwrap()
        .reflect_partial_eq(&0usize)
        .unwrap());

        assert!(<&'static Path>::from_script_value(
            ScriptValue::String("hello".into()),
            guard.clone(),
            TypeId::of::<&'static Path>()
        )
        .unwrap()
        .unwrap()
        .reflect_partial_eq(&Path::new("hello"))
        .unwrap());

        assert!(<PathBuf>::from_script_value(
            ScriptValue::String("hello".into()),
            guard.clone(),
            TypeId::of::<PathBuf>()
        )
        .unwrap()
        .unwrap()
        .reflect_partial_eq(&PathBuf::from("hello"))
        .unwrap());

        assert!(<OsString>::from_script_value(
            ScriptValue::String("hello".into()),
            guard.clone(),
            TypeId::of::<OsString>()
        )
        .unwrap()
        .unwrap()
        .reflect_partial_eq(&OsString::from("hello"))
        .unwrap());

        assert!(<OsString>::from_script_value(
            ScriptValue::String("hello".into()),
            guard.clone(),
            TypeId::of::<OsString>()
        )
        .unwrap()
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

        assert!(<dyn PartialReflect>::from_script_value(
            ScriptValue::Reference(usize_reference.clone()),
            guard.clone(),
            TypeId::of::<usize>()
        )
        .unwrap()
        .unwrap()
        .reflect_partial_eq(&2usize)
        .unwrap());

        assert!(<dyn PartialReflect>::from_script_value(
            ScriptValue::Reference(string_reference.clone()),
            guard.clone(),
            TypeId::of::<&'static str>()
        )
        .unwrap()
        .unwrap()
        .reflect_partial_eq(&"hello")
        .unwrap());
        assert!(<dyn PartialReflect>::from_script_value(
            ScriptValue::Reference(usize_reference.clone()),
            guard.clone(),
            TypeId::of::<Option<usize>>()
        )
        .unwrap()
        .unwrap()
        .reflect_partial_eq(&Some(2usize))
        .unwrap());

        assert!(<dyn PartialReflect>::from_script_value(
            ScriptValue::Unit,
            guard.clone(),
            TypeId::of::<Option<usize>>()
        )
        .unwrap()
        .unwrap()
        .reflect_partial_eq(&None::<usize>)
        .unwrap());

        assert!(<dyn PartialReflect>::from_script_value(
            ScriptValue::Reference(usize_reference.clone()),
            guard.clone(),
            TypeId::of::<Option<Option<usize>>>()
        )
        .unwrap()
        .unwrap()
        .reflect_partial_eq(&Some(Some(2usize)))
        .unwrap());

        assert!(<dyn PartialReflect>::from_script_value(
            ScriptValue::Unit,
            guard.clone(),
            TypeId::of::<Option<Option<usize>>>()
        )
        .unwrap()
        .unwrap()
        .reflect_partial_eq(&Some(None::<usize>))
        .unwrap());

        assert!(<dyn PartialReflect>::from_script_value(
            ScriptValue::Reference(vec_option_reference.clone()),
            guard.clone(),
            TypeId::of::<Vec<Option<usize>>>()
        )
        .unwrap()
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

        assert!(<dyn PartialReflect>::from_script_value(
            ScriptValue::String("hello".into()),
            guard.clone(),
            TypeId::of::<Option<String>>()
        )
        .unwrap()
        .unwrap()
        .reflect_partial_eq(&Some("hello".to_string()))
        .unwrap());

        assert!(<dyn PartialReflect>::from_script_value(
            ScriptValue::Unit,
            guard.clone(),
            TypeId::of::<Option<String>>()
        )
        .unwrap()
        .unwrap()
        .reflect_partial_eq(&None::<String>)
        .unwrap());

        assert!(<dyn PartialReflect>::from_script_value(
            ScriptValue::Unit,
            guard.clone(),
            TypeId::of::<Option<Option<String>>>()
        )
        .unwrap()
        .unwrap()
        .reflect_partial_eq(&None::<String>)
        .unwrap());

        assert!(<dyn PartialReflect>::from_script_value(
            ScriptValue::String("hello".into()),
            guard.clone(),
            TypeId::of::<Option<Option<String>>>()
        )
        .unwrap()
        .unwrap()
        .reflect_partial_eq(&Some(Some("hello".to_string())))
        .unwrap());
    }
}
