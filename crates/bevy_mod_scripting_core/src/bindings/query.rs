use super::{ReflectReference, WorldAccessGuard, WorldCallbackAccess};
use crate::{
    bindings::{CONCURRENT_WORLD_ACCESS_MSG, STALE_WORLD_MSG},
    prelude::ScriptResult,
};
use bevy::{
    ecs::{component::ComponentId, entity::Entity},
    prelude::{EntityRef, QueryBuilder},
    reflect::TypeRegistration,
};
use std::{any::TypeId, collections::VecDeque, sync::Arc};

/// A wrapper around a `TypeRegistration` that provides additional information about the type.
///
/// This is used as a hook to a rust type from a scripting language. We should be able to easily convert between a type name and a [`ScriptTypeRegistration`].
#[derive(Clone)]
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
    pub fn short_name(&self) -> &str {
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

    /// Returns the [`ComponentId`] for this type, if it is a component or a resource.
    #[inline(always)]
    pub fn component_id(&self) -> Option<ComponentId> {
        self.component_id
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

#[derive(Clone, Default)]
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

#[derive(Clone)]
pub struct ScriptQueryResult(pub Entity, pub Vec<ReflectReference>);

impl WorldCallbackAccess {
    pub fn query(&self, query: ScriptQueryBuilder) -> ScriptResult<VecDeque<ScriptQueryResult>> {
        // find the set of components
        self.read()
            .unwrap_or_else(|| panic!("{STALE_WORLD_MSG}"))
            .query(query)
    }
}

impl<'w> WorldAccessGuard<'w> {
    pub fn query(&self, query: ScriptQueryBuilder) -> ScriptResult<VecDeque<ScriptQueryResult>> {
        let world = self
            .get_whole_world_access()
            .unwrap_or_else(|| panic!("{CONCURRENT_WORLD_ACCESS_MSG}"));

        let mut dynamic_query = QueryBuilder::<EntityRef>::new(world);

        // we don't actually want to fetch the data for components now, only figure out
        // which entities match the query
        // so we might be being slightly overkill
        for c in &query.components {
            dynamic_query.ref_id(c.component_id().unwrap());
        }

        for w in query.with {
            dynamic_query.with_id(w.component_id.unwrap());
        }

        for without_id in query.without {
            dynamic_query.without_id(without_id.component_id.unwrap());
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
                            base_id: super::ReflectBase::Component(r.id(), c.component_id.unwrap()),
                        },
                        reflect_path: vec![],
                    })
                    .collect();
                ScriptQueryResult(r.id(), references)
            })
            .collect())
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
