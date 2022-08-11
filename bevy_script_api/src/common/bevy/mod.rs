use std::{sync::{Arc}, marker::PhantomData};

/// Common functionality for all script hosts

use bevy::{prelude::{Entity, ReflectComponent, World, ReflectResource, ReflectDefault, Children}, reflect::{DynamicStruct, DynamicTupleStruct, DynamicTuple, DynamicList, TypeInfo, TypeRegistration}};
use parking_lot::{RwLock, MappedRwLockReadGuard, RwLockReadGuard, MappedRwLockWriteGuard, RwLockWriteGuard};
use crate::ScriptRef;




    // /// Retrieves a reference to the given component type on the given entity
    // pub fn get_component(&self,entity: Entity, component_data: &ReflectComponent) -> Option<ScriptRef>{
    //     let w = self.read();

    //     component_data.reflect(&w, entity)
    //         .map(|_component| 
    //             ScriptRef::new_component_ref(component_data.clone(), entity, self.clone()))
    // }

    // /// Returns true if the given component exists on the given entity
    // pub fn has_component(&self, entity : Entity, component_data: &ReflectComponent) -> bool {
    //     let w = self.read();
    //     component_data.reflect(&w, entity).is_some()
    // }

    // /// Inserts a default instance of the component to the given entity and returns a reference to it
    // pub fn add_default_component(&self, entity : Entity, component_data : &ReflectComponent, default_data: &ReflectDefault) -> ScriptRef {
    //     let mut w = self.write();

    //     let default_instance = default_data.default();

    //     component_data.apply(&mut w,entity,default_instance.as_ref());

    //     ScriptRef::new_component_ref(component_data.clone(), entity, self.clone())
    // }

    // /// Removes given component type off of the given entity
    // pub fn remove_component(&self, entity : Entity, component_data: &ReflectComponent) {
    //     let mut w = self.write();
    //     component_data.remove(&mut w, entity)
    // }

    // /// Retrieves the given resource 
    // pub fn get_resource(&self, resource_data: &ReflectResource) -> Option<ScriptRef>{
    //     let w = self.read();
    //     resource_data.reflect(&w).map(|_res| {
    //         ScriptRef::new_resource_ref(resource_data.clone(), self.clone())
    //     })
    // }

    // /// Removes the given resource
    // pub fn remove_resource(&self, resource_data: &ReflectResource) {
    //     let mut w = self.write();
    //     resource_data.remove(&mut w)
    // }

    // /// Returns true if the given resource exists
    // pub fn has_resource(&self, resource_data: &ReflectResource) -> bool{
    //     let w = self.read();
    //     resource_data.reflect(&w).is_some()
    // }

    // /// Spawns a new entity and returns its ID
    // pub fn spawn(&self) -> Entity {
    //     let mut w = self.write();
    //     w.spawn().id()
    // }

    // /// Despawns given entity ID, returns true if successfull
    // pub fn despawn(&self, entity: Entity) -> bool {
    //     let mut w = self.write();
    //     w.despawn(entity)
    // }