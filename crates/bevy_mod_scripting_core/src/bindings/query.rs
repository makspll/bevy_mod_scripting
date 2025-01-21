use super::{ReflectReference, WorldAccessGuard};
use crate::error::InteropError;
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
pub struct ScriptComponentRegistration {
    pub(crate) registration: ScriptTypeRegistration,
    pub(crate) component_id: ComponentId,
}

#[derive(Clone, Reflect, Debug)]
pub struct ScriptResourceRegistration {
    pub(crate) registration: ScriptTypeRegistration,
    pub(crate) resource_id: ComponentId,
}

impl ScriptTypeRegistration {
    pub fn new(registration: Arc<TypeRegistration>) -> Self {
        Self { registration }
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

    pub fn type_registration(&self) -> &TypeRegistration {
        &self.registration
    }
}
impl ScriptResourceRegistration {
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
}

impl ScriptComponentRegistration {
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
    components: Vec<ScriptComponentRegistration>,
    with: Vec<ScriptComponentRegistration>,
    without: Vec<ScriptComponentRegistration>,
}

impl ScriptQueryBuilder {
    pub fn components(&mut self, components: Vec<ScriptComponentRegistration>) -> &mut Self {
        self.components.extend(components);
        self
    }
    pub fn component(&mut self, component: ScriptComponentRegistration) -> &mut Self {
        self.components.push(component);
        self
    }

    pub fn with_components(&mut self, with: Vec<ScriptComponentRegistration>) -> &mut Self {
        self.with.extend(with);
        self
    }

    pub fn with_component(&mut self, with: ScriptComponentRegistration) -> &mut Self {
        self.with.push(with);
        self
    }

    pub fn without_components(&mut self, without: Vec<ScriptComponentRegistration>) -> &mut Self {
        self.without.extend(without);
        self
    }

    pub fn without_component(&mut self, without: ScriptComponentRegistration) -> &mut Self {
        self.without.push(without);
        self
    }
}

#[derive(Clone, Reflect)]
#[reflect(opaque)]
pub struct ScriptQueryResult {
    pub entity: Entity,
    pub components: Vec<ReflectReference>,
}

// impl WorldCallbackAccess {
//     pub fn query(
//         &self,
//         query: ScriptQueryBuilder,
//     ) -> Result<VecDeque<ScriptQueryResult>, InteropError> {
//         // find the set of components
//         self.try_read().and_then(|world| world.query(query))
//     }
// }

impl WorldAccessGuard<'_> {
    pub fn query(
        &self,
        query: ScriptQueryBuilder,
    ) -> Result<VecDeque<ScriptQueryResult>, InteropError> {
        self.with_global_access(|world| {
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
