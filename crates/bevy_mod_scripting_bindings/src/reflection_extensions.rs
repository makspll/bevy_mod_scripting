//! Various utility functions for working with reflection types.

use std::{
    any::{Any, TypeId},
    cmp::max,
};

use bevy_reflect::{PartialReflect, Reflect, ReflectFromReflect, ReflectMut, ReflectRef, TypeInfo};

use crate::{ReflectReference, WorldGuard, error::InteropError};

/// Extension trait for [`PartialReflect`] providing additional functionality for working with specific types.
pub trait PartialReflectExt {
    /// Try to get a reference to the given key in an underyling map, if the type is a map.
    fn try_map_get(
        &self,
        key: &dyn PartialReflect,
    ) -> Result<Option<&dyn PartialReflect>, InteropError>;

    /// Try to remove the value at the given key, if the type supports removing with the given key.
    fn try_remove_boxed(
        &mut self,
        key: Box<dyn PartialReflect>,
    ) -> Result<Option<Box<dyn PartialReflect>>, InteropError>;

    /// Convert index keys to 0-indexed keys if this type is an index key.
    fn convert_to_0_indexed_key(&mut self);

    /// Try to create a new instance of the concrete type from a possibly untyped reference.
    fn from_reflect_or_clone(
        reflect: &dyn PartialReflect,
        world: WorldGuard,
    ) -> Result<Box<dyn PartialReflect>, InteropError>;

    /// Allocate a new boxed reflect reference from a boxed reflect.
    fn allocate(boxed: Box<dyn Reflect>, world: WorldGuard) -> ReflectReference;

    /// Check if the represented type is from the given crate and has the given type identifier,
    /// returns false if not representing any type or if the type is not from the given crate or does not have the given type identifier.
    fn is_type(&self, crate_name: Option<&str>, type_ident: &str) -> bool;

    /// Equivalent to [`PartialReflectExt::is_type`] but returns an appropriate error if the type is not the expected one.  
    fn expect_type(&self, crate_name: Option<&str>, type_ident: &str) -> Result<(), InteropError>;

    /// If the type is an option, returns either the inner value or None if the option is None.
    /// Errors if the type is not an option.
    fn as_option(&self) -> Result<Option<&dyn PartialReflect>, InteropError>;

    /// Similar to [`PartialReflectExt::as_option`] but for mutable references.
    fn as_option_mut(&mut self) -> Result<Option<&mut dyn PartialReflect>, InteropError>;

    /// If the type is an iterable list-like type, returns an iterator over the elements.
    fn as_list(&self) -> Result<impl Iterator<Item = &dyn PartialReflect>, InteropError>;

    /// If the type is a usize, returns the value as a usize otherwise throws a convenient error
    fn as_usize(&self) -> Result<usize, InteropError>;

    /// If the type is an iterable list-like type, sets the elements of the list to the elements of the other list-like type.
    /// This acts as a set operation, so the left list will have the same length as the right list after this operation.
    fn set_as_list<
        F: Fn(&mut dyn PartialReflect, &dyn PartialReflect) -> Result<(), InteropError>,
    >(
        &mut self,
        other: Box<dyn PartialReflect>,
        apply: F,
    ) -> Result<(), InteropError>;

    /// Inserts into the type at the given key, if the type supports inserting with the given key
    fn try_insert_boxed(
        &mut self,
        index: Box<dyn PartialReflect>,
        value: Box<dyn PartialReflect>,
    ) -> Result<(), InteropError>;

    /// Tries to insert the given value into the type, if the type is a container type.
    /// The insertion will happen at the natural `end` of the container.
    /// For lists, this is the length of the list.
    /// For sets, this will simply insert the value into the set.
    /// For maps, there is no natural `end`, so the push will error out
    fn try_push_boxed(&mut self, value: Box<dyn PartialReflect>) -> Result<(), InteropError>;

    /// If the type has a natural last element to pop, pops the last element and returns it as a boxed value.
    fn try_pop_boxed(&mut self) -> Result<Box<dyn PartialReflect>, InteropError>;

    /// If the type is a container type, empties the contents
    fn try_clear(&mut self) -> Result<(), InteropError>;

    /// If the type is a container type, returns the type id of the elements in the container.
    /// For maps, this is the type id of the values.
    fn element_type_id(&self) -> Option<TypeId>;

    /// If the type is a container type, returns the type id of the keys in the container.
    /// For lists and arrays, this is the type id of usize.
    /// For maps, this is the type id of the keys.
    fn key_type_id(&self) -> Option<TypeId>;

    /// Tries to construct the concrete underlying type from a possibly untyped reference
    fn from_reflect(
        reflect: &dyn PartialReflect,
        world: WorldGuard,
    ) -> Result<Box<dyn Reflect>, InteropError>;
}

impl<T: PartialReflect + ?Sized> PartialReflectExt for T {
    fn is_type(&self, crate_name: Option<&str>, type_ident: &str) -> bool {
        self.get_represented_type_info().is_some_and(|v| {
            let table = v.type_path_table();
            table.crate_name() == crate_name && table.ident() == Some(type_ident)
        })
    }

    fn expect_type(&self, crate_name: Option<&str>, type_ident: &str) -> Result<(), InteropError> {
        if !self.is_type(crate_name, type_ident) {
            return Err(InteropError::string_type_mismatch(
                type_ident.to_owned(),
                self.get_represented_type_info().map(|ti| ti.type_id()),
            ));
        }
        Ok(())
    }

    fn as_option(&self) -> Result<Option<&dyn PartialReflect>, InteropError> {
        if let ReflectRef::Enum(e) = self.reflect_ref()
            && e.is_type(Some("core"), "Option")
        {
            if let Some(field) = e.field_at(0) {
                return Ok(Some(field));
            } else {
                return Ok(None);
            }
        }

        Err(InteropError::string_type_mismatch(
            "Option<T>".to_owned(),
            self.get_represented_type_info().map(|ti| ti.type_id()),
        ))
    }

    fn as_option_mut(&mut self) -> Result<Option<&mut dyn PartialReflect>, InteropError> {
        let type_info = self.get_represented_type_info().map(|ti| ti.type_path());
        match self.reflect_mut() {
            ReflectMut::Enum(e) => {
                if let Some(field) = e.field_at_mut(0) {
                    Ok(Some(field))
                } else {
                    Ok(None)
                }
            }
            _ => Err(InteropError::string_type_mismatch(
                "Option<T>".to_owned(),
                type_info.map(|t| t.type_id()),
            )),
        }
    }

    fn as_list(&self) -> Result<impl Iterator<Item = &dyn PartialReflect>, InteropError> {
        if let ReflectRef::List(l) = self.reflect_ref() {
            Ok(l.iter())
        } else {
            Err(InteropError::string_type_mismatch(
                "List<T>".to_owned(),
                self.get_represented_type_info().map(|ti| ti.type_id()),
            ))
        }
    }

    fn set_as_list<
        F: Fn(&mut dyn PartialReflect, &dyn PartialReflect) -> Result<(), InteropError>,
    >(
        &mut self,
        mut other: Box<dyn PartialReflect>,
        apply: F,
    ) -> Result<(), InteropError> {
        match (self.reflect_mut(), other.reflect_mut()) {
            (ReflectMut::List(l), ReflectMut::List(r)) => {
                let to_be_inserted_elems = max(r.len() as isize - l.len() as isize, 0) as usize;
                let apply_range = 0..(r.len() - to_be_inserted_elems);

                // remove in reverse order
                (r.len()..l.len()).rev().for_each(|i| {
                    l.remove(i);
                });

                // pop then insert in reverse order of popping (last elem -> first elem to insert)
                let to_insert = (0..to_be_inserted_elems)
                    .rev()
                    .map_while(|_| r.pop())
                    .collect::<Vec<_>>();

                to_insert.into_iter().rev().for_each(|e| {
                    l.push(e);
                });

                // at this point l is at least as long as r

                // apply to existing elements in the list
                for i in apply_range {
                    let left = l.get_mut(i);
                    let right = r.get(i);
                    match (left, right) {
                        (Some(left), Some(right)) => apply(left, right)?,
                        _ => return Err(InteropError::invariant("set_as_list failed")),
                    };
                }

                Ok(())
            }
            (ReflectMut::List(_), _) => Err(InteropError::string_type_mismatch(
                "List<T>".to_owned(),
                other.get_represented_type_info().map(|ti| ti.type_id()),
            )),
            (_, _) => Err(InteropError::string_type_mismatch(
                "List<T>".to_owned(),
                self.get_represented_type_info().map(|ti| ti.type_id()),
            )),
        }
    }

    fn as_usize(&self) -> Result<usize, InteropError> {
        self.as_partial_reflect()
            .try_downcast_ref::<usize>()
            .copied()
            .ok_or_else(|| {
                InteropError::type_mismatch(
                    TypeId::of::<usize>(),
                    self.get_represented_type_info().map(|ti| ti.type_id()),
                )
            })
    }

    fn try_insert_boxed(
        &mut self,
        key: Box<dyn PartialReflect>,
        value: Box<dyn PartialReflect>,
    ) -> Result<(), InteropError> {
        match self.reflect_mut() {
            ReflectMut::List(l) => {
                l.insert(key.as_usize()?, value);
                Ok(())
            }
            ReflectMut::Map(m) => {
                m.insert_boxed(key, value);
                Ok(())
            }
            ReflectMut::Set(s) => {
                s.insert_boxed(value);
                Ok(())
            }
            _ => Err(InteropError::unsupported_operation(
                self.get_represented_type_info().map(|ti| ti.type_id()),
                Some(value),
                "insert".to_owned(),
            )),
        }
    }

    fn try_push_boxed(&mut self, value: Box<dyn PartialReflect>) -> Result<(), InteropError> {
        match self.reflect_mut() {
            ReflectMut::List(l) => {
                l.push(value);
                Ok(())
            }
            ReflectMut::Set(s) => {
                s.insert_boxed(value);
                Ok(())
            }
            _ => Err(InteropError::unsupported_operation(
                self.get_represented_type_info().map(|ti| ti.type_id()),
                Some(value),
                "push".to_owned(),
            )),
        }
    }

    fn convert_to_0_indexed_key(&mut self) {
        if let Some(usize) = self
            .try_as_reflect_mut()
            .and_then(|r| r.downcast_mut::<usize>())
        {
            *usize = usize.saturating_sub(1);
        }
    }

    fn try_clear(&mut self) -> Result<(), InteropError> {
        match self.reflect_mut() {
            ReflectMut::List(l) => {
                let _ = l.drain();
                Ok(())
            }
            ReflectMut::Map(m) => {
                let _ = m.drain();
                Ok(())
            }
            ReflectMut::Set(s) => {
                let _ = s.drain();
                Ok(())
            }
            _ => Err(InteropError::unsupported_operation(
                self.get_represented_type_info().map(|ti| ti.type_id()),
                None,
                "clear".to_owned(),
            )),
        }
    }

    fn try_map_get(
        &self,
        key: &dyn PartialReflect,
    ) -> Result<Option<&dyn PartialReflect>, InteropError> {
        match self.reflect_ref() {
            ReflectRef::Map(m) => Ok(m.get(key)),
            _ => Err(InteropError::unsupported_operation(
                self.get_represented_type_info().map(|ti| ti.type_id()),
                None,
                "map_get".to_owned(),
            )),
        }
    }

    fn try_pop_boxed(&mut self) -> Result<Box<dyn PartialReflect>, InteropError> {
        match self.reflect_mut() {
            ReflectMut::List(l) => l.pop().ok_or_else(|| {
                InteropError::unsupported_operation(
                    self.get_represented_type_info().map(|ti| ti.type_id()),
                    None,
                    "pop from empty list".to_owned(),
                )
            }),
            _ => Err(InteropError::unsupported_operation(
                self.get_represented_type_info().map(|ti| ti.type_id()),
                None,
                "pop".to_owned(),
            )),
        }
    }

    fn try_remove_boxed(
        &mut self,
        key: Box<dyn PartialReflect>,
    ) -> Result<Option<Box<dyn PartialReflect>>, InteropError> {
        match self.reflect_mut() {
            ReflectMut::List(l) => Ok(Some(l.remove(key.as_usize()?))),
            ReflectMut::Map(m) => Ok(m.remove(key.as_partial_reflect())),
            ReflectMut::Set(s) => {
                let removed = s.remove(key.as_partial_reflect());
                Ok(removed.then_some(key))
            }
            _ => Err(InteropError::unsupported_operation(
                self.get_represented_type_info().map(|ti| ti.type_id()),
                Some(key),
                "remove".to_owned(),
            )),
        }
    }

    fn element_type_id(&self) -> Option<TypeId> {
        let elem: TypeId = match self.get_represented_type_info()? {
            TypeInfo::List(list_info) => list_info.item_ty().id(),
            TypeInfo::Array(array_info) => array_info.item_ty().id(),
            TypeInfo::Map(map_info) => map_info.value_ty().id(),
            TypeInfo::Set(set_info) => set_info.value_ty().id(),
            _ => return None,
        };
        Some(elem)
    }

    fn key_type_id(&self) -> Option<TypeId> {
        let key: TypeId = match self.get_represented_type_info()? {
            TypeInfo::Map(map_info) => map_info.key_ty().id(),
            TypeInfo::List(_) | TypeInfo::Array(_) => TypeId::of::<usize>(),
            _ => return None,
        };
        Some(key)
    }

    fn from_reflect(
        reflect: &dyn PartialReflect,
        world: WorldGuard,
    ) -> Result<Box<dyn Reflect>, InteropError> {
        let type_info = reflect.get_represented_type_info().ok_or_else(|| {
            InteropError::failed_from_reflect(
                None,
                "tried to construct a concrete type from dynamic type with no type information"
                    .to_owned(),
            )
        })?;
        let type_id = type_info.type_id();

        let type_registry = world.type_registry();
        let type_registry = type_registry.read();

        let from_reflect_type_data: &ReflectFromReflect =
            type_registry.get_type_data(type_id).ok_or_else(|| {
                InteropError::missing_type_data(type_id, "ReflectFromReflect".to_owned())
            })?;
        from_reflect_type_data.from_reflect(reflect).ok_or_else(|| {
            InteropError::failed_from_reflect(
                Some(type_id),
                "from_reflect returned None".to_owned(),
            )
        })
    }

    /// Try creating an owned partial reflect from a reference. Will try using [`ReflectFromReflect`] first, and if that fails, will clone the value using [`PartialReflect::clone_value`].
    fn from_reflect_or_clone(
        reflect: &dyn PartialReflect,
        world: WorldGuard,
    ) -> Result<Box<dyn PartialReflect>, InteropError> {
        // try from reflect
        match <dyn PartialReflect>::from_reflect(reflect, world.clone()) {
            Ok(v) => Ok(v.into_partial_reflect()),
            Err(_) => reflect
                .reflect_clone()
                .map(|v| v.into_partial_reflect())
                .map_err(|e| {
                    InteropError::failed_from_reflect(
                        reflect.get_represented_type_info().map(|ti| ti.type_id()),
                        e.to_string(),
                    )
                }),
        }
    }

    fn allocate(boxed: Box<dyn Reflect>, world: WorldGuard) -> ReflectReference {
        let allocator = world.allocator();
        let mut allocator = allocator.write();
        ReflectReference::new_allocated_boxed(boxed, &mut allocator)
    }
}

/// Extension trait for TypeInfos providing additional functionality for working with type information.
pub trait TypeInfoExtensions {
    /// Returns the inner type of the set if the type is a set, otherwise None
    fn set_inner_type(&self) -> Option<TypeId>;
    /// Returns true if the type is a result.
    fn is_result(&self) -> bool;
    /// Returns the inner type of the map if the type is a map, otherwise None.
    fn map_inner_types(&self) -> Option<(TypeId, TypeId)>;
    /// Returns the inner type of the list if the type is a list, otherwise None.
    fn list_inner_type(&self) -> Option<TypeId>;
    /// Returns true if the type is a list.
    fn is_list(&self) -> bool;
    /// Returns true if the type is an option.
    fn is_option(&self) -> bool;
    /// Returns the inner type of the option if the type is an option, otherwise None.
    fn option_inner_type(&self) -> Option<TypeId>;
    /// Returns true if the type is the given type from the given crate.
    fn is_type(&self, crate_name: Option<&str>, type_ident: &str) -> bool;
}

impl TypeInfoExtensions for TypeInfo {
    fn is_option(&self) -> bool {
        self.is_type(Some("core"), "Option")
    }

    fn is_result(&self) -> bool {
        self.is_type(Some("core"), "Result")
    }

    fn is_list(&self) -> bool {
        matches!(self, TypeInfo::List(_))
    }

    fn option_inner_type(&self) -> Option<TypeId> {
        if self.is_option() {
            self.generics().first().map(|g| g.type_id())
        } else {
            None
        }
    }

    fn list_inner_type(&self) -> Option<TypeId> {
        Some(self.as_list().ok()?.item_ty().id())
    }

    fn map_inner_types(&self) -> Option<(TypeId, TypeId)> {
        let map = self.as_map().ok()?;
        Some((map.key_ty().id(), map.value_ty().id()))
    }

    fn set_inner_type(&self) -> Option<TypeId> {
        match self {
            TypeInfo::Set(info) => Some(info.value_ty().id()),
            _ => None,
        }
    }

    fn is_type(&self, crate_name: Option<&str>, type_ident: &str) -> bool {
        self.type_path_table().ident() == Some(type_ident)
            && self.type_path_table().crate_name() == crate_name
    }
}

#[cfg(test)]
mod test {
    use bevy_platform::collections::HashMap;
    use bevy_reflect::{DynamicMap, Map};

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
        #[derive(Reflect)]
        enum Test {
            Unit,
        }

        assert!(None::<i32>.as_option().unwrap().is_none());
        assert!(Test::Unit.as_option().is_err())
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
        list.set_as_list(other_ref, |l, r| {
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
        list.set_as_list(other_ref, |l, r| {
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
        list.set_as_list(other_ref, |l, r| {
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
        list.set_as_list(other_ref, |l, r| {
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
        list.set_as_list(other_ref, |l, r| {
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
        let mut map = HashMap::<i32, i32>::default();
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
        let mut map = HashMap::<i32, HashMap<i32, i32>>::default();
        let value = DynamicMap::from_iter(vec![(1, 2), (2, 3), (3, 4)]);
        let value_ref: Box<dyn PartialReflect> = Box::new(value.to_dynamic_map());
        map.insert(1, HashMap::<i32, i32>::default());
        map.insert(2, HashMap::<i32, i32>::default());
        map.insert(3, HashMap::<i32, i32>::default());
        map.try_insert_boxed(Box::new(1), value_ref).unwrap();
        assert!(value.reflect_partial_eq(&map[&1]).unwrap());
    }
}
