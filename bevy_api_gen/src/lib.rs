use rustdoc_types::Type;

pub mod config;
pub mod cratepath;
pub mod function_data;
pub mod impl_item;
pub mod item_data;
pub mod name_type;
pub mod template_data;
pub mod type_meta;
pub mod valid_type;
pub use {
    config::*, cratepath::*, function_data::*, impl_item::*, item_data::*, name_type::*,
    template_data::*, type_meta::*, valid_type::*,
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
