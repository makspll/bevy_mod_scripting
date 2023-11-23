use bevy_api_gen_lib::{
    get_path, path_to_import, Args, Config, ItemData, NewtypeConfig, TemplateData, TypeMeta,
    ValidType,
};

use clap::Parser;

use indexmap::{IndexMap, IndexSet};
use rustdoc_types::{Crate, Id, Impl, Item, ItemEnum, Path, Type};
use sailfish::{runtime::Buffer, TemplateOnce};
use serde_json::from_reader;
use std::{
    borrow::Cow,
    collections::HashSet,
    error::Error,
    fmt::Display,
    fs::{read_to_string, File},
    io::{self, BufReader, Write},
};

pub fn main() -> Result<(), io::Error> {
    let args = Args::parse();

    let crates: Vec<_> = args
        .json
        .iter()
        .map(|json| {
            let f = File::open(json).unwrap_or_else(|e| panic!("Could not open {}, {e}", &json));
            let rdr = BufReader::new(f);
            from_reader(rdr)
        })
        .collect::<Result<Vec<_>, _>>()?;

    let f = read_to_string(&args.config)?;
    let mut config: Config =
        toml::from_str(&f).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    config.types_.reverse();

    while !config.types_.is_empty() {
        let t = config.types_.remove(config.types_.len() - 1);
        config.types.insert(t.type_.to_string(), t);
    }

    generate(crates, config, args);

    Ok(())
}

fn generate_macro_data<'a>(crates: &'a [Crate], config: &'a Config) -> Vec<TypeMeta<'a>> {
    // the items we want to generate macro instantiations for
    let mut unmatched_types: HashSet<&String> = config.types.iter().map(|(k, _v)| k).collect();

    let mut type_meta: Vec<_> = crates
        .iter()
        .flat_map(|source| {
            source
                .index
                .iter()
                .filter(|(_id, item)| {
                    item.name
                        .as_ref()
                        .and_then(|k| config.types.get(k))
                        .map(|k| k.matches_result(item, source))
                        .unwrap_or(false)
                })
                .map(|(id, item)| {
                    // extract all available associated constants,methods etc available to this item
                    let mut self_impl: Option<&Impl> = None;
                    let mut impl_items: IndexMap<&str, Vec<(&Impl, &Item)>> = Default::default();
                    let mut implemented_traits: IndexSet<String> = Default::default();
                    let wrapped_type = item.name.as_ref().unwrap();

                    let impls = match &item.inner {
                        ItemEnum::Struct(s) => &s.impls,
                        ItemEnum::Enum(e) => &e.impls,
                        _ => panic!("Only structs or enums are allowed!"),
                    };

                    impls.iter().for_each(|id| {
                        if let ItemEnum::Impl(i) = &source.index.get(id).unwrap().inner {
                            // filter out impls not for this type
                            let for_type = ValidType::try_new(false, &i.for_).map_err(|e| {
                                log::debug!("Ignoring impl block as could not parse type impl block is for: `{e}`")
                            });
                            if let Ok(for_) = &for_type {
                                // TODO: we need a more solid light `Type` enum with proper equality and ease of use
                                if !for_.base_ident().is_some_and(|base| base == wrapped_type){
                                    log::debug!("Ignoring impl block as it's not for current type: for: {:?}, current_type: {wrapped_type}, block: {i:#?}", for_.base_ident());
                                    return
                                }
                            } else {
                                return
                            }

                            match &i.trait_ {
                                Some(t) => {
                                    implemented_traits.insert(t.name.to_owned());
                                }
                                None => self_impl = Some(i),
                            }
                            i.items.iter().for_each(|id| {
                                let it = source.index.get(id).unwrap();
                                impl_items
                                    .entry(it.name.as_ref().unwrap().as_str())
                                    .or_default()
                                    .push((i, it));
                            })
                        } else {
                            panic!("Expected impl items here!")
                        }
                    });

                    let config = config.types.get(item.name.as_ref().unwrap()).unwrap();

                    let path_components = get_path(id, source).unwrap_or_else(|| {
                        panic!("path not found for {:?} in {:?}", id, source.root)
                    });
                    let path_components = path_to_import(path_components, source);

                    TypeMeta {
                        wrapped_type,
                        path_components: Cow::Owned(path_components),
                        source,
                        config,
                        item,
                        self_impl,
                        impl_items,
                        crates,
                        has_global_methods: false,
                        implemented_traits,
                    }
                })
        })
        .collect();

    type_meta.iter().for_each(|v| {
        unmatched_types.remove(&v.wrapped_type);
    });

    if !unmatched_types.is_empty() {
        panic!("Some types were not found in the given crates: {unmatched_types:#?}")
    }
    // we want to preserve the original ordering from the config file
    type_meta.sort_by_cached_key(|f| config.types.get_index_of(f.wrapped_type).unwrap());

    type_meta
}

fn unwrap_or_pretty_error<T, E: Display>(val: Result<T, E>) -> T {
    match val {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    }
}

fn generate(crates: Vec<Crate>, config: Config, args: Args) {
    pretty_env_logger::init();
    log::info!("Beginning code gen");
    let wrapped_items = generate_macro_data(&crates, &config);

    let mut buffer = Buffer::new();

    let template_data = TemplateData {
        items: wrapped_items
            .into_iter()
            .filter(|item| {
                args.type_allowlist
                    .as_ref()
                    .map(|allow_list| allow_list.contains(item.wrapped_type))
                    .unwrap_or(true)
            })
            .zip(std::iter::repeat(&config))
            .map(ItemData::from)
            .map(|i| (i.import_path.components.last().unwrap().to_owned(), i))
            .collect(),
    };

    unwrap_or_pretty_error(template_data.render_once_to(&mut buffer));
    let output = buffer.into_string();
    // log::info!("Prettyfying output..");
    // let parsed_file = unwrap_or_pretty_error(syn::parse_file(output.as_str()));
    // let pretty_output = prettyplease::unparse(&parsed_file);
    let mut f = unwrap_or_pretty_error(File::create(args.output));
    unwrap_or_pretty_error(f.write_all(output.as_bytes()));
    unwrap_or_pretty_error(f.flush());
}
