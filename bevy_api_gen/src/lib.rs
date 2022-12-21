use rustdoc_types::Type;

pub mod arg_validator;
pub mod config;
pub mod wrapper;
pub mod writer;

pub use {arg_validator::*, config::*, wrapper::*, writer::*};

/// Currently only used for stringifying simple trait names
pub fn stringify_type(type_: &Type) -> Option<String> {
    match type_ {
        Type::ResolvedPath(path) => Some(path.name.to_owned()),
        Type::Generic(s) | Type::Primitive(s) => Some(s.to_owned()),
        Type::QualifiedPath {
            name,
            args: _,
            self_type: _,
            trait_: _,
        } => Some(name.to_owned()),
        _ => None,
    }
}
