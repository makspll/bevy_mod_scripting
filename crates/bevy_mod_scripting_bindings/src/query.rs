//! Utilities for querying the world.

use bevy_ecs::{ptr::OwningPtr, query::QueryBuilder};
use bevy_mod_scripting_world::WorldGuard;

use super::{DynamicBundle, DynamicComponent, ReflectReference};
use crate::{
    PartialReflectExt, ReflectBase, ReflectBaseType, ScriptValue, WorldExtensions,
    error::InteropError,
    function::{
        from::{Union, V},
        from_ref::FromScriptRef,
    },
    world_extensions::default_instance,
};
use ::{
    bevy_ecs::{
        component::ComponentId,
        entity::Entity,
        query::{QueryData, QueryState},
        reflect::{ReflectBundle, ReflectComponent, ReflectFromWorld},
        world::World,
    },
    bevy_reflect::{
        PartialReflect, Reflect, ReflectFromReflect, TypeRegistration, std_traits::ReflectDefault,
    },
};
use bevy_platform::collections::HashMap;
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

/// A component which is part of a bundle, see [`ScriptBundleRegistration`].
#[derive(Clone, Debug)]
pub struct BundleMember {
    /// The name of the member, the short type name for native components or the registered name for script components.
    ///
    /// This is the key under which the member's value is stored in a [`DynamicBundle`].
    pub name: String,
    /// The registration of the member component
    pub registration: ScriptComponentRegistration,
}

#[derive(Clone, Reflect, Debug)]
#[reflect(opaque)]
/// A reference to a bundle type's reflection registration.
///
/// In general think of this as a handle to a type.
///
/// Native bundles are types with `ReflectBundle` type data, script registered bundles are backed by [`DynamicBundle`].
pub struct ScriptBundleRegistration {
    /// The type registration of the bundle
    pub registration: ScriptTypeRegistration,
    /// The components making up the bundle, nested bundles are flattened
    pub members: Arc<[BundleMember]>,
    /// whether this is a bundle registered BY a script
    pub is_dynamic_script_bundle: bool,
    /// the script namespace the bundle was registered in, if registered by a script
    pub script_namespace: Option<String>,
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
    ///
    /// Bundles registered by scripts are in the namespace they were registered in, not in the namespace of the type backing them.
    pub fn namespace(&self) -> &str {
        match (&self.script_namespace, self.is_dynamic_script_bundle) {
            (Some(namespace), _) => namespace,
            (None, true) => "",
            (None, false) => self.registration.namespace(),
        }
    }

    /// Creates a new [`ScriptBundleRegistration`] from a [`ScriptTypeRegistration`] and the bundle's members.
    pub fn new(registration: ScriptTypeRegistration, members: Vec<BundleMember>) -> Self {
        Self {
            is_dynamic_script_bundle: registration.type_id() == TypeId::of::<DynamicBundle>(),
            script_namespace: None,
            registration,
            members: members.into(),
        }
    }

    /// Returns the components making up this bundle.
    pub fn members(&self) -> &[BundleMember] {
        &self.members
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
        let instance = <Box<dyn PartialReflect>>::from_script_ref(type_id, value, world.clone())?;
        let instance = match instance.try_into_reflect() {
            Ok(instance) if instance.as_any().type_id() == type_id => instance,
            Ok(instance) => {
                <dyn PartialReflect>::from_reflect(instance.as_partial_reflect(), world)?
            }
            Err(instance) => <dyn PartialReflect>::from_reflect(instance.as_ref(), world)?,
        };
        Ok(instance)
    }

    /// Default constructs a value which can be inserted as this bundle.
    pub fn default_instance(&self, world: WorldGuard) -> Result<Box<dyn Reflect>, InteropError> {
        default_instance(world, self.registration.type_registration())
    }

    /// Inserts an instance of this bundle into the given entity.
    ///
    /// Native bundles are inserted all at once, members of script bundles are inserted one by one,
    /// members without a value in the [`DynamicBundle`] are default constructed.
    ///
    /// Requires whole world access
    pub fn insert_into_entity(
        &self,
        world: WorldGuard,
        entity: Entity,
        instance: Box<dyn Reflect>,
    ) -> Result<(), InteropError> {
        if self.is_dynamic_script_bundle {
            let bundle = instance.downcast::<DynamicBundle>().map_err(|v| {
                InteropError::type_mismatch(TypeId::of::<DynamicBundle>(), Some(v.type_id()))
            })?;
            let mut values = match bundle.into_data() {
                ScriptValue::Map(values) => values,
                ScriptValue::Unit => Default::default(),
                other => {
                    return Err(InteropError::value_mismatch(
                        TypeId::of::<DynamicBundle>(),
                        other,
                    ));
                }
            };

            // convert all members before inserting anything, so a bad value does not leave a partial bundle behind
            let instances = self
                .members
                .iter()
                .map(|member| {
                    let instance = member.instance(world.clone(), values.remove(&member.name))?;
                    Ok((member, instance))
                })
                .collect::<Result<Vec<_>, InteropError>>()?;
            if !values.is_empty() {
                let mut unknown = values.into_keys().collect::<Vec<_>>();
                unknown.sort_unstable();
                return Err(InteropError::unsupported_operation(
                    Some(TypeId::of::<DynamicBundle>()),
                    None,
                    format!(
                        "inserting bundle with values for unknown members: {}",
                        unknown.join(", ")
                    ),
                ));
            }

            if !world.is_valid_entity(entity)? {
                return Err(InteropError::missing_entity(entity));
            }
            for (member, instance) in instances {
                member
                    .registration
                    .insert_into_entity(world.clone(), entity, instance)?;
            }
            Ok(())
        } else {
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

    /// Removes the components of this bundle from the given entity, components which are not present are ignored.
    pub fn remove_from_entity(
        &self,
        world: WorldGuard,
        entity: Entity,
    ) -> Result<(), InteropError> {
        let reflect_bundle = if self.is_dynamic_script_bundle {
            None
        } else {
            Some(self.reflect_bundle()?)
        };
        world.with_world_mut(|world| {
            let mut entity = world
                .get_entity_mut(entity)
                .map_err(|_| InteropError::missing_entity(entity))?;
            match reflect_bundle {
                Some(reflect_bundle) => {
                    reflect_bundle.remove(&mut entity);
                }
                None => {
                    let ids = self
                        .members
                        .iter()
                        .map(|m| m.registration.component_id)
                        .collect::<Vec<_>>();
                    entity.remove_by_ids(&ids);
                }
            }
            Ok(())
        })?
    }

    /// Checks if the given entity has all the components of this bundle.
    pub fn is_in_entity(&self, world: WorldGuard, entity: Entity) -> Result<bool, InteropError> {
        let cell = world.as_unsafe_world_cell()?;
        let entity = cell
            .get_entity(entity)
            .map_err(|_| InteropError::missing_entity(entity))?;
        Ok(self
            .members
            .iter()
            .all(|m| entity.contains_id(m.registration.component_id)))
    }

    /// Removes the components of this bundle from the entity and returns them as an instance of the bundle.
    ///
    /// If the entity does not have all of the components, nothing is removed and `None` is returned.
    pub fn take_from_entity(
        &self,
        world: WorldGuard,
        entity: Entity,
    ) -> Result<Option<ReflectReference>, InteropError> {
        if !self.is_dynamic_script_bundle {
            let reflect_bundle = self.reflect_bundle()?;
            let taken = world.with_world_mut(|world| {
                let mut entity = world
                    .get_entity_mut(entity)
                    .map_err(|_| InteropError::missing_entity(entity))?;
                Ok::<_, InteropError>(reflect_bundle.take(&mut entity))
            })??;
            return Ok(taken.map(|bundle| <dyn PartialReflect>::allocate(bundle, world)));
        }

        if !self.is_in_entity(world.clone(), entity)? {
            return Ok(None);
        }
        let mut values = HashMap::default();
        for member in self.members.iter() {
            let reference = ReflectReference {
                base: ReflectBaseType {
                    type_id: member.registration.type_registration().type_id(),
                    base_id: ReflectBase::Component(entity, member.registration.component_id),
                },
                reflect_path: Default::default(),
            };
            let value = if member.registration.is_dynamic_script_component {
                reference
                    .with_reflect(world.clone(), |r| {
                        r.try_downcast_ref::<DynamicComponent>()
                            .map(|c| c.data().clone())
                    })?
                    .ok_or_else(|| {
                        InteropError::type_mismatch(TypeId::of::<DynamicComponent>(), None)
                    })?
            } else {
                let owned = reference.to_owned_value(world.clone())?;
                let allocator = world.allocator();
                let mut allocator = allocator.write();
                ScriptValue::Reference(ReflectReference::new_allocated_boxed_parial_reflect(
                    owned,
                    &mut allocator,
                )?)
            };
            values.insert(member.name.clone(), value);
        }
        self.remove_from_entity(world.clone(), entity)?;
        let bundle = DynamicBundle::new(ScriptValue::Map(values));
        let allocator = world.allocator();
        let mut allocator = allocator.write();
        Ok(Some(ReflectReference::new_allocated(
            bundle,
            &mut allocator,
        )))
    }
}

#[profiling::all_functions]
impl BundleMember {
    /// Converts a script provided member value, or a default if none is provided, into an instance of the member component.
    fn instance(
        &self,
        world: WorldGuard,
        value: Option<ScriptValue>,
    ) -> Result<Box<dyn Reflect>, InteropError> {
        let registration = self.registration.type_registration();
        let Some(value) = value else {
            return default_instance(world, registration.type_registration());
        };

        if self.registration.is_dynamic_script_component {
            // script components accept their data directly, or an existing `DynamicComponent`
            let is_component = match &value {
                ScriptValue::Reference(r) => {
                    r.tail_type_id(world.clone())? == Some(TypeId::of::<DynamicComponent>())
                }
                _ => false,
            };
            if !is_component {
                return Ok(Box::new(DynamicComponent::new(value)));
            }
        }

        let instance =
            <Box<dyn PartialReflect>>::from_script_ref(registration.type_id(), value, world)?;
        instance.try_into_reflect().map_err(|v| {
            InteropError::failed_from_reflect(
                Some(registration.type_id()),
                format!("bundle member value is not a full reflect type: {v:?}"),
            )
        })
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
    /// Components that must be present.
    pub with: Vec<ScriptComponentRegistration>,
    /// Components that must not be present.
    pub without: Vec<ScriptComponentRegistration>,
}

#[profiling::all_functions]
impl ScriptQueryBuilder {
    /// Creates an empty query builder
    pub fn new() -> Self {
        Default::default()
    }

    /// Adds components to the query.
    pub fn components(&mut self, components: Vec<ScriptComponentRegistration>) -> &mut Self {
        self.components.extend(components);
        self
    }
    /// Adds a component to the query.
    pub fn component(&mut self, component: ScriptComponentRegistration) -> &mut Self {
        self.components.push(component);
        self
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

    /// Builds the query into a query state as used in systems.
    pub fn as_query_state<Q: QueryData>(&self, world: &mut World) -> QueryState<Q> {
        let mut dynamic_query = QueryBuilder::<Q>::new(world);
        // we don't actually want to fetch the data for components now, only figure out
        // which entities match the query
        // so we might be being slightly overkill
        for c in &self.components {
            dynamic_query.ref_id(c.component_id());
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
    /// The components that matched the query.
    pub components: Vec<ReflectReference>,
}
