//! Operators are special functions which de-sugar to nicer syntactic constructs, for example `a + b` in rust de-sugars to `a.add(b)`
//! This module contains abstractions for describing such operators across many languages.

mod generic_bound;
mod operators;
mod primitive;
pub use generic_bound::*;
pub use operators::*;
pub use primitive::*;
