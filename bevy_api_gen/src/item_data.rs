use std::error::Error;

use indexmap::{IndexMap, IndexSet};
use rustdoc_types::{Crate, Generics, Item, ItemEnum, StructKind, Type, Visibility};

use crate::{
    cratepath::ImportPath, CrateId, FunctionData, ImplItem, NameType, OperatorType, ValidType,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemType {
    UnitStruct,
    TupleStruct,
    Struct,
    Enum,
    Unsupported,
}

impl From<StructKind> for ItemType {
    fn from(value: StructKind) -> Self {
        match value {
            StructKind::Unit => Self::UnitStruct,
            StructKind::Tuple(_) => Self::TupleStruct,
            StructKind::Plain { .. } => Self::Struct,
        }
    }
}

pub struct ItemData {
    pub source_crate: String,
    pub generics: Generics,
    pub import_path: ImportPath,
    pub implemented_traits: IndexSet<String>,
    pub functions: IndexMap<String, Vec<FunctionData>>,
    pub has_globals: bool,
    pub fields: Vec<NameType>,
    pub docstrings: Vec<String>,
    pub item_type: ItemType,
}

impl ItemData {
    pub fn new(
        item: &Item,
        item_type: ItemType,
        import_path: ImportPath,
        impl_items: &IndexMap<&str, Vec<ImplItem>>,
        implemented_traits: IndexSet<String>,
        source_crate: CrateId,
        config: &Config,
    ) -> Result<Self, Box<dyn Error>> {
        let mut functions: IndexMap<String, Vec<FunctionData>> = Default::default();

        for (name, impl_items) in impl_items {
            log::debug!("Processing item: `{name}` in type: `{}`", import_path);
            for ImplItem {
                impl_,
                item,
                foreign,
                trait_import_path,
            } in impl_items
            {
                let operator = OperatorType::from_impl_name(name);
                let mut foreign_self = None;
                match (|| {
                    let trait_path = impl_
                        .trait_
                        .as_ref()
                        .map(|trait_| {
                            // TODO: this is probably pretty slow, pre-calculate this
                            // meta.crates.iter().find_map(|crate_|
                            if let Some(op) = operator {
                                foreign_self =
                                    Some(ValidType::try_new(false, &impl_.for_).map_err(|e| {
                                        format!("Could not parse foreign self type: `{e}`")
                                    })?);
                                return Ok::<_, Box<dyn Error>>(op.trait_path());
                            }
                            log::info!("trait: {}, {:?}", trait_.name, trait_.id);
                            Ok(trait_import_path
                                .as_ref()
                                .expect("Trait without import path")
                                .clone())
                        })
                        .transpose()?;

                    let assoc_types = impl_
                        .items
                        .iter()
                        .filter_map(|id| {
                            source_crate.index.get(id).and_then(|i| {
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
                        if *foreign {
                            foreign_self.as_ref()
                        } else {
                            None
                        },
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
        if let ItemEnum::Struct(struct_) = &item.inner {
            let field_pairs: Vec<Result<(String, Type), Box<dyn Error>>> = match &struct_.kind {
                rustdoc_types::StructKind::Unit => Default::default(),
                rustdoc_types::StructKind::Tuple(t) => t
                    .iter()
                    .enumerate()
                    .map(|(idx, id)| {
                        let type_ = id
                            .as_ref()
                            .map(|id| {
                                let meta = source_crate.index.get(id).ok_or::<Box<dyn Error>>(
                                    "Expected to find field in the same crate as struct".into(),
                                )?;
                                if !matches!(meta.visibility, Visibility::Public) {
                                    return Err("Private field".into());
                                }
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
                        let meta = source_crate.index.get(field).ok_or::<Box<dyn Error>>(
                            "Expected to find field in the same crate as struct".into(),
                        )?;
                        if !matches!(meta.visibility, Visibility::Public) {
                            return Err("Private field".into());
                        }

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
                    match NameType::try_new(name.clone(), type_, config, Default::default(), None) {
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

        Ok(Self {
            source_crate: source_crate.crate_name().to_owned(),
            import_path,
            generics: Generics {
                params: vec![],
                where_predicates: vec![],
            },
            implemented_traits,
            docstrings: vec![],
            fields,
            item_type,
            has_globals: functions
                .iter()
                .any(|(_, fns)| fns.iter().any(|f| f.is_static)),
            functions,
        })
    }
}
