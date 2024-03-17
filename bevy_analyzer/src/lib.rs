#![feature(rustc_private)]
#![deny(rustc::internal)]
extern crate rustc_ast;
extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_infer;
extern crate rustc_interface;
extern crate rustc_lint;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_trait_selection;

mod callback;
mod context;
mod modifying_file_loader;
mod passes;
mod plugin;

pub use callback::*;
pub use context::*;
pub use modifying_file_loader::*;
pub use passes::*;
pub use plugin::*;
