use std::error::Error;

use rustdoc_types::{Item, ItemEnum};

use crate::{ImportPath, NameType, ValidType};

#[derive(Clone, Copy)]
pub enum OperatorType {
    Add,
    Sub,
    Div,
    Mul,
    Rem,
    Neg,
    Eq,
}

impl OperatorType {
    pub fn trait_path(self) -> ImportPath {
        let components = match self {
            OperatorType::Add => "std::ops::Add",
            OperatorType::Sub => "std::ops::Sub",
            OperatorType::Div => "std::ops::Div",
            OperatorType::Mul => "std::ops::Mul",
            OperatorType::Rem => "std::ops::Rem",
            OperatorType::Neg => "std::ops::Neg",
            OperatorType::Eq => "std::cmp::PartialEq",
        }
        .split("::")
        .map(str::to_owned)
        .collect::<Vec<_>>();
        ImportPath::new_public(components)
    }
    pub fn from_impl_name(impl_name: &str) -> Option<Self> {
        match impl_name {
            "add" => Some(OperatorType::Add),
            "sub" => Some(OperatorType::Sub),
            "div" => Some(OperatorType::Div),
            "mul" => Some(OperatorType::Mul),
            "rem" => Some(OperatorType::Rem),
            "neg" => Some(OperatorType::Neg),
            "eq" => Some(OperatorType::Eq),
            _ => None,
        }
    }
    pub fn impl_name(self) -> &'static str {
        match self {
            OperatorType::Add => "add",
            OperatorType::Sub => "sub",
            OperatorType::Div => "div",
            OperatorType::Mul => "mul",
            OperatorType::Rem => "rem",
            OperatorType::Neg => "neg",
            OperatorType::Eq => "eq",
        }
    }

    pub fn function_name(self) -> &'static str {
        match self {
            OperatorType::Add => "Add",
            OperatorType::Sub => "Sub",
            OperatorType::Div => "Div",
            OperatorType::Mul => "Mul",
            OperatorType::Rem => "Mod",
            OperatorType::Neg => "Unm",
            OperatorType::Eq => "Eq",
        }
    }
}

pub struct FunctionData {
    pub is_static: bool,
    pub args: Vec<NameType>,
    pub output: Option<NameType>,
    pub trait_path: Option<ImportPath>,
    pub docstrings: Vec<String>,
    pub operator: Option<OperatorType>,
    pub kind: String,
}

impl FunctionData {
    pub fn try_new(
        trait_path: Option<ImportPath>,
        item: Item,
        operator: Option<OperatorType>,
        assoc_types: Vec<&Item>,
        resolve_self_with: Option<&ValidType>,
    ) -> Result<Self, Box<dyn Error>> {
        let (decl, generics) = match item.inner {
            ItemEnum::Function(f) => (f.decl, f.generics),
            _ => return Err("Not a function item".into()),
        };

        log::trace!("Converting function: {}", item.name.unwrap());
        log::trace!("inputs: {:?}", decl.inputs);

        if !generics.params.is_empty() {
            return Err("Generics are not supported".into());
        }

        let args = decl
            .inputs
            .into_iter()
            .map(|(name, type_)| NameType::try_new(name, type_, &assoc_types, resolve_self_with))
            .collect::<Result<Vec<_>, _>>()?;

        let is_static = args
            .first()
            .map(|first: &NameType| !first.type_.is_receiver())
            .unwrap_or(true);

        log::trace!(
            "function is static?: {is_static}, first arg: {:?}, is_receiver: {:?}",
            args.first(),
            args.first().map(|arg| arg.type_.is_receiver())
        );

        let output = if decl.output.is_none() && operator.is_some() {
            assoc_types.iter().find_map(|i| {
                if let ItemEnum::AssocType { default, .. } = &i.inner {
                    if i.name.as_ref().is_some_and(|name| name == "Output") && default.is_some() {
                        log::trace!("Using associated type `Output` for operator function");
                        return Some(default.as_ref().unwrap().clone());
                    }
                }
                None
            })
        } else {
            decl.output
        };

        let output = output
            .map(|type_| {
                // any idx apart from 0, don't want receivers here
                NameType::try_new("output".to_owned(), type_, &assoc_types, resolve_self_with)
                    .and_then(|arg: NameType| {
                        (!matches!(arg.type_, ValidType::Ref { .. }))
                            .then_some(arg)
                            .ok_or("Reference are not supported in output position".into())
                    })
            })
            .transpose()?;

        let docstrings = item
            .docs
            .unwrap_or("".to_owned())
            .lines()
            .map(|s| s.to_owned())
            .collect();

        let receiver = args
            .first()
            .filter(|first_arg| first_arg.type_.is_receiver());
        let kind = receiver
            .map(|receiver| {
                if operator.is_some() {
                    "MetaFunction".to_owned()
                } else if matches!(receiver.type_, ValidType::Ref { is_mut, .. } if is_mut) {
                    "MutatingMethod".to_owned()
                } else {
                    "Method".to_owned()
                }
            })
            .unwrap_or("Function".to_owned());
        Ok(Self {
            is_static,
            args,
            output,
            docstrings,
            operator,
            trait_path,
            kind,
        })
    }
}
