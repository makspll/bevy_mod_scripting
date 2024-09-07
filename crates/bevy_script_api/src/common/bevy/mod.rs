use crate::ReflectReference;
/// Common functionality for all script hosts
use bevy::{
    ecs::{
        component::ComponentId,
        query::QueryBuilder,
        world::{Command, EntityRef, World},
    },
    prelude::{
        AppTypeRegistry, BuildWorldChildren, Children, DespawnChildrenRecursive, DespawnRecursive,
        Entity, Parent, ReflectComponent, ReflectDefault, ReflectResource,
    },
    reflect::{
        DynamicArray, DynamicEnum, DynamicList, DynamicMap, DynamicStruct, DynamicTuple,
        DynamicTupleStruct, TypeRegistration,
    },
};
use bevy_mod_scripting_core::{prelude::ScriptError, world::WorldPointer};
use parking_lot::MappedRwLockWriteGuard;
use std::{
    any::Any,
    ops::{Deref, DerefMut},
    sync::Arc,
};

/// Helper trait for retrieving a world pointer from a script context.
pub trait GetWorld {
    type Error;
    fn get_world(&self) -> Result<WorldPointer, Self::Error>;
}

#[derive(Clone)]
pub struct ScriptTypeRegistration(pub(crate) Arc<TypeRegistration>);

impl ScriptTypeRegistration {
    pub fn new(arc: Arc<TypeRegistration>) -> Self {
        Self(arc)
    }

    #[inline(always)]
    pub fn short_name(&self) -> &str {
        self.0.type_info().type_path_table().short_path()
    }

    #[inline(always)]
    pub fn type_name(&self) -> &'static str {
        self.0.type_info().type_path_table().path()
    }
}

impl std::fmt::Debug for ScriptTypeRegistration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ScriptTypeRegistration")
            .field(&self.0.type_info().type_path())
            .finish()
    }
}

impl std::fmt::Display for ScriptTypeRegistration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.type_info().type_path())
    }
}

impl Deref for ScriptTypeRegistration {
    type Target = TypeRegistration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone)]
pub struct ScriptQueryBuilder {
    world: ScriptWorld,
    components: Vec<ScriptTypeRegistration>,
    with: Vec<ScriptTypeRegistration>,
    without: Vec<ScriptTypeRegistration>,
}

impl ScriptQueryBuilder {
    pub fn new(world: ScriptWorld) -> Self {
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

    pub fn build(&mut self) -> Result<Vec<ScriptQueryResult>, ScriptError> {
        self.world.query(
            std::mem::take(&mut self.components),
            std::mem::take(&mut self.with),
            std::mem::take(&mut self.without),
        )
    }
}

#[derive(Clone)]
pub struct ScriptQueryResult(pub Entity, pub Vec<ReflectReference>);

#[derive(Clone, Debug)]
pub struct ScriptWorld(WorldPointer);

impl std::fmt::Display for ScriptWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("World")
    }
}

impl Deref for ScriptWorld {
    type Target = WorldPointer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ScriptWorld {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<WorldPointer> for ScriptWorld {
    fn as_ref(&self) -> &WorldPointer {
        &self.0
    }
}

impl From<ScriptWorld> for WorldPointer {
    fn from(val: ScriptWorld) -> Self {
        val.0
    }
}

impl ScriptWorld {
    pub fn new(ptr: WorldPointer) -> Self {
        Self(ptr)
    }

    pub fn get_children(&self, parent: Entity) -> Vec<Entity> {
        let w = self.read();
        w.get::<Children>(parent)
            .map(|v| v.to_vec())
            .unwrap_or_default()
    }

    pub fn get_parent(&self, entity: Entity) -> Option<Entity> {
        let w = self.read();
        w.get::<Parent>(entity).map(|parent| parent.get())
    }

    pub fn push_child(&self, parent: Entity, child: Entity) {
        let mut w = self.write();
        if let Some(mut entity) = w.get_entity_mut(parent) {
            entity.push_children(&[child]);
        }
    }

    pub fn remove_children(&self, parent: Entity, children: &[Entity]) {
        let mut w = self.write();

        if let Some(mut entity) = w.get_entity_mut(parent) {
            entity.remove_children(children);
        }
    }

    pub fn insert_children(&self, parent: Entity, index: usize, children: &[Entity]) {
        let mut w = self.write();

        if let Some(mut entity) = w.get_entity_mut(parent) {
            entity.insert_children(index, children);
        }
    }

    pub fn despawn_children_recursive(&self, entity: Entity) {
        let mut w = self.write();
        DespawnChildrenRecursive { entity }.apply(&mut w);
    }

    pub fn despawn_recursive(&self, entity: Entity) {
        let mut w = self.write();
        DespawnRecursive { entity }.apply(&mut w);
    }

    pub fn get_type_by_name(&self, type_name: &str) -> Option<ScriptTypeRegistration> {
        let w = self.read();

        let registry: &AppTypeRegistry = w.get_resource().unwrap();

        let registry = registry.read();

        registry
            .get_with_short_type_path(type_name)
            .or_else(|| registry.get_with_type_path(type_name))
            .map(|registration| ScriptTypeRegistration::new(Arc::new(registration.clone())))
    }

    pub fn add_default_component(
        &self,
        entity: Entity,
        comp_type: ScriptTypeRegistration,
    ) -> Result<ReflectReference, ScriptError> {
        let mut w = self.write();

        // Remove: AppTypeRegistry
        let registry: AppTypeRegistry = w.remove_resource().unwrap();

        let mut entity_ref = w
            .get_entity_mut(entity)
            .ok_or_else(|| ScriptError::Other(format!("Entity is not valid {:#?}", entity)))?;

        let component_data = comp_type.data::<ReflectComponent>().ok_or_else(|| {
            ScriptError::Other(format!("Not a component {}", comp_type.short_name()))
        })?;

        let registry_lock = registry.read();

        // this is just a formality
        // TODO: maybe get an add_default impl added to ReflectComponent
        // this means that we don't require ReflectDefault for adding components!
        match comp_type.0.type_info(){
            bevy::reflect::TypeInfo::Struct(_) => component_data.insert(&mut entity_ref, &DynamicStruct::default(), &registry_lock),
            bevy::reflect::TypeInfo::TupleStruct(_) => component_data.insert(&mut entity_ref, &DynamicTupleStruct::default(), &registry_lock),
            bevy::reflect::TypeInfo::Tuple(_) => component_data.insert(&mut entity_ref, &DynamicTuple::default(), &registry_lock),
            bevy::reflect::TypeInfo::List(_) => component_data.insert(&mut entity_ref, &DynamicList::default(), &registry_lock),
            bevy::reflect::TypeInfo::Array(_) => component_data.insert(&mut entity_ref, &DynamicArray::new(Box::new([])), &registry_lock),
            bevy::reflect::TypeInfo::Map(_) => component_data.insert(&mut entity_ref, &DynamicMap::default(), &registry_lock),
            bevy::reflect::TypeInfo::Value(_) => component_data.insert(&mut entity_ref,
                comp_type.data::<ReflectDefault>().ok_or_else(||
                    ScriptError::Other(format!("Component {} is a value or dynamic type with no `ReflectDefault` type_data, cannot instantiate sensible value",comp_type.short_name())))?
                    .default()
                    .as_ref(),
                    &registry_lock),
            bevy::reflect::TypeInfo::Enum(_) => component_data.insert(&mut entity_ref, &DynamicEnum::default(), &registry_lock)
        };
        // if we do not drop the lock here, line below will complain registry is still borrowed at drop
        drop(registry_lock);

        // Insert: AppTypeRegistry
        w.insert_resource(registry);

        Ok(ReflectReference::new_component_ref(
            component_data.clone(),
            entity,
            self.clone().into(),
        ))
    }

    pub fn get_component(
        &self,
        entity: Entity,
        comp_type: ScriptTypeRegistration,
    ) -> Result<Option<ReflectReference>, ScriptError> {
        let w = self.read();

        let entity_ref = w
            .get_entity(entity)
            .ok_or_else(|| ScriptError::Other(format!("Entity is not valid {:#?}", entity)))?;

        let component_data = comp_type.data::<ReflectComponent>().ok_or_else(|| {
            ScriptError::Other(format!("Not a component {}", comp_type.short_name()))
        })?;

        Ok(component_data.reflect(entity_ref).map(|_component| {
            ReflectReference::new_component_ref(component_data.clone(), entity, self.clone().into())
        }))
    }

    pub fn has_component(
        &self,
        entity: Entity,
        comp_type: ScriptTypeRegistration,
    ) -> Result<bool, ScriptError> {
        let w = self.read();
        let component_data = comp_type.data::<ReflectComponent>().ok_or_else(|| {
            ScriptError::Other(format!("Not a component {}", comp_type.short_name()))
        })?;

        let entity_ref = w
            .get_entity(entity)
            .ok_or_else(|| ScriptError::Other(format!("Entity is not valid {:#?}", entity)))?;

        Ok(component_data.reflect(entity_ref).is_some())
    }

    pub fn remove_component(
        &mut self,
        entity: Entity,
        comp_type: ScriptTypeRegistration,
    ) -> Result<(), ScriptError> {
        let mut w = self.write();

        let mut entity_ref = w
            .get_entity_mut(entity)
            .ok_or_else(|| ScriptError::Other(format!("Entity is not valid {:#?}", entity)))?;

        let component_data = comp_type.data::<ReflectComponent>().ok_or_else(|| {
            ScriptError::Other(format!("Not a component {}", comp_type.short_name()))
        })?;
        component_data.remove(&mut entity_ref);
        Ok(())
    }

    pub fn get_resource(
        &self,
        res_type: ScriptTypeRegistration,
    ) -> Result<Option<ReflectReference>, ScriptError> {
        let w = self.read();

        let resource_data = res_type.data::<ReflectResource>().ok_or_else(|| {
            ScriptError::Other(format!("Not a resource {}", res_type.short_name()))
        })?;

        Ok(resource_data.reflect(&w).map(|_res| {
            ReflectReference::new_resource_ref(resource_data.clone(), self.clone().into())
        }))
    }

    pub fn has_resource(&self, res_type: ScriptTypeRegistration) -> Result<bool, ScriptError> {
        let w = self.read();

        let resource_data = res_type.data::<ReflectResource>().ok_or_else(|| {
            ScriptError::Other(format!("Not a resource {}", res_type.short_name()))
        })?;

        Ok(resource_data.reflect(&w).is_some())
    }

    pub fn remove_resource(&mut self, res_type: ScriptTypeRegistration) -> Result<(), ScriptError> {
        let mut w = self.write();

        let resource_data = res_type.data::<ReflectResource>().ok_or_else(|| {
            ScriptError::Other(format!("Not a resource {}", res_type.short_name()))
        })?;
        resource_data.remove(&mut w);
        Ok(())
    }

    pub fn query(
        &mut self,
        components: Vec<ScriptTypeRegistration>,
        with: Vec<ScriptTypeRegistration>,
        without: Vec<ScriptTypeRegistration>,
    ) -> Result<Vec<ScriptQueryResult>, ScriptError> {
        let mut w = self.write();

        let get_id = |component: &ScriptTypeRegistration,
                      w: &MappedRwLockWriteGuard<World>|
         -> Result<ComponentId, ScriptError> {
            w.components()
                .get_id(component.type_info().type_id())
                .ok_or_else(|| {
                    ScriptError::Other(format!("Not a component {}", component.short_name()))
                })
        };

        let components: Vec<(ReflectComponent, ComponentId)> = components
            .into_iter()
            .map(|component| {
                let reflect_component = component.data::<ReflectComponent>().ok_or_else(|| {
                    ScriptError::Other(format!("Not a component {}", component.short_name()))
                });

                let component_id = get_id(&component, &w);
                reflect_component.map(|v1| component_id.map(|v2| (v1.clone(), v2)))?
            })
            .collect::<Result<Vec<_>, ScriptError>>()?;

        let with_ids: Vec<ComponentId> = with
            .iter()
            .map(|component| get_id(component, &w))
            .collect::<Result<Vec<_>, ScriptError>>()?;

        let without_ids: Vec<ComponentId> = without
            .iter()
            .map(|component| get_id(component, &w))
            .collect::<Result<Vec<_>, ScriptError>>()?;

        let mut q = QueryBuilder::<EntityRef>::new(&mut w);

        for (_, id) in &components {
            q.ref_id(*id);
        }

        for with_id in with_ids {
            q.with_id(with_id);
        }

        for without_id in without_ids {
            q.without_id(without_id);
        }

        let query_result: Vec<EntityRef<'_>> = q.build().iter_mut(&mut w).collect();

        query_result
            .into_iter()
            .map(|filtered_entity| {
                components
                    .clone()
                    .into_iter()
                    .map(|(reflect_component, _)| {
                        let type_id = reflect_component.type_id();
                        reflect_component
                            .reflect(filtered_entity)
                            .map(|_component| {
                                ReflectReference::new_component_ref(
                                    reflect_component,
                                    filtered_entity.id(),
                                    self.clone().into(),
                                )
                            })
                            .ok_or_else(|| {
                                ScriptError::Other(format!(
                                    "Failed to reflect component during query: {:?}",
                                    type_id
                                ))
                            })
                    })
                    .collect::<Result<Vec<_>, ScriptError>>()
                    .map(|references| ScriptQueryResult(filtered_entity.id(), references))
            })
            .collect::<Result<Vec<_>, ScriptError>>()
    }
}
