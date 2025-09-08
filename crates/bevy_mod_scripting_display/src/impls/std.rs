use std::{borrow::Cow, panic::Location, sync::Arc};

use crate::*;

impl_debug_with_type_info_via_display!(String, &'_ str, Cow<'_, str>);
impl_debug_with_type_info_via_display!(u8, u16, u32, u64, u128, usize);
impl_debug_with_type_info_via_display!(i8, i16, i32, i64, i128, isize);
impl_debug_with_type_info_via_display!(f32, f64);
impl_debug_with_type_info_via_display!(bool, char);

impl DebugWithTypeInfo for TypeId {
    fn to_string_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        if *self == TypeId::of::<FakeType>() {
            return f.write_str("Unknown Type");
        } else if *self == TypeId::of::<bevy_ecs::world::World>() {
            // does not implement Reflect, so we do this manually
            return f.write_str("World");
        }

        let name = if let Some(type_info_provider) = type_info_provider {
            if let Some(type_info) = type_info_provider.get_type_info(*self) {
                type_info.type_path_table().path().to_string()
            } else {
                format!("Unregistered Type - {self:?}")
            }
        } else {
            format!("{self:?}")
        };

        f.debug_tuple("TypeId").field(&name).finish()
    }
}

impl<T: DebugWithTypeInfo> DebugWithTypeInfo for Arc<T> {
    fn to_string_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        (**self).to_string_with_type_info(f, type_info_provider)
    }
}

impl<T: DebugWithTypeInfo> DebugWithTypeInfo for Box<T> {
    fn to_string_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        (**self).to_string_with_type_info(f, type_info_provider)
    }
}

impl<T: DebugWithTypeInfo> DebugWithTypeInfo for Option<T> {
    fn to_string_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        match self {
            Some(value) => f
                .debug_tuple_with_type_info("Some", type_info_provider)
                .field(value as &dyn DebugWithTypeInfo)
                .finish(),
            None => f
                .debug_tuple_with_type_info("None", type_info_provider)
                .finish(),
        }
    }
}

impl<T: DebugWithTypeInfo, E: DebugWithTypeInfo> DebugWithTypeInfo for Result<T, E> {
    fn to_string_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        match self {
            Ok(v) => f
                .debug_tuple_with_type_info("Ok", type_info_provider)
                .field(v as &dyn DebugWithTypeInfo)
                .finish(),
            Err(v) => f
                .debug_tuple_with_type_info("Err", type_info_provider)
                .field(v as &dyn DebugWithTypeInfo)
                .finish(),
        }
    }
}

impl<T: DebugWithTypeInfo> DebugWithTypeInfo for Vec<T> {
    fn to_string_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        f.debug_list_with_type_info(type_info_provider)
            .entries(self.iter().map(|v| v as &dyn DebugWithTypeInfo))
            .finish()
    }
}

impl<K: DebugWithTypeInfo, V: DebugWithTypeInfo, S> DebugWithTypeInfo
    for std::collections::HashMap<K, V, S>
{
    fn to_string_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        f.debug_map_with_type_info(type_info_provider)
            .entries(
                self.iter()
                    .map(|(k, v)| (k as &dyn DebugWithTypeInfo, v as &dyn DebugWithTypeInfo)),
            )
            .finish()
    }
}

impl<K: DebugWithTypeInfo, S> DebugWithTypeInfo for std::collections::HashSet<K, S> {
    fn to_string_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        f.debug_set_with_type_info(type_info_provider)
            .entries(self.iter().map(|v| v as &dyn DebugWithTypeInfo))
            .finish()
    }
}

impl<K: DebugWithTypeInfo> DebugWithTypeInfo for std::collections::BTreeSet<K> {
    fn to_string_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        f.debug_set_with_type_info(type_info_provider)
            .entries(self.iter().map(|v| v as &dyn DebugWithTypeInfo))
            .finish()
    }
}

impl<K: DebugWithTypeInfo, V: DebugWithTypeInfo> DebugWithTypeInfo
    for std::collections::BTreeMap<K, V>
{
    fn to_string_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        f.debug_map_with_type_info(type_info_provider)
            .entries(
                self.iter()
                    .map(|(k, v)| (k as &dyn DebugWithTypeInfo, v as &dyn DebugWithTypeInfo)),
            )
            .finish()
    }
}

impl DebugWithTypeInfo for Location<'_> {
    fn to_string_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        f.debug_struct("Location")
            .field("file", &self.file())
            .field("line", &self.line())
            .field("column", &self.column())
            .finish()
    }
}

// -- DisplayWithTypeInfo implementations --
// For primitive/displayable types we delegate to their Display impls.
macro_rules! impl_display_with_type_info_via_display {
    ($($t:ty),*) => {
        $(
            impl DisplayWithTypeInfo for $t {
                fn display_with_type_info(
                    &self,
                    f: &mut std::fmt::Formatter<'_>,
                    _type_info_provider: Option<&dyn GetTypeInfo>,
                ) -> std::fmt::Result {
                    <Self as std::fmt::Display>::fmt(self, f)
                }
            }
        )*
    };
}

impl_display_with_type_info_via_display!(String, &'_ str, Cow<'_, str>);
impl_display_with_type_info_via_display!(u8, u16, u32, u64, u128, usize);
impl_display_with_type_info_via_display!(i8, i16, i32, i64, i128, isize);
impl_display_with_type_info_via_display!(f32, f64);
impl_display_with_type_info_via_display!(bool, char);

impl DisplayWithTypeInfo for TypeId {
    fn display_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        if *self == TypeId::of::<FakeType>() {
            return f.write_str("Unknown Type");
        } else if *self == TypeId::of::<bevy_ecs::world::World>() {
            return f.write_str("World");
        }

        let name = if let Some(type_info_provider) = type_info_provider {
            if let Some(type_info) = type_info_provider.get_type_info(*self) {
                type_info.type_path_table().path().to_string()
            } else {
                format!("Unregistered Type - {self:?}")
            }
        } else {
            format!("{self:?}")
        };

        // Display should be prettier: just print the resolved name (no tuple/struct wrapper)
        f.write_str(&name)
    }
}

impl<T: DisplayWithTypeInfo> DisplayWithTypeInfo for Arc<T> {
    fn display_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        (**self).display_with_type_info(f, type_info_provider)
    }
}

impl DisplayWithTypeInfo for Arc<dyn DisplayWithTypeInfo> {
    fn display_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        (**self).display_with_type_info(f, type_info_provider)
    }
}

impl<T: DisplayWithTypeInfo> DisplayWithTypeInfo for Box<T> {
    fn display_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        (**self).display_with_type_info(f, type_info_provider)
    }
}

impl DisplayWithTypeInfo for Box<dyn DisplayWithTypeInfo> {
    fn display_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        (**self).display_with_type_info(f, type_info_provider)
    }
}

impl DisplayWithTypeInfo for Location<'_> {
    fn display_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        // prettier display: file:line:column
        write!(f, "{}:{}:{}", self.file(), self.line(), self.column())
    }
}

impl<T: DisplayWithTypeInfo> DisplayWithTypeInfo for Vec<T> {
    fn display_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        f.write_str("[")?;
        let mut first = true;
        for var in self {
            if !first {
                f.write_str(", ")?;
            }
            first = false;
            var.display_with_type_info(f, type_info_provider)?;
        }
        f.write_str("]")
    }
}
