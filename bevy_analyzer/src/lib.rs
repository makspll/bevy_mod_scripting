#![feature(rustc_private)]
#![deny(rustc::internal)]
extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_lint;
extern crate rustc_middle;

mod callback;
mod context;
mod passes;
mod plugin;

pub use callback::*;
pub use context::*;
pub use passes::*;
pub use plugin::*;
