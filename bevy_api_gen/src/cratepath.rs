use rustdoc_types::{Crate, Id, ItemEnum, Visibility};

pub(crate) fn get_path(id: &Id, source: &Crate) -> Option<Vec<Id>> {
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
    path.iter()
        .rev()
        .enumerate()
        .rev()
        .enumerate()
        .map(|(starti, (endi, id))| {
            let ind = source.index.get(id).unwrap();
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
