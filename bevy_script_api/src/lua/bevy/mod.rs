use crate::impl_tealr_type;
use std::convert::AsRef;
use std::ops::Deref;
use std::sync::Arc;

use crate::script_ref::ScriptRef;
use bevy::ecs::system::Command;
use bevy::hierarchy::BuildWorldChildren;
use bevy::reflect::DynamicEnum;
use bevy::{
    hierarchy::{Children, DespawnChildrenRecursive, DespawnRecursive, Parent},
    prelude::{ReflectComponent, ReflectDefault, ReflectResource},
    reflect::{
        DynamicArray, DynamicList, DynamicMap, DynamicStruct, DynamicTuple, DynamicTupleStruct,
        TypeRegistration,
    },
};
use bevy_mod_scripting_core::prelude::*;
use bevy_mod_scripting_core::world::WorldPointer;
use bevy_mod_scripting_lua::tealr;

use tealr::mlu::{
    mlua::{self},
    TealData, TealDataMethods,
};

pub use crate::generated::*;

use super::util::TypeRegistryWrapper;

#[derive(Clone)]
pub struct LuaTypeRegistration(Arc<TypeRegistration>);
impl_tealr_type!(LuaTypeRegistration);

impl TealData for LuaTypeRegistration {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document_type("An object representing an existing and registered rust type.");
        methods.document_type("Can be obtained via [`LuaWorld::get_type_by_name`].");
    }

    fn add_fields<'lua, F: tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut F) {
        fields.document("The [short name](https://docs.rs/bevy/latest/bevy/reflect/struct.TypeRegistration.html#method.get_short_name) of a type");
        fields.add_field_method_get("short_name", |_, s| Ok(s.0.short_name().to_string()));

        fields.document("The full name of the type");
        fields.add_field_method_get("type_name", |_, s| Ok(s.0.type_name()));
    }
}

impl Deref for LuaTypeRegistration {
    type Target = TypeRegistration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct LuaScriptData {
    sid: u32,
}

impl From<&ScriptData<'_>> for LuaScriptData {
    fn from(sd: &ScriptData) -> Self {
        Self { sid: sd.sid }
    }
}

impl_tealr_type!(LuaScriptData);

impl TealData for LuaScriptData {
    fn add_fields<'lua, F: tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut F) {
        fields.document("The unique ID of this script");
        fields.add_field_method_get("sid", |_, s| Ok(s.sid))
    }

    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_meta_method(tealr::mlu::mlua::MetaMethod::ToString, |_, s, ()| {
            Ok(format!("{:?}", s))
        });
    }
}

#[derive(Clone, Debug)]
pub struct LuaWorld(WorldPointer);

impl Deref for LuaWorld {
    type Target = WorldPointer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<WorldPointer> for LuaWorld {
    fn as_ref(&self) -> &WorldPointer {
        &self.0
    }
}

impl LuaWorld {
    pub fn new(w: WorldPointer) -> Self {
        Self(w)
    }
}

impl_tealr_type!(LuaWorld);

impl TealData for LuaWorld {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document_type("Represents the bevy world all scripts live in.");
        methods.document_type("Provides ways to interact with and modify the world.");

        methods.add_meta_method(tealr::mlu::mlua::MetaMethod::ToString, |_, s, ()| {
            Ok(format!("{s:?}"))
        });

        methods.document("Retrieves children entities of the parent entity if it has any.");
        methods.add_method("get_children", |_, world, parent: LuaEntity| {
            let w = world.read();

            let children: Option<Vec<LuaEntity>> =
                w.get::<Children>(parent.inner()?).map(|children| {
                    children
                        .iter()
                        .map(|e| LuaEntity::new(*e))
                        .collect::<Vec<_>>()
                });

            Ok(children)
        });

        methods.document("Retrieves the parent entity of the given entity if it has any.");
        methods.add_method("get_parent", |_, world, parent: LuaEntity| {
            let w = world.read();

            let parent: Option<LuaEntity> = w
                .get::<Parent>(parent.inner()?)
                .map(|parent| LuaEntity::new(parent.get()));

            Ok(parent)
        });

        methods.document("Attaches children entities to the given parent entity.");
        methods.add_method(
            "push_children",
            |_, world, (parent, children): (LuaEntity, Vec<LuaEntity>)| {
                let mut w = world.write();
                let children = children
                    .iter()
                    .map(|e| e.inner())
                    .collect::<Result<Vec<_>, _>>()?;

                if let Some(mut entity) = w.get_entity_mut(parent.inner()?) {
                    entity.push_children(&children);
                }

                Ok(())
            },
        );

        methods.document("Attaches child entity to the given parent entity.");
        methods.add_method(
            "push_child",
            |_, world, (parent, child): (LuaEntity, LuaEntity)| {
                let mut w = world.write();
                let child = child.inner()?;
                if let Some(mut entity) = w.get_entity_mut(parent.inner()?) {
                    entity.push_children(&[child]);
                }

                Ok(())
            },
        );

        methods.document("Removes children entities from the given parent entity.");
        methods.add_method(
            "remove_children",
            |_, world, (parent, children): (LuaEntity, Vec<LuaEntity>)| {
                let mut w = world.write();
                let children = children
                    .iter()
                    .map(|e| e.inner())
                    .collect::<Result<Vec<_>, _>>()?;

                if let Some(mut entity) = w.get_entity_mut(parent.inner()?) {
                    entity.remove_children(&children);
                }

                Ok(())
            },
        );

        methods.document("Removes child entity from the given parent entity.");
        methods.add_method(
            "remove_child",
            |_, world, (parent, child): (LuaEntity, LuaEntity)| {
                let mut w = world.write();
                let child = child.inner()?;
                if let Some(mut entity) = w.get_entity_mut(parent.inner()?) {
                    entity.remove_children(&[child]);
                }

                Ok(())
            },
        );

        methods
            .document("Inserts children entities to the given parent entity at the given index.");
        methods.add_method(
            "insert_children",
            |_, world, (parent, index, children): (LuaEntity, usize, Vec<LuaEntity>)| {
                let mut w = world.write();
                let children = children
                    .iter()
                    .map(|e| e.inner())
                    .collect::<Result<Vec<_>, _>>()?;

                if let Some(mut entity) = w.get_entity_mut(parent.inner()?) {
                    entity.insert_children(index, &children);
                }

                Ok(())
            },
        );

        methods.document("Inserts child entity to the given parent entity at the given index.");
        methods.add_method(
            "insert_child",
            |_, world, (parent, index, child): (LuaEntity, usize, LuaEntity)| {
                let mut w = world.write();
                let child = child.inner()?;
                if let Some(mut entity) = w.get_entity_mut(parent.inner()?) {
                    entity.insert_children(index, &[child]);
                }

                Ok(())
            },
        );

        methods.document("Despawns the given entity's children recursively");
        methods.add_method(
            "despawn_children_recursive",
            |_, world, entity: LuaEntity| {
                let mut w = world.write();
                DespawnChildrenRecursive {
                    entity: entity.inner()?,
                }
                .write(&mut w);
                Ok(())
            },
        );

        methods.document("Despawns the given entity and the entity's children recursively");
        methods.add_method("despawn_recursive", |_, world, entity: LuaEntity| {
            let mut w = world.write();
            DespawnRecursive {
                entity: entity.inner()?,
            }
            .write(&mut w);
            Ok(())
        });

        methods.document("Despawns the given entity and the entity's children recursively");
        methods.add_method("insert_children", |_, world, entity: LuaEntity| {
            let mut w = world.write();
            DespawnRecursive {
                entity: entity.inner()?,
            }
            .write(&mut w);
            Ok(())
        });

        methods.document("Retrieves type information given either a short (`MyType`) or fully qualified rust type name (`MyModule::MyType`).");
        methods.document(
            "Returns `nil` if no such type exists or if one wasn't registered on the rust side.",
        );
        methods.document("\n");
        methods.document("This is used extensively in [`LuaWorld`]");
        methods.add_method("get_type_by_name", |_, world, type_name: String| {
            let w = world.read();

            let registry: &TypeRegistryWrapper = w.get_resource().unwrap();

            let registry = registry.0.read();

            Ok(registry
                .get_with_short_name(&type_name)
                .or_else(|| registry.get_with_name(&type_name))
                .map(|registration| LuaTypeRegistration(Arc::new(registration.clone()))))
        });

        methods.document("Inserts a component of the given type to the given entity by instantiating a default version of it.");
        methods.document("The component can then be modified using field access.");
        methods.add_method("add_default_component", |_, world, (entity, comp_type): (LuaEntity, LuaTypeRegistration)| {
            // grab this entity before acquiring a lock in case it's a reference
            let entity = entity.inner()?;
            let mut w = world.write();

            let component_data = comp_type.data::<ReflectComponent>()
                .ok_or_else(|| mlua::Error::RuntimeError(format!("Not a component {}",comp_type.short_name())))?;

            // this is just a formality
            // TODO: maybe get an add_default impl added to ReflectComponent
            // this means that we don't require ReflectDefault for adding components!
            match comp_type.0.type_info(){
                bevy::reflect::TypeInfo::Struct(_) => component_data.insert(&mut w, entity, &DynamicStruct::default()),
                bevy::reflect::TypeInfo::TupleStruct(_) => component_data.insert(&mut w, entity, &DynamicTupleStruct::default()),
                bevy::reflect::TypeInfo::Tuple(_) => component_data.insert(&mut w, entity, &DynamicTuple::default()),
                bevy::reflect::TypeInfo::List(_) => component_data.insert(&mut w, entity, &DynamicList::default()),
                bevy::reflect::TypeInfo::Array(_) => component_data.insert(&mut w, entity, &DynamicArray::new(Box::new([]))),
                bevy::reflect::TypeInfo::Map(_) => component_data.insert(&mut w, entity, &DynamicMap::default()),
                bevy::reflect::TypeInfo::Value(_) |
                bevy::reflect::TypeInfo::Dynamic(_) => component_data.insert(&mut w, entity,
                    comp_type.data::<ReflectDefault>().ok_or_else(||
                        mlua::Error::RuntimeError(format!("Component {} is a value or dynamic type with no `ReflectDefault` type_data, cannot instantiate sensible value",comp_type.short_name())))?
                        .default()
                        .as_ref()),
                bevy::reflect::TypeInfo::Enum(_) => component_data.insert(&mut w, entity, &DynamicEnum::default()),
            };

            Ok(ScriptRef::new_component_ref(component_data.clone(), entity, world.0.clone()))
        });

        methods.document("Retrieves a component of the given type from the given entity.");
        methods.document("If such a component does not exist returns `nil`.");
        methods.add_method(
            "get_component",
            |_, world, (entity, comp_type): (LuaEntity, LuaTypeRegistration)| {
                // grab this entity before acquiring a lock in case it's a reference
                let entity = entity.inner()?;
                let w = world.read();

                let component_data = comp_type.data::<ReflectComponent>().ok_or_else(|| {
                    mlua::Error::RuntimeError(format!("Not a component {}", comp_type.short_name()))
                })?;

                Ok(component_data.reflect(&w, entity).map(|_component| {
                    ScriptRef::new_component_ref(
                        component_data.clone(),
                        entity,
                        world.as_ref().clone(),
                    )
                }))
            },
        );

        methods
            .document("Returns `true` if the given entity contains a component of the given type.");
        methods.add_method(
            "has_component",
            |_, world, (entity, comp_type): (LuaEntity, LuaTypeRegistration)| {
                // grab this entity before acquiring a lock in case it's a reference
                let entity = entity.inner()?;
                let w = world.read();

                let component_data = comp_type.data::<ReflectComponent>().ok_or_else(|| {
                    mlua::Error::RuntimeError(format!("Not a component {}", comp_type.short_name()))
                })?;

                Ok(component_data.reflect(&w, entity).is_some())
            },
        );

        methods.document("Removes the given component from the given entity, does nothing if it doesn't exist on the entity.");
        methods.add_method(
            "remove_component",
            |_, world, (entity, comp_type): (LuaEntity, LuaTypeRegistration)| {
                // grab this entity before acquiring a lock in case it's a reference
                let entity = entity.inner()?;
                let mut w = world.write();

                let component_data = comp_type.data::<ReflectComponent>().ok_or_else(|| {
                    mlua::Error::RuntimeError(format!("Not a component {}", comp_type.short_name()))
                })?;
                component_data.remove(&mut w, entity);
                Ok(())
            },
        );

        methods.document("Retrieves a resource of the given type from the world.");
        methods.document("If such a resource does not exist returns `nil`.");
        methods.add_method("get_resource", |_, world, res_type: LuaTypeRegistration| {
            let w = world.read();

            let resource_data = res_type.data::<ReflectResource>().ok_or_else(|| {
                mlua::Error::RuntimeError(format!("Not a resource {}", res_type.short_name()))
            })?;

            Ok(resource_data.reflect(&w).map(|_res| {
                ScriptRef::new_resource_ref(resource_data.clone(), world.as_ref().clone())
            }))
        });

        methods.document(
            "Removes the given resource from the world, if one doesn't exist it does nothing.",
        );
        methods.add_method(
            "remove_resource",
            |_, world, res_type: LuaTypeRegistration| {
                let mut w = world.write();

                let resource_data = res_type.data::<ReflectResource>().ok_or_else(|| {
                    mlua::Error::RuntimeError(format!("Not a resource {}", res_type.short_name()))
                })?;
                resource_data.remove(&mut w);
                Ok(())
            },
        );

        methods.document("Returns `true` if the world contains a resource of the given type.");
        methods.add_method("has_resource", |_, world, res_type: LuaTypeRegistration| {
            let w = world.read();

            let resource_data = res_type.data::<ReflectResource>().ok_or_else(|| {
                mlua::Error::RuntimeError(format!("Not a resource {}", res_type.short_name()))
            })?;

            Ok(resource_data.reflect(&w).is_some())
        });

        methods.document("Spawns a new entity and returns its Entity ID");
        methods.add_method("spawn", |_, world, ()| {
            let mut w = world.write();

            Ok(LuaEntity::new(w.spawn(()).id()))
        });

        methods.document(
            "Despawns the given entity if it exists, returns true if deletion was successfull",
        );
        methods.add_method("despawn", |_, world, entity: LuaEntity| {
            let mut w = world.write();

            Ok(w.despawn(entity.inner()?))
        });
    }
}