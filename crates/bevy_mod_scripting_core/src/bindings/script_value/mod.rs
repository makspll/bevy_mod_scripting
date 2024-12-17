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
    error::{InteropError, InteropErrorInner, ScriptError, ScriptResult},
    reflection_extensions::{PartialReflectExt, TypeIdExtensions, TypeInfoExtensions},
};

mod from;
mod into;
pub use {from::*, into::*};

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
    Error(InteropError),
    /// A placeholder for a [`crate::bindings::WorldCallbackAccess`] value.
    World,
}

impl ScriptValue {
    pub fn type_name(&self) -> String {
        match self {
            ScriptValue::Unit => "Unit".to_owned(),
            ScriptValue::Bool(_) => "Bool".to_owned(),
            ScriptValue::Integer(_) => "Integer".to_owned(),
            ScriptValue::Float(_) => "Float".to_owned(),
            ScriptValue::String(_) => "String".to_owned(),
            ScriptValue::List(_) => "List".to_owned(),
            ScriptValue::Reference(_) => "Reference".to_owned(),
            ScriptValue::Error(_) => "Error".to_owned(),
            ScriptValue::World => "World".to_owned(),
        }
    }
}

impl From<()> for ScriptValue {
    fn from(_: ()) -> Self {
        ScriptValue::Unit
    }
}

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

// impl From<ScriptError> for ScriptValue {
//     fn from(value: ScriptError) -> Self {
//         ScriptValue::Error(value)
//     }
// }

impl From<InteropError> for ScriptValue {
    fn from(value: InteropError) -> Self {
        ScriptValue::Error(value)
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

impl<T: Into<ScriptValue>, E: Into<InteropError>> From<Result<T, E>> for ScriptValue {
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

/// A trait for converting a value into a [`ScriptValue`].
///
/// If a [`crate::error::InteropError::better_conversion_exists`] is thrown, the conversion is not possible and you should treat this as a sign to try another method.
pub trait IntoScriptValue {
    /// Converts the value into a [`ScriptValue`]. This conversion should:
    /// - Ideally convert to a concrete instance of [`Self`] or at least a concrete type representing [`Self`].
    /// - If the value is not possible to convert nicely as a value throw a [`crate::error::InteropError::better_conversion_exists`] error so the caller can try another method.
    fn into_script_value(self, world: WorldGuard) -> Result<ScriptValue, InteropError>;

    /// Some values are better represented as references returned to a script.
    /// This method should be called when such values might be returned to a script.
    /// By default this will call [`IntoScriptValue::into_script_value`] and convert the underlying [`&dyn PartialReflect`]
    /// However if `into_script_value` throws a [`crate::error::InteropError::better_conversion_exists`] error, this method will directly return the reference instead.
    fn reference_into_script_value(
        // type_id: TypeId,
        self_ref: ReflectReference,
        world: WorldGuard,
    ) -> Result<ScriptValue, InteropError> {
        match self_ref.with_reflect(world.clone(), |r| r.into_script_value(world))? {
            Err(e) if matches!(e.inner(), InteropErrorInner::BetterConversionExists { .. }) => {
                Ok(ScriptValue::Reference(self_ref))
            }
            e => e,
        }
    }
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
