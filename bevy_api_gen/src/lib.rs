use rustdoc_types::Type;

pub mod writer;
pub mod arg_validator;
pub mod config;
pub mod wrapper;

pub use {writer::*,arg_validator::*, config::*, wrapper::*};

/// Currently only used for stringifying simple trait names
pub fn stringify_type(type_: &Type) -> Option<String>{
    match type_ {
        Type::ResolvedPath { name, id, args, param_names } => Some(name.to_owned()),
        Type::Generic(s) |
        Type::Primitive(s) => Some(s.to_owned()),
        Type::QualifiedPath { name, args, self_type, trait_ } => Some(name.to_owned()),
        _ => None
    }
}