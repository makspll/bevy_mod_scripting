//! What scripts reach without going through a type: the namespaces they import (see [`catalogue`]), and the functions and values registered into those namespaces from Rust.
//!
//! The only globals are the builtins: the functions registered in [`crate::FunctionScope::Global`] (i.e. `import_namespace`), and `world`.

use crate::{
    DynamicScriptFunction, docgen::typed_through::ThroughTypeInfo, error::InteropError,
    script_value::ScriptValue,
};
use bevy_ecs::resource::Resource;
use bevy_mod_scripting_world::WorldGuard;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::{borrow::Cow, collections::BTreeMap, sync::Arc};

pub mod catalogue;
pub mod core;
pub use catalogue::*;
pub use core::*;

/// Marker type for registering functions into a script namespace instead of on a type:
///
/// ```rust,ignore
/// #[script_bindings(namespace = "my_game")]
/// impl NamespaceMembers {
///     /// Scripts call this as `import_namespace("my_game").spawn_enemy(5)`
///     fn spawn_enemy(ctxt: FunctionCallContext, hp: usize) -> Result<(), InteropError> { .. }
/// }
/// ```
pub struct NamespaceMembers;

/// Makes the value of a namespace member, see [`NamespaceValue`].
pub type NamespaceValueMaker =
    dyn Fn(WorldGuard) -> Result<ScriptValue, InteropError> + 'static + Send + Sync;

/// A value in a namespace, made each time the namespace is imported.
#[derive(Clone)]
pub struct NamespaceValue {
    /// Makes the value
    pub maker: Arc<NamespaceValueMaker>,
    /// The type of the value, for documentation
    pub type_information: ThroughTypeInfo,
    /// The documentation of the value
    pub documentation: Option<Cow<'static, str>>,
}

/// The functions and values registered into one namespace
#[derive(Default, Clone)]
pub struct NamespaceContents {
    /// Functions by name
    pub functions: BTreeMap<Cow<'static, str>, DynamicScriptFunction>,
    /// Values by name
    pub values: BTreeMap<Cow<'static, str>, NamespaceValue>,
}

/// The functions and values registered into script namespaces from Rust, by namespace.
///
/// Script namespaces (`script.*`) are filled by scripts instead, see [`crate::ScriptComponentRegistry`].
#[derive(Default)]
pub struct ScriptNamespaceMembers {
    namespaces: BTreeMap<Cow<'static, str>, NamespaceContents>,
}

impl ScriptNamespaceMembers {
    /// Registers a function as a member of the namespace, replacing any member function with the same name
    pub fn register_function(
        &mut self,
        namespace: impl Into<Cow<'static, str>>,
        name: impl Into<Cow<'static, str>>,
        function: DynamicScriptFunction,
    ) {
        self.namespaces
            .entry(namespace.into())
            .or_default()
            .functions
            .insert(name.into(), function);
    }

    /// Registers a value as a member of the namespace, replacing any member value with the same name
    pub fn register_value(
        &mut self,
        namespace: impl Into<Cow<'static, str>>,
        name: impl Into<Cow<'static, str>>,
        value: NamespaceValue,
    ) {
        self.namespaces
            .entry(namespace.into())
            .or_default()
            .values
            .insert(name.into(), value);
    }

    /// The members of the given namespace, if it has any
    pub fn get(&self, namespace: &str) -> Option<&NamespaceContents> {
        self.namespaces.get(namespace)
    }

    /// Every namespace with members, in order
    pub fn iter(&self) -> impl Iterator<Item = (&Cow<'static, str>, &NamespaceContents)> {
        self.namespaces.iter()
    }
}

/// A send + sync wrapper around the [`ScriptNamespaceMembers`].
#[derive(Default, Resource, Clone)]
pub struct AppScriptNamespaceMembers(Arc<RwLock<ScriptNamespaceMembers>>);

#[profiling::all_functions]
impl AppScriptNamespaceMembers {
    /// Returns a reference to the inner [`ScriptNamespaceMembers`].
    pub fn read(&self) -> RwLockReadGuard<'_, ScriptNamespaceMembers> {
        self.0.read()
    }

    /// Returns a mutable reference to the inner [`ScriptNamespaceMembers`].
    pub fn write(&self) -> RwLockWriteGuard<'_, ScriptNamespaceMembers> {
        self.0.write()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{docgen::TypedThrough, function::script_function::DynamicScriptFunction};

    #[test]
    fn members_are_grouped_by_namespace() {
        let mut members = ScriptNamespaceMembers::default();
        let function: DynamicScriptFunction = (|_, _| ScriptValue::Unit).into();
        members.register_function("my_game", "spawn", function);
        members.register_value(
            "my_game",
            "difficulty",
            NamespaceValue {
                maker: Arc::new(|_| Ok(ScriptValue::Integer(1))),
                type_information: i64::through_type_info(),
                documentation: None,
            },
        );

        let contents = members.get("my_game").expect("namespace exists");
        assert!(contents.functions.contains_key("spawn"));
        assert!(contents.values.contains_key("difficulty"));
        assert!(members.get("other").is_none());
    }
}
