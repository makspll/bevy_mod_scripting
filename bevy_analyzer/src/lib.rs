#![feature(rustc_private, let_chains)]
#![deny(rustc::internal)]
extern crate rustc_ast;
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

mod callback;
mod context;
mod meta;
mod modifying_file_loader;
mod passes;
mod plugin;
mod template;

pub(crate) use callback::*;
pub(crate) use context::*;
pub(crate) use meta::*;
pub(crate) use passes::*;
pub(crate) use template::*;

pub use meta::MetaLoader;
pub use plugin::{Args, BevyAnalyzer, Command, TemplateKind, TARGET_DIR_ENV_NAME};
pub use template::{configure_tera, Collect, Crate, TEMPLATE_DIR};
