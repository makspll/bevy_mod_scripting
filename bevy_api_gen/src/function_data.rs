use std::error::Error;

use rustdoc_types::{Item, ItemEnum};

use crate::{
    template_data::{ImportPath, NameType},
    ArgType, Config,
};

pub struct FunctionData {
    pub is_static: bool,
    pub args: Vec<NameType>,
    pub output: Option<NameType>,
    pub trait_path: Option<ImportPath>,
    pub docstrings: Vec<String>,
    pub kind: String,
}

impl TryFrom<(Option<ImportPath>, Item, &Config)> for FunctionData {
    type Error = Box<dyn Error>;

    fn try_from(
        (trait_path, item, config): (Option<ImportPath>, Item, &Config),
    ) -> Result<Self, Self::Error> {
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
            .map(|(name, type_)| ((name, type_), config).try_into())
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
        let output = decl
            .output
            .map(|type_| {
                // any idx apart from 0, don't want receivers here
                (("output".to_owned(), type_), config)
                    .try_into()
                    .and_then(|arg: NameType| {
                        (!matches!(arg.type_, ArgType::Ref { .. }))
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
                if matches!(receiver.type_, ArgType::Ref { is_mut, .. } if is_mut) {
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
            trait_path,
            kind,
        })
    }
}
