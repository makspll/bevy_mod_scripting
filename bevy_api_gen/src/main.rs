pub mod cratepath;

use bevy_api_gen_lib::{Args, Config, PrettyWriter, WrappedItem, WRAPPER_PREFIX};

use clap::Parser;
use cratepath::{get_path, path_to_import};
use indexmap::{IndexMap, IndexSet};
use rustdoc_types::{Crate, Impl, Item, ItemEnum};
use serde_json::from_reader;
use std::{
    borrow::Cow,
    collections::HashSet,
    fs::{read_to_string, File},
    io::{self, BufReader},
};

pub(crate) fn write_use_items_from_path(
    module_name: &str,
    path_components: &[String],
    writer: &mut PrettyWriter,
) {
    // generate imports for each item
    writer.write_no_newline("use ");

    if module_name.starts_with("bevy") && module_name.len() > 5 {
        writer.write_inline("bevy::");
        writer.write_inline(&module_name[5..]);
    } else {
        writer.write_inline(module_name);
    }

    for item in path_components {
        writer.write_inline("::");
        writer.write_inline(item);
    }
    writer.write_inline(";");
    writer.newline();
}

pub(crate) fn generate_macros(
    crates: &[Crate],
    config: Config,
    args: &Args,
) -> Result<String, io::Error> {
    // the items we want to generate macro instantiations for
    let mut unmatched_types: HashSet<&String> = config.types.iter().map(|(k, _v)| k).collect();

    let mut wrapped_items: Vec<_> = crates
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

                    let impls = match &item.inner {
                        ItemEnum::Struct(s) => &s.impls,
                        ItemEnum::Enum(e) => &e.impls,
                        _ => panic!("Only structs or enums are allowed!"),
                    };

                    impls.iter().for_each(|id| {
                        if let ItemEnum::Impl(i) = &source.index.get(id).unwrap().inner {
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

                    //let path_components = &source.paths.get(id).unwrap().path;
                    let path_components = get_path(id, source).unwrap_or_else(|| {
                        panic!("path not found for {:?} in {:?}", id, source.root)
                    });
                    //eprintln!("{:?}", path_components);
                    let path_components = path_to_import(path_components, source);
                    //eprintln!("{:?}", path_components);

                    let wrapper_name = format!("{WRAPPER_PREFIX}{}", item.name.as_ref().unwrap());
                    let wrapped_type = item.name.as_ref().unwrap();
                    WrappedItem {
                        wrapper_name,
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

    wrapped_items.iter().for_each(|v| {
        unmatched_types.remove(&v.wrapped_type);
    });

    if !unmatched_types.is_empty() {
        panic!("Some types were not found in the given crates: {unmatched_types:#?}")
    }

    let mut writer = PrettyWriter::new();

    // we want to preserve the original ordering from the config file
    wrapped_items.sort_by_cached_key(|f| config.types.get_index_of(f.wrapped_type).unwrap());

    writer.write_line("#![allow(clippy::all,unused_imports)]");
    writer.write_line("// This file is generated by `bevy_mod_scripting_derive/main.rs` change the template not this file");
    writer.write_line("extern crate self as bevy_script_api;");
    writer.write_line("use bevy_mod_scripting_derive::impl_script_newtype;");

    // user defined
    config.imports.lines().for_each(|import| {
        writer.write_line(import);
    });
    // automatic

    wrapped_items.iter().for_each(|item| {
        write_use_items_from_path(
            &item.config.source.0,
            &item.path_components[1..],
            &mut writer,
        );
    });

    let mut imported = HashSet::<String>::default();

    wrapped_items.iter().for_each(|item| {
        item.config.traits.iter().for_each(|trait_methods| {
            if !imported.contains(&trait_methods.name) {
                writer.write_no_newline("use ");
                writer.write_inline(&trait_methods.import_path);
                writer.write_inline(";");
                writer.newline();
                imported.insert(trait_methods.name.to_owned());
            }
        })
    });

    // make macro calls for each wrapped item
    wrapped_items.iter_mut().for_each(|v| {
        // macro invocation
        writer.write_no_newline("impl_script_newtype!");
        writer.open_brace();
        writer.write_line("#[languages(on_feature(lua))]");

        v.write_type_docstring(&mut writer, args);
        writer.write_indentation();
        v.write_inline_full_path(&mut writer, args);
        writer.write_inline(" : ");
        writer.newline();

        v.write_derive_flags_body(&config, &mut writer, args);

        writer.write_line("lua impl");
        writer.open_brace();
        v.write_impl_block_body(&mut writer, args);
        writer.close_brace();

        writer.close_brace();
    });

    // write other code
    for line in config.other.lines() {
        writer.write_line(line);
    }

    // now create the BevyAPIProvider
    // first the globals
    writer.write_line("#[cfg(feature=\"lua\")]");
    writer.write_line("#[derive(Default)]");
    writer.write_line("pub(crate) struct BevyAPIGlobals;");

    writer.write_line("#[cfg(feature=\"lua\")]");
    writer.write_no_newline(
        "impl bevy_mod_scripting_lua::tealr::mlu::ExportInstances for BevyAPIGlobals",
    );
    writer.open_brace();
    writer.write_line("fn add_instances<'lua, T: bevy_mod_scripting_lua::tealr::mlu::InstanceCollector<'lua>>(self, instances: &mut T) -> bevy_mod_scripting_lua::tealr::mlu::mlua::Result<()>");
    writer.open_brace();
    for (global_name, type_, dummy_proxy) in wrapped_items
        .iter()
        .filter_map(|i| {
            i.has_global_methods.then_some((
                i.wrapped_type.as_str(),
                i.wrapper_name.as_str(),
                false,
            ))
        })
        .chain(config.manual_lua_types.iter().filter_map(|i| {
            i.include_global_proxy.then_some((
                i.proxy_name.as_str(),
                i.name.as_str(),
                i.use_dummy_proxy,
            ))
        }))
    {
        writer.write_no_newline("instances.add_instance(");
        // type name
        writer.write_inline("\"");
        writer.write_inline(global_name);
        writer.write_inline("\"");
        // corresponding proxy
        if dummy_proxy {
            writer.write_inline(", crate::lua::util::DummyTypeName::<");
            writer.write_inline(type_);
            writer.write_inline(">::new");
            writer.write_inline(")?;");
            writer.newline();
        } else {
            writer.write_inline(", bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<");
            writer.write_inline(type_);
            writer.write_inline(">::new)?;");
            writer.newline();
        }
    }

    writer.write_line("Ok(())");
    writer.close_brace();
    writer.close_brace();

    // then the actual provider
    writer.write_line("#[cfg(feature=\"lua\")]");
    writer.write_line("pub struct LuaBevyAPIProvider;");

    // begin impl {
    writer.write_line("#[cfg(feature=\"lua\")]");
    writer.write_no_newline("impl APIProvider for LuaBevyAPIProvider");
    writer.open_brace();

    writer.write_line("type APITarget = Mutex<bevy_mod_scripting_lua::tealr::mlu::mlua::Lua>;");
    writer.write_line("type ScriptContext = Mutex<bevy_mod_scripting_lua::tealr::mlu::mlua::Lua>;");
    writer.write_line("type DocTarget = LuaDocFragment;");

    // attach_api {
    writer.write_no_newline(
        "fn attach_api(&mut self, ctx: &mut Self::APITarget) -> Result<(), ScriptError>",
    );
    writer.open_brace();
    writer.write_line("let ctx = ctx.get_mut().expect(\"Unable to acquire lock on Lua context\");");
    writer.write_line("bevy_mod_scripting_lua::tealr::mlu::set_global_env(BevyAPIGlobals,ctx).map_err(|e| ScriptError::Other(e.to_string()))");
    writer.close_brace();
    // } attach_api

    // get_doc_fragment
    writer.write_no_newline("fn get_doc_fragment(&self) -> Option<Self::DocTarget>");
    writer.open_brace();
    writer.write_no_newline("Some(LuaDocFragment::new(|tw|");
    writer.open_brace();
    writer.write_line("tw");
    writer.write_line(".document_global_instance::<BevyAPIGlobals>().expect(\"Something went wrong documenting globals\")");

    // include external types not generated by this file as well
    for (type_, include_proxy) in
        wrapped_items
            .iter()
            .map(|i| (i.wrapper_name.as_str(), i.has_global_methods))
            .chain(config.manual_lua_types.iter().filter_map(|i| {
                (!i.dont_process).then_some((i.name.as_str(), i.include_global_proxy))
            }))
    {
        writer.write_no_newline(".process_type::<");
        writer.write_inline(type_);
        writer.write_inline(">()");
        writer.newline();

        if include_proxy {
            writer.write_no_newline(
                ".process_type::<bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<",
            );
            writer.write_inline(type_);
            writer.write_inline(">>()");
            writer.newline();
        }
    }

    writer.close_brace();
    writer.write_line("))");

    writer.close_brace();
    // } get_doc_fragment

    // impl default members
    for line in config.lua_api_defaults.lines() {
        writer.write_line(line);
    }

    // register_with_app {
    writer.write_no_newline("fn register_with_app(&self, app: &mut App)");
    writer.open_brace();
    for item in wrapped_items
        .iter()
        .map(|i| i.wrapped_type)
        .chain(config.primitives.iter())
    {
        writer.write_no_newline("app.register_foreign_lua_type::<");
        writer.write_inline(item);
        writer.write_inline(">();");
        writer.newline();
    }
    writer.close_brace();
    // } regiser_with_app

    writer.close_brace();
    // } end impl

    Ok(writer.finish())
}

pub fn main() -> Result<(), io::Error> {
    let args = Args::parse();

    let crates: Vec<_> = args
        .json
        .iter()
        .map(|json| {
            let f = File::open(json).unwrap_or_else(|_| panic!("Could not open {}", &json));
            let rdr = BufReader::new(f);
            from_reader(rdr)
        })
        .collect::<Result<Vec<_>, _>>()?;

    let f = read_to_string(&args.config)?;
    let mut config: Config = toml::from_str(&f)?;

    config.types_.reverse();

    while !config.types_.is_empty() {
        let t = config.types_.remove(config.types_.len() - 1);
        config.types.insert(t.type_.to_string(), t);
    }

    let out = generate_macros(&crates, config, &args)?;

    println!("{}", out);

    Ok(())
}
