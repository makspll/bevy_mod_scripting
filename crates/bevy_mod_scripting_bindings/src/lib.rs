//! Abstractions to help with creating bindings between bevy and scripting languages.

pub mod allocator;
pub mod builtins;
pub mod callback_runners;
pub mod construct;
pub mod conversions;
pub mod dispatch;
pub mod docgen;
pub mod enum_variant;
pub mod error;
pub mod function;
pub mod globals;
pub mod observer;
pub mod path;
pub mod query;
pub mod reference;
pub mod reflection_extensions;
pub mod schedule;
pub mod script_component;
pub mod script_event;
pub mod script_value;
pub mod static_type;
pub mod system_set;
pub mod type_data;
pub mod world_extensions;

pub use allocator::*;
pub use callback_runners::*;
pub use docgen::*;
pub use enum_variant::*;
pub use error::*;
pub use function::*;
pub use globals::*;
// pub use pretty_print::*;
pub use bevy_mod_scripting_world::*;
pub use conversions::*;
pub use observer::*;
pub use path::*;
pub use query::*;
pub use reference::*;
pub use reflection_extensions::*;
pub use schedule::*;
pub use script_component::*;
pub use script_event::*;
pub use script_value::*;
pub use static_type::*;
pub use system_set::*;
pub use type_data::*;
pub use world_extensions::*;

/// Paths the derives of `bevy_mod_scripting_derive` use, which don't depend on what the deriving crate imports.
#[doc(hidden)]
pub mod __derive {
    pub use bevy_reflect::TypeRegistry;

    /// Implemented by every reflected type, and by each type deriving `ScriptData`, so a type doing both doesn't
    /// compile: "conflicting implementations of trait `ReflectedTypesCannotDeriveScriptData`".
    ///
    /// A type is either reflected, which scripts hold references to, or plain data copied across, never both:
    /// it would be a reference as the field of a component and a copied table as an argument.
    pub trait ReflectedTypesCannotDeriveScriptData {}

    impl<T: bevy_reflect::PartialReflect + ?Sized> ReflectedTypesCannotDeriveScriptData for T {}
}
