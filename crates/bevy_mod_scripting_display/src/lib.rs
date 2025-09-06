//! Abstractions for displaying reflect values, potentially with access to the type registry

use std::{any::TypeId, ops::Deref};
mod impls;
mod printer;
pub use printer::*;

use bevy_ecs::{
    component::{ComponentId, ComponentInfo},
    reflect::AppTypeRegistry,
    world::World,
};
use bevy_reflect::{TypeInfo, TypeRegistry};

/// An abstraction for getting type information, potentially using the type registry.
pub trait GetTypeInfo {
    /// Get a string representation of the type, potentially using the type registry.
    fn get_type_info(&self, type_id: TypeId) -> Option<&TypeInfo>;

    /// Get component info for a given component id, if available
    fn get_component_info(&self, component_id: ComponentId) -> Option<&ComponentInfo>;
}

impl GetTypeInfo for TypeRegistry {
    fn get_type_info(&self, type_id: TypeId) -> Option<&TypeInfo> {
        self.get(type_id)
            .map(|registration| registration.type_info())
    }

    fn get_component_info(&self, _component_id: ComponentId) -> Option<&ComponentInfo> {
        None
    }
}

impl GetTypeInfo for World {
    fn get_type_info(&self, type_id: TypeId) -> Option<&TypeInfo> {
        self.get_resource::<AppTypeRegistry>()
            .and_then(|r| r.read().get_type_info(type_id))
    }

    fn get_component_info(&self, component_id: ComponentId) -> Option<&ComponentInfo> {
        self.components().get_info(component_id)
    }
}

/// An trait for displaying values with access to type information
pub trait DisplayWithTypeInfo {
    /// Format the value using the provided type info provider if available
    fn display_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result;
}

impl<T: DisplayWithTypeInfo> DisplayWithTypeInfo for WithTypeInfo<'_, T> {
    fn display_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        self.0.display_with_type_info(f, type_info_provider)
    }
}

impl<T: DisplayWithTypeInfo> std::fmt::Display for WithTypeInfo<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let provider = GLOBAL_TYPE_INFO_PROVIDER
            .get()
            .and_then(|get_provider| get_provider());
        self.0.display_with_type_info(f, provider)
    }
}

/// An extension trait for displaying values with access to type information
/// implementations should respect the following formatter options:
/// - `#` - should display the value in a pretty-printed way if possible, if not provided a debug representation should be used instead
pub trait DebugWithTypeInfo {
    /// Format the value using the provided type info provider if available
    fn to_string_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result;
}

/// A global type info provider that can be set once and used throughout the application
/// It does not do the retrieval itself, but provides a function that points to the retrieval mechanism.
pub static GLOBAL_TYPE_INFO_PROVIDER: std::sync::OnceLock<
    fn() -> Option<&'static dyn GetTypeInfo>,
> = std::sync::OnceLock::new();

/// newtype adapter for opting into DisplayWithTypeInfo for any T: DisplayWithTypeInfo
/// Use as follows
/// ```rust,no_run
/// use bevy_mod_scripting_display::WithTypeInfo;
///
/// let my_value = std::any::TypeId::of::<u32>();
/// format!("{:?}", WithTypeInfo(&my_value)); // non-pretty print
/// format!("{:#?}", WithTypeInfo(&my_value)); // pretty print
/// ```
pub struct WithTypeInfo<'a, T: ?Sized>(pub &'a T);

impl<'a, T: DebugWithTypeInfo + ?Sized> WithTypeInfo<'a, T> {
    /// Create a new WithTypeInfo wrapper
    pub fn new(value: &'a T) -> Self {
        Self(value)
    }
}

impl<T: DebugWithTypeInfo + ?Sized> Deref for WithTypeInfo<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, T: DebugWithTypeInfo + ?Sized> std::fmt::Debug for WithTypeInfo<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let provider = GLOBAL_TYPE_INFO_PROVIDER
            .get()
            .and_then(|get_provider| get_provider());
        self.0.to_string_with_type_info(f, provider)
    }
}

impl<T: DebugWithTypeInfo + ?Sized> DebugWithTypeInfo for WithTypeInfo<'_, T> {
    fn to_string_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        self.0.to_string_with_type_info(f, type_info_provider)
    }
}

struct FakeType;
/// A utility for getting a TypeId or a fake one if not present
pub trait OrFakeId {
    /// Get the TypeId or a fake one if not present
    fn or_fake_id(&self) -> TypeId;

    /// Get the fake TypeId
    fn fake_id() -> TypeId {
        TypeId::of::<FakeType>()
    }
}

impl OrFakeId for Option<TypeId> {
    fn or_fake_id(&self) -> TypeId {
        self.unwrap_or_else(TypeId::of::<FakeType>)
    }
}

trait SelfBuilder: Sized {
    /// Apply a function to self and return the result
    fn build_with<F: FnOnce(Self) -> Self>(self, f: F) -> Self;
}

impl<T: Sized> SelfBuilder for T {
    fn build_with<F: FnOnce(Self) -> Self>(self, f: F) -> Self {
        f(self)
    }
}

/// A utility for formatting structs but with type info available
/// Helper wrapper around `std::fmt::DebugStruct` that formats struct fields
/// using `DebugWithTypeInfo` so that type-aware formatting is available for
/// each field.
pub struct DebugStruct<'a, 'b: 'a> {
    builder: std::fmt::DebugStruct<'a, 'b>,
}

impl<'a, 'b: 'a> DebugStruct<'a, 'b> {
    /// Create a new `DebugStruct` builder for a struct with the given name.
    ///
    /// The returned value can be used to add fields which implement
    /// `DebugWithTypeInfo` and then finished to produce the final formatting
    /// result.
    pub fn new(f: &'a mut std::fmt::Formatter<'b>, name: &str) -> Self {
        Self {
            builder: f.debug_struct(name),
        }
    }

    /// Add a field to the struct being formatted.
    ///
    /// The `value` will be displayed using its `DebugWithTypeInfo`
    /// implementation so that any available type information can be used.
    pub fn field(&mut self, name: &str, value: &dyn DebugWithTypeInfo) -> &mut Self {
        self.builder.field(name, &WithTypeInfo::new(value));
        self
    }

    /// Finish building the struct formatter and write the output to the
    /// underlying `Formatter`.
    pub fn finish(&mut self) -> std::fmt::Result {
        self.builder.finish()
    }
}

/// Helper wrapper around `std::fmt::DebugTuple` that formats tuple elements
/// using `DebugWithTypeInfo` so elements can render with optional type
/// information.
pub struct DebugTuple<'a, 'b: 'a> {
    builder: std::fmt::DebugTuple<'a, 'b>,
}

impl<'a, 'b: 'a> DebugTuple<'a, 'b> {
    /// Create a new `DebugTuple` builder for a tuple-like value with the
    /// given name.
    pub fn new(f: &'a mut std::fmt::Formatter<'b>, name: &str) -> Self {
        Self {
            builder: f.debug_tuple(name),
        }
    }

    /// Add an element to the tuple being formatted. The element will be
    /// formatted via its `DebugWithTypeInfo` implementation.
    pub fn field(&mut self, value: &dyn DebugWithTypeInfo) -> &mut Self {
        self.builder.field(&WithTypeInfo::new(value));
        self
    }

    /// Finish building the tuple formatter and write the output to the
    /// underlying `Formatter`.
    pub fn finish(&mut self) -> std::fmt::Result {
        self.builder.finish()
    }
}

/// Helper wrapper around `std::fmt::DebugList` which formats list entries
/// using `DebugWithTypeInfo` so each entry can use available type
/// information during formatting.
pub struct DebugList<'a, 'b: 'a> {
    builder: std::fmt::DebugList<'a, 'b>,
}

impl<'a, 'b: 'a> DebugList<'a, 'b> {
    /// Create a new `DebugList` builder.
    pub fn new(f: &'a mut std::fmt::Formatter<'b>) -> Self {
        Self {
            builder: f.debug_list(),
        }
    }

    /// Add a single entry to the list. The entry will be formatted via its
    /// `DebugWithTypeInfo` implementation.
    pub fn entry(&mut self, value: &dyn DebugWithTypeInfo) -> &mut Self {
        self.builder.entry(&WithTypeInfo::new(value));
        self
    }

    /// Add multiple entries from an iterator of `DebugWithTypeInfo` references.
    pub fn entries<I: IntoIterator<Item = &'a dyn DebugWithTypeInfo>>(
        &mut self,
        values: I,
    ) -> &mut Self {
        for value in values {
            self.builder.entry(&WithTypeInfo::new(value));
        }
        self
    }

    /// Finish building the list formatter and write the output to the
    /// underlying `Formatter`.
    pub fn finish(&mut self) -> std::fmt::Result {
        self.builder.finish()
    }
}

/// Helper wrapper around `std::fmt::DebugSet` that formats set entries
/// using `DebugWithTypeInfo` so entries can render with optional type
/// information.
pub struct DebugSet<'a, 'b: 'a> {
    builder: std::fmt::DebugSet<'a, 'b>,
}

impl<'a, 'b: 'a> DebugSet<'a, 'b> {
    /// Create a new `DebugSet` builder.
    pub fn new(f: &'a mut std::fmt::Formatter<'b>) -> Self {
        Self {
            builder: f.debug_set(),
        }
    }

    /// Add a single entry to the set. The entry will be formatted via its
    /// `DebugWithTypeInfo` implementation.
    pub fn entry(&mut self, value: &dyn DebugWithTypeInfo) -> &mut Self {
        self.builder.entry(&WithTypeInfo::new(value));
        self
    }

    /// Add multiple entries from an iterator of `DebugWithTypeInfo` references.
    pub fn entries<I: IntoIterator<Item = &'a dyn DebugWithTypeInfo>>(
        &mut self,
        values: I,
    ) -> &mut Self {
        for value in values {
            self.builder.entry(&WithTypeInfo::new(value));
        }
        self
    }

    /// Finish building the set formatter and write the output to the
    /// underlying `Formatter`.
    pub fn finish(&mut self) -> std::fmt::Result {
        self.builder.finish()
    }
}

/// Helper wrapper around `std::fmt::DebugMap` that formats map keys and
/// values using `DebugWithTypeInfo` so both sides of each entry can render
/// with optional type information.
pub struct DebugMap<'a, 'b: 'a> {
    builder: std::fmt::DebugMap<'a, 'b>,
}

impl<'a, 'b: 'a> DebugMap<'a, 'b> {
    /// Create a new `DebugMap` builder.
    pub fn new(f: &'a mut std::fmt::Formatter<'b>) -> Self {
        Self {
            builder: f.debug_map(),
        }
    }

    /// Add a single key/value entry to the map. Both key and value will be
    /// formatted via their `DebugWithTypeInfo` implementations.
    pub fn entry(
        &mut self,
        key: &dyn DebugWithTypeInfo,
        value: &dyn DebugWithTypeInfo,
    ) -> &mut Self {
        self.builder
            .entry(&WithTypeInfo::new(key), &WithTypeInfo::new(value));
        self
    }

    /// Add multiple key/value entries from an iterator of pairs.
    pub fn entries<
        I: IntoIterator<Item = (&'a dyn DebugWithTypeInfo, &'a dyn DebugWithTypeInfo)>,
    >(
        &mut self,
        values: I,
    ) -> &mut Self {
        for (key, value) in values {
            self.builder
                .entry(&WithTypeInfo::new(key), &WithTypeInfo::new(value));
        }
        self
    }

    /// Add a key to the map being formatted (used when constructing an
    /// entry in separate steps).
    pub fn key(&mut self, key: &dyn DebugWithTypeInfo) -> &mut Self {
        self.builder.key(&WithTypeInfo::new(key));
        self
    }

    /// Add a value to the map being formatted (used when constructing an
    /// entry in separate steps).
    pub fn value(&mut self, value: &dyn DebugWithTypeInfo) -> &mut Self {
        self.builder.value(&WithTypeInfo::new(value));
        self
    }

    /// Finish building the map formatter and write the output to the
    /// underlying `Formatter`.
    pub fn finish(&mut self) -> std::fmt::Result {
        self.builder.finish()
    }
}

/// Extension methods for `std::fmt::Formatter` which create builders that
/// format values using `DebugWithTypeInfo`. These helpers mirror the
/// standard formatter builders (`debug_struct`, `debug_tuple`, etc.) but
/// ensure that fields, entries and keys/values are displayed through the
/// `DebugWithTypeInfo` adapter.
pub trait DebugWithTypeInfoBuilder<'a, 'b: 'a> {
    /// Start formatting a struct with the given name using type-aware
    /// field formatting.
    fn debug_struct_with_type_info(&'a mut self, name: &str) -> DebugStruct<'a, 'b>;

    /// Start formatting a tuple-like value with the given name using
    /// type-aware element formatting.
    fn debug_tuple_with_type_info(&'a mut self, name: &str) -> DebugTuple<'a, 'b>;

    /// Start formatting a list using type-aware entry formatting.
    fn debug_list_with_type_info(&'a mut self) -> DebugList<'a, 'b>;

    /// Start formatting a set using type-aware entry formatting.
    fn debug_set_with_type_info(&'a mut self) -> DebugSet<'a, 'b>;

    /// Start formatting a map using type-aware key/value formatting.
    fn debug_map_with_type_info(&'a mut self) -> DebugMap<'a, 'b>;
}

impl<'a, 'b: 'a> DebugWithTypeInfoBuilder<'a, 'b> for std::fmt::Formatter<'b> {
    fn debug_struct_with_type_info(&'a mut self, name: &str) -> DebugStruct<'a, 'b> {
        DebugStruct::new(self, name)
    }
    fn debug_tuple_with_type_info(&'a mut self, name: &str) -> DebugTuple<'a, 'b> {
        DebugTuple::new(self, name)
    }
    fn debug_list_with_type_info(&'a mut self) -> DebugList<'a, 'b> {
        DebugList::new(self)
    }
    fn debug_set_with_type_info(&'a mut self) -> DebugSet<'a, 'b> {
        DebugSet::new(self)
    }
    fn debug_map_with_type_info(&'a mut self) -> DebugMap<'a, 'b> {
        DebugMap::new(self)
    }
}

macro_rules! impl_debug_with_type_info_via_debug {
    ($t:ty) => {
        impl $crate::DebugWithTypeInfo for $t {
            fn to_string_with_type_info(
                &self,
                f: &mut std::fmt::Formatter<'_>,
                _type_info_provider: Option<&dyn $crate::GetTypeInfo>,
            ) -> std::fmt::Result {
                std::fmt::Debug::fmt(self, f)
            }
        }
    };
}

macro_rules! impl_debug_with_type_info_via_display {
    ($($t:ty),*) => {
        $(
            impl DebugWithTypeInfo for $t {
                fn to_string_with_type_info(
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

use impl_debug_with_type_info_via_debug;
use impl_debug_with_type_info_via_display;

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

use impl_display_with_type_info_via_display;
