use std::{
    any::TypeId,
    borrow::Cow,
    ffi::{CStr, CString, OsStr, OsString},
    path::{Path, PathBuf},
};

use bevy::reflect::{
    DynamicEnum, DynamicList, DynamicTuple, DynamicVariant, PartialReflect, ReflectFromReflect,
};

use crate::{
    bindings::{ReflectReference, WorldGuard},
    error::InteropError,
    reflection_extensions::{PartialReflectExt, TypeIdExtensions},
};

use super::{FromScriptValue, IntoScriptValue, ScriptValue};

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
        bevy::log::trace!("Converting {:?} to ScriptValue", self);

        let target_type_id = self
            .get_represented_type_info()
            .map(|ti| ti.type_id())
            .type_id_or_fake_id();

        match target_type_id {
            // for arbitrary result types we support ScriptValue returns
            _ if TypeId::of::<ScriptValue>() == target_type_id => {
                match self.try_downcast_ref::<ScriptValue>() {
                    Some(ScriptValue::Error(e)) => return Err(e.clone()),
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

        // if let Ok(list) = self.as_list() {
        //     let list: Vec<_> = list.collect();
        //     return list.into_script_value(world);
        // }

        // this is us saying, we cannot convert this into a nice script value
        // you're gonna have to allocate and ref to it
        Err(InteropError::better_conversion_exists::<Self>())
    }
}

impl IntoScriptValue for Option<&dyn PartialReflect> {
    fn into_script_value(self, world: WorldGuard) -> Result<ScriptValue, InteropError> {
        bevy::log::trace!("Converting Option {:?} to ScriptValue", self);
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
