//! The type catalogue: a shared, language-agnostic index of the types available to scripts.
//!
//! The catalogue is built once from the type registry and shared by every script context,
//! instead of each context re-resolving every registered type on creation.
//! It is rebuilt lazily whenever the set of registered types or components changes.

use std::sync::Arc;

use bevy_ecs::resource::Resource;
use bevy_platform::collections::HashMap;
use bevy_reflect::TypeRegistration;
use parking_lot::RwLock;

use crate::{
    AnyTypeRegistration, AnyTypeRegistrationV, ScriptBundleRegistration,
    ScriptComponentRegistration, ScriptResourceRegistration, ScriptTypeRegistration,
    WorldExtensions, error::InteropError, function::from::V,
};
use bevy_mod_scripting_world::WorldGuard;

/// The registration of a catalogue entry, resolved to the most specific kind of type it is.
#[derive(Clone)]
pub enum CatalogueRegistration {
    /// A plain type
    Type(ScriptTypeRegistration),
    /// A component type
    Component(ScriptComponentRegistration),
    /// A resource type
    Resource(ScriptResourceRegistration),
    /// A bundle type
    Bundle(ScriptBundleRegistration),
}

impl CatalogueRegistration {
    fn from_any(any: AnyTypeRegistration) -> Self {
        match any.into_left() {
            Ok(r) => Self::Type(r),
            Err(other) => match other.into_left() {
                Ok(c) => Self::Component(c),
                Err(other) => match other.into_left() {
                    Ok(r) => Self::Resource(r),
                    Err(b) => Self::Bundle(b),
                },
            },
        }
    }

    /// Converts to the form scripts receive, cloning the registration.
    pub fn to_script(&self) -> AnyTypeRegistrationV {
        use crate::function::from::Union;
        match self.clone() {
            Self::Type(r) => Union::new_left(V(r)),
            Self::Component(c) => Union::new_right(Union::new_left(V(c))),
            Self::Resource(r) => Union::new_right(Union::new_right(Union::new_left(V(r)))),
            Self::Bundle(b) => Union::new_right(Union::new_right(Union::new_right(V(b)))),
        }
    }
}

/// Where a catalogue entry came from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CatalogueSource {
    /// Registered through the type registry
    Native,
    /// Registered at runtime by a script
    Script,
}

/// A single type known to the catalogue.
#[derive(Clone)]
pub struct CatalogueEntry {
    /// The full type path, i.e. `bevy_transform::components::transform::Transform`
    pub path: Arc<str>,
    /// The short type name, i.e. `Transform`
    pub short_name: Arc<str>,
    /// The namespace of the type, see [`ScriptTypeRegistration::namespace`]. Empty for the root namespace.
    pub namespace: Arc<str>,
    /// The registration of the type
    pub registration: CatalogueRegistration,
    /// Where the entry came from
    pub source: CatalogueSource,
}

/// An index of types available to scripts.
#[derive(Clone, Default)]
pub struct ScriptTypeCatalogue {
    entries: Vec<CatalogueEntry>,
    by_short_name: HashMap<Arc<str>, usize>,
    by_namespace: HashMap<Arc<str>, Vec<usize>>,
    /// duplicate short names encountered while building, first one wins in lookups
    duplicates: Vec<Arc<str>>,
}

impl ScriptTypeCatalogue {
    /// Iterates over all entries
    pub fn iter(&self) -> impl Iterator<Item = &CatalogueEntry> {
        self.entries.iter()
    }

    /// The number of entries in the catalogue
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the catalogue is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Looks up an entry by short name
    pub fn get_by_short_name(&self, name: &str) -> Option<&CatalogueEntry> {
        self.by_short_name.get(name).map(|i| &self.entries[*i])
    }

    /// The names of all namespaces with at least one entry, in no particular order
    pub fn namespaces(&self) -> impl Iterator<Item = &str> {
        self.by_namespace.keys().map(|n| n.as_ref())
    }

    /// Iterates over the entries in the given namespace
    pub fn iter_namespace<'a>(
        &'a self,
        namespace: &str,
    ) -> impl Iterator<Item = &'a CatalogueEntry> + 'a {
        self.by_namespace
            .get(namespace)
            .into_iter()
            .flatten()
            .map(|i| &self.entries[*i])
    }

    /// The entries of the given namespace keyed by short name, as returned by `import`, or `None` if the namespace has no entries.
    ///
    /// Types sharing a short name within the namespace resolve to the first one registered.
    pub fn namespace_view(&self, namespace: &str) -> Option<HashMap<String, AnyTypeRegistrationV>> {
        let indices = self.by_namespace.get(namespace)?;
        let mut view = HashMap::with_capacity(indices.len());
        for entry in indices.iter().map(|i| &self.entries[*i]) {
            view.entry(entry.short_name.to_string())
                .or_insert_with(|| entry.registration.to_script());
        }
        Some(view)
    }

    /// Namespaces with names close to `requested`, closest first, for error messages.
    pub fn similar_namespaces(&self, requested: &str) -> Vec<String> {
        self.similar_namespaces_with(requested, &[])
    }

    /// Like [`Self::similar_namespaces`], also considering the `extra` namespaces (i.e. ones which only have exports).
    pub fn similar_namespaces_with(&self, requested: &str, extra: &[String]) -> Vec<String> {
        let all = self
            .namespaces()
            .chain(extra.iter().map(String::as_str))
            .collect::<std::collections::BTreeSet<_>>();
        let mut scored = all
            .into_iter()
            .filter(|n| !n.is_empty())
            .filter_map(|n| {
                let distance = edit_distance(requested, n);
                // close in spelling, or one contains the other (i.e. `transform` and `bevy_transform`)
                let related =
                    !requested.is_empty() && (n.contains(requested) || requested.contains(n));
                (distance <= 3.max(requested.len() / 3) || related).then_some((distance, n))
            })
            .collect::<Vec<_>>();
        scored.sort();
        scored
            .into_iter()
            .take(3)
            .map(|(_, n)| n.to_owned())
            .collect()
    }

    /// Short names which were claimed by more than one type
    pub fn duplicate_short_names(&self) -> &[Arc<str>] {
        &self.duplicates
    }

    /// Adds an entry, returns false if the short name was already taken
    /// (the entry is still stored, but short name lookups keep resolving to the first one)
    pub fn insert(&mut self, entry: CatalogueEntry) -> bool {
        let index = self.entries.len();
        self.by_namespace
            .entry(entry.namespace.clone())
            .or_default()
            .push(index);
        let fresh = match self.by_short_name.entry(entry.short_name.clone()) {
            bevy_platform::collections::hash_map::Entry::Vacant(v) => {
                v.insert(index);
                true
            }
            bevy_platform::collections::hash_map::Entry::Occupied(_) => {
                self.duplicates.push(entry.short_name.clone());
                false
            }
        };
        self.entries.push(entry);
        fresh
    }

    /// Builds the catalogue from the type registry of the world.
    pub fn build(
        guard: WorldGuard,
        filter: fn(&TypeRegistration) -> bool,
    ) -> Result<Self, InteropError> {
        let type_registry = guard.type_registry();
        let type_registry = type_registry.read();
        let mut catalogue = Self::default();
        for registration in type_registry.iter().filter(|r| filter(r)) {
            let path_table = registration.type_info().type_path_table();
            let path: Arc<str> = path_table.path().into();
            let short_name: Arc<str> = path_table.short_path().into();
            let registration = ScriptTypeRegistration::new(Arc::new(registration.clone()));
            let namespace: Arc<str> = registration.namespace().into();
            let registration = guard.clone().get_type_registration(registration)?;
            catalogue.insert(CatalogueEntry {
                path,
                short_name,
                namespace,
                registration: CatalogueRegistration::from_any(registration),
                source: CatalogueSource::Native,
            });
        }
        drop(type_registry);

        // script registered entries go last so native types win short name clashes
        let script_registry = guard.component_registry();
        let script_registry = script_registry.read();
        for info in script_registry.iter_components() {
            catalogue.insert(CatalogueEntry {
                path: info.path().into(),
                short_name: info.name.as_str().into(),
                namespace: info.namespace.as_str().into(),
                registration: CatalogueRegistration::Component(info.registration.clone()),
                source: CatalogueSource::Script,
            });
        }
        for bundle in script_registry.iter_bundles() {
            catalogue.insert(CatalogueEntry {
                path: bundle.path().into(),
                short_name: bundle.name.as_str().into(),
                namespace: bundle.namespace.as_str().into(),
                registration: CatalogueRegistration::Bundle(bundle.registration.clone()),
                source: CatalogueSource::Script,
            });
        }
        Ok(catalogue)
    }

    /// The flat view of the catalogue keyed by short name, as exposed by the `types` global.
    ///
    /// Types sharing a short name resolve to the first one registered.
    pub fn flat_view(&self) -> HashMap<String, AnyTypeRegistrationV> {
        self.by_short_name
            .iter()
            .map(|(name, i)| (name.to_string(), self.entries[*i].registration.to_script()))
            .collect()
    }
}

struct Cached {
    /// (registered types, registered components, script registry generation) at build time
    stamp: (usize, usize, u64),
    catalogue: Arc<ScriptTypeCatalogue>,
}

/// The shared, lazily refreshed [`ScriptTypeCatalogue`] of the app.
#[derive(Resource, Clone)]
pub struct AppScriptTypeCatalogue {
    filter: fn(&TypeRegistration) -> bool,
    cached: Arc<RwLock<Option<Cached>>>,
}

impl Default for AppScriptTypeCatalogue {
    fn default() -> Self {
        Self::new(|_| true)
    }
}

impl AppScriptTypeCatalogue {
    /// Creates a catalogue which only includes types accepted by the filter.
    pub fn new(filter: fn(&TypeRegistration) -> bool) -> Self {
        Self {
            filter,
            cached: Default::default(),
        }
    }

    fn stamp(guard: &WorldGuard) -> Result<(usize, usize, u64), InteropError> {
        let types = guard.type_registry().read().iter().count();
        // only reads component metadata, which can't change while systems run in parallel (registering needs exclusive access),
        // and unlike `with_world_access` doesn't claim access to the whole world, which non exclusive script systems don't have
        let components = guard.as_unsafe_world_cell()?.components().len();
        let script_generation = guard.component_registry().read().generation();
        Ok((types, components, script_generation))
    }

    /// Returns an up to date catalogue, rebuilding it if types or components were registered since the last call.
    pub fn get(&self, guard: WorldGuard) -> Result<Arc<ScriptTypeCatalogue>, InteropError> {
        let stamp = Self::stamp(&guard)?;
        if let Some(cached) = self.cached.read().as_ref()
            && cached.stamp == stamp
        {
            return Ok(cached.catalogue.clone());
        }
        let catalogue = Arc::new(ScriptTypeCatalogue::build(guard, self.filter)?);
        *self.cached.write() = Some(Cached {
            stamp,
            catalogue: catalogue.clone(),
        });
        Ok(catalogue)
    }
}

/// The Levenshtein distance between two strings.
fn edit_distance(a: &str, b: &str) -> usize {
    let b: Vec<char> = b.chars().collect();
    let mut previous: Vec<usize> = (0..=b.len()).collect();
    for (i, ca) in a.chars().enumerate() {
        let mut current = vec![i + 1];
        for (j, cb) in b.iter().enumerate() {
            let substitution = previous[j] + usize::from(ca != *cb);
            current.push(substitution.min(previous[j + 1] + 1).min(current[j] + 1));
        }
        previous = current;
    }
    previous[b.len()]
}
