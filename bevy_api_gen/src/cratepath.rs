use rustdoc_types::{Crate, Id, ItemEnum, Visibility};

pub(crate) fn get_path(id: &Id, source: &Crate, verbose: bool) -> Option<Vec<String>> {
    let verbose = verbose || (id == &Id(String::from("0:6568:5720")));
    if verbose {
        eprintln!("Search for {:?}", id);
    }
    match source.paths.get(id) {
        Some(p) => return Some(p.path.clone()),
        None => {
            let ind = source.index.get(id)?;
            if let Visibility::Restricted { parent, path } = &ind.visibility {
                if let Some(mut p_path) = get_path(parent, source, verbose) {
                    let path_parse: Vec<String> = path[2..]
                        .to_string()
                        .split("::")
                        .map(|s| s.to_string())
                        .collect();
                    if !p_path.ends_with(path_parse.as_slice()) {
                        p_path.extend(path_parse);
                    }
                    if verbose {
                        eprintln!("restrict: {:?}", p_path);
                    }
                    return Some(p_path);
                }
            }
            let parents = (&source.index)
                .into_iter()
                .filter(|(_, p_item)| {
                    if let Some(name) = &ind.name {
                        if p_item.links.contains_key(name) {
                            return true;
                        }
                    }
                    if let ItemEnum::Impl(p_impl) = &p_item.inner {
                        return p_impl.items.contains(id);
                    }
                    if let ItemEnum::Import(p_import) = &p_item.inner {
                        if let Some(p_inner) = &p_import.id {
                            return p_inner == id;
                        }
                        return false;
                    }
                    if let ItemEnum::Module(p_mod) = &p_item.inner {
                        return p_mod.items.contains(id);
                    }
                    return false;
                })
                .map(|(p_id, p_item)| {
                    if let ItemEnum::Module(_) = &p_item.inner {
                        return (p_id, None);
                    }
                    return (p_id, None);
                });

            for (parent, additional) in parents {
                if verbose {}
                let path_o = get_path(parent, source, verbose);
                if let Some(mut path) = path_o {
                    if let Some(in_path) = additional {
                        path.push(in_path);
                    }
                    return Some(path);
                } else if verbose {
                    eprintln!("{:?}", parent);
                }
            }
            if verbose {
                eprintln!("no match");
            }
        }
    };
    None
}
