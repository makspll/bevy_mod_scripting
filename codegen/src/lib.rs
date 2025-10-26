#![feature(rustc_private, let_chains)]
#![deny(rustc::internal)]

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

mod args;
mod callback;
mod context;
mod import_path;
mod meta;
mod modifying_file_loader;
mod passes;
mod plugin;
mod template;

// pub(crate) use args::*;
pub use args::{Args, Command, WorkspaceMeta};
pub(crate) use callback::*;
pub(crate) use context::*;
pub(crate) use import_path::*;
pub use meta::MetaLoader;
pub(crate) use meta::*;
pub(crate) use passes::*;
pub use plugin::BevyAnalyzer;
pub(crate) use template::*;
pub use template::{
    Collect, Crate, TEMPLATE_DIR, TemplateKind, configure_tera, extend_context_with_args,
};
pub mod driver;
