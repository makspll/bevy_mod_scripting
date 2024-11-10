#![feature(rustc_private, let_chains)]
#![deny(rustc::internal)]
extern crate rustc_ast;
extern crate rustc_const_eval;
extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_hir_analysis;
extern crate rustc_infer;
extern crate rustc_interface;
extern crate rustc_lint;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_trait_selection;

mod args;
mod callback;
mod context;
mod feature_graph;
mod import_path;
mod meta;
mod modifying_file_loader;
mod passes;
mod plugin;
mod template;

// pub(crate) use args::*;
pub(crate) use callback::*;
pub(crate) use context::*;
pub(crate) use import_path::*;
pub(crate) use meta::*;
pub(crate) use passes::*;
pub(crate) use template::*;

pub use args::{Args, Command, WorkspaceMeta};
pub use feature_graph::*;
pub use meta::MetaLoader;
pub use plugin::BevyAnalyzer;
pub use template::{
    configure_tera, extend_context_with_args, Collect, Crate, TemplateKind, TEMPLATE_DIR,
};
