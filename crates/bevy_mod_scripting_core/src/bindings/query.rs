//! Utilities for querying the world.

use super::{ReflectReference, WorldAccessGuard};
use crate::{error::InteropError, with_global_access};
use bevy::{
    ecs::{component::ComponentId, entity::Entity},
    prelude::{EntityRef, QueryBuilder},
    reflect::{ParsedPath, Reflect, TypeRegistration},
};
use std::{any::TypeId, collections::VecDeque, sync::Arc};

/// A wrapper around a `TypeRegistration` that provides additional information about the type.
///
/// This is used as a hook to a rust type from a scripting language. We should be able to easily convert between a type name and a [`ScriptTypeRegistration`].
#[derive(Clone, Reflect)]
#[reflect(opaque)]
pub struct ScriptTypeRegistration {
    pub(crate) registration: Arc<TypeRegistration>,
}

#[derive(Clone, Reflect, Debug)]
/// A registration for a component type.
pub struct ScriptComponentRegistration {
    pub(crate) registration: ScriptTypeRegistration,
    pub(crate) component_id: ComponentId,
}

#[derive(Clone, Reflect, Debug)]
/// A registration for a resource type.
pub struct ScriptResourceRegistration {
    pub(crate) registration: ScriptTypeRegistration,
    pub(crate) resource_id: ComponentId,
}

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

impl ScriptComponentRegistration {
    /// Creates a new [`ScriptComponentRegistration`] from a [`ScriptTypeRegistration`] and a [`ComponentId`].
    pub fn new(registration: ScriptTypeRegistration, component_id: ComponentId) -> Self {
        Self {
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
/// A builder for a query.
pub struct ScriptQueryBuilder {
    components: Vec<ScriptComponentRegistration>,
    with: Vec<ScriptComponentRegistration>,
    without: Vec<ScriptComponentRegistration>,
}

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
        with_global_access!(self.inner.accesses, "Could not query", {
            let world = unsafe { self.as_unsafe_world_cell()?.world_mut() };
            let mut dynamic_query = QueryBuilder::<EntityRef>::new(world);

            // we don't actually want to fetch the data for components now, only figure out
            // which entities match the query
            // so we might be being slightly overkill
            for c in &query.components {
                dynamic_query.ref_id(c.component_id());
            }

            for w in query.with {
                dynamic_query.with_id(w.component_id());
            }

            for without_id in query.without {
                dynamic_query.without_id(without_id.component_id());
            }

            let mut built_query = dynamic_query.build();
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
