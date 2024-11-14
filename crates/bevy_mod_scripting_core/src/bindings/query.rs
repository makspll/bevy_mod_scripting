use super::{ReflectReference, WorldCallbackAccess};
use crate::prelude::ScriptResult;
use bevy::{
    ecs::{component::ComponentId, entity::Entity},
    reflect::TypeRegistration,
};
use std::{any::TypeId, ops::Deref, sync::Arc};

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

#[derive(Clone)]
pub struct ScriptQueryBuilder {
    world: WorldCallbackAccess,
    components: Vec<ScriptTypeRegistration>,
    with: Vec<ScriptTypeRegistration>,
    without: Vec<ScriptTypeRegistration>,
}

impl ScriptQueryBuilder {
    pub fn new(world: WorldCallbackAccess) -> Self {
        Self {
            world,
            components: vec![],
            with: vec![],
            without: vec![],
        }
    }

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

    pub fn build(&mut self) -> ScriptResult<Vec<ScriptQueryResult>> {
        self.world.query(
            std::mem::take(&mut self.components),
            std::mem::take(&mut self.with),
            std::mem::take(&mut self.without),
        )
    }
}

#[derive(Clone)]
pub struct ScriptQueryResult(pub Entity, pub Vec<ReflectReference>);

impl WorldCallbackAccess {
    pub fn query(
        &mut self,
        components: Vec<ScriptTypeRegistration>,
        with: Vec<ScriptTypeRegistration>,
        without: Vec<ScriptTypeRegistration>,
    ) -> ScriptResult<Vec<ScriptQueryResult>> {
        // for c in components {

        // }
        todo!()
    }
}
