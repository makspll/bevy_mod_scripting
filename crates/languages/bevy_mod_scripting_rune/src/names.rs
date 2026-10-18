//! Collects the names scripts can use after a `.`, which Rune needs registered up front, see [`crate::types`].
use std::collections::BTreeSet;

use bevy_mod_scripting_bindings::{AppScriptNamespaceMembers, WorldExtensions, WorldGuard};
use bevy_reflect::{TypeInfo, enums::VariantInfo};

use crate::types::intern;

/// What the names were collected from, if it changes the names need to be collected again.
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct Fingerprint {
    functions: usize,
    types: usize,
    /// the components, events and exports registered by scripts
    dynamic: (u64, usize),
}

impl Fingerprint {
    /// The fingerprint of the current state of the world, which is cheap to take.
    pub fn of(world: &WorldGuard) -> Self {
        Self {
            functions: world
                .script_function_registry()
                .read()
                .iter_all()
                .size_hint()
                .0,
            types: world.type_registry().read().iter().size_hint().0,
            dynamic: {
                let registry = world.component_registry();
                let registry = registry.read();
                (
                    registry.generation(),
                    registry.export_and_event_namespaces().count(),
                )
            },
        }
    }
}

fn tuple_field_names(len: usize, names: &mut BTreeSet<&'static str>) {
    for index in 0..len {
        names.insert(intern(&format!("_{index}")));
    }
}

fn type_member_names(info: &TypeInfo, names: &mut BTreeSet<&'static str>) {
    match info {
        TypeInfo::Struct(info) => {
            for name in info.field_names() {
                names.insert(intern(name));
            }
        }
        TypeInfo::TupleStruct(info) => tuple_field_names(info.field_len(), names),
        TypeInfo::Tuple(info) => tuple_field_names(info.field_len(), names),
        TypeInfo::Enum(info) => {
            for variant in info.iter() {
                names.insert(intern(variant.name()));
                match variant {
                    VariantInfo::Struct(variant) => {
                        for name in variant.field_names() {
                            names.insert(intern(name));
                        }
                    }
                    VariantInfo::Tuple(variant) => tuple_field_names(variant.field_len(), names),
                    VariantInfo::Unit(_) => {}
                }
            }
        }
        _ => {}
    }
}

/// Collects the names of everything which can be accessed with the `.` syntax: functions, fields, enum variants, types and namespace members.
pub fn collect_names(world: &WorldGuard) -> BTreeSet<&'static str> {
    let mut names = BTreeSet::new();

    for (key, _) in world.script_function_registry().read().iter_all() {
        names.insert(intern(&key.name));
    }

    for registration in world.type_registry().read().iter() {
        type_member_names(registration.type_info(), &mut names);
    }

    if let Ok(catalogue) = world.type_catalogue().clone().get(world.clone()) {
        for entry in catalogue.iter() {
            names.insert(intern(&entry.short_name));
        }
    }

    {
        let registry = world.component_registry();
        let registry = registry.read();
        for component in registry.iter_components() {
            names.insert(intern(&component.name));
        }
        // the names of exports and events are the last part of their path
        for namespace in registry
            .export_and_event_namespaces()
            .collect::<BTreeSet<_>>()
        {
            for export in registry.iter_exports_in(namespace) {
                names.insert(intern(&export.name));
            }
            for (name, _) in registry.iter_events_in(namespace) {
                names.insert(intern(name));
            }
        }
    }

    let _ = world.with_resource(|members: &AppScriptNamespaceMembers| {
        for (_, contents) in members.read().iter() {
            for name in contents.functions.keys().chain(contents.values.keys()) {
                names.insert(intern(name));
            }
        }
    });

    names
}
