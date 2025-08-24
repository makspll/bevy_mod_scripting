//! Utilities for querying the world.

use bevy_ecs::{ptr::OwningPtr, query::QueryBuilder, world::EntityRef};

use super::{DynamicComponent, ReflectReference, WorldAccessGuard, WorldGuard, with_global_access};
use crate::error::InteropError;
use ::{
    bevy_ecs::{
        component::ComponentId,
        entity::Entity,
        query::{QueryData, QueryState},
        reflect::ReflectComponent,
        world::World,
    },
    bevy_reflect::{ParsedPath, Reflect, TypeRegistration},
};
use std::{any::TypeId, collections::VecDeque, ptr::NonNull, sync::Arc};

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
    pub(crate) registration: ScriptTypeRegistration,
    pub(crate) component_id: ComponentId,
    /// whether this is a component registered BY a script
    pub(crate) is_dynamic_script_component: bool,
}

#[derive(Clone, Reflect, Debug)]
/// A reference to a resource type's reflection registration.
///
/// In general think of this as a handle to a type.
pub struct ScriptResourceRegistration {
    pub(crate) registration: ScriptTypeRegistration,
    pub(crate) resource_id: ComponentId,
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
        world.with_global_access(|world| {
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

            world.with_global_access(|world| {
                let mut entity = world
                    .get_entity_mut(entity)
                    .map_err(|_| InteropError::missing_entity(entity))?;
                let cast = instance.downcast::<DynamicComponent>().map_err(|v| {
                    InteropError::type_mismatch(TypeId::of::<DynamicComponent>(), Some(v.type_id()))
                })?;
                // the reason we leak the box, is because we don't want to double drop the owning ptr

                let ptr = (Box::leak(cast) as *mut DynamicComponent).cast();
                // Safety: cannot be null as we just created it from a valid reference
                let non_null_ptr = unsafe { NonNull::new_unchecked(ptr) };
                // Safety:
                // - we know the type is ScriptComponent, as we just created the pointer
                // - the box will stay valid for the life of this function, and we do not return the ptr
                // - pointer is alligned correctly
                // - nothing else will call drop on this
                let owning_ptr = unsafe { OwningPtr::new(non_null_ptr) };
                // Safety:
                // - Owning Ptr is valid as we just created it
                // - TODO: do we need to check if ComponentId is from this world? How?
                unsafe { entity.insert_by_id(self.component_id, owning_ptr) };
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
            world.with_global_access(|world| {
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
    pub(crate) components: Vec<ScriptComponentRegistration>,
    with: Vec<ScriptComponentRegistration>,
    without: Vec<ScriptComponentRegistration>,
}

#[profiling::all_functions]
impl ScriptQueryBuilder {
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

#[profiling::all_functions]
impl WorldAccessGuard<'_> {
    /// Queries the world for entities that match the given query.
    pub fn query(
        &self,
        query: ScriptQueryBuilder,
    ) -> Result<VecDeque<ScriptQueryResult>, InteropError> {
        with_global_access!(&self.inner.accesses, "Could not query", {
            let world = unsafe { self.as_unsafe_world_cell()?.world_mut() };
            let mut built_query = query.as_query_state::<EntityRef>(world);
            let query_result = built_query.iter(world);

            Ok(query_result
                .map(|r| {
                    let references: Vec<_> = query
                        .components
                        .iter()
                        .map(|c| ReflectReference {
                            base: super::ReflectBaseType {
                                type_id: c.type_registration().type_id(),
                                base_id: super::ReflectBase::Component(r.id(), c.component_id()),
                            },
                            reflect_path: ParsedPath(vec![]),
                        })
                        .collect();
                    ScriptQueryResult {
                        entity: r.id(),
                        components: references,
                    }
                })
                .collect())
        })?
    }
}
