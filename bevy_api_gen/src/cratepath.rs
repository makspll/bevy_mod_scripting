use rustdoc_types::{Crate, Id, ItemEnum, Visibility};

pub(crate) fn crate_name(crate_: &Crate) -> String {
    crate_
        .index
        .get(&crate_.root)
        .as_ref()
        .unwrap()
        .name
        .to_owned()
        .unwrap()
}

pub(crate) fn lookup_external_item_crate_source_name(id: &Id, found_in: &Crate) -> String {
    let crate_id = found_in
        .paths
        .get(id)
        .expect("Missing link to external item")
        .crate_id;
    found_in
        .external_crates
        .get(&crate_id)
        .unwrap()
        .name
        .to_owned()
}

pub(crate) fn lookup_item_crate_source<'a>(id: &'a Id, crates: &'a [Crate]) -> Option<&'a Crate> {
    crates.iter().find(|crate_| crate_.index.contains_key(id))
}

pub(crate) fn get_path(id: &Id, source: &Crate) -> Option<Vec<Id>> {
    log::debug!(
        "Trying to find path for item id: `{id:?}` has index entry: `{}` has path entry: `{}`",
        source.index.get(id).is_some(),
        source.paths.get(id).is_some()
    );
    if source.index.get(id).is_none() {
        panic!("Trying to find path for item which is external to the provided source crate, the item lives in crate: `{}` not in `{}`",            
            source.external_crates.get(&source.paths.get(id).as_ref().unwrap().crate_id).unwrap().name,
            crate_name(source)
        );
    }
    match source.paths.get(id) {
        Some(_) => return Some(vec![id.to_owned()]),
        None => {
            let ind = source.index.get(id)?;
            if let Visibility::Restricted { parent, .. } = &ind.visibility {
                if let Some(p_path) = get_path(parent, source) {
                    return Some(p_path);
                }
            }
            let parents = source.index.iter().filter(|(_, p_item)| {
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
                false
            });

            for (parent, _) in parents {
                let path_o = get_path(parent, source);
                if let Some(mut path) = path_o {
                    path.push(id.to_owned());
                    return Some(path);
                }
            }
        }
    };
    None
}

pub(crate) fn path_to_import(path: Vec<Id>, source: &Crate) -> Vec<String> {
    log::debug!(
        "Trying to convert id path to path components: `{path:?}` with names: [{:?}] in crate: `{}`",
        path.iter()
            .map(|id| source
                .index
                .get(id)
                .and_then(|item| item.name.as_deref())
                .unwrap_or("None"))
            .collect::<Vec<_>>()
            .join(","),
            crate_name(source)
    );
    path.iter()
        .rev()
        .enumerate()
        .rev()
        .enumerate()
        .map(|(starti, (endi, id))| {
            log::trace!("starti: {starti}, endi: {endi}, id: {id:?}");

            let ind = source
                .index
                .get(id)
                .expect("Trying to find path to item which is not in the provided source crate");

            if starti == 0 {
                return source.paths.get(id).unwrap().path.clone();
            } else if endi == 0 {
                if let Some(name) = &ind.name {
                    return vec![name.to_owned()];
                }
            } else if let Visibility::Restricted { parent: _, path } = &ind.visibility {
                return path[2..].split("::").map(|x| x.to_string()).collect();
            } else if let ItemEnum::Module(module) = &ind.inner {
                if !module.is_stripped {
                    return vec![source.index.get(id).unwrap().name.clone().unwrap()];
                } else {
                    return vec![];
                }
            }
            vec![]
        })
        .reduce(|mut x, y| {
            x.extend(y);
            x
        })
        .unwrap()
}
