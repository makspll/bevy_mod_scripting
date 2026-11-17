//! Conversions between script values and Rust values.
//!
//! - Typed, by the Rust type in a signature: [`from::FromScript`], [`into::IntoScript`], the wrappers deciding how a
//!   value is passed ([`from::V`], [`from::R`], [`from::M`], [`from::Union`], [`by_name::ByName`]) and the plain data
//!   of `#[derive(ScriptData)]` ([`script_data`]).
//! - Erased, by the [`std::any::TypeId`] of a reflected value: [`from_ref::FromScriptErased`],
//!   [`into_ref::IntoScriptErased`] and [`primitive`].

pub mod by_name;
pub mod from;
pub mod from_ref;
pub mod into;
pub mod into_ref;
mod primitive;
pub mod script_data;

pub use by_name::*;
pub use from::*;
pub use from_ref::*;
pub use into::*;
pub use into_ref::*;
pub use primitive::*;
pub use script_data::*;
