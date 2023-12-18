use bevy_api_gen_lib::{crate_name, Args, CrateUnion, ImportPathCrawler};

use clap::Parser;

use rustdoc_types::Crate;
use serde_json::from_reader;
use std::{
    fmt::Display,
    fs::{read_to_string, File},
    io::{self, BufReader},
    path::MAIN_SEPARATOR,
    time::Instant,
};
use wildmatch::WildMatch;

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
    generate(crates, args);

    Ok(())
}

// fn generate_macro_data<'a>(
//     crate_data: &'a CrawledImportData<'a>,
//     crates: &'a [Crate],
//     config : &Config,
// ) -> Vec<ItemData> {
//     // now look through everything in the crates
//     let mut type_meta: Vec<_> = Default::default();

//     // pre process the impls a little
//     let all_impls = crate_data
//         .get_impls()
//         .map(|(impl_crate, impl_id)| {
//             let item = impl_crate.index.get(impl_id).unwrap();
//             if let ItemEnum::Impl(impl_) = &item.inner {
//                 let ty = ValidType::try_new(false, &impl_.for_).map_err(|e| format!("impl id: {impl_id:?}, crate: {impl_crate:?}, {e}"))?;
//                 let trait_path = if let Some(trait_path) = impl_.trait_.as_ref() {
//                     Some(
//                         crate_data
//                             .get_public_trait_path(&(*impl_crate, trait_path.id.clone()))
//                             .ok_or_else(||
//                                 format!("Impl id: {impl_id:?}, crate: {impl_crate:?}, No public path to trait")
//                             )?,
//                     )
//                 } else {
//                     None
//                 };
//                 Ok::<_, Box<dyn Error>>((impl_crate, item, ty, trait_path))
//             } else {
//                 unreachable!()
//             }
//         })
//         .filter_map(|out| match out {
//             Ok(v) => Some(v),
//             Err(e) => {
//                 log::trace!("Impl is filtered out from all impls {e}");
//                 None
//             }
//         })
//         .collect::<Vec<_>>();
//     // for found_in_crate in crates {
//     for (crate_, item_id) in crate_data.get_public_types() {
//         let item = crate_.index.get(item_id).unwrap();
//         // extract all available associated constants,methods etc available to this item
//         let (generics, item_type) = match &item.inner {
//             ItemEnum::Struct(s) => {
//                 (&s.generics, s.kind.clone().into())},
//             ItemEnum::Enum(e) => (&e.generics, ItemType::Enum),
//             _ => panic!("types contain an unexpected item"),
//         };

//         if !(generics.params.is_empty() && generics.where_predicates.is_empty()) {
//             log::debug!("Skipping type: `{}` as it has generics", item.name.as_ref().unwrap());
//             continue;
//         };

//         let mut self_impl: Option<&Impl> = None;
//         let mut impl_items: IndexMap<&str, Vec<ImplItem>> = Default::default();
//         let mut implemented_traits: IndexSet<String> = Default::default();
//         let import_path = crate_data
//             .get_public_item_path(&(*crate_, item_id.to_owned()))
//             .expect("Item has no public path!")
//             .clone();
//         let wrapped_type = import_path.components.last().unwrap();

//         // find all implementations on this type including from foreign crates
//         all_impls
//             .iter()
//             // TODO: we need a more solid light `Type` enum with proper equality, crate idea? small_syn
//             // or somehow resolve these types to stable cross-crate Id's
//             .filter(|(_, _, ty, _)| ty.base_ident().unwrap() == wrapped_type)
//             .for_each(|(impl_crate, item, for_ty, trait_path)| {
//                 if let ItemEnum::Impl(impl_) = &item.inner {
//                     let foreign = for_ty
//                         .base_ident()
//                         .is_some_and(|base| config.primitives.contains(base));

//                     match &impl_.trait_ {
//                         Some(t) => {
//                             implemented_traits.insert(t.name.to_owned());
//                         }
//                         None => self_impl = Some(impl_),
//                     }

//                     impl_.items.iter().for_each(|id| {
//                         let impl_item = impl_crate.index.get(id).unwrap();
//                         impl_items
//                             .entry(impl_item.name.as_ref().unwrap().as_str())
//                             .or_default()
//                             .push(ImplItem {
//                                 impl_,
//                                 item: impl_item,
//                                 foreign,
//                                 trait_import_path: trait_path.cloned(),
//                             });
//                     })
//                 } else {
//                     unreachable!()
//                 }
//             });

//         // check this type implements reflect
//         if !implemented_traits.contains("Reflect")
//             || !implemented_traits.contains("GetTypeRegistration")
//         {
//             log::debug!("Skipping type: `{wrapped_type}` as it doesn't implemnet Reflect and GetTypeRegistration");
//             continue;
//         };

//         type_meta.push(ItemData::new(
//             item,
//             item_type,
//             import_path,
//             &impl_items,
//             implemented_traits,
//             CrateId(crate_, crate_.crate_name()),
//             config,
//         ).unwrap());
//     }

//     // sort to ensure stable generation each time
//     type_meta.sort_by(|a,b| a.import_path.cmp(&b.import_path));

//     type_meta
// }

// fn unwrap_or_pretty_error<T, E: Display>(val: Result<T, E>) -> T {
//     match val {
//         Ok(v) => v,
//         Err(e) => panic!("{}", e),
//     }
// }

fn generate(crates: Vec<Crate>, args: Args) {
    pretty_env_logger::init();
    log::info!("Beginning code gen..");
    // figure out the import paths for all items we might need
    let mut path_crawler = ImportPathCrawler::new();
    for c in &crates {
        log::info!("Crawling crate: `{}`", crate_name(c));
        let before = Instant::now();
        path_crawler.crawl_crate(c);
        log::info!("Crawling took: {}s", before.elapsed().as_secs_f32())
    }
    log::info!("Finalizing crawler output..");
    let before = Instant::now();
    let paths = path_crawler.finalize(&crates);
    log::info!(
        "Finalizing crawler output took: {}s",
        before.elapsed().as_secs_f32()
    );

    let filters = args
        .filters
        .map(|e| e.iter().map(|e| WildMatch::new(e)).collect::<Vec<_>>())
        .unwrap_or_default();

    let excludes = args
        .excludes
        .map(|e| e.iter().map(|e| WildMatch::new(e)).collect::<Vec<_>>())
        .unwrap_or_default();

    let union = CrateUnion::new(paths, &filters, &excludes);

    println!("{:#?}", union);

    // let wrapped_items = generate_macro_data(&paths, &crates, &config);

    // let mut buffer = Buffer::new();

    // let template_data = TemplateData {
    //     items: wrapped_items
    //         .into_iter()
    //         .map(|i| (i.import_path.components.last().unwrap().to_owned(), i))
    //         .collect(),
    //     primitives: config.primitives,
    // };

    // unwrap_or_pretty_error(template_data.render_once_to(&mut buffer));
    // let output = buffer.into_string();
    // log::info!("Prettyfying output..");
    // let parsed_file = unwrap_or_pretty_error(syn::parse_file(output.as_str()));
    // let pretty_output = prettyplease::unparse(&parsed_file);
    // let mut f = unwrap_or_pretty_error(File::create(args.output));
    // unwrap_or_pretty_error(f.write_all(pretty_output.as_bytes()));
    // unwrap_or_pretty_error(f.flush());
}
