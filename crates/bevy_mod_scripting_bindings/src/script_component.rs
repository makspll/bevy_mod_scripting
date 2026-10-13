//! Everything necessary to support scripts registering their own components

use super::{
    ScriptBundleRegistration, ScriptComponentRegistration, ScriptEventRegistration, ScriptValue,
};
use ::{
    bevy_app::{App, Plugin},
    bevy_ecs::component::{Component, Mutable, StorageType},
};
use bevy_ecs::resource::Resource;
use bevy_platform::collections::HashMap;
use bevy_reflect::std_traits::ReflectDefault;
use parking_lot::RwLock;
use std::sync::Arc;
/// Implements [`Reflect`](bevy_reflect::Reflect) for a type holding a single `data: ScriptValue` field, as a struct whose fields are
/// the `data` field and, if `data` is a table, its entries.
///
/// So `value.health` is `value.data.health` to everything going through reflection, including scripts,
/// and new fields can be added by setting them.
/// * Reading a field `data` has no entry for gives `nil`, as for any table.
/// * Writing a field turns `data` into a table if it is `nil`, and fails if it is anything else.
macro_rules! dynamic_data_reflect {
    ($ty:ident) => {
        impl ::bevy_reflect::TypePath for $ty {
            fn type_path() -> &'static str {
                concat!(module_path!(), "::", stringify!($ty))
            }
            fn short_type_path() -> &'static str {
                stringify!($ty)
            }
            fn type_ident() -> Option<&'static str> {
                Some(stringify!($ty))
            }
            fn crate_name() -> Option<&'static str> {
                module_path!().split("::").next()
            }
            fn module_path() -> Option<&'static str> {
                Some(module_path!())
            }
        }

        impl ::bevy_reflect::Typed for $ty {
            fn type_info() -> &'static ::bevy_reflect::TypeInfo {
                static CELL: ::bevy_reflect::utility::NonGenericTypeInfoCell =
                    ::bevy_reflect::utility::NonGenericTypeInfoCell::new();
                CELL.get_or_set(|| {
                    ::bevy_reflect::TypeInfo::Struct(
                        ::bevy_reflect::structs::StructInfo::new::<Self>(&[
                            ::bevy_reflect::NamedField::new::<ScriptValue>("data"),
                        ]),
                    )
                })
            }
        }

        impl ::bevy_reflect::structs::Struct for $ty {
            fn field(&self, name: &str) -> Option<&dyn ::bevy_reflect::PartialReflect> {
                static MISSING: ScriptValue = ScriptValue::Unit;
                if name == "data" {
                    return Some(&self.data);
                }
                match &self.data {
                    ScriptValue::Map(entries) => Some(entries.get(name).unwrap_or(&MISSING)
                        as &dyn ::bevy_reflect::PartialReflect),
                    ScriptValue::Unit => Some(&MISSING),
                    _ => None,
                }
            }

            fn field_mut(&mut self, name: &str) -> Option<&mut dyn ::bevy_reflect::PartialReflect> {
                if name == "data" {
                    return Some(&mut self.data);
                }
                if matches!(self.data, ScriptValue::Unit) {
                    self.data = ScriptValue::Map(Default::default());
                }
                match &mut self.data {
                    ScriptValue::Map(entries) => {
                        // only allocate the key for new entries
                        if !entries.contains_key(name) {
                            entries.insert(name.to_owned(), ScriptValue::Unit);
                        }
                        entries
                            .get_mut(name)
                            .map(|value| value as &mut dyn ::bevy_reflect::PartialReflect)
                    }
                    _ => None,
                }
            }

            fn field_at(&self, index: usize) -> Option<&dyn ::bevy_reflect::PartialReflect> {
                (index == 0).then_some(&self.data as &dyn ::bevy_reflect::PartialReflect)
            }

            fn field_at_mut(
                &mut self,
                index: usize,
            ) -> Option<&mut dyn ::bevy_reflect::PartialReflect> {
                (index == 0).then_some(&mut self.data as &mut dyn ::bevy_reflect::PartialReflect)
            }

            fn name_at(&self, index: usize) -> Option<&str> {
                (index == 0).then_some("data")
            }

            fn index_of_name(&self, name: &str) -> Option<usize> {
                (name == "data").then_some(0)
            }

            fn field_len(&self) -> usize {
                1
            }

            fn iter_fields(&self) -> ::bevy_reflect::structs::FieldIter<'_> {
                ::bevy_reflect::structs::FieldIter::new(self)
            }
        }

        impl ::bevy_reflect::PartialReflect for $ty {
            fn get_represented_type_info(&self) -> Option<&'static ::bevy_reflect::TypeInfo> {
                Some(<Self as ::bevy_reflect::Typed>::type_info())
            }

            fn into_partial_reflect(self: Box<Self>) -> Box<dyn ::bevy_reflect::PartialReflect> {
                self
            }

            fn as_partial_reflect(&self) -> &dyn ::bevy_reflect::PartialReflect {
                self
            }

            fn as_partial_reflect_mut(&mut self) -> &mut dyn ::bevy_reflect::PartialReflect {
                self
            }

            fn try_into_reflect(
                self: Box<Self>,
            ) -> Result<Box<dyn ::bevy_reflect::Reflect>, Box<dyn ::bevy_reflect::PartialReflect>>
            {
                Ok(self)
            }

            fn try_as_reflect(&self) -> Option<&dyn ::bevy_reflect::Reflect> {
                Some(self)
            }

            fn try_as_reflect_mut(&mut self) -> Option<&mut dyn ::bevy_reflect::Reflect> {
                Some(self)
            }

            fn try_apply(
                &mut self,
                value: &dyn ::bevy_reflect::PartialReflect,
            ) -> Result<(), ::bevy_reflect::ApplyError> {
                let other = value.reflect_ref().as_struct()?;
                for (name, value) in other {
                    if let Some(field) = ::bevy_reflect::structs::Struct::field_mut(self, name) {
                        field.try_apply(value)?;
                    }
                }
                Ok(())
            }

            fn reflect_kind(&self) -> ::bevy_reflect::ReflectKind {
                ::bevy_reflect::ReflectKind::Struct
            }

            fn reflect_ref(&self) -> ::bevy_reflect::ReflectRef<'_> {
                ::bevy_reflect::ReflectRef::Struct(self)
            }

            fn reflect_mut(&mut self) -> ::bevy_reflect::ReflectMut<'_> {
                ::bevy_reflect::ReflectMut::Struct(self)
            }

            fn reflect_owned(self: Box<Self>) -> ::bevy_reflect::ReflectOwned {
                ::bevy_reflect::ReflectOwned::Struct(self)
            }

            fn reflect_clone(
                &self,
            ) -> Result<Box<dyn ::bevy_reflect::Reflect>, ::bevy_reflect::ReflectCloneError> {
                Ok(Box::new(self.clone()))
            }

            fn debug(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Debug::fmt(self, f)
            }
        }

        impl ::bevy_reflect::Reflect for $ty {
            fn into_any(self: Box<Self>) -> Box<dyn ::core::any::Any> {
                self
            }

            fn as_any(&self) -> &dyn ::core::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                self
            }

            fn into_reflect(self: Box<Self>) -> Box<dyn ::bevy_reflect::Reflect> {
                self
            }

            fn as_reflect(&self) -> &dyn ::bevy_reflect::Reflect {
                self
            }

            fn as_reflect_mut(&mut self) -> &mut dyn ::bevy_reflect::Reflect {
                self
            }

            fn set(
                &mut self,
                value: Box<dyn ::bevy_reflect::Reflect>,
            ) -> Result<(), Box<dyn ::bevy_reflect::Reflect>> {
                *self = value.take()?;
                Ok(())
            }
        }

        impl ::bevy_reflect::FromReflect for $ty {
            fn from_reflect(reflect: &dyn ::bevy_reflect::PartialReflect) -> Option<Self> {
                if let Some(this) = reflect.try_downcast_ref::<Self>() {
                    return Some(this.clone());
                }
                let other = reflect.reflect_ref().as_struct().ok()?;
                let mut this = Self::default();
                if let Some(data) = other.field("data") {
                    this.data = data.try_downcast_ref::<ScriptValue>()?.clone();
                }
                Some(this)
            }
        }

        impl ::bevy_reflect::GetTypeRegistration for $ty {
            fn get_type_registration() -> ::bevy_reflect::TypeRegistration {
                let mut registration = ::bevy_reflect::TypeRegistration::of::<Self>();
                registration.register_type_data::<::bevy_reflect::ReflectFromPtr, Self>();
                registration.register_type_data::<::bevy_reflect::ReflectFromReflect, Self>();
                registration.register_type_data::<ReflectDefault, Self>();
                registration
            }
        }
    };
}

/// A dynamic script component
#[derive(Clone, Default, Debug)]
pub struct DynamicComponent {
    data: ScriptValue,
}

impl DynamicComponent {
    /// Creates a new dynamic component holding the given data
    pub fn new(data: ScriptValue) -> Self {
        Self { data }
    }

    /// Returns the data held by this component
    pub fn data(&self) -> &ScriptValue {
        &self.data
    }
}

dynamic_data_reflect!(DynamicComponent);

/// A dynamic script bundle value.
///
/// `data` maps the names of the bundle's members to their values, members without an entry are default constructed.
#[derive(Clone, Default, Debug)]
pub struct DynamicBundle {
    data: ScriptValue,
}

impl DynamicBundle {
    /// Creates a new dynamic bundle holding the given member values
    pub fn new(data: ScriptValue) -> Self {
        Self { data }
    }

    /// Returns the member values held by this bundle
    pub fn data(&self) -> &ScriptValue {
        &self.data
    }

    /// Consumes the bundle returning the member values
    pub fn into_data(self) -> ScriptValue {
        self.data
    }
}

dynamic_data_reflect!(DynamicBundle);

/// Some metadata about dynamic script components
pub struct DynamicComponentInfo {
    /// The name of the component
    pub name: String,
    /// The type registration for the component
    pub registration: ScriptComponentRegistration,
}

impl Component for DynamicComponent {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    type Mutability = Mutable;
}

/// A registry of dynamically registered script components
#[derive(Clone, Resource, Default)]
pub struct AppScriptComponentRegistry(pub Arc<RwLock<ScriptComponentRegistry>>);

#[profiling::all_functions]
impl AppScriptComponentRegistry {
    /// Reads the underlying registry
    pub fn read(&self) -> parking_lot::RwLockReadGuard<'_, ScriptComponentRegistry> {
        self.0.read()
    }

    /// Writes to the underlying registry
    pub fn write(&self) -> parking_lot::RwLockWriteGuard<'_, ScriptComponentRegistry> {
        self.0.write()
    }
}

#[derive(Default)]
/// A registry of dynamically registered script components and bundles
pub struct ScriptComponentRegistry {
    components: HashMap<String, DynamicComponentInfo>,
    bundles: HashMap<String, ScriptBundleRegistration>,
    events: HashMap<String, ScriptEventRegistration>,
    generation: u64,
}

#[profiling::all_functions]
impl ScriptComponentRegistry {
    /// Registers a dynamic script component, possibly overwriting an existing one
    pub fn register(&mut self, info: DynamicComponentInfo) {
        self.components.insert(info.name.clone(), info);
        self.generation += 1;
    }

    /// Gets a dynamic script component by name
    pub fn get(&self, name: &str) -> Option<&DynamicComponentInfo> {
        self.components.get(name)
    }

    /// Registers a dynamic script bundle, possibly overwriting an existing one
    pub fn register_bundle(&mut self, name: String, registration: ScriptBundleRegistration) {
        self.bundles.insert(name, registration);
        self.generation += 1;
    }

    /// A counter which changes whenever a component or bundle is registered.
    ///
    /// Lookups of existing registrations do not change it.
    pub fn generation(&self) -> u64 {
        self.generation
    }

    /// Iterates over all registered script components
    pub fn iter_components(&self) -> impl Iterator<Item = &DynamicComponentInfo> {
        self.components.values()
    }

    /// Iterates over all registered script bundles by name
    pub fn iter_bundles(&self) -> impl Iterator<Item = (&String, &ScriptBundleRegistration)> {
        self.bundles.iter()
    }

    /// Registers a script event, possibly overwriting an existing one
    pub fn register_event(&mut self, registration: ScriptEventRegistration) {
        self.events
            .insert(registration.name.to_string(), registration);
    }

    /// Gets a script event by name
    pub fn get_event(&self, name: &str) -> Option<&ScriptEventRegistration> {
        self.events.get(name)
    }

    /// Gets a dynamic script bundle by name
    pub fn get_bundle(&self, name: &str) -> Option<&ScriptBundleRegistration> {
        self.bundles.get(name)
    }
}

/// A plugin to support dynamic script components
pub struct DynamicScriptComponentPlugin;

impl Plugin for DynamicScriptComponentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AppScriptComponentRegistry>()
            .register_type::<DynamicComponent>()
            .register_type::<DynamicBundle>();
    }
}

#[cfg(test)]
mod test {
    use bevy_ecs::world::World;
    use bevy_mod_scripting_world::{WorldAccessGuard, WorldGuard};

    use crate::{CurrentScriptAttachment, WorldExtensions};

    use super::*;
    use bevy_reflect::structs::Struct;

    #[test]
    fn dynamic_data_exposes_table_entries_as_fields() {
        let mut component = DynamicComponent::new(ScriptValue::Map(
            [("hp".to_owned(), ScriptValue::Integer(5))]
                .into_iter()
                .collect(),
        ));

        // entries are fields, `data` stays a field
        let hp = component.field("hp").unwrap();
        assert_eq!(
            hp.try_downcast_ref::<ScriptValue>(),
            Some(&ScriptValue::Integer(5))
        );
        assert!(component.field("data").is_some());
        // a missing entry reads as nil
        assert_eq!(
            component
                .field("mana")
                .unwrap()
                .try_downcast_ref::<ScriptValue>(),
            Some(&ScriptValue::Unit)
        );
        // reflection only knows about `data` as a declared field
        assert_eq!(component.field_len(), 1);

        // writing creates entries
        component
            .field_mut("mana")
            .unwrap()
            .try_apply(&ScriptValue::Integer(3))
            .unwrap();
        let ScriptValue::Map(entries) = component.data() else {
            panic!("data should still be a table");
        };
        assert_eq!(entries.get("mana"), Some(&ScriptValue::Integer(3)));
    }

    #[test]
    fn dynamic_data_fields_turn_nil_data_into_a_table_but_not_other_values() {
        let mut component = DynamicComponent::default();
        assert!(component.field("anything").is_some());
        component
            .field_mut("a")
            .unwrap()
            .try_apply(&ScriptValue::Bool(true))
            .unwrap();
        assert!(matches!(component.data(), ScriptValue::Map(_)));

        let mut component = DynamicComponent::new(ScriptValue::Integer(1));
        assert!(component.field("a").is_none());
        assert!(component.field_mut("a").is_none());
        // the data itself is still reachable
        assert!(component.field_mut("data").is_some());
    }

    #[test]
    fn test_script_component() {
        let mut world = World::new();
        world.init_resource::<AppScriptComponentRegistry>();
        let cache = WorldGuard::setup_cache(&world, CurrentScriptAttachment::empty());
        let registration = {
            let guard = WorldAccessGuard::new_exclusive(&mut world, cache);

            guard
                .register_script_component("ScriptTest".to_string(), false, vec![])
                .unwrap()
        };

        let registry = world.get_resource::<AppScriptComponentRegistry>().unwrap();

        let registry = registry.read();
        let info = registry.get("ScriptTest").unwrap();
        assert_eq!(info.registration.component_id, registration.component_id);
        assert_eq!(info.name, "ScriptTest");

        // can get the component through the world
        let component = world
            .components()
            .get_info(info.registration.component_id)
            .unwrap();

        assert_eq!(component.name(), "ScriptTest".into());
    }
}
