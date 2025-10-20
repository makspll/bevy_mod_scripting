use std::{any::TypeId, borrow::Cow, ffi::OsString, path::PathBuf};

use bevy_mod_scripting_display::OrFakeId;
use bevy_reflect::PartialReflect;

/// Attempts to convert the value given by `value` into a suitable value of the target type.
///
/// This will match up primitive types against "similar" or "appropriate" types.
///
/// Types which are not really considered primitives, will be converted via a downcast, meaning
/// they must be of the same type as the target, dynamics won't work.
///
/// This operation is mostly intended to be used when primitives will generally be expected and flexibility is required.
///
/// Note: string ref types cannot be effectively converted into their static reference versions without leaking, so they are not supported here.
pub fn convert(value: &dyn PartialReflect, target: TypeId) -> Option<Box<dyn PartialReflect>> {
    let primitive = Primitive::from(value);
    primitive.convert(target)
}

/// A coercion primitive used for intermediate normalizations.
#[derive(Debug)]
pub enum Primitive<'a> {
    /// Integer family
    I(i64),
    /// Unsized Integer family
    U(u64),
    /// Floating point family
    F(f64),
    /// Boolean family
    B(bool),
    /// String-like family
    S(Cow<'a, str>),
    /// Non primitive
    O(&'a dyn PartialReflect),
    /// A zero sized type with no value
    Unit,
}

impl<'a> Primitive<'a> {
    /// Converts the primitive into the target TypeId
    pub fn convert(self, target: TypeId) -> Option<Box<dyn PartialReflect>> {
        use Primitive::*;
        match self {
            // Integer conversions
            I(i) => {
                if target == TypeId::of::<i8>() {
                    Some(Box::new(i as i8))
                } else if target == TypeId::of::<i16>() {
                    Some(Box::new(i as i16))
                } else if target == TypeId::of::<i32>() {
                    Some(Box::new(i as i32))
                } else if target == TypeId::of::<i64>() {
                    Some(Box::new(i))
                } else if target == TypeId::of::<i128>() {
                    Some(Box::new(i as i128))
                } else if target == TypeId::of::<isize>() {
                    Some(Box::new(i as isize))
                } else if target == TypeId::of::<u8>() {
                    Some(Box::new(i as u8))
                } else if target == TypeId::of::<u16>() {
                    Some(Box::new(i as u16))
                } else if target == TypeId::of::<u32>() {
                    Some(Box::new(i as u32))
                } else if target == TypeId::of::<u64>() {
                    Some(Box::new(i as u64))
                } else if target == TypeId::of::<u128>() {
                    Some(Box::new(i as u128))
                } else if target == TypeId::of::<usize>() {
                    Some(Box::new(i as usize))
                } else if target == TypeId::of::<f32>() {
                    Some(Box::new(i as f32))
                } else if target == TypeId::of::<f64>() {
                    Some(Box::new(i as f64))
                } else {
                    None
                }
            }

            // Unsigned integers
            U(u) => {
                if target == TypeId::of::<i8>() {
                    Some(Box::new(u as i8))
                } else if target == TypeId::of::<i16>() {
                    Some(Box::new(u as i16))
                } else if target == TypeId::of::<i32>() {
                    Some(Box::new(u as i32))
                } else if target == TypeId::of::<i64>() {
                    Some(Box::new(u as i64))
                } else if target == TypeId::of::<i128>() {
                    Some(Box::new(u as i128))
                } else if target == TypeId::of::<isize>() {
                    Some(Box::new(u as isize))
                } else if target == TypeId::of::<u8>() {
                    Some(Box::new(u as u8))
                } else if target == TypeId::of::<u16>() {
                    Some(Box::new(u as u16))
                } else if target == TypeId::of::<u32>() {
                    Some(Box::new(u as u32))
                } else if target == TypeId::of::<u64>() {
                    Some(Box::new(u))
                } else if target == TypeId::of::<u128>() {
                    Some(Box::new(u as u128))
                } else if target == TypeId::of::<usize>() {
                    Some(Box::new(u as usize))
                } else if target == TypeId::of::<f32>() {
                    Some(Box::new(u as f32))
                } else if target == TypeId::of::<f64>() {
                    Some(Box::new(u as f64))
                } else {
                    None
                }
            }

            // Floating point conversions
            F(f) => {
                if target == TypeId::of::<f32>() {
                    Some(Box::new(f as f32))
                } else if target == TypeId::of::<f64>() {
                    Some(Box::new(f))
                } else if target == TypeId::of::<i8>() {
                    Some(Box::new(f as i8))
                } else if target == TypeId::of::<i16>() {
                    Some(Box::new(f as i16))
                } else if target == TypeId::of::<i32>() {
                    Some(Box::new(f as i32))
                } else if target == TypeId::of::<i64>() {
                    Some(Box::new(f as i64))
                } else if target == TypeId::of::<i128>() {
                    Some(Box::new(f as i128))
                } else if target == TypeId::of::<isize>() {
                    Some(Box::new(f as isize))
                } else if target == TypeId::of::<u8>() {
                    Some(Box::new(f as u8))
                } else if target == TypeId::of::<u16>() {
                    Some(Box::new(f as u16))
                } else if target == TypeId::of::<u32>() {
                    Some(Box::new(f as u32))
                } else if target == TypeId::of::<u64>() {
                    Some(Box::new(f as u64))
                } else if target == TypeId::of::<u128>() {
                    Some(Box::new(f as u128))
                } else if target == TypeId::of::<usize>() {
                    Some(Box::new(f as usize))
                } else {
                    None
                }
            }

            // Boolean conversions
            B(b) => {
                if target == TypeId::of::<bool>() {
                    Some(Box::new(b))
                } else if target == TypeId::of::<i64>() {
                    Some(Box::new(if b { 1 } else { 0 }))
                } else if target == TypeId::of::<u64>() {
                    Some(Box::new(if b { 1u64 } else { 0u64 }))
                } else if target == TypeId::of::<f64>() {
                    Some(Box::new(if b { 1.0 } else { 0.0 }))
                } else {
                    None
                }
            }

            // Strings
            S(s) => {
                if target == TypeId::of::<String>() {
                    Some(Box::new(String::from(s)))
                } else if target == TypeId::of::<PathBuf>() {
                    Some(Box::new(PathBuf::from(s.into_owned())))
                } else if target == TypeId::of::<OsString>() {
                    Some(Box::new(OsString::from(s.into_owned())))
                } else {
                    None
                }
            }

            // Non-primitives: only downcast if type matches exactly
            O(o) => {
                if o.get_represented_type_info()
                    .map(|info| info.type_id())
                    .or_fake_id()
                    == target
                {
                    let err = o.reflect_clone();
                    Some(err.ok()?)
                } else {
                    None
                }
            }

            Unit => {
                if target == TypeId::of::<()>() {
                    Some(Box::new(()))
                } else {
                    None
                }
            }
        }
    }
}

impl<'a> From<&'a dyn PartialReflect> for Primitive<'a> {
    fn from(v: &'a dyn PartialReflect) -> Self {
        let t = v
            .get_represented_type_info()
            .map(|info| info.type_id())
            .or_fake_id();

        if t == TypeId::of::<usize>()
            && let Some(v) = v.try_downcast_ref::<usize>()
        {
            Primitive::U(*v as u64)
        } else if t == TypeId::of::<isize>()
            && let Some(v) = v.try_downcast_ref::<isize>()
        {
            Primitive::I(*v as i64)
        } else if t == TypeId::of::<bool>()
            && let Some(v) = v.try_downcast_ref::<bool>()
        {
            Primitive::B(*v)
        } else if t == TypeId::of::<u8>()
            && let Some(v) = v.try_downcast_ref::<u8>()
        {
            Primitive::U(*v as u64)
        } else if t == TypeId::of::<u16>()
            && let Some(v) = v.try_downcast_ref::<u16>()
        {
            Primitive::U(*v as u64)
        } else if t == TypeId::of::<u32>()
            && let Some(v) = v.try_downcast_ref::<u32>()
        {
            Primitive::U(*v as u64)
        } else if t == TypeId::of::<u64>()
            && let Some(v) = v.try_downcast_ref::<u64>()
        {
            Primitive::U(*v)
        } else if t == TypeId::of::<u128>()
            && let Some(v) = v.try_downcast_ref::<u128>()
        {
            Primitive::U(*v as u64)
        } else if t == TypeId::of::<i8>()
            && let Some(v) = v.try_downcast_ref::<i8>()
        {
            Primitive::I(*v as i64)
        } else if t == TypeId::of::<i16>()
            && let Some(v) = v.try_downcast_ref::<i16>()
        {
            Primitive::I(*v as i64)
        } else if t == TypeId::of::<i32>()
            && let Some(v) = v.try_downcast_ref::<i32>()
        {
            Primitive::I(*v as i64)
        } else if t == TypeId::of::<i64>()
            && let Some(v) = v.try_downcast_ref::<i64>()
        {
            Primitive::I(*v)
        } else if t == TypeId::of::<i128>()
            && let Some(v) = v.try_downcast_ref::<i128>()
        {
            Primitive::I(*v as i64)
        } else if t == TypeId::of::<f32>()
            && let Some(v) = v.try_downcast_ref::<f32>()
        {
            Primitive::F(*v as f64)
        } else if t == TypeId::of::<f64>()
            && let Some(v) = v.try_downcast_ref::<f64>()
        {
            Primitive::F(*v)
        } else if t == TypeId::of::<Cow<'static, str>>()
            && let Some(v) = v.try_downcast_ref::<Cow<'static, str>>()
        {
            Primitive::S(v.clone())
        } else if t == TypeId::of::<String>()
            && let Some(v) = v.try_downcast_ref::<String>()
        {
            Primitive::S(Cow::Borrowed(v.as_str()))
        } else if t == TypeId::of::<&'static str>()
            && let Some(v) = v.try_downcast_ref::<&'static str>()
        {
            Primitive::S(Cow::Borrowed(*v))
        } else if t == TypeId::of::<PathBuf>()
            && let Some(v) = v.try_downcast_ref::<PathBuf>()
        {
            Primitive::S(v.to_string_lossy())
        } else if t == TypeId::of::<OsString>()
            && let Some(v) = v.try_downcast_ref::<OsString>()
        {
            Primitive::S(v.as_os_str().to_string_lossy())
        } else if t == TypeId::of::<()>()
            && let Some(_) = v.try_downcast_ref::<()>()
        {
            Primitive::Unit
        } else {
            Primitive::O(v)
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy_reflect::Reflect;

    use super::*;

    #[test]
    fn test_integer_conversions() {
        let i: i64 = 42;
        let value: &dyn PartialReflect = &i;

        // i64 -> other integer types
        let targets: &[(TypeId, i64)] = &[
            (TypeId::of::<i8>(), 42),
            (TypeId::of::<i16>(), 42),
            (TypeId::of::<i32>(), 42),
            (TypeId::of::<i64>(), 42),
            (TypeId::of::<i128>(), 42),
            (TypeId::of::<isize>(), 42),
            (TypeId::of::<u8>(), 42),
            (TypeId::of::<u16>(), 42),
            (TypeId::of::<u32>(), 42),
            (TypeId::of::<u64>(), 42),
            (TypeId::of::<u128>(), 42),
            (TypeId::of::<usize>(), 42),
        ];

        for (ty, expected) in targets {
            let boxed = convert(value, *ty).unwrap();
            let downcasted = match *ty {
                t if t == TypeId::of::<i8>() => *boxed.try_downcast_ref::<i8>().unwrap() as i64,
                t if t == TypeId::of::<i16>() => *boxed.try_downcast_ref::<i16>().unwrap() as i64,
                t if t == TypeId::of::<i32>() => *boxed.try_downcast_ref::<i32>().unwrap() as i64,
                t if t == TypeId::of::<i64>() => *boxed.try_downcast_ref::<i64>().unwrap(),
                t if t == TypeId::of::<i128>() => *boxed.try_downcast_ref::<i128>().unwrap() as i64,
                t if t == TypeId::of::<isize>() => {
                    *boxed.try_downcast_ref::<isize>().unwrap() as i64
                }
                t if t == TypeId::of::<u8>() => *boxed.try_downcast_ref::<u8>().unwrap() as i64,
                t if t == TypeId::of::<u16>() => *boxed.try_downcast_ref::<u16>().unwrap() as i64,
                t if t == TypeId::of::<u32>() => *boxed.try_downcast_ref::<u32>().unwrap() as i64,
                t if t == TypeId::of::<u64>() => *boxed.try_downcast_ref::<u64>().unwrap() as i64,
                t if t == TypeId::of::<u128>() => *boxed.try_downcast_ref::<u128>().unwrap() as i64,
                t if t == TypeId::of::<usize>() => {
                    *boxed.try_downcast_ref::<usize>().unwrap() as i64
                }
                _ => panic!("Unexpected type"),
            };
            assert_eq!(downcasted, *expected);
        }
    }

    #[test]
    fn test_float_conversions() {
        let f: f64 = 3.5;
        let value: &dyn PartialReflect = &f;

        let targets: &[(TypeId, f64)] = &[
            (TypeId::of::<f32>(), 3.5), // f32 precision
            (TypeId::of::<f64>(), 3.5),
            (TypeId::of::<i64>(), 3.00),
            (TypeId::of::<u64>(), 3.00),
            (TypeId::of::<i32>(), 3.00),
            (TypeId::of::<u32>(), 3.00),
        ];

        for (ty, expected) in targets {
            let boxed = convert(value, *ty).unwrap();
            let val = if *ty == TypeId::of::<f32>() {
                *boxed.try_downcast_ref::<f32>().unwrap() as f64
            } else if *ty == TypeId::of::<f64>() {
                *boxed.try_downcast_ref::<f64>().unwrap()
            } else if *ty == TypeId::of::<i64>() {
                *boxed.try_downcast_ref::<i64>().unwrap() as f64
            } else if *ty == TypeId::of::<u64>() {
                *boxed.try_downcast_ref::<u64>().unwrap() as f64
            } else if *ty == TypeId::of::<i32>() {
                *boxed.try_downcast_ref::<i32>().unwrap() as f64
            } else if *ty == TypeId::of::<u32>() {
                *boxed.try_downcast_ref::<u32>().unwrap() as f64
            } else {
                panic!("Unexpected type");
            };
            assert_eq!(val, *expected);
        }
    }

    #[test]
    fn test_boolean_conversions() {
        let b = true;
        let value: &dyn PartialReflect = &b;

        let targets: &[(TypeId, bool)] = &[(TypeId::of::<bool>(), true)];

        for (ty, expected) in targets {
            let boxed = convert(value, *ty).unwrap();
            let val = *boxed.try_downcast_ref::<bool>().unwrap();
            assert_eq!(val, *expected);
        }
    }

    #[test]
    fn test_string_conversions() {
        let s = "hello".to_string();
        let value: &dyn PartialReflect = &s;

        let targets: &[(TypeId, Option<&str>)] = &[
            (TypeId::of::<String>(), Some("hello")),
            (TypeId::of::<&'static str>(), None),
            (TypeId::of::<Cow<'static, str>>(), None),
            (TypeId::of::<PathBuf>(), Some("hello")),
            (TypeId::of::<OsString>(), Some("hello")),
        ];

        for (ty, expected) in targets {
            let boxed = convert(value, *ty);
            if *ty == TypeId::of::<String>() {
                assert_eq!(
                    boxed.unwrap().try_downcast_ref::<String>().unwrap(),
                    expected.unwrap()
                );
            } else if *ty == TypeId::of::<&'static str>() || *ty == TypeId::of::<Cow<str>>() {
                assert!(boxed.is_none());
            } else if *ty == TypeId::of::<PathBuf>() {
                assert_eq!(
                    boxed
                        .unwrap()
                        .try_downcast_ref::<PathBuf>()
                        .unwrap()
                        .to_str(),
                    *expected
                );
            } else if *ty == TypeId::of::<OsString>() {
                assert_eq!(
                    boxed
                        .unwrap()
                        .try_downcast_ref::<OsString>()
                        .unwrap()
                        .to_str(),
                    *expected
                );
            }
        }
    }

    #[test]
    fn test_unit_conversion() {
        let unit: &dyn PartialReflect = &();
        let boxed = convert(unit, TypeId::of::<()>()).unwrap();
        assert!(boxed.try_downcast_ref::<()>().is_some());
    }

    #[test]
    fn test_nonprimitive_downcast() {
        #[derive(Reflect)]
        struct MyStruct(u32);

        let s = MyStruct(10);
        let value: &dyn PartialReflect = &s;

        // Exact type match works
        let boxed = convert(value, TypeId::of::<MyStruct>()).unwrap();
        let val = boxed.try_downcast_ref::<MyStruct>().unwrap();
        assert_eq!(val.0, 10);

        // Mismatched type fails
        assert!(convert(value, TypeId::of::<i32>()).is_none());
    }
}
