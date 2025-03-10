//! Contains abstractions for exposing "globals" to scripts, in a language-agnostic way.

use super::{
    function::arg_meta::{ScriptReturn, TypedScriptReturn},
    script_value::ScriptValue,
    WorldGuard,
};
use crate::{docgen::typed_through::ThroughTypeInfo, error::InteropError};
use bevy::{ecs::system::Resource, utils::hashbrown::HashMap};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::{any::TypeId, borrow::Cow, sync::Arc};

pub mod core;

/// A send + sync wrapper around the [`ScriptGlobalsRegistry`].
#[derive(Default, Resource, Clone)]
pub struct AppScriptGlobalsRegistry(Arc<RwLock<ScriptGlobalsRegistry>>);

impl AppScriptGlobalsRegistry {
    /// Returns a reference to the inner [`ScriptGlobalsRegistry`].
    pub fn read(&self) -> RwLockReadGuard<ScriptGlobalsRegistry> {
        self.0.read()
    }

    /// Returns a mutable reference to the inner [`ScriptGlobalsRegistry`].
    pub fn write(&self) -> RwLockWriteGuard<ScriptGlobalsRegistry> {
        self.0.write()
    }
}

/// A function that creates a global variable.
pub type ScriptGlobalMakerFn<T> =
    dyn Fn(WorldGuard) -> Result<T, InteropError> + 'static + Send + Sync;

/// A global variable that can be exposed to scripts.
pub struct ScriptGlobal {
    /// The function that creates the global variable.
    /// if not present, this is assumed to be a static global, one that
    /// cannot be instantiated, but carries type information.
    pub maker: Option<Arc<ScriptGlobalMakerFn<ScriptValue>>>,
    /// The documentation for the global variable.
    pub documentation: Option<Cow<'static, str>>,
    /// The type ID of the global variable.
    pub type_id: TypeId,
    /// Rich type information the global variable.
    pub type_information: Option<ThroughTypeInfo>,
}

/// A registry of global variables that can be exposed to scripts.
#[derive(Default)]
pub struct ScriptGlobalsRegistry {
    globals: HashMap<Cow<'static, str>, ScriptGlobal>,
}

impl ScriptGlobalsRegistry {
    /// Gets the global with the given name
    pub fn get(&self, name: &str) -> Option<&ScriptGlobal> {
        self.globals.get(name)
    }

    /// Gets the global with the given name mutably
    pub fn get_mut(&mut self, name: &str) -> Option<&mut ScriptGlobal> {
        self.globals.get_mut(name)
    }

    /// Counts the number of globals in the registry
    pub fn len(&self) -> usize {
        self.globals.len()
    }

    /// Checks if the registry is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Iterates over the globals in the registry
    pub fn iter(&self) -> impl Iterator<Item = (&Cow<'static, str>, &ScriptGlobal)> {
        self.globals.iter()
    }

    /// Iterates over the globals in the registry mutably
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Cow<'static, str>, &mut ScriptGlobal)> {
        self.globals.iter_mut()
    }

    fn type_erase_maker<
        T: ScriptReturn,
        F: Fn(WorldGuard) -> Result<T, InteropError> + Send + Sync + 'static,
    >(
        maker: F,
    ) -> Arc<ScriptGlobalMakerFn<ScriptValue>> {
        Arc::new(move |world| T::into_script(maker(world.clone())?, world))
    }

    /// Inserts a global into the registry, returns the previous value if it existed
    pub fn register<
        T: ScriptReturn + 'static,
        F: Fn(WorldGuard) -> Result<T, InteropError> + 'static + Send + Sync,
    >(
        &mut self,
        name: Cow<'static, str>,
        maker: F,
    ) -> Option<ScriptGlobal> {
        self.globals.insert(
            name,
            ScriptGlobal {
                maker: Some(Self::type_erase_maker(maker)),
                documentation: None,
                type_id: TypeId::of::<T>(),
                type_information: None,
            },
        )
    }

    /// Inserts a global into the registry, returns the previous value if it existed.
    ///
    /// This is a version of [`Self::register`] which stores type information regarding the global.
    pub fn register_documented<
        T: TypedScriptReturn + 'static,
        F: Fn(WorldGuard) -> Result<T, InteropError> + 'static + Send + Sync,
    >(
        &mut self,
        name: Cow<'static, str>,
        maker: F,
        documentation: Cow<'static, str>,
    ) -> Option<ScriptGlobal> {
        self.globals.insert(
            name,
            ScriptGlobal {
                maker: Some(Self::type_erase_maker(maker)),
                documentation: Some(documentation),
                type_id: TypeId::of::<T>(),
                type_information: Some(T::through_type_info()),
            },
        )
    }

    /// Registers a static global into the registry.
    pub fn register_static<T: 'static>(&mut self, name: Cow<'static, str>) {
        self.globals.insert(
            name,
            ScriptGlobal {
                maker: None,
                documentation: None,
                type_id: TypeId::of::<T>(),
                type_information: None,
            },
        );
    }

    /// Registers a static global into the registry.
    ///
    /// This is a version of [`Self::register_static`] which stores rich type information regarding the global.
    pub fn register_static_documented<T: TypedScriptReturn + 'static>(
        &mut self,
        name: Cow<'static, str>,
        documentation: Cow<'static, str>,
    ) {
        self.globals.insert(
            name,
            ScriptGlobal {
                maker: None,
                documentation: Some(documentation),
                type_id: TypeId::of::<T>(),
                type_information: Some(T::through_type_info()),
            },
        );
    }

    /// Registers a static global into the registry.
    ///
    /// This is a version of [`Self::register_static_documented`] which does not require compile time type knowledge.
    pub fn register_static_documented_dynamic(
        &mut self,
        type_id: TypeId,
        type_information: Option<ThroughTypeInfo>,
        name: Cow<'static, str>,
        documentation: Cow<'static, str>,
    ) {
        self.globals.insert(
            name,
            ScriptGlobal {
                maker: None,
                documentation: Some(documentation),
                type_id,
                type_information,
            },
        );
    }
}

#[cfg(test)]
mod test {
    use bevy::ecs::world::World;

    use super::*;

    #[test]
    fn test_script_globals_registry() {
        let mut registry = ScriptGlobalsRegistry::default();

        let maker = |_: WorldGuard| Ok(ScriptValue::from(42));
        let maker2 = |_: WorldGuard| Ok(ScriptValue::from(43));

        assert_eq!(registry.len(), 0);
        assert!(registry.is_empty());

        assert!(registry.register(Cow::Borrowed("foo"), maker).is_none());
        assert_eq!(registry.len(), 1);

        assert_eq!(
            (registry.get("foo").unwrap().maker.clone().unwrap())(WorldGuard::new_exclusive(
                &mut World::new()
            ))
            .unwrap(),
            ScriptValue::from(42)
        );

        assert!(registry.register(Cow::Borrowed("foo"), maker2).is_some());
        assert_eq!(registry.len(), 1);

        assert_eq!(
            (registry.get("foo").unwrap().maker.clone().unwrap())(WorldGuard::new_exclusive(
                &mut World::new()
            ))
            .unwrap(),
            ScriptValue::from(43)
        );
    }

    #[test]
    fn test_documentation_is_stored() {
        let mut registry = ScriptGlobalsRegistry::default();

        let maker = |_: WorldGuard| Ok(ScriptValue::from(42));

        assert!(registry
            .register_documented(Cow::Borrowed("foo"), maker, Cow::Borrowed("This is a test"))
            .is_none());

        let global = registry.get("foo").unwrap();
        assert_eq!(global.documentation.as_deref(), Some("This is a test"));
    }

    #[test]
    fn test_static_globals() {
        let mut registry = ScriptGlobalsRegistry::default();

        registry.register_static::<i32>(Cow::Borrowed("foo"));

        let global = registry.get("foo").unwrap();
        assert!(global.maker.is_none());
        assert_eq!(global.type_id, TypeId::of::<i32>());

        // the same but documented
        registry.register_static_documented::<i32>(
            Cow::Borrowed("bar"),
            Cow::Borrowed("This is a test"),
        );

        let global = registry.get("bar").unwrap();
        assert!(global.maker.is_none());
        assert_eq!(global.type_id, TypeId::of::<i32>());
        assert_eq!(global.documentation.as_deref(), Some("This is a test"));
    }
}
