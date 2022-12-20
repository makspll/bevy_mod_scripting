use rustdoc_types::{Crate, Id, ItemEnum, Visibility};

pub(crate) fn get_path(id: &Id, source: &Crate, verbose: bool) -> Option<Vec<Id>> {
    let verbose = verbose;// || (id == &Id(String::from("0:6568:5720")));
    if verbose {
        eprintln!("Search for {:?}", id);
    }
    match source.paths.get(id) {
        Some(_) => return Some(vec![id.to_owned()]),
        None => {
            let ind = source.index.get(id)?;
            if let Visibility::Restricted { parent, path: _ } = &ind.visibility {
                if let Some(p_path) = get_path(parent, source, verbose) {
                    
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
                });

            for (parent, _) in parents {
                if verbose {}
                let path_o = get_path(parent, source, verbose);
                if let Some(mut path) = path_o {
                    path.push(id.to_owned());
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

pub(crate) fn path_to_import(path: Vec<Id>, source: &Crate) -> Vec<String> {
    /* 
    for i in path {
        eprintln!("{:?}", source.index.get(&i).unwrap());
    }
    eprintln!();
    
    if path.last().unwrap().0 == String::from("0:3728:1594") {
        eprintln!("{:?}",path);
        eprintln!("{:?}", source.index.get(path.last().unwrap()).unwrap());
    }
    */
    path.iter().rev().enumerate().rev().enumerate().map(|(starti, (endi, id))| {
        let ind = source.index.get(id).unwrap();
        if starti == 0 {
            return source.paths.get(id).unwrap().path.clone();
        }
        else if endi==0 {
            if let Some(name) = &ind.name {
                return vec![name.to_owned()];
            }
        }
        else if let Visibility::Restricted { parent: _, path } = &ind.visibility {
            return path[2..].split("::").map(|x| x.to_string()).collect();
        }
        else if let ItemEnum::Module(_) = &ind.inner {
            //eprintln!("{:?}\n", ind);
            //return vec![ind.name.to_owned().unwrap()];
            return vec![];
        }
        vec![]
    }).reduce(|mut x,y| {x.extend(y); x}).unwrap()
}