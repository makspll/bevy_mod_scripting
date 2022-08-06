use std::{sync::{Arc}, marker::PhantomData};

/// Common functionality for all script hosts

use bevy::{prelude::{Entity, ReflectComponent, World, ReflectResource, ReflectDefault, Children}, reflect::{DynamicStruct, DynamicTupleStruct, DynamicTuple, DynamicList, TypeInfo, TypeRegistration}};
use parking_lot::{RwLock, MappedRwLockReadGuard, RwLockReadGuard, MappedRwLockWriteGuard, RwLockWriteGuard};
use crate::ScriptRef;

/// Pointer to a bevy world, safely allows multiple access via RwLock
/// # Safety
/// This pointer does not prevent dangling pointers, i.e. you must ensure the world is not dropped while any world pointers still exist,
/// the world must also not change, from the moment a world pointer is created it must always point to the same world.
#[derive(Debug,Clone)]
pub struct WorldPointer(Arc<RwLock<*mut World>>);


unsafe impl Send for WorldPointer {}
unsafe impl Sync for WorldPointer {}

impl WorldPointer {
    /// Creates a new world pointer.
    /// # Safety
    /// satisfies world constancy, since it's impossible to change the underlying pointer
    /// However you must ensure that the world does not go out of scope while this pointer is live    
    pub unsafe fn new(world: &mut World) -> Self{
        WorldPointer(Arc::new(RwLock::new(world)))
    }

    /// Returns a read guard which can be used for immutable world access.
    pub fn read(&self) -> MappedRwLockReadGuard<World> {
        RwLockReadGuard::map(self.0.try_read().expect(""), |ptr : &*mut World| {
            unsafe{&**ptr}
        })
    }

    /// Returns a write guard which can be used for mutable world access.
    pub fn write(&self) -> MappedRwLockWriteGuard<World> {
        RwLockWriteGuard::map(self.0.try_write().expect(""), |ptr : &mut *mut World| {
            unsafe{&mut **ptr}
        })
    }


    /// Retrieves a reference to the given component type on the given entity
    pub fn get_component(&self,entity: Entity, component_data: &ReflectComponent) -> Option<ScriptRef>{
        let w = self.read();

        component_data.reflect(&w, entity)
            .map(|_component| 
                ScriptRef::new_component_ref(component_data.clone(), entity, self.clone()))
    }

    /// Returns true if the given component exists on the given entity
    pub fn has_component(&self, entity : Entity, component_data: &ReflectComponent) -> bool {
        let w = self.read();
        component_data.reflect(&w, entity).is_some()
    }

    /// Inserts a default instance of the component to the given entity and returns a reference to it
    pub fn add_default_component(&self, entity : Entity, component_data : &ReflectComponent, default_data: &ReflectDefault) -> ScriptRef {
        let mut w = self.write();

        let default_instance = default_data.default();

        component_data.apply(&mut w,entity,default_instance.as_ref());

        ScriptRef::new_component_ref(component_data.clone(), entity, self.clone())
    }

    /// Removes given component type off of the given entity
    pub fn remove_component(&self, entity : Entity, component_data: &ReflectComponent) {
        let mut w = self.write();
        component_data.remove(&mut w, entity)
    }

    /// Retrieves the given resource 
    pub fn get_resource(&self, resource_data: &ReflectResource) -> Option<ScriptRef>{
        let w = self.read();
        resource_data.reflect(&w).map(|_res| {
            ScriptRef::new_resource_ref(resource_data.clone(), self.clone())
        })
    }

    /// Removes the given resource
    pub fn remove_resource(&self, resource_data: &ReflectResource) {
        let mut w = self.write();
        resource_data.remove(&mut w)
    }

    /// Returns true if the given resource exists
    pub fn has_resource(&self, resource_data: &ReflectResource) -> bool{
        let w = self.read();
        resource_data.reflect(&w).is_some()
    }

    /// Spawns a new entity and returns its ID
    pub fn spawn(&self) -> Entity {
        let mut w = self.write();
        w.spawn().id()
    }

    /// Despawns given entity ID, returns true if successfull
    pub fn despawn(&self, entity: Entity) -> bool {
        let mut w = self.write();
        w.despawn(entity)
    }
}