use super::{ReflectReference, WorldAccessGuard, WorldCallbackAccess};
use crate::{
    bindings::{CONCURRENT_WORLD_ACCESS_MSG, STALE_WORLD_MSG},
    error::InteropError,
    with_global_access,
};
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
    pub component_id: Option<ComponentId>,
    pub resource_id: Option<ComponentId>,
}

impl ScriptTypeRegistration {
    pub fn new(
        registration: Arc<TypeRegistration>,
        component_id: Option<ComponentId>,
        resource_id: Option<ComponentId>,
    ) -> Self {
        Self {
            registration,
            component_id,
            resource_id,
        }
    }

    #[inline(always)]
    pub fn short_name(&self) -> &'static str {
        self.registration.type_info().type_path_table().short_path()
    }

    #[inline(always)]
    pub fn type_name(&self) -> &'static str {
        self.registration.type_info().type_path_table().path()
    }

    #[inline(always)]
    pub fn type_id(&self) -> TypeId {
        self.registration.type_info().type_id()
    }

    /// Returns the [`ComponentId`] for this type, if it is a component.
    #[inline(always)]
    pub fn component_id(&self) -> Option<ComponentId> {
        self.component_id
    }

    /// Returns the [`ComponentId`] for this type, if it is a resource.
    #[inline(always)]
    pub fn resource_id(&self) -> Option<ComponentId> {
        self.resource_id
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
pub struct ScriptQueryBuilder {
    components: Vec<ScriptTypeRegistration>,
    with: Vec<ScriptTypeRegistration>,
    without: Vec<ScriptTypeRegistration>,
}

impl ScriptQueryBuilder {
    pub fn components(&mut self, components: Vec<ScriptTypeRegistration>) -> &mut Self {
        self.components.extend(components);
        self
    }

    pub fn with(&mut self, with: Vec<ScriptTypeRegistration>) -> &mut Self {
        self.with.extend(with);
        self
    }

    pub fn without(&mut self, without: Vec<ScriptTypeRegistration>) -> &mut Self {
        self.without.extend(without);
        self
    }
}

#[derive(Clone, Reflect)]
#[reflect(opaque)]
pub struct ScriptQueryResult {
    pub entity: Entity,
    pub components: Vec<ReflectReference>,
}

impl WorldCallbackAccess {
    pub fn query(
        &self,
        query: ScriptQueryBuilder,
    ) -> Result<VecDeque<ScriptQueryResult>, InteropError> {
        // find the set of components
        self.try_read().and_then(|world| world.query(query))
    }
}

impl<'w> WorldAccessGuard<'w> {
    pub fn query(
        &self,
        query: ScriptQueryBuilder,
    ) -> Result<VecDeque<ScriptQueryResult>, InteropError> {
        with_global_access!(self.0.accesses, "Could not query", {
            let world = unsafe { self.as_unsafe_world_cell().world_mut() };
            let mut dynamic_query = QueryBuilder::<EntityRef>::new(world);

            // we don't actually want to fetch the data for components now, only figure out
            // which entities match the query
            // so we might be being slightly overkill
            for c in &query.components {
                dynamic_query.ref_id(c.component_id().ok_or_else(|| {
                    InteropError::unsupported_operation(
                        Some(c.type_id()),
                        None,
                        "query for component on non-component type".to_owned(),
                    )
                })?);
            }

            for w in query.with {
                dynamic_query.with_id(w.component_id.ok_or_else(|| {
                    InteropError::unsupported_operation(
                        Some(w.type_id()),
                        None,
                        "query for entity with component which is non-component type".to_owned(),
                    )
                })?);
            }

            for without_id in query.without {
                dynamic_query.without_id(without_id.component_id.ok_or_else(|| {
                    InteropError::unsupported_operation(
                        Some(without_id.type_id()),
                        None,
                        "query for entity without component which is non-component type".to_owned(),
                    )
                })?);
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
                                type_id: c.type_id(),
                                base_id: super::ReflectBase::Component(
                                    r.id(),
                                    c.component_id.unwrap(),
                                ),
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
        })
    }
}

#[cfg(test)]
mod test {
    use test_utils::test_data::setup_world;

    use super::*;

    // #[test]
    // fn test_simple_query() {
    //     let world = setup_world(|w,r|{
    //         w.spawn(TestComponent::init())
    //         w.spawn(Te)
    //     })
    // }
}
