use std::error::Error;

use indexmap::{IndexMap, IndexSet};
use rustdoc_types::{ItemEnum, Type};

use crate::{
    cratepath::*,
    template_data::{ImportPath, NameType},
    ArgType, Config, FunctionData, TypeMeta,
};

pub enum ItemType {
    UnitStruct,
    TupleStruct,
    Struct,
    Unsupported,
}

pub struct ItemData {
    pub source_crate: String,
    pub import_path: ImportPath,
    pub implemented_traits: IndexSet<String>,
    pub functions: IndexMap<String, FunctionData>,
    pub fields: Vec<NameType>,
    pub docstrings: Vec<String>,
    pub item_type: ItemType,
}

impl From<(TypeMeta<'_>, &Config)> for ItemData {
    fn from((meta, config): (TypeMeta<'_>, &Config)) -> Self {
        let import_path: Vec<String> = meta.path_components.into();
        let import_path: ImportPath = import_path.into();
        let functions = meta
            .impl_items
            .iter()
            .flat_map(|(name, impl_items)| {
                impl_items.iter().filter_map(move |(impl_, item)| {
                    let trait_path = impl_
                        .trait_
                        .as_ref()
                        .map(|trait_| {
                            // TODO: this is probably pretty slow, pre-calculate this
                            // meta.crates.iter().find_map(|crate_|

                            let origin_crate =
                                lookup_item_crate_source(&trait_.id, meta.crates).ok_or(
                                    format!("Could not find trait in any crates `{}`", trait_.name),
                                )?;

                            let path_components = get_path(&trait_.id, origin_crate).unwrap();
                            let import_path = path_to_import(path_components, meta.source).into();
                            Ok(import_path)
                        })
                        .transpose()
                        .map_err(|e: Box<dyn Error>| {
                            log::info!(
                                "Skipping trait impl in type: `{}` because: `{}`",
                                meta.config.type_,
                                e
                            )
                        })
                        .ok()?;

                    let fn_item = (trait_path, (*item).clone(), config)
                        .try_into()
                        .map_err(|e| {
                            log::info!(
                                "Skipping function: `{}` in type: `{}` due to: `{e}`",
                                name,
                                meta.config.type_
                            );
                            e
                        })
                        .ok()?;

                    Some((name.to_string(), fn_item))
                })
                // })
            })
            .collect();

        let mut fields: Vec<NameType> = Default::default();
        let mut item_type = ItemType::Unsupported;
        if let ItemEnum::Struct(struct_) = &meta.item.inner {
            item_type = match struct_.kind {
                rustdoc_types::StructKind::Unit => ItemType::UnitStruct,
                rustdoc_types::StructKind::Tuple(_) => ItemType::TupleStruct,
                rustdoc_types::StructKind::Plain { .. } => ItemType::Struct,
            };

            let field_pairs: Vec<Result<(String, Type), Box<dyn Error>>> = match &struct_.kind {
                rustdoc_types::StructKind::Unit => Default::default(),
                rustdoc_types::StructKind::Tuple(t) => t
                    .iter()
                    .enumerate()
                    .map(|(idx, id)| {
                        let type_ = id
                            .as_ref()
                            .map(|id| {
                                let meta = &meta.source.index.get(id).ok_or::<Box<dyn Error>>(
                                    "Expected to find field in the same crate as struct".into(),
                                )?;

                                if meta.attrs.iter().any(|attr| attr == "#[reflect(ignore)]") {
                                    return Err("Field ignored by reflection".into());
                                }

                                match &meta.inner {
                                    ItemEnum::StructField(field) => {
                                        Ok::<_, Box<dyn Error>>(field.clone())
                                    }
                                    _ => panic!("Expected struct field"),
                                }
                            })
                            .ok_or_else::<Box<dyn Error>, _>(|| {
                                "Could not find StructField in the json index".into()
                            })??;

                        Ok::<_, Box<dyn Error>>((idx.to_string(), type_))
                    })
                    .collect(),
                rustdoc_types::StructKind::Plain { fields, .. } => fields
                    .iter()
                    .map(|field| {
                        let meta = meta.source.index.get(field).ok_or::<Box<dyn Error>>(
                            "Expected to find field in the same crate as struct".into(),
                        )?;
                        let type_ = match &meta.inner {
                            ItemEnum::StructField(field) => field.clone(),
                            _ => panic!("Expected struct field"),
                        };
                        let name = meta.name.clone().expect("Expected name on struct field");
                        Ok((name, type_))
                    })
                    .collect(),
            };

            let name_types = field_pairs
                .into_iter()
                .filter_map(|field_pair| {
                    field_pair
                        .map_err(|e| log::info!("skipping field due to `{e}`"))
                        .ok()
                })
                .zip(std::iter::repeat(config))
                .map(|((name, type_), config)| {
                    match NameType::try_from(((name.clone(), type_), config)) {
                        Ok(v) => v,
                        Err(_) => NameType {
                            name,
                            type_: ArgType::Base("ReflectedValue".to_owned()),
                            is_proxied: false,
                        },
                    }
                })
                .collect::<Vec<NameType>>();

            fields.extend(name_types);
        };

        Self {
            source_crate: meta.config.source.0.to_owned(),
            import_path,
            implemented_traits: meta.implemented_traits,
            functions,
            docstrings: vec![],
            fields,
            item_type,
        }
    }
}
