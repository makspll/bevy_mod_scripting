use std::{any::{Any, TypeId}, borrow::Cow, cmp::max, ffi::{CStr, CString, OsStr, OsString}, os::unix::ffi::OsStrExt, path::{Path, PathBuf}, str::FromStr};

use bevy::reflect::{FromType, List, PartialReflect, Reflect, ReflectFromReflect, TypeData, TypeInfo};
use itertools::Itertools;

use crate::{bindings::{pretty_print::DisplayWithWorld, script_val::{IntoScriptValue, ScriptValue}, ReflectReference, WorldAccessGuard, WorldGuard}, error::{ScriptError, ScriptResult}};
use crate::bindings::script_val::FromScriptValue;
/// Extension trait for [`PartialReflect`] providing additional functionality for working with specific types.
pub trait PartialReflectExt {
    fn allocate(boxed: Box<dyn PartialReflect>, world: WorldGuard) -> ReflectReference;

    fn from_primitive(value: ScriptValue, _: WorldGuard, target_type_id: TypeId) -> Option<ScriptResult<Box<dyn PartialReflect>>>;

    /// If the type is a primitive type, returns the value as a [`ScriptValue`].
    /// Primitive types are all simple types that are not the [`ScriptValue::Reference`] variant.
    fn as_primitive(&self, guard: WorldGuard) -> Option<ScriptValue>;

    /// Check if the represented type is from the given crate and has the given type identifier,
    /// returns false if not representing any type or if the type is not from the given crate or does not have the given type identifier.
    fn is_type(&self, crate_name: Option<&str>, type_ident: &str) -> bool;

    /// Equivalent to [`PartialReflectExt::is_type`] but returns an appropriate error if the type is not the expected one.  
    fn expect_type(&self, crate_name: Option<&str>, type_ident: &str) -> Result<(), ScriptError>;

    /// If the type is an option, returns either the inner value or None if the option is None.
    /// Errors if the type is not an option.
    fn as_option(&self) -> Result<Option<&dyn PartialReflect>, ScriptError>;

    /// Similar to [`PartialReflectExt::as_option`] but for mutable references.
    fn as_option_mut(&mut self) -> Result<Option<&mut dyn PartialReflect>, ScriptError>;

    /// If the type is an iterable list-like type, returns an iterator over the elements.
    fn as_list(&self) -> Result<impl Iterator<Item = &dyn PartialReflect>, ScriptError>;

    /// If the type is a usize, returns the value as a usize otherwise throws a convenient error
    fn as_usize(&self) -> Result<usize, ScriptError>;

    /// If the type is an iterable list-like type, sets the elements of the list to the elements of the other list-like type.
    /// This acts as a set operation, so the left list will have the same length as the right list after this operation.
    fn set_as_list<
        F: Fn(&mut dyn PartialReflect, &dyn PartialReflect) -> Result<(), ScriptError>,
    >(
        &mut self,
        other: Box<dyn PartialReflect>,
        apply: F,
        ) -> Result<(), ScriptError>;
    

    /// Inserts into the type at the given key, if the type supports inserting with the given key
    fn try_insert_boxed(&mut self, index: Box<dyn PartialReflect>, value: Box<dyn PartialReflect>) -> Result<(), ScriptError>;

    /// Tries to insert the given value into the type, if the type is a container type. 
    /// The insertion will happen at the natural `end` of the container.
    /// For lists, this is the length of the list.
    /// For sets, this will simply insert the value into the set.
    /// For maps, there is no natural `end`, so the push will error out
    fn try_push_boxed(&mut self, value: Box<dyn PartialReflect>) -> Result<(), ScriptError>;


    // If the type has a natural last element to pop, pops the last element and returns it as a boxed value.
    fn try_pop_boxed(&mut self) -> Result<Box<dyn PartialReflect>, ScriptError>;

    /// If the type is a container type, empties the contents
    fn try_clear(&mut self) -> Result<(), ScriptError>;

    /// If the type is a container type, returns the type id of the elements in the container.
    /// For maps, this is the type id of the values.
    fn element_type_id(&self) -> Option<TypeId>;

    /// If the type is a container type, returns the type id of the keys in the container.
    /// For lists and arrays, this is the type id of usize.
    /// For maps, this is the type id of the keys.
    fn key_type_id(&self) -> Option<TypeId>;

    /// Tries to construct the concrete underlying type from a possibly untyped reference
    fn from_reflect(reflect: &dyn PartialReflect, world: WorldGuard) -> ScriptResult<Box<dyn Reflect>>;
}
pub trait TypeIdExtensions {
    fn type_id_or_fake_id(&self) -> TypeId;
}

impl TypeIdExtensions for Option<TypeId> {
    fn type_id_or_fake_id(&self) -> TypeId {
        struct UknownType;
        match self {
            Some(t) => *t,
            None => TypeId::of::<UknownType>(),
        }
    }
}

impl<T: PartialReflect + ?Sized> PartialReflectExt for T {
    fn is_type(&self, crate_name: Option<&str>, type_ident: &str) -> bool {
        self.get_represented_type_info().is_some_and(|v| {
            let table = v.type_path_table();
            table.crate_name() == crate_name && table.ident() == Some(type_ident)
        })
    }

    fn expect_type(&self, crate_name: Option<&str>, type_ident: &str) -> Result<(), ScriptError> {
        if !self.is_type(crate_name, type_ident) {
            return Err(ScriptError::new_runtime_error(format!(
                "Expected type {type_ident}{}, but got {}",
                crate_name
                    .map(|s| format!(" from crate {s}"))
                    .unwrap_or_default(),
                self.get_represented_type_info()
                    .map(|ti| ti.type_path())
                    .unwrap_or_else(|| "dynamic type with no type information")
            )));
        }
        Ok(())
    }

    fn from_primitive(value: ScriptValue, world: WorldGuard, target_type_id: TypeId) -> Option<ScriptResult<Box<dyn PartialReflect>>> {
        Some(match target_type_id{
            // TODO: if these types ever support reflect, we can uncomment these lines
            // For some of these we specifically require the borrowed static variant, this will never let you use a dynamically created string from the script
            // we should instead allocate and leak perhaps. then garbage collect later
            t if t == TypeId::of::<()>() => <()>::from_script_value(value, world, target_type_id),
            t if t == TypeId::of::<bool>() => bool::from_script_value(value, world, target_type_id),
            t if t == TypeId::of::<str>() => <&str>::from_script_value(value, world, target_type_id),
            // t if t == TypeId::of::<CStr>() => <CStr>::from_script_value(value, world, target_type_id),
            // t if t == TypeId::of::<OsStr>() => <OsStr>::from_script_value(value, world, target_type_id),
            t if t == TypeId::of::<Path>() => <&Path>::from_script_value(value, world, target_type_id),
            t if t == TypeId::of::<Cow<'static, str>>() => <Cow<'static, str>>::from_script_value(value, world, target_type_id),
            // t if t == TypeId::of::<Cow<'static, CStr>>() => <Cow<'static, CStr>>::from_script_value(value, world, target_type_id),
            t if t == TypeId::of::<f32>() => f32::from_script_value(value, world, target_type_id),
            t if t == TypeId::of::<f64>() => f64::from_script_value(value, world, target_type_id),
            t if t == TypeId::of::<i8>() => i8::from_script_value(value, world, target_type_id),
            t if t == TypeId::of::<i16>() => i16::from_script_value(value, world, target_type_id),
            t if t == TypeId::of::<i32>() => i32::from_script_value(value, world, target_type_id),
            t if t == TypeId::of::<i64>() => i64::from_script_value(value, world, target_type_id),
            t if t == TypeId::of::<i128>() => i128::from_script_value(value, world, target_type_id),
            t if t == TypeId::of::<isize>() => isize::from_script_value(value, world, target_type_id),
            t if t == TypeId::of::<u8>() => u8::from_script_value(value, world, target_type_id),
            t if t == TypeId::of::<u16>() => u16::from_script_value(value, world, target_type_id),
            t if t == TypeId::of::<u32>() => u32::from_script_value(value, world, target_type_id),
            t if t == TypeId::of::<u64>() => u64::from_script_value(value, world, target_type_id),
            t if t == TypeId::of::<u128>() => u128::from_script_value(value, world, target_type_id),
            t if t == TypeId::of::<usize>() => usize::from_script_value(value, world, target_type_id),
            // t if t == TypeId::of::<Box<str>>() => <Box<str>>::from_script_value(value, world, target_type_id),
            // t if t == TypeId::of::<CString>() => <CString>::from_script_value(value, world, target_type_id),
            t if t == TypeId::of::<String>() => <String>::from_script_value(value, world, target_type_id),
            t if t == TypeId::of::<OsString>() => <OsString>::from_script_value(value, world, target_type_id),
            t if t == TypeId::of::<PathBuf>() => <PathBuf>::from_script_value(value, world, target_type_id),
            _ => return None,
        })
    } 

    fn as_primitive(&self, guard: WorldGuard) -> Option<ScriptValue> {
        let type_id = self.get_represented_type_info().map(|ti| ti.type_id()).type_id_or_fake_id();

        match type_id {
            _ if TypeId::of::<()>() == type_id => self.as_partial_reflect().try_downcast_ref::<()>().and_then(|v| v.into_script_value(guard).ok()),
            _ if TypeId::of::<bool>() == type_id => self.as_partial_reflect().try_downcast_ref::<bool>().and_then(|v| v.into_script_value(guard).ok()),
            _ if TypeId::of::<&'static str>() == type_id => self.as_partial_reflect().try_downcast_ref::<&'static str>().and_then(|v| v.into_script_value(guard).ok()),
            _ if TypeId::of::<&'static CStr>() == type_id => self.as_partial_reflect().try_downcast_ref::<&'static CStr>().and_then(|v| v.into_script_value(guard).ok()),
            _ if TypeId::of::<&'static OsStr>() == type_id => self.as_partial_reflect().try_downcast_ref::<&'static OsStr>().and_then(|v| v.into_script_value(guard).ok()),
            _ if TypeId::of::<Path>() == type_id => self.as_partial_reflect().try_downcast_ref::<&'static Path>().and_then(|v| v.into_script_value(guard).ok()),
            _ if TypeId::of::<Cow<'static, str>>() == type_id => self.as_partial_reflect().try_downcast_ref::<Cow<'static, str>>().and_then(|v| v.clone().into_script_value(guard).ok()),
            _ if TypeId::of::<Cow<'static, CStr>>() == type_id => self.as_partial_reflect().try_downcast_ref::<Cow<'static, CStr>>().and_then(|v| v.clone().into_script_value(guard).ok()),
            _ if TypeId::of::<f32>() == type_id => self.as_partial_reflect().try_downcast_ref::<f32>().and_then(|v| v.into_script_value(guard).ok()),
            _ if TypeId::of::<f64>() == type_id => self.as_partial_reflect().try_downcast_ref::<f64>().and_then(|v| v.into_script_value(guard).ok()),
            _ if TypeId::of::<i8>() == type_id => self.as_partial_reflect().try_downcast_ref::<i8>().and_then(|v| v.into_script_value(guard).ok()),
            _ if TypeId::of::<i16>() == type_id => self.as_partial_reflect().try_downcast_ref::<i16>().and_then(|v| v.into_script_value(guard).ok()),
            _ if TypeId::of::<i32>() == type_id => self.as_partial_reflect().try_downcast_ref::<i32>().and_then(|v| v.into_script_value(guard).ok()),
            _ if TypeId::of::<i64>() == type_id => self.as_partial_reflect().try_downcast_ref::<i64>().and_then(|v| v.into_script_value(guard).ok()),
            _ if TypeId::of::<i128>() == type_id => self.as_partial_reflect().try_downcast_ref::<i128>().and_then(|v| v.into_script_value(guard).ok()),
            _ if TypeId::of::<isize>() == type_id => self.as_partial_reflect().try_downcast_ref::<isize>().and_then(|v| v.into_script_value(guard).ok()),
            _ if TypeId::of::<u8>() == type_id => self.as_partial_reflect().try_downcast_ref::<u8>().and_then(|v| v.into_script_value(guard).ok()),
            _ if TypeId::of::<u16>() == type_id => self.as_partial_reflect().try_downcast_ref::<u16>().and_then(|v| v.into_script_value(guard).ok()),
            _ if TypeId::of::<u32>() == type_id => self.as_partial_reflect().try_downcast_ref::<u32>().and_then(|v| v.into_script_value(guard).ok()),
            _ if TypeId::of::<u64>() == type_id => self.as_partial_reflect().try_downcast_ref::<u64>().and_then(|v| v.into_script_value(guard).ok()),
            _ if TypeId::of::<u128>() == type_id => self.as_partial_reflect().try_downcast_ref::<u128>().and_then(|v| v.into_script_value(guard).ok()),
            _ if TypeId::of::<usize>() == type_id => self.as_partial_reflect().try_downcast_ref::<usize>().and_then(|v| v.into_script_value(guard).ok()),
            _ if TypeId::of::<Box<str>>() == type_id => self.as_partial_reflect().try_downcast_ref::<Box<str>>().and_then(|v| v.to_owned().into_script_value(guard).ok()),
            _ if TypeId::of::<CString>() == type_id => self.as_partial_reflect().try_downcast_ref::<CString>().and_then(|v| v.to_owned().into_script_value(guard).ok()),
            _ if TypeId::of::<String>() == type_id => self.as_partial_reflect().try_downcast_ref::<String>().and_then(|v| v.to_owned().into_script_value(guard).ok()),
            _ if TypeId::of::<OsString>() == type_id => self.as_partial_reflect().try_downcast_ref::<OsString>().and_then(|v| v.to_owned().into_script_value(guard).ok()),
            _ if TypeId::of::<PathBuf>() == type_id => self.as_partial_reflect().try_downcast_ref::<PathBuf>().and_then(|v| v.to_owned().into_script_value(guard).ok()),        
            _ => None,
        }
    }

    fn as_option(&self) -> Result<Option<&dyn PartialReflect>, ScriptError> {
        if let bevy::reflect::ReflectRef::Enum(e) = self.reflect_ref() {
            if let Some(field) = e.field_at(0) {
                return Ok(Some(field));
            } else {
                return Ok(None);
            }
        }

        Err(ScriptError::new_runtime_error(format!(
            "Expected enum type, but got type which is not an enum: {}",
            self.get_represented_type_info()
                .map(|ti| ti.type_path())
                .unwrap_or_else(|| "dynamic type with no type information")
        )))
    }

    fn as_option_mut(&mut self) -> Result<Option<&mut dyn PartialReflect>, ScriptError> {
        let type_info = self.get_represented_type_info().map(|ti| ti.type_path());
        match self.reflect_mut() {
            bevy::reflect::ReflectMut::Enum(e) => {
                if let Some(field) = e.field_at_mut(0) {
                    Ok(Some(field))
                } else {
                    Ok(None)
                }
            }
            _ => Err(ScriptError::new_runtime_error(format!(
                "Expected enum type, but got type which is not an enum: {}",
                type_info.unwrap_or("dynamic type with no type information")
            ))),
        }
    }

    fn as_list(&self) -> Result<impl Iterator<Item = &dyn PartialReflect>, ScriptError> {
        if let bevy::reflect::ReflectRef::List(l) = self.reflect_ref() {
            Ok(l.iter())
        } else {
            Err(ScriptError::new_runtime_error(format!(
                "Expected list-like type from crate core, but got {}",
                self.get_represented_type_info()
                    .map(|ti| ti.type_path())
                    .unwrap_or_else(|| "dynamic type with no type information")
            )))
        }
    }

    fn set_as_list<
        F: Fn(&mut dyn PartialReflect, &dyn PartialReflect) -> Result<(), ScriptError>,
    >(
        &mut self,
        mut other: Box<dyn PartialReflect>,
        apply: F,
    ) -> Result<(), ScriptError> {
        match (self.reflect_mut(), other.reflect_mut()) {
            (bevy::reflect::ReflectMut::List(l), bevy::reflect::ReflectMut::List(r)) => {

                let to_be_inserted_elems = max(r.len() as isize - l.len() as isize, 0) as usize;
                let apply_range = 0..(r.len() - to_be_inserted_elems);

                // remove in reverse order
                (r.len()..l.len()).rev().for_each(|i| {
                    l.remove(i);
                });

                // pop then insert in reverse order of popping (last elem -> first elem to insert)
                let to_insert = (0..to_be_inserted_elems).rev().map(|_| {
                    r.pop().expect("invariant")
                }).collect::<Vec<_>>();

                to_insert.into_iter().rev().for_each(|e| {
                    l.push(e);
                });

                // at this point l is at least as long as r

                // apply to existing elements in the list
                for i in apply_range {
                    apply(l.get_mut(i).expect("invariant"), r.get(i).expect("invariant"))?;
                };
                
                Ok(())
            }
            _ => Err(ScriptError::new_reflection_error(format!(
                "Could not set {} with {}. Both need to reflect as list types, but at least one does not.",
                self.reflect_type_path(),
                other.reflect_type_path()
            ))),
        }
    }

    fn as_usize(&self) -> Result<usize, ScriptError> {
        self.as_partial_reflect().try_downcast_ref::<usize>().copied()
            .ok_or_else(|| ScriptError::new_runtime_error(format!(
                "Expected type usize, but got {}",
                self.get_represented_type_info()
                    .map(|ti| ti.type_path())
                    .unwrap_or_else(|| "dynamic type with no type information")
            )))
    }

    fn try_insert_boxed(&mut self, key: Box<dyn PartialReflect>, value: Box<dyn PartialReflect>) -> Result<(), ScriptError> {
        match self.reflect_mut() {
            bevy::reflect::ReflectMut::List(l) => {
                l.insert(key.as_usize()?, value);
                Ok(())
            },
            bevy::reflect::ReflectMut::Map(m) => {
                m.insert_boxed(key, value);
                Ok(())
            },
            bevy::reflect::ReflectMut::Set(s) => {
                s.insert_boxed(value);
                Ok(())
            },
            _ => Err(ScriptError::new_reflection_error(format!(
                "Could not insert into {}. The type does not support insertion at a key.",
                self.reflect_type_path()
            ))),
        }
    }

    fn try_push_boxed(&mut self, value: Box<dyn PartialReflect>) -> Result<(), ScriptError> {
        match self.reflect_mut() {
            bevy::reflect::ReflectMut::List(l) => {
                l.push(value);
                Ok(())
            },
            bevy::reflect::ReflectMut::Set(s) => {
                s.insert_boxed(value);
                Ok(())
            },
            _ => Err(ScriptError::new_reflection_error(format!(
                "Could not push into {}. The type does not support pushing.",
                self.reflect_type_path()
            ))),
        }
    }

    fn try_clear(&mut self) -> Result<(), ScriptError> {
        match self.reflect_mut() {
            bevy::reflect::ReflectMut::List(l) => {
                let _ = l.drain();
                Ok(())
            },
            bevy::reflect::ReflectMut::Map(m) => {
                let _ = m.drain();
                Ok(())
            },
            bevy::reflect::ReflectMut::Set(s) => {
                let _ = s.drain();
                Ok(())
            },
            _ => Err(ScriptError::new_reflection_error(format!(
                "Could not clear {}. The type does not support clearing.",
                self.reflect_type_path()
            ))),
        }
    }

    
    fn try_pop_boxed(&mut self) -> Result<Box<dyn PartialReflect>, ScriptError> {
        match self.reflect_mut() {
            bevy::reflect::ReflectMut::List(l) => {
                l.pop().ok_or_else(|| ScriptError::new_runtime_error("Tried to pop from an empty list"))
            },
            _ => Err(ScriptError::new_reflection_error(format!(
                "Could not pop from {}. The type does not support popping.",
                self.reflect_type_path()
            ))),
        }
    }    

    fn element_type_id(&self) -> Option<TypeId> {
        let elem: TypeId = match self.get_represented_type_info()? {
            bevy::reflect::TypeInfo::List(list_info) => list_info.item_ty().id(),
            bevy::reflect::TypeInfo::Array(array_info) => array_info.item_ty().id(),
            bevy::reflect::TypeInfo::Map(map_info) => map_info.value_ty().id(),
            bevy::reflect::TypeInfo::Set(set_info) => set_info.value_ty().id(),
            _ => return None,
        };
        Some(elem)
    }
    
    fn key_type_id(&self) -> Option<TypeId> {
        let key: TypeId = match self.get_represented_type_info()? {
            bevy::reflect::TypeInfo::Map(map_info) => map_info.key_ty().id(),
            bevy::reflect::TypeInfo::List(_) |  bevy::reflect::TypeInfo::Array(_) => TypeId::of::<usize>(),
            _ => return None,
        };
        Some(key)
    }

    fn from_reflect(reflect: &dyn PartialReflect, world: WorldGuard) -> ScriptResult<Box<dyn Reflect>> {

        let type_info = reflect.get_represented_type_info().ok_or_else(|| ScriptError::new_runtime_error("Could not construct concrete type as the reference does not contain type information."))?;
        let type_id = type_info.type_id();

        let type_registry = world.type_registry();
        let type_registry = type_registry.read();
        
        let from_reflect_type_data: &ReflectFromReflect = type_registry.get_type_data(type_id).ok_or_else(|| ScriptError::new_runtime_error(format!("Could not construct concrete type from type id {} as it does not have a FromReflect type data registered.", type_id.display_with_world(world.clone()))))?;
        from_reflect_type_data.from_reflect(reflect)
            .ok_or_else(|| ScriptError::new_runtime_error(format!("Could not construct concrete type from type: {}. From Reflect implementation failed", type_id.display_with_world(world.clone()))))
    }

    fn allocate(boxed: Box<dyn PartialReflect>, world: WorldGuard) -> ReflectReference {
        let allocator = world.allocator();
        let mut allocator = allocator.write();
        ReflectReference::new_allocated_boxed(boxed, &mut allocator)
    }
    

}

pub trait TypeInfoExtensions {
    fn is_option(&self) -> bool;
    fn option_inner_type(&self) -> Option<TypeId>;
    fn is_type(&self, crate_name: Option<&str>, type_ident: &str) -> bool;
}

impl TypeInfoExtensions for TypeInfo {
    fn is_option(&self) -> bool {
        self.is_type(Some("core"), "Option")
    }
    
    fn option_inner_type(&self) -> Option<TypeId> {
        if self.is_option() {
            self.generics().first().map(|g| g.type_id())
        } else {
            None
        }
    }

    fn is_type(&self, crate_name: Option<&str>, type_ident: &str) -> bool {
        self.type_path_table().ident() == Some(type_ident)
            && self.type_path_table().crate_name() == crate_name
    }

}


#[cfg(test)]
mod test {
    use bevy::{prelude::{AppTypeRegistry, World}, reflect::{DynamicMap, Map}};

    use crate::prelude::AppReflectAllocator;

    use super::*;

    #[test]
    fn test_type_info_is_option() {
        let type_info = Some("hello").get_represented_type_info().unwrap();
        assert!(type_info.is_option());
    }

    #[test]
    fn test_type_info_is_type() {
        let type_info = Some("hello").get_represented_type_info().unwrap();
        assert!(type_info.is_type(Some("core"), "Option"));
    }

    #[test]
    fn test_type_no_crate() {
        assert!(42.is_type(None, "i32"));
        assert!(42.expect_type(None, "i32").is_ok());
    }

    #[test]
    fn test_is_type_with_crate() {
        assert!(Some(42).is_type(Some("core"), "Option"));
        assert!(Some(42).expect_type(Some("core"), "Option").is_ok());
    }

    #[test]
    fn test_is_type_negative() {
        assert!(!Some(42).is_type(Some("std"), "Option"));
        assert_eq!(
            "Encountered Runtime Error error in a script: Expected type Option from crate std, but got core::option::Option<i32>",
            Some(42)
                .expect_type(Some("std"), "Option")
                .unwrap_err()
                .to_string()
        );
    }

    #[test]
    fn test_as_option_some() {
        assert_eq!(
            &42,
            Some(42)
                .as_option()
                .unwrap()
                .unwrap()
                .try_downcast_ref::<i32>()
                .unwrap()
        );
    }

    #[test]
    fn test_as_option_none() {
        assert!(None::<i32>.as_option().unwrap().is_none());
    }

    #[test]
    fn test_as_option_error() {
        assert_eq!(
            "Encountered Runtime Error error in a script: Expected type Option from crate core, but got i32",
            42.as_option().unwrap_err().to_string()
        );
    }

    #[test]
    fn test_as_list() {
        let list = vec![1, 2, 3];
        let list_ref: &dyn PartialReflect = &list;
        let iter = list_ref
            .as_list()
            .unwrap()
            .map(|r| *r.try_downcast_ref::<i32>().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(list, iter);
    }

    #[test]
    fn test_set_as_list_equal_length() {
        let mut list = vec![1, 2, 3];
        let other = vec![4, 5, 6];
        let other_ref: Box<dyn PartialReflect> = Box::new(other.clone());
        list
            .set_as_list(other_ref, |l, r| {
                *l.try_downcast_mut::<i32>().unwrap() = *r.try_downcast_ref::<i32>().unwrap();
                Ok(())
            })
            .unwrap();
        assert_eq!(other, list);
    }

    
    #[test]
    fn test_set_as_list_shortening() {
        let mut list = vec![1, 2, 3];
        let other = vec![4, 5];
        let other_ref: Box<dyn PartialReflect> = Box::new(other.clone());
        list
            .set_as_list(other_ref, |l, r| {
                *l.try_downcast_mut::<i32>().unwrap() = *r.try_downcast_ref::<i32>().unwrap();
                Ok(())
            })
            .unwrap();
        assert_eq!(other, list);
    }

    #[test]
    fn test_set_as_list_lengthening() {
        let mut list = vec![1, 2];
        let other = vec![4, 5, 6];
        let other_ref: Box<dyn PartialReflect> = Box::new(other.clone());
        list
            .set_as_list(other_ref, |l, r| {
                *l.try_downcast_mut::<i32>().unwrap() = *r.try_downcast_ref::<i32>().unwrap();
                Ok(())
            })
            .unwrap();
        assert_eq!(other, list);
    }

    
    #[test]
    fn test_set_as_list_empty() {
        let mut list = vec![1, 2];
        let other = Vec::<i32>::default();
        let other_ref: Box<dyn PartialReflect> = Box::new(other.clone());
        list
            .set_as_list(other_ref, |l, r| {
                *l.try_downcast_mut::<i32>().unwrap() = *r.try_downcast_ref::<i32>().unwrap();
                Ok(())
            })
            .unwrap();
        assert_eq!(other, list);
    }

    
    #[test]
    fn test_set_as_list_targe_empty() {
        let mut list = Vec::<i32>::default();
        let other = vec![1];
        let other_ref: Box<dyn PartialReflect> = Box::new(other.clone());
        list
            .set_as_list(other_ref, |l, r| {
                *l.try_downcast_mut::<i32>().unwrap() = *r.try_downcast_ref::<i32>().unwrap();
                Ok(())
            })
            .unwrap();
        assert_eq!(other, list);
    }

    #[test]
    fn test_try_insert_vec() {
        let mut list = vec![1, 2, 3];
        let value = 4;
        let value_ref: Box<dyn PartialReflect> = Box::new(value);
        list.try_insert_boxed(Box::new(1usize), value_ref).unwrap();
        assert_eq!(vec![1, 4, 2, 3], list);
    }

    #[test]
    fn test_try_insert_map() {
        let mut map = std::collections::HashMap::<i32, i32>::default();
        let value = 4;
        let value_ref: Box<dyn PartialReflect> = Box::new(value);
        map.insert(1, 2);
        map.insert(2, 3);
        map.insert(3, 4);
        map.try_insert_boxed(Box::new(1), value_ref).unwrap();
        assert_eq!(4, map[&1]);
    }

    #[test]
    fn test_try_insert_set() {
        let mut set = std::collections::HashSet::<i32>::default();
        let value = 4;
        let value_ref: Box<dyn PartialReflect> = Box::new(value);
        set.insert(1);
        set.insert(2);
        set.insert(3);
        set.try_insert_boxed(Box::new(1), value_ref).unwrap();
        assert!(set.contains(&4));
    }

    #[test]
    fn test_try_insert_dynamic_map_into_map_of_maps() {
        let mut map = std::collections::HashMap::<i32, std::collections::HashMap<i32, i32>>::default();
        let value = DynamicMap::from_iter(vec![(1, 2), (2, 3), (3, 4)]);
        let value_ref: Box<dyn PartialReflect> = Box::new(value.clone_dynamic());
        map.insert(1, std::collections::HashMap::<i32, i32>::default());
        map.insert(2, std::collections::HashMap::<i32, i32>::default());
        map.insert(3, std::collections::HashMap::<i32, i32>::default());
        map.try_insert_boxed(Box::new(1), value_ref).unwrap();
        assert!(value.reflect_partial_eq(&map[&1]).unwrap());
    }


    #[test]
    fn test_as_primitive() {
        let mut world = World::new();
        world.insert_resource(AppReflectAllocator::default());
        world.insert_resource(AppTypeRegistry::default());
        let world = WorldAccessGuard::new(&mut world);
        let world = WorldGuard::new(world);

        assert_eq!(true.as_primitive(world.clone()), Some(ScriptValue::Bool(true)));
        assert_eq!(false.as_primitive(world.clone()), Some(ScriptValue::Bool(false)));
        assert_eq!(0i64.as_primitive(world.clone()), Some(ScriptValue::Integer(0)));
        assert_eq!(0.0f64.as_primitive(world.clone()), Some(ScriptValue::Float(0.0)));
        assert_eq!("".as_primitive(world.clone()), Some(ScriptValue::String("".into())));
        assert_eq!("hello".as_primitive(world.clone()), Some(ScriptValue::String("hello".into())));
        // assert_eq!(CString::new("hello").unwrap().as_primtive(world.clone()), Some(ScriptValue::String("hello".into())));
        // assert_eq!(OsStr::new("hello").as_primtive(world.clone()), Some(ScriptValue::String("hello".into())));
        assert_eq!(Path::new("hello").as_primitive(world.clone()), Some(ScriptValue::String("hello".into())));
        assert_eq!(Cow::Borrowed("hello").as_primitive(world.clone()), Some(ScriptValue::String("hello".into())));
        assert_eq!(Cow::Owned::<str>("hello".to_string()).as_primitive(world.clone()), Some(ScriptValue::String("hello".into())));
        // assert_eq!(Box::<str>::from("hello").as_primtive(world.clone()), Some(ScriptValue::String("hello".into())));
        assert_eq!(().as_primitive(world.clone()), Some(ScriptValue::Unit));       
    }

    #[test]
    fn test_from_primitive() {
        let mut world = World::new();
        world.insert_resource(AppReflectAllocator::default());
        world.insert_resource(AppTypeRegistry::default());
        let world = WorldAccessGuard::new(&mut world);
        let world = WorldGuard::new(world);

        assert!(<()>::from_primitive(ScriptValue::Unit, world.clone(), TypeId::of::<()>()).unwrap().unwrap().reflect_partial_eq(&()).unwrap());
        assert!(bool::from_primitive(ScriptValue::Bool(true), world.clone(), TypeId::of::<bool>()).unwrap().unwrap().reflect_partial_eq(&true).unwrap());
        assert!(i32::from_primitive(ScriptValue::Integer(42), world.clone(), TypeId::of::<i32>()).unwrap().unwrap().reflect_partial_eq(&42).unwrap());
        assert!(f64::from_primitive(ScriptValue::Float(42.0), world.clone(), TypeId::of::<f64>()).unwrap().unwrap().reflect_partial_eq(&42.0).unwrap());
        assert!(String::from_primitive(ScriptValue::String("hello".into()), world.clone(), TypeId::of::<String>()).unwrap().unwrap().reflect_partial_eq(&"hello".to_string()).unwrap());
        assert!(OsString::from_primitive(ScriptValue::String("hello".into()), world.clone(), TypeId::of::<OsString>()).unwrap().unwrap().reflect_partial_eq(&OsString::from("hello")).unwrap());
        assert!(PathBuf::from_primitive(ScriptValue::String("hello".into()), world.clone(), TypeId::of::<PathBuf>()).unwrap().unwrap().reflect_partial_eq(&PathBuf::from("hello")).unwrap());
        assert!(Cow::<str>::from_primitive(ScriptValue::String("hello".into()), world.clone(), TypeId::of::<Cow<'static, str>>()).unwrap().unwrap().reflect_partial_eq(&Cow::Borrowed("hello")).unwrap());
        assert!(i32::from_primitive(ScriptValue::Float(42.0), world.clone(), TypeId::of::<i32>()).unwrap().unwrap().reflect_partial_eq(&42).unwrap());
        assert!(f32::from_primitive(ScriptValue::Float(42.0), world.clone(), TypeId::of::<i32>()).unwrap().unwrap().reflect_partial_eq(&42).unwrap());
    }
}
