#![feature(rustc_private)]
#![deny(rustc::internal)]

// use rustc_hir::def_id::DefId;
// use rustc_middle::ty::{Ty, TyCtxt};
extern crate rustc_ast;
extern crate rustc_const_eval;
extern crate rustc_driver;
extern crate rustc_driver_impl;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_infer;
extern crate rustc_interface;
extern crate rustc_log;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_trait_selection;
extern crate rustc_data_structures;

pub mod plugin;
pub mod query;
pub use emitter::CaptureState;

pub(crate) mod emitter;
mod args;