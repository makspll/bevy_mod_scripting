use bevy_reflect::{DynamicTypePath, PartialReflect, ReflectRef, VariantField};

use crate::*;

/// Contains a strategy for printing a `Reflect` value as if it was its native `Debug` implementation.
pub struct ReflectPrinter<'f, 'b: 'f> {
    pub(crate) formatter: &'f mut std::fmt::Formatter<'b>,
    pub(crate) result: std::fmt::Result,
}

impl<'f, 'b: 'f> ReflectPrinter<'f, 'b> {
    /// Creates a new `ReflectPrinter` with the given formatter.
    pub fn new(formatter: &'f mut std::fmt::Formatter<'b>) -> ReflectPrinter<'f, 'b> {
        ReflectPrinter {
            formatter,
            result: Ok(()),
        }
    }

    /// Prints a `Reflect` value as if it was its native `Debug` implementation.
    pub fn debug(&mut self, value: &dyn PartialReflect) -> std::fmt::Result {
        // try to print the value as if it was its native Debug implementation
        match value.reflect_ref() {
            ReflectRef::Struct(s) => self
                .formatter
                .debug_struct_with_type_info(s.reflect_ident_or_short_path())
                .build_with(|mut b| {
                    for (i, field) in s.iter_fields().enumerate() {
                        b.field(
                            s.name_at(i).unwrap_or("unknown"),
                            &PrintReflectAsDebug(field),
                        );
                    }
                    b
                })
                .finish(),
            ReflectRef::TupleStruct(s) => self
                .formatter
                .debug_tuple_with_type_info(s.reflect_ident_or_short_path())
                .build_with(|mut b| {
                    for field in s.iter_fields() {
                        b.field(&PrintReflectAsDebug(field));
                    }
                    b
                })
                .finish(),
            ReflectRef::Tuple(t) => self
                .formatter
                .debug_tuple_with_type_info(t.reflect_ident_or_short_path())
                .build_with(|mut b| {
                    for field in t.iter_fields() {
                        b.field(&PrintReflectAsDebug(field));
                    }
                    b
                })
                .finish(),
            ReflectRef::List(l) => self
                .formatter
                .debug_list_with_type_info()
                .build_with(|mut b| {
                    for field in l.iter() {
                        b.entry(&PrintReflectAsDebug(field));
                    }
                    b
                })
                .finish(),
            ReflectRef::Array(a) => self
                .formatter
                .debug_list_with_type_info()
                .build_with(|mut b| {
                    for field in a.iter() {
                        b.entry(&PrintReflectAsDebug(field));
                    }
                    b
                })
                .finish(),
            ReflectRef::Map(m) => self
                .formatter
                .debug_map_with_type_info()
                .build_with(|mut b| {
                    for (k, v) in m.iter() {
                        b.entry(&PrintReflectAsDebug(k), &PrintReflectAsDebug(v));
                    }
                    b
                })
                .finish(),
            ReflectRef::Set(s) => self
                .formatter
                .debug_set_with_type_info()
                .build_with(|mut b| {
                    for v in s.iter() {
                        b.entry(&PrintReflectAsDebug(v));
                    }
                    b
                })
                .finish(),
            ReflectRef::Enum(e) => {
                let is_tuple = !matches!(e.variant_type(), bevy_reflect::VariantType::Struct);
                let variant_path = e.variant_name();
                if is_tuple {
                    self.formatter
                        .debug_tuple_with_type_info(variant_path)
                        .build_with(|mut b| {
                            for f in e.iter_fields() {
                                if let VariantField::Tuple(v) = f {
                                    b.field(&PrintReflectAsDebug(v));
                                }
                                // should not be possible
                            }
                            b
                        })
                        .finish()
                } else {
                    self.formatter
                        .debug_struct_with_type_info(variant_path)
                        .build_with(|mut b| {
                            for f in e.iter_fields() {
                                if let VariantField::Struct(name, value) = f {
                                    b.field(name, &PrintReflectAsDebug(value));
                                }
                                // should not be possible
                            }
                            b
                        })
                        .finish()
                }
            }
            ReflectRef::Opaque(o) => o.debug(self.formatter),
        }
    }

    /// Finalizes the printing process and returns the result.
    pub fn finish(self) -> std::fmt::Result {
        self.result
    }
}

/// A helper trait to get either the ident or the path of a type.
trait GetIdentOrPath {
    fn reflect_ident_or_short_path(&self) -> &str;
}

impl<T: DynamicTypePath + ?Sized> GetIdentOrPath for T {
    fn reflect_ident_or_short_path(&self) -> &str {
        self.reflect_type_ident()
            .unwrap_or_else(|| self.reflect_short_type_path())
    }
}

/// A wrapper type that implements `Debug` for any `PartialReflect` by using `ReflectPrinter`.
pub struct PrintReflectAsDebug<'a>(pub &'a dyn PartialReflect);

impl DebugWithTypeInfo for PrintReflectAsDebug<'_> {
    fn to_string_with_type_info(
        &self,
        f: &mut std::fmt::Formatter,
        _type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        ReflectPrinter::new(f).debug(self.0)
    }
}

impl std::fmt::Debug for PrintReflectAsDebug<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        WithTypeInfo(self).fmt(f)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reflect_printer() {
        assert_eq!(format!("{:?}", PrintReflectAsDebug(&42u32)), "42");
        assert_eq!(format!("{:?}", PrintReflectAsDebug(&"asd")), "\"asd\"");
        assert_eq!(
            format!("{:?}", PrintReflectAsDebug(&vec![1, 2, 3])),
            "[1, 2, 3]"
        );
        assert_eq!(
            format!("{:?}", PrintReflectAsDebug(&Some("value"))),
            "Some(\"value\")"
        );
        assert_eq!(format!("{:?}", PrintReflectAsDebug(&None::<u32>)), "None");
        assert_eq!(
            format!("{:?}", PrintReflectAsDebug(&("a", 42, true))),
            "(&str, i32, bool)(\"a\", 42, true)"
        );
        assert_eq!(
            format!(
                "{:?}",
                PrintReflectAsDebug(&bevy_platform::collections::HashMap::<u32, &str>::from([(
                    1, "a"
                ),]))
            ),
            "{1: \"a\"}"
        );
        assert_eq!(
            format!("{:?}", PrintReflectAsDebug(&[1, 2, 3])),
            "[1, 2, 3]"
        );
        assert_eq!(
            format!("{:?}", PrintReflectAsDebug(&Some(vec![1, 2, 3]))),
            "Some([1, 2, 3])"
        );
    }
}
