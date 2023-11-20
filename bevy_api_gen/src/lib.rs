use rustdoc_types::Type;

pub mod arg_type;
pub mod config;
pub mod cratepath;
pub mod function_data;
pub mod item_data;
pub mod template_data;
pub mod type_meta;

pub use {
    arg_type::*, config::*, cratepath::*, function_data::*, item_data::*, template_data::*,
    type_meta::*,
};

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
