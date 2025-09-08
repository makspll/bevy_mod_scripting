use bevy_reflect::{DynamicTypePath, PartialReflect, ReflectRef, VariantField};

use crate::*;

/// Contains a strategy for printing a `Reflect` value as if it was its native `Debug` implementation.
pub struct ReflectPrinter<'f, 'b: 'f, 't> {
    pub(crate) formatter: &'f mut std::fmt::Formatter<'b>,
    pub(crate) result: std::fmt::Result,
    pub(crate) type_info: Option<&'t dyn GetTypeInfo>,
}

impl<'f, 'b: 'f, 't> ReflectPrinter<'f, 'b, 't> {
    /// Creates a new `ReflectPrinter` with the given formatter.
    pub fn new(
        formatter: &'f mut std::fmt::Formatter<'b>,
        type_info: Option<&'t dyn GetTypeInfo>,
    ) -> ReflectPrinter<'f, 'b, 't> {
        ReflectPrinter {
            formatter,
            result: Ok(()),
            type_info,
        }
    }

    /// Prints a `Reflect` value as if it was its native `Debug` implementation.
    pub fn debug(&mut self, value: &dyn PartialReflect) -> std::fmt::Result {
        if let Some(type_info_provider) = &self.type_info {
            if let Some(reflect_type) = value.try_as_reflect()
                && let Some(display_type_data) = type_info_provider
                    .get_type_data::<ReflectDisplayWithTypeInfo>(reflect_type.type_id())
                && let Some(as_dyn_trait) = display_type_data.get(reflect_type)
            {
                return as_dyn_trait.display_with_type_info(self.formatter, self.type_info);
            }
        }

        // try to print the value as if it was its native Debug implementation
        match value.reflect_ref() {
            ReflectRef::Struct(s) => self
                .formatter
                .debug_struct_with_type_info(s.reflect_ident_or_short_path(), self.type_info)
                .build_with(|mut b| {
                    for (i, field) in s.iter_fields().enumerate() {
                        b.field(
                            s.name_at(i).unwrap_or("unknown"),
                            &PrintReflectAsDebug::new_with_opt_info(field, self.type_info),
                        );
                    }
                    b
                })
                .finish(),
            ReflectRef::TupleStruct(s) => self
                .formatter
                .debug_tuple_with_type_info(s.reflect_ident_or_short_path(), self.type_info)
                .build_with(|mut b| {
                    for field in s.iter_fields() {
                        b.field(&PrintReflectAsDebug::new_with_opt_info(
                            field,
                            self.type_info,
                        ));
                    }
                    b
                })
                .finish(),
            ReflectRef::Tuple(t) => self
                .formatter
                .debug_tuple_with_type_info(t.reflect_ident_or_short_path(), self.type_info)
                .build_with(|mut b| {
                    for field in t.iter_fields() {
                        b.field(&PrintReflectAsDebug::new_with_opt_info(
                            field,
                            self.type_info,
                        ));
                    }
                    b
                })
                .finish(),
            ReflectRef::List(l) => self
                .formatter
                .debug_list_with_type_info(self.type_info)
                .build_with(|mut b| {
                    for field in l.iter() {
                        b.entry(&PrintReflectAsDebug::new_with_opt_info(
                            field,
                            self.type_info,
                        ));
                    }
                    b
                })
                .finish(),
            ReflectRef::Array(a) => self
                .formatter
                .debug_list_with_type_info(self.type_info)
                .build_with(|mut b| {
                    for field in a.iter() {
                        b.entry(&PrintReflectAsDebug::new_with_opt_info(
                            field,
                            self.type_info,
                        ));
                    }
                    b
                })
                .finish(),
            ReflectRef::Map(m) => self
                .formatter
                .debug_map_with_type_info(self.type_info)
                .build_with(|mut b| {
                    for (k, v) in m.iter() {
                        b.entry(
                            &PrintReflectAsDebug::new_with_opt_info(k, self.type_info),
                            &PrintReflectAsDebug::new_with_opt_info(v, self.type_info),
                        );
                    }
                    b
                })
                .finish(),
            ReflectRef::Set(s) => self
                .formatter
                .debug_set_with_type_info(self.type_info)
                .build_with(|mut b| {
                    for v in s.iter() {
                        b.entry(&PrintReflectAsDebug::new_with_opt_info(v, self.type_info));
                    }
                    b
                })
                .finish(),
            ReflectRef::Enum(e) => {
                let is_tuple = !matches!(e.variant_type(), bevy_reflect::VariantType::Struct);
                let variant_path = e.variant_name();
                if is_tuple {
                    self.formatter
                        .debug_tuple_with_type_info(variant_path, self.type_info)
                        .build_with(|mut b| {
                            for f in e.iter_fields() {
                                if let VariantField::Tuple(v) = f {
                                    b.field(&PrintReflectAsDebug::new_with_opt_info(
                                        v,
                                        self.type_info,
                                    ));
                                }
                                // should not be possible
                            }
                            b
                        })
                        .finish()
                } else {
                    self.formatter
                        .debug_struct_with_type_info(variant_path, self.type_info)
                        .build_with(|mut b| {
                            for f in e.iter_fields() {
                                if let VariantField::Struct(name, value) = f {
                                    b.field(
                                        name,
                                        &PrintReflectAsDebug::new_with_opt_info(
                                            value,
                                            self.type_info,
                                        ),
                                    );
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
///
/// For opaque types will optionally seek [`ReflectDisplayWithTypeInfo`] type data in the registry
pub struct PrintReflectAsDebug<'a, 'g>(&'a dyn PartialReflect, Option<&'g dyn GetTypeInfo>);

impl<'a, 'g> PrintReflectAsDebug<'a, 'g> {
    /// Constructs a new [`PrintReflectAsDebug`] which will use the global type info provider
    pub fn new(val: &'a dyn PartialReflect) -> Self {
        Self(val, None)
    }

    /// Constructs a new [`PrintReflectAsDebug`] which will use the provided type info provider and fallback to the global
    pub fn new_with_opt_info(
        val: &'a dyn PartialReflect,
        info: Option<&'g dyn GetTypeInfo>,
    ) -> Self {
        Self(val, info)
    }
}

impl DebugWithTypeInfo for PrintReflectAsDebug<'_, '_> {
    fn to_string_with_type_info(
        &self,
        f: &mut std::fmt::Formatter,
        type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        ReflectPrinter::new(f, self.1.or(type_info_provider)).debug(self.0)
    }
}

impl std::fmt::Debug for PrintReflectAsDebug<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        WithTypeInfo::new_with_opt_info(self, self.1).fmt(f)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reflect_printer() {
        assert_eq!(format!("{:?}", PrintReflectAsDebug::new(&42u32)), "42");
        assert_eq!(format!("{:?}", PrintReflectAsDebug::new(&"asd")), "\"asd\"");
        assert_eq!(
            format!("{:?}", PrintReflectAsDebug::new(&vec![1, 2, 3])),
            "[1, 2, 3]"
        );
        assert_eq!(
            format!("{:?}", PrintReflectAsDebug::new(&Some("value"))),
            "Some(\"value\")"
        );
        assert_eq!(
            format!("{:?}", PrintReflectAsDebug::new(&None::<u32>)),
            "None"
        );
        assert_eq!(
            format!("{:?}", PrintReflectAsDebug::new(&("a", 42, true))),
            "(&str, i32, bool)(\"a\", 42, true)"
        );
        assert_eq!(
            format!(
                "{:?}",
                PrintReflectAsDebug::new(&bevy_platform::collections::HashMap::<u32, &str>::from(
                    [(1, "a"),]
                ))
            ),
            "{1: \"a\"}"
        );
        assert_eq!(
            format!("{:?}", PrintReflectAsDebug::new(&[1, 2, 3])),
            "[1, 2, 3]"
        );
        assert_eq!(
            format!("{:?}", PrintReflectAsDebug::new(&Some(vec![1, 2, 3]))),
            "Some([1, 2, 3])"
        );
    }
}
