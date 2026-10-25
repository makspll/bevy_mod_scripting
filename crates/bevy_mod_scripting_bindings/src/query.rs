//! Utilities for querying the world.

use bevy_ecs::{ptr::OwningPtr, query::QueryBuilder};
use bevy_mod_scripting_world::WorldGuard;

use super::{DynamicComponent, ReflectReference};
use crate::{
    PartialReflectExt, ScriptValue, WorldExtensions,
    error::InteropError,
    function::{
        from::{Union, V},
        from_ref::FromScriptErased,
    },
};
use ::{
    bevy_ecs::{
        change_detection::Tick,
        component::ComponentId,
        entity::Entity,
        query::{QueryData, QueryState},
        reflect::{ReflectBundle, ReflectComponent, ReflectFromWorld},
        world::{World, unsafe_world_cell::UnsafeEntityCell},
    },
    bevy_reflect::{
        PartialReflect, Reflect, ReflectFromReflect, TypeRegistration, std_traits::ReflectDefault,
    },
};
use std::{any::TypeId, sync::Arc};

/// A reference to a type which is not a `Resource` or `Component`.
///
/// In general think of this as a handle to a type.
#[derive(Clone, Reflect)]
#[reflect(opaque)]
pub struct ScriptTypeRegistration {
    pub(crate) registration: Arc<TypeRegistration>,
}

#[derive(Clone, Reflect, Debug)]
/// A reference to a component type's reflection registration.
///
/// In general think of this as a handle to a type.
///
/// Not to be confused with script registered dynamic components, although this can point to a script registered component.
pub struct ScriptComponentRegistration {
    /// The type registration of the component
    pub registration: ScriptTypeRegistration,
    /// the component id of the component
    pub component_id: ComponentId,
    /// whether this is a component registered BY a script
    pub is_dynamic_script_component: bool,
    /// the script namespace the component was registered in, if registered by a script
    pub script_namespace: Option<String>,
}

#[derive(Clone, Reflect, Debug)]
/// A reference to a resource type's reflection registration.
///
/// In general think of this as a handle to a type.
pub struct ScriptResourceRegistration {
    /// the type registration of the resource
    pub registration: ScriptTypeRegistration,
    /// the component id of the resource
    pub resource_id: ComponentId,
}

#[derive(Clone, Reflect, Debug)]
#[reflect(opaque)]
/// A reference to a bundle type's reflection registration, i.e. a type with `ReflectBundle` type data.
///
/// In general think of this as a handle to a type.
pub struct ScriptBundleRegistration {
    /// The type registration of the bundle
    pub registration: ScriptTypeRegistration,
}

/// Any of the kinds of type registration, as returned by type lookups.
///
/// The kind decides what can be done with the type, i.e. only components can be added to entities.
pub type AnyTypeRegistration = Union<
    ScriptTypeRegistration,
    Union<ScriptComponentRegistration, Union<ScriptResourceRegistration, ScriptBundleRegistration>>,
>;

/// [`AnyTypeRegistration`] as it appears in script function signatures, i.e. as scripts receive and pass types around.
pub type AnyTypeRegistrationV = Union<
    V<ScriptTypeRegistration>,
    Union<
        V<ScriptComponentRegistration>,
        Union<V<ScriptResourceRegistration>, V<ScriptBundleRegistration>>,
    >,
>;

impl AnyTypeRegistration {
    /// Convert to the form used in script function signatures.
    pub fn into_script(self) -> AnyTypeRegistrationV {
        self.map_both(V, |other| other.map_both(V, |other| other.map_both(V, V)))
    }
}

impl AnyTypeRegistrationV {
    /// Convert to a generic [`ScriptTypeRegistration`] ditching any component, resource or bundle information.
    pub fn into_type_registration(self) -> ScriptTypeRegistration {
        match self.into_left() {
            Ok(registration) => registration.into_inner(),
            Err(other) => match other.into_left() {
                Ok(component) => component.into_inner().into_type_registration(),
                Err(other) => match other.into_left() {
                    Ok(resource) => resource.into_inner().into_type_registration(),
                    Err(bundle) => bundle.into_inner().into_type_registration(),
                },
            },
        }
    }
}

#[profiling::all_functions]
impl ScriptTypeRegistration {
    /// Creates a new [`ScriptTypeRegistration`] from a [`TypeRegistration`].
    pub fn new(registration: Arc<TypeRegistration>) -> Self {
        Self { registration }
    }

    #[inline(always)]
    /// Returns the short name of the type.
    pub fn short_name(&self) -> &'static str {
        self.registration.type_info().type_path_table().short_path()
    }

    #[inline(always)]
    /// Returns the full name of the type.
    pub fn type_name(&self) -> &'static str {
        self.registration.type_info().type_path_table().path()
    }

    #[inline(always)]
    /// Returns the [`TypeId`] of the type.
    pub fn type_id(&self) -> TypeId {
        self.registration.type_info().type_id()
    }

    /// Returns the script namespace of the type.
    ///
    /// This is the namespace set with [`ScriptNamespace`](crate::ScriptNamespace) type data if any,
    /// otherwise the name of the crate defining the type, or the empty string (the root namespace) for types without one, such as primitives.
    pub fn namespace(&self) -> &str {
        match self.registration.data::<crate::ScriptNamespace>() {
            Some(namespace) => namespace.0.as_ref(),
            None => self
                .registration
                .type_info()
                .type_path_table()
                .crate_name()
                .unwrap_or(""),
        }
    }

    /// Returns the [`TypeRegistration`] for this type.
    pub fn type_registration(&self) -> &TypeRegistration {
        &self.registration
    }
}

#[profiling::all_functions]
impl ScriptResourceRegistration {
    /// Creates a new [`ScriptResourceRegistration`] from a [`ScriptTypeRegistration`] and a [`ComponentId`].
    pub fn new(registration: ScriptTypeRegistration, resource_id: ComponentId) -> Self {
        Self {
            registration,
            resource_id,
        }
    }

    /// Returns the [`ComponentId`] for this type, if it is a resource.
    #[inline(always)]
    pub fn resource_id(&self) -> ComponentId {
        self.resource_id
    }

    /// Returns the [`ScriptTypeRegistration`] for this type.
    pub fn type_registration(&self) -> &ScriptTypeRegistration {
        &self.registration
    }

    /// Convert to a generic [`ScriptTypeRegistration`] ditching the resource information.
    pub fn into_type_registration(self) -> ScriptTypeRegistration {
        self.registration
    }
}

#[profiling::all_functions]
impl ScriptComponentRegistration {
    /// Creates a new [`ScriptComponentRegistration`] from a [`ScriptTypeRegistration`] and a [`ComponentId`].
    pub fn new(registration: ScriptTypeRegistration, component_id: ComponentId) -> Self {
        Self {
            is_dynamic_script_component: registration.type_id()
                == std::any::TypeId::of::<DynamicComponent>(),
            registration,
            component_id,
            script_namespace: None,
        }
    }

    /// Returns the script namespace of the component, see [`ScriptTypeRegistration::namespace`].
    ///
    /// Components registered by scripts are in the namespace they were registered in, not in the namespace of the type backing them.
    pub fn namespace(&self) -> &str {
        match (&self.script_namespace, self.is_dynamic_script_component) {
            (Some(namespace), _) => namespace,
            (None, true) => "",
            (None, false) => self.registration.namespace(),
        }
    }

    /// Returns the [`ComponentId`] for this type, if it is a component.
    #[inline(always)]
    pub fn component_id(&self) -> ComponentId {
        self.component_id
    }

    /// Returns the [`ScriptTypeRegistration`] for this type.
    pub fn type_registration(&self) -> &ScriptTypeRegistration {
        &self.registration
    }

    /// Convert to a generic [`ScriptTypeRegistration`] ditching the component information.
    pub fn into_type_registration(self) -> ScriptTypeRegistration {
        self.registration
    }

    /// Removes an instance of this component from the given entity
    pub fn remove_from_entity(
        &self,
        world: WorldGuard,
        entity: Entity,
    ) -> Result<(), InteropError> {
        world.with_world_mut(|world| {
            let mut entity = world
                .get_entity_mut(entity)
                .map_err(|_| InteropError::missing_entity(entity))?;
            entity.remove_by_id(self.component_id);
            Ok(())
        })?
    }

    /// Inserts an instance of this component into the given entity
    ///
    /// Requires whole world access
    pub fn insert_into_entity(
        &self,
        world: WorldGuard,
        entity: Entity,
        instance: Box<dyn Reflect>,
    ) -> Result<(), InteropError> {
        if self.is_dynamic_script_component {
            // if dynamic we already know the type i.e. `ScriptComponent`
            // so we can just insert it

            world.with_world_mut(|world| {
                let mut entity = world
                    .get_entity_mut(entity)
                    .map_err(|_| InteropError::missing_entity(entity))?;
                let cast = instance.downcast::<DynamicComponent>().map_err(|v| {
                    InteropError::type_mismatch(TypeId::of::<DynamicComponent>(), Some(v.type_id()))
                })?;
                // move the value out of the box so its allocation is freed, `OwningPtr::make` ensures
                // the value itself is not dropped here, as ownership passes to the world
                OwningPtr::make(*cast, |owning_ptr| {
                    // Safety:
                    // - the pointer points to a valid `DynamicComponent`, the type backing all script components
                    // - TODO: do we need to check if ComponentId is from this world? How?
                    unsafe { entity.insert_by_id(self.component_id, owning_ptr) };
                });
                Ok(())
            })?
        } else {
            let component_data = self
                .type_registration()
                .type_registration()
                .data::<ReflectComponent>()
                .ok_or_else(|| {
                    InteropError::missing_type_data(
                        self.registration.type_id(),
                        "ReflectComponent".to_owned(),
                    )
                })?;

            //  TODO: this shouldn't need entire world access it feels
            let type_registry = world.type_registry();
            world.with_world_mut(|world| {
                let mut entity = world
                    .get_entity_mut(entity)
                    .map_err(|_| InteropError::missing_entity(entity))?;
                {
                    let registry = type_registry.read();
                    component_data.insert(&mut entity, instance.as_partial_reflect(), &registry);
                }
                Ok(())
            })?
        }
    }
}

#[profiling::all_functions]
impl ScriptBundleRegistration {
    /// Returns the script namespace of the bundle, see [`ScriptTypeRegistration::namespace`].
    pub fn namespace(&self) -> &str {
        self.registration.namespace()
    }

    /// Creates a new [`ScriptBundleRegistration`] from a [`ScriptTypeRegistration`].
    pub fn new(registration: ScriptTypeRegistration) -> Self {
        Self { registration }
    }

    /// Returns the [`ScriptTypeRegistration`] for this type.
    pub fn type_registration(&self) -> &ScriptTypeRegistration {
        &self.registration
    }

    /// Convert to a generic [`ScriptTypeRegistration`] ditching the bundle information.
    pub fn into_type_registration(self) -> ScriptTypeRegistration {
        self.registration
    }

    fn reflect_bundle(&self) -> Result<&ReflectBundle, InteropError> {
        self.registration
            .type_registration()
            .data::<ReflectBundle>()
            .ok_or_else(|| {
                InteropError::missing_type_data(
                    self.registration.type_id(),
                    "ReflectBundle".to_owned(),
                )
            })
    }

    /// Converts a script provided value into a value which can be inserted as this bundle.
    pub fn instance_from_script(
        &self,
        world: WorldGuard,
        value: ScriptValue,
    ) -> Result<Box<dyn Reflect>, InteropError> {
        let type_id = self.registration.type_id();
        let instance =
            <Box<dyn PartialReflect>>::from_script_erased(type_id, value, world.clone())?;
        let instance = match instance.try_into_reflect() {
            Ok(instance) if instance.as_any().type_id() == type_id => instance,
            Ok(instance) => {
                <dyn PartialReflect>::from_reflect(instance.as_partial_reflect(), world)?
            }
            Err(instance) => <dyn PartialReflect>::from_reflect(instance.as_ref(), world)?,
        };
        Ok(instance)
    }

    /// Inserts an instance of this bundle into the given entity, all components at once.
    ///
    /// Requires whole world access
    pub fn insert_into_entity(
        &self,
        world: WorldGuard,
        entity: Entity,
        instance: Box<dyn Reflect>,
    ) -> Result<(), InteropError> {
        let reflect_bundle = self.reflect_bundle()?;
        // `ReflectBundle::insert` panics if it cannot construct the bundle from the value
        let type_registration = self.registration.type_registration();
        if instance.as_any().type_id() != self.registration.type_id() {
            return Err(InteropError::type_mismatch(
                self.registration.type_id(),
                Some(instance.as_any().type_id()),
            ));
        }
        if type_registration.data::<ReflectFromReflect>().is_none()
            && type_registration.data::<ReflectDefault>().is_none()
            && type_registration.data::<ReflectFromWorld>().is_none()
        {
            return Err(InteropError::missing_type_data(
                self.registration.type_id(),
                "ReflectFromReflect, ReflectDefault or ReflectFromWorld".to_owned(),
            ));
        }

        let type_registry = world.type_registry();
        world.with_world_mut(|world| {
            let mut entity = world
                .get_entity_mut(entity)
                .map_err(|_| InteropError::missing_entity(entity))?;
            let registry = type_registry.read();
            reflect_bundle.insert(&mut entity, instance.as_partial_reflect(), &registry);
            Ok(())
        })?
    }
}

impl std::fmt::Debug for ScriptTypeRegistration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ScriptTypeRegistration")
            .field(&self.registration.type_info().type_path())
            .finish()
    }
}

impl std::fmt::Display for ScriptTypeRegistration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.registration.type_info().type_path())
    }
}

/// How [`ScriptQueryBuilder::as_query_state`] declares the accesses of the query.
///
/// Read and written components match the same entities, the difference is only in the access the query reports.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QueryAccess {
    /// Every component is declared as read, for finding the matching entities, i.e. with
    /// [`bevy_ecs::query::QueryState::iter`] on a `&World`, while the actual component accesses are claimed through
    /// the world guard.
    ///
    /// The query state must not be used to write components, or have its access reported to the scheduler.
    MatchOnly,
    /// The written components are declared as written, for queries whose access is reported to the scheduler,
    /// i.e. the queries of script systems.
    Declared,
}

#[derive(Clone, Default, Reflect)]
#[reflect(opaque)]
/// The query builder is used to build ECS queries which retrieve spefific components filtered by specific conditions.
///
/// For example:
/// ```rust,ignore
/// builder.component(componentA)
///     .component(componentB)
///     .with(componentC)
///     .without(componentD)  
/// ```
///
/// Will retrieve entities which:
/// - Have componentA
/// - Have componentB
/// - Have componentC
/// - Do not have componentD
///
/// As well as references to components:
/// - componentA
/// - componentB
pub struct ScriptQueryBuilder {
    /// The components to query for.
    pub components: Vec<ScriptComponentRegistration>,
    /// Whether each of the components is written, components without an entry are written.
    writes: Vec<bool>,
    /// Whether each of the components is optional, components without an entry are required.
    optionals: Vec<bool>,
    /// Components that must be present.
    pub with: Vec<ScriptComponentRegistration>,
    /// Components that must not be present.
    pub without: Vec<ScriptComponentRegistration>,
    /// Components that must have been added since the last run (of the system, or of the app's update outside systems).
    pub added: Vec<ScriptComponentRegistration>,
    /// Components that must have been changed since the last run (of the system, or of the app's update outside systems).
    pub changed: Vec<ScriptComponentRegistration>,
}

#[profiling::all_functions]
impl ScriptQueryBuilder {
    /// Creates an empty query builder
    pub fn new() -> Self {
        Default::default()
    }

    /// Adds components to the query, which can be read and written.
    pub fn components(&mut self, components: Vec<ScriptComponentRegistration>) -> &mut Self {
        for component in components {
            self.component(component);
        }
        self
    }
    /// Adds a component to the query, which can be read and written.
    pub fn component(&mut self, component: ScriptComponentRegistration) -> &mut Self {
        self.push_component(component, true, false)
    }

    /// Adds a component to the query, which can only be read.
    ///
    /// Systems reading a component can run in parallel with each other.
    pub fn component_read(&mut self, component: ScriptComponentRegistration) -> &mut Self {
        self.push_component(component, false, false)
    }

    /// Adds a component to the query which entities don't need to have, which can be read and written.
    pub fn optional(&mut self, component: ScriptComponentRegistration) -> &mut Self {
        self.push_component(component, true, true)
    }

    /// Adds a component to the query which entities don't need to have, which can only be read.
    pub fn optional_read(&mut self, component: ScriptComponentRegistration) -> &mut Self {
        self.push_component(component, false, true)
    }

    fn push_component(
        &mut self,
        component: ScriptComponentRegistration,
        write: bool,
        optional: bool,
    ) -> &mut Self {
        // components pushed to the public field directly are written and required
        self.writes.resize(self.components.len(), true);
        self.optionals.resize(self.components.len(), false);
        self.components.push(component);
        self.writes.push(write);
        self.optionals.push(optional);
        self
    }

    /// Returns true if the component at the given index of [`Self::components`] is written
    pub fn is_written(&self, index: usize) -> bool {
        self.writes.get(index).copied().unwrap_or(true)
    }

    /// Returns true if the component at the given index of [`Self::components`] is optional
    pub fn is_optional(&self, index: usize) -> bool {
        self.optionals.get(index).copied().unwrap_or(false)
    }

    /// Only matches entities whose component was added since the last run, see [`Self::added`]
    pub fn added_component(&mut self, component: ScriptComponentRegistration) -> &mut Self {
        self.added.push(component);
        self
    }

    /// Only matches entities whose component was changed since the last run, see [`Self::changed`]
    pub fn changed_component(&mut self, component: ScriptComponentRegistration) -> &mut Self {
        self.changed.push(component);
        self
    }

    /// The components the query accesses, and whether it writes them:
    /// the retrieved ones, and the ones whose changes it checks (read)
    pub fn accessed_components(&self) -> impl Iterator<Item = (ComponentId, bool)> + '_ {
        self.components
            .iter()
            .enumerate()
            .map(|(index, c)| (c.component_id(), self.is_written(index)))
            .chain(
                self.added
                    .iter()
                    .chain(&self.changed)
                    .map(|c| (c.component_id(), false)),
            )
    }

    /// Builds the result for an entity matched by the query state from [`Self::as_query_state`],
    /// or returns `None` if it doesn't pass the [`Self::added`] and [`Self::changed`] filters, which query states can't express.
    ///
    /// Absent optional components are `None`.
    ///
    /// # Safety
    /// The caller must be allowed to read the change ticks of the components in the [`Self::added`] and [`Self::changed`] filters
    pub unsafe fn result_for(
        &self,
        entity: UnsafeEntityCell,
        last_run: Tick,
        this_run: Tick,
    ) -> Option<ScriptQueryResult> {
        // Safety: the caller can read the ticks
        let ticks = |c: &ScriptComponentRegistration| unsafe {
            entity.get_change_ticks_by_id(c.component_id())
        };
        let passes = self
            .added
            .iter()
            .all(|c| ticks(c).is_some_and(|t| t.is_added(last_run, this_run)))
            && self
                .changed
                .iter()
                .all(|c| ticks(c).is_some_and(|t| t.is_changed(last_run, this_run)));
        if !passes {
            return None;
        }

        let components = self
            .components
            .iter()
            .enumerate()
            .map(|(index, c)| {
                (!self.is_optional(index) || entity.contains_id(c.component_id())).then(|| {
                    ReflectReference::new_component_ref_by_id(
                        entity.id(),
                        c.component_id(),
                        c.type_registration().type_id(),
                    )
                })
            })
            .collect();
        Some(ScriptQueryResult {
            entity: entity.id(),
            components,
        })
    }

    /// Adds components to the query that must be present.
    pub fn with_components(&mut self, with: Vec<ScriptComponentRegistration>) -> &mut Self {
        self.with.extend(with);
        self
    }

    /// Adds a component to the query that must be present.
    pub fn with_component(&mut self, with: ScriptComponentRegistration) -> &mut Self {
        self.with.push(with);
        self
    }

    /// Adds components to the query that must not be present.
    pub fn without_components(&mut self, without: Vec<ScriptComponentRegistration>) -> &mut Self {
        self.without.extend(without);
        self
    }

    /// Adds a component to the query that must not be present.
    pub fn without_component(&mut self, without: ScriptComponentRegistration) -> &mut Self {
        self.without.push(without);
        self
    }

    /// Builds the query into a query state, declaring its component accesses as given by `access`.
    pub fn as_query_state<Q: QueryData>(
        &self,
        world: &mut World,
        access: QueryAccess,
    ) -> QueryState<Q> {
        let mut dynamic_query = QueryBuilder::<Q>::new(world);
        // we don't actually want to fetch the data for components now, only figure out
        // which entities match the query, and declare the access
        for (index, c) in self.components.iter().enumerate() {
            let write = access == QueryAccess::Declared && self.is_written(index);
            let id = c.component_id();
            if self.is_optional(index) {
                dynamic_query.optional(|optional| {
                    if write {
                        optional.mut_id(id);
                    } else {
                        optional.ref_id(id);
                    }
                });
            } else if write {
                dynamic_query.mut_id(id);
            } else {
                dynamic_query.ref_id(id);
            }
        }

        // change filters read the components' ticks, and imply the components are present
        for c in self.added.iter().chain(&self.changed) {
            let id = c.component_id();
            dynamic_query.with_id(id);
            let retrieved = self
                .components
                .iter()
                .enumerate()
                .any(|(index, other)| other.component_id() == id && !self.is_optional(index));
            if !retrieved {
                dynamic_query.ref_id(id);
            }
        }

        for w in &self.with {
            dynamic_query.with_id(w.component_id());
        }

        for without_id in &self.without {
            dynamic_query.without_id(without_id.component_id());
        }

        dynamic_query.build()
    }
}

#[derive(Clone, Reflect)]
#[reflect(opaque)]
/// A result from a query.
pub struct ScriptQueryResult {
    /// The entity that matched the query.
    pub entity: Entity,
    /// The components that matched the query, `None` for absent optional components.
    pub components: Vec<Option<ReflectReference>>,
}
