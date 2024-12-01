use std::{any::TypeId, cmp::max};

use bevy::reflect::{List, PartialReflect};
use itertools::Itertools;

use crate::error::ScriptError;

/// Extension trait for [`PartialReflect`] providing additional functionality for working with specific types.
pub trait PartialReflectExt {
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

    /// If the type is an iterable list-like type, sets the elements of the list to the elements of the other list-like type.
    /// This acts as a set operation, so the left list will have the same length as the right list after this operation.
    fn set_as_list<
        F: Fn(&mut dyn PartialReflect, &dyn PartialReflect) -> Result<(), ScriptError>,
    >(
        &mut self,
        other: Box<dyn PartialReflect>,
        apply: F,
        ) -> Result<(), ScriptError>;
    }

pub trait TypeIdExtensions {
    fn type_id_or_dummy(&self) -> TypeId;
}

impl TypeIdExtensions for Option<TypeId> {
    fn type_id_or_dummy(&self) -> TypeId {
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

                let excess_elems = max(l.len() as isize - r.len() as isize, 0) as usize;
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



                // for right_elem in r.iter() {
                //     if let Some(left_elem) = l.get_mut(i) {
                //         apply(left_elem, right_elem)?
                //     } else {
                //         shorter = true;
                //         break;
                //     }
                //     i+=1;
                // };
                

                
                Ok(())
            }
            _ => Err(ScriptError::new_reflection_error(format!(
                "Could not set {} with {}. Both need to reflect as list types, but at least one does not.",
                self.reflect_type_path(),
                other.reflect_type_path()
            ))),
        }
    }

    // fn set_as_list<I: IntoIterator<Item = Box<dyn PartialReflect>>>(
    //     &mut self,
    //     other: I,
    // ) -> Result<(), ScriptError> {
    //     if let bevy::reflect::ReflectMut::List(list) = self.reflect_mut() {
    //         let mut left_index = 0;
    //         for i in other {
    //             if let Some(left_item) = list.get_mut(left_index) {
    //                 left_item.apply
    //             } else {
    //             }
    //             left_index += 1;
    //         }
    //         Ok(())
    //     } else {
    //         Err(ScriptError::new_runtime_error(format!(
    //             "Expected list-like type from crate core, but got {}",
    //             self.get_represented_type_info()
    //                 .map(|ti| ti.type_path())
    //                 .unwrap_or_else(|| "dynamic type with no type information")
    //         )))
    //     }
    // }
}




#[cfg(test)]
mod test {
    use super::*;

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
}
