use std::error::Error;

use indexmap::{IndexMap, IndexSet};
use rustdoc_types::{ItemEnum, Type};

use crate::{
    cratepath::*, template_data::ImportPath, Config, FunctionData, NameType, OperatorType,
    TypeMeta, ValidType,
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
    pub functions: IndexMap<String, Vec<FunctionData>>,
    pub fields: Vec<NameType>,
    pub docstrings: Vec<String>,
    pub item_type: ItemType,
}

impl From<(TypeMeta<'_>, &Config)> for ItemData {
    fn from((meta, config): (TypeMeta<'_>, &Config)) -> Self {
        let import_path: Vec<String> = meta.path_components.into();
        let import_path: ImportPath = import_path.into();
        let mut functions: IndexMap<String, Vec<FunctionData>> = Default::default();

        for (name, impl_items) in &meta.impl_items {
            log::debug!("Processing item: `{name}` in type: `{}`", meta.wrapped_type);
            for (impl_, item) in impl_items {
                let operator = OperatorType::from_impl_name(name);
                match (|| {
                    let trait_path = impl_
                        .trait_
                        .as_ref()
                        .map(|trait_| {
                            // TODO: this is probably pretty slow, pre-calculate this
                            // meta.crates.iter().find_map(|crate_|
                            if let Some(op) = operator {
                                return Ok::<_, Box<dyn Error>>(op.trait_path());
                            }

                            let origin_crate = lookup_item_crate_source(&trait_.id, meta.crates)
                                .ok_or(format!(
                                    "Could not find trait in any crates `{}` for function: `{}`",
                                    trait_.name, name
                                ))?;

                            let path_components = get_path(&trait_.id, origin_crate).unwrap();
                            let import_path = path_to_import(path_components, meta.source).into();
                            Ok(import_path)
                        })
                        .transpose()?;

                    let assoc_types = impl_
                        .items
                        .iter()
                        .filter_map(|id| {
                            meta.source.index.get(id).and_then(|i| {
                                matches!(i.inner, ItemEnum::AssocType { .. }).then_some(i)
                            })
                        })
                        .collect::<Vec<_>>();

                    let fn_item = FunctionData::try_new(
                        trait_path,
                        (*item).clone(),
                        config,
                        operator,
                        assoc_types,
                    )?;
                    Ok::<_, Box<dyn Error>>(fn_item)
                })() {
                    Ok(v) => functions.entry((*name).to_owned()).or_default().push(v),
                    Err(e) => {
                        log::debug!("Skipping item/function: {name} because: {e}");
                        continue;
                    }
                };
            }
        }

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
                    match NameType::try_new(name.clone(), type_, config, Default::default()) {
                        Ok(v) => v,
                        Err(_) => NameType {
                            name,
                            type_: ValidType::Base("ReflectedValue".to_owned()),
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
