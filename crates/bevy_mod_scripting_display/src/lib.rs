//! Abstractions for displaying reflect values, potentially with access to the type registry

use std::{
    any::{Any, TypeId},
    ops::Deref,
};
mod handle;
mod impls;
mod printer;
pub use {handle::*, printer::*};

use bevy_ecs::{
    component::{ComponentId, ComponentInfo},
    reflect::AppTypeRegistry,
    world::World,
};
use bevy_reflect::{TypeData, TypeInfo, TypeRegistry, reflect_trait};

/// An abstraction for getting type information, potentially using the type registry.
pub trait GetTypeInfo {
    /// Get a string representation of the type, potentially using the type registry.
    fn get_type_info(&self, type_id: TypeId) -> Option<&TypeInfo>;

    /// Queries against arbitrary type data
    fn query_type_registration(
        &self,
        type_id: TypeId,
        type_data_id: TypeId,
    ) -> Option<Box<dyn TypeData>>;

    /// Get component info for a given component id, if available
    fn get_component_info(&self, component_id: ComponentId) -> Option<&ComponentInfo>;

    /// A potentially unsafe function depending on the implementation which allows you to downcast to a concrete type without
    /// requiring 'static on the type.
    ///
    /// # Safety
    /// - Ensure the safety invariants for the concrete type you are expecting are respected
    unsafe fn as_any_static(&self) -> &dyn Any;
}

/// Extension trait for GetTypeInfo which provides non-type safe extensions
pub trait GetTypeInfoExtensions<'s> {
    /// Typed equivalent to [`GetTypeInfo::query_type_registration`]
    fn get_type_data<T: TypeData + 'static>(&'s self, type_id: TypeId) -> Option<T>;
}

impl<'s> GetTypeInfoExtensions<'s> for &'s dyn GetTypeInfo {
    fn get_type_data<T: TypeData + 'static>(&'s self, type_id: TypeId) -> Option<T> {
        self.query_type_registration(type_id, std::any::TypeId::of::<T>())
            .and_then(|t| t.downcast().ok())
            .map(|b| *b)
    }
}

impl GetTypeInfo for TypeRegistry {
    fn get_type_info(&self, type_id: TypeId) -> Option<&TypeInfo> {
        self.get(type_id)
            .map(|registration| registration.type_info())
    }

    fn query_type_registration(
        &self,
        type_id: TypeId,
        type_data_id: TypeId,
    ) -> Option<Box<dyn TypeData>> {
        self.get(type_id)
            .and_then(|r| r.data_by_id(type_data_id).map(|t| t.clone_type_data()))
    }

    fn get_component_info(&self, _component_id: ComponentId) -> Option<&ComponentInfo> {
        None
    }

    unsafe fn as_any_static(&self) -> &dyn Any {
        self
    }
}

impl GetTypeInfo for World {
    fn get_type_info(&self, type_id: TypeId) -> Option<&TypeInfo> {
        self.get_resource::<AppTypeRegistry>()
            .and_then(|r| r.read().get_type_info(type_id))
    }

    fn query_type_registration(
        &self,
        type_id: TypeId,
        type_data_id: TypeId,
    ) -> Option<Box<dyn TypeData>> {
        self.get_resource::<AppTypeRegistry>().and_then(|r| {
            r.read()
                .get(type_id)
                .and_then(|r| r.data_by_id(type_data_id).map(|t| t.clone_type_data()))
        })
    }

    fn get_component_info(&self, component_id: ComponentId) -> Option<&ComponentInfo> {
        self.components().get_info(component_id)
    }

    unsafe fn as_any_static(&self) -> &dyn Any {
        self
    }
}

/// An trait for displaying values with access to type information
#[reflect_trait]
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
        let provider = self.1.or_else(|| {
            GLOBAL_TYPE_INFO_PROVIDER
                .get()
                .and_then(|get_provider| get_provider())
        });
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

/// newtype adapter for opting into [`DisplayWithTypeInfo`] for any T: [`DisplayWithTypeInfo`]
/// Use as follows
/// ```rust,no_run
/// use bevy_mod_scripting_display::WithTypeInfo;
///
/// let my_value = std::any::TypeId::of::<u32>();
/// format!("{:?}", WithTypeInfo::new(&my_value)); // non-pretty print
/// format!("{:#?}", WithTypeInfo::new(&my_value)); // pretty print
/// ```
pub struct WithTypeInfo<'a, T: ?Sized>(&'a T, Option<&'a dyn GetTypeInfo>);

impl<'a, T: ?Sized> WithTypeInfo<'a, T> {
    /// Create a new WithTypeInfo wrapper.
    /// Will retrieve type information using the [`GLOBAL_TYPE_INFO_PROVIDER`].
    ///
    /// If you're using this type in the context of [`DebugWithTypeInfo`] or [`DisplayWithTypeInfo`] traits,
    /// use [`Self::new_with_opt_info`] instead, to pass down the context correctly.
    pub fn new(value: &'a T) -> Self {
        Self(value, None)
    }

    /// Create a new WithTypeInfo wrapper with a specific type info provider
    pub fn new_with_info(value: &'a T, provider: &'a dyn GetTypeInfo) -> Self {
        Self(value, Some(provider))
    }

    /// Create a new WithTypeInfo wrapper passing down an optional type info provider.
    /// Useful for nested implementations which want to avoid multiple retrievals
    pub fn new_with_opt_info(value: &'a T, provider: Option<&'a dyn GetTypeInfo>) -> Self {
        Self(value, provider)
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
        let provider = self.1.or_else(|| {
            GLOBAL_TYPE_INFO_PROVIDER
                .get()
                .and_then(|get_provider| get_provider())
        });
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
pub struct DebugStruct<'a, 'b: 'a, 't> {
    builder: std::fmt::DebugStruct<'a, 'b>,
    type_info: Option<&'t dyn GetTypeInfo>,
}

impl<'a, 'b: 'a, 't> DebugStruct<'a, 'b, 't> {
    /// Create a new `DebugStruct` builder for a struct with the given name.
    ///
    /// The returned value can be used to add fields which implement
    /// `DebugWithTypeInfo` and then finished to produce the final formatting
    /// result.
    pub fn new(
        f: &'a mut std::fmt::Formatter<'b>,
        name: &str,
        type_info: Option<&'t dyn GetTypeInfo>,
    ) -> Self {
        Self {
            builder: f.debug_struct(name),
            type_info,
        }
    }

    /// Add a field to the struct being formatted.
    ///
    /// The `value` will be displayed using its `DebugWithTypeInfo`
    /// implementation so that any available type information can be used.
    pub fn field(&mut self, name: &str, value: &dyn DebugWithTypeInfo) -> &mut Self {
        self.builder.field(
            name,
            &WithTypeInfo::new_with_opt_info(value, self.type_info),
        );
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
pub struct DebugTuple<'a, 'b: 'a, 't> {
    builder: std::fmt::DebugTuple<'a, 'b>,
    type_info: Option<&'t dyn GetTypeInfo>,
}

impl<'a, 'b: 'a, 't> DebugTuple<'a, 'b, 't> {
    /// Create a new `DebugTuple` builder for a tuple-like value with the
    /// given name.
    pub fn new(
        f: &'a mut std::fmt::Formatter<'b>,
        name: &str,
        type_info: Option<&'t dyn GetTypeInfo>,
    ) -> Self {
        Self {
            builder: f.debug_tuple(name),
            type_info,
        }
    }

    /// Add an element to the tuple being formatted. The element will be
    /// formatted via its `DebugWithTypeInfo` implementation.
    pub fn field(&mut self, value: &dyn DebugWithTypeInfo) -> &mut Self {
        self.builder
            .field(&WithTypeInfo::new_with_opt_info(value, self.type_info));
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
pub struct DebugList<'a, 'b: 'a, 't> {
    builder: std::fmt::DebugList<'a, 'b>,
    type_info: Option<&'t dyn GetTypeInfo>,
}

impl<'a, 'b: 'a, 't> DebugList<'a, 'b, 't> {
    /// Create a new `DebugList` builder.
    pub fn new(f: &'a mut std::fmt::Formatter<'b>, type_info: Option<&'t dyn GetTypeInfo>) -> Self {
        Self {
            builder: f.debug_list(),
            type_info,
        }
    }

    /// Add a single entry to the list. The entry will be formatted via its
    /// `DebugWithTypeInfo` implementation.
    pub fn entry(&mut self, value: &dyn DebugWithTypeInfo) -> &mut Self {
        self.builder
            .entry(&WithTypeInfo::new_with_opt_info(value, self.type_info));
        self
    }

    /// Add multiple entries from an iterator of `DebugWithTypeInfo` references.
    pub fn entries<I: IntoIterator<Item = &'a dyn DebugWithTypeInfo>>(
        &mut self,
        values: I,
    ) -> &mut Self {
        for value in values {
            self.builder
                .entry(&WithTypeInfo::new_with_opt_info(value, self.type_info));
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
pub struct DebugSet<'a, 'b: 'a, 't> {
    builder: std::fmt::DebugSet<'a, 'b>,
    type_info: Option<&'t dyn GetTypeInfo>,
}

impl<'a, 'b: 'a, 't> DebugSet<'a, 'b, 't> {
    /// Create a new `DebugSet` builder.
    pub fn new(f: &'a mut std::fmt::Formatter<'b>, type_info: Option<&'t dyn GetTypeInfo>) -> Self {
        Self {
            builder: f.debug_set(),
            type_info,
        }
    }

    /// Add a single entry to the set. The entry will be formatted via its
    /// `DebugWithTypeInfo` implementation.
    pub fn entry(&mut self, value: &dyn DebugWithTypeInfo) -> &mut Self {
        self.builder
            .entry(&WithTypeInfo::new_with_opt_info(value, self.type_info));
        self
    }

    /// Add multiple entries from an iterator of `DebugWithTypeInfo` references.
    pub fn entries<I: IntoIterator<Item = &'a dyn DebugWithTypeInfo>>(
        &mut self,
        values: I,
    ) -> &mut Self {
        for value in values {
            self.builder
                .entry(&WithTypeInfo::new_with_opt_info(value, self.type_info));
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
pub struct DebugMap<'a, 'b: 'a, 't> {
    builder: std::fmt::DebugMap<'a, 'b>,
    type_info: Option<&'t dyn GetTypeInfo>,
}

impl<'a, 'b: 'a, 't> DebugMap<'a, 'b, 't> {
    /// Create a new `DebugMap` builder.
    pub fn new(f: &'a mut std::fmt::Formatter<'b>, type_info: Option<&'t dyn GetTypeInfo>) -> Self {
        Self {
            builder: f.debug_map(),
            type_info,
        }
    }

    /// Add a single key/value entry to the map. Both key and value will be
    /// formatted via their `DebugWithTypeInfo` implementations.
    pub fn entry(
        &mut self,
        key: &dyn DebugWithTypeInfo,
        value: &dyn DebugWithTypeInfo,
    ) -> &mut Self {
        self.builder.entry(
            &WithTypeInfo::new_with_opt_info(key, self.type_info),
            &WithTypeInfo::new_with_opt_info(value, self.type_info),
        );
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
            self.builder.entry(
                &WithTypeInfo::new_with_opt_info(key, self.type_info),
                &WithTypeInfo::new_with_opt_info(value, self.type_info),
            );
        }
        self
    }

    /// Add a key to the map being formatted (used when constructing an
    /// entry in separate steps).
    pub fn key(&mut self, key: &dyn DebugWithTypeInfo) -> &mut Self {
        self.builder
            .key(&WithTypeInfo::new_with_opt_info(key, self.type_info));
        self
    }

    /// Add a value to the map being formatted (used when constructing an
    /// entry in separate steps).
    pub fn value(&mut self, value: &dyn DebugWithTypeInfo) -> &mut Self {
        self.builder
            .value(&WithTypeInfo::new_with_opt_info(value, self.type_info));
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
pub trait DebugWithTypeInfoBuilder<'a, 'b: 'a, 't> {
    /// Start formatting a struct with the given name using type-aware
    /// field formatting.
    fn debug_struct_with_type_info(
        &'a mut self,
        name: &str,
        type_info: Option<&'t dyn GetTypeInfo>,
    ) -> DebugStruct<'a, 'b, 't>;

    /// Start formatting a tuple-like value with the given name using
    /// type-aware element formatting.
    fn debug_tuple_with_type_info(
        &'a mut self,
        name: &str,
        type_info: Option<&'t dyn GetTypeInfo>,
    ) -> DebugTuple<'a, 'b, 't>;

    /// Start formatting a list using type-aware entry formatting.
    fn debug_list_with_type_info(
        &'a mut self,
        type_info: Option<&'t dyn GetTypeInfo>,
    ) -> DebugList<'a, 'b, 't>;

    /// Start formatting a set using type-aware entry formatting.
    fn debug_set_with_type_info(
        &'a mut self,
        type_info: Option<&'t dyn GetTypeInfo>,
    ) -> DebugSet<'a, 'b, 't>;

    /// Start formatting a map using type-aware key/value formatting.
    fn debug_map_with_type_info(
        &'a mut self,
        type_info: Option<&'t dyn GetTypeInfo>,
    ) -> DebugMap<'a, 'b, 't>;
}

impl<'a, 'b: 'a, 't> DebugWithTypeInfoBuilder<'a, 'b, 't> for std::fmt::Formatter<'b> {
    fn debug_struct_with_type_info(
        &'a mut self,
        name: &str,
        type_info: Option<&'t dyn GetTypeInfo>,
    ) -> DebugStruct<'a, 'b, 't> {
        DebugStruct::new(self, name, type_info)
    }
    fn debug_tuple_with_type_info(
        &'a mut self,
        name: &str,
        type_info: Option<&'t dyn GetTypeInfo>,
    ) -> DebugTuple<'a, 'b, 't> {
        DebugTuple::new(self, name, type_info)
    }
    fn debug_list_with_type_info(
        &'a mut self,
        type_info: Option<&'t dyn GetTypeInfo>,
    ) -> DebugList<'a, 'b, 't> {
        DebugList::new(self, type_info)
    }
    fn debug_set_with_type_info(
        &'a mut self,
        type_info: Option<&'t dyn GetTypeInfo>,
    ) -> DebugSet<'a, 'b, 't> {
        DebugSet::new(self, type_info)
    }
    fn debug_map_with_type_info(
        &'a mut self,
        type_info: Option<&'t dyn GetTypeInfo>,
    ) -> DebugMap<'a, 'b, 't> {
        DebugMap::new(self, type_info)
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
