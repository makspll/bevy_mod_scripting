use crate::common::bevy::{ScriptTypeRegistration, ScriptWorld};
use crate::impl_tealr_type;

use std::sync::Arc;

use bevy::hierarchy::BuildWorldChildren;
use bevy::prelude::AppTypeRegistry;

use bevy::prelude::ReflectResource;
use bevy_mod_scripting_core::prelude::*;
use bevy_mod_scripting_lua::tealr;

use tealr::mlu::{
    mlua::{self},
    TealData, TealDataMethods,
};

pub use crate::generated::*;

use super::util::LuaIndex;

pub type LuaTypeRegistration = ScriptTypeRegistration;
impl_tealr_type!(LuaTypeRegistration);

impl TealData for LuaTypeRegistration {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document_type("An object representing an existing and registered rust type.");
        methods.document_type("Can be obtained via [`LuaWorld::get_type_by_name`].");
    }

    fn add_fields<'lua, F: tealr::mlu::TealDataFields<'lua, Self>>(fields: &mut F) {
        fields.document("The [short name](https://docs.rs/bevy/latest/bevy/reflect/struct.TypeRegistration.html#method.get_short_name) of a type");
        fields.add_field_method_get("short_name", |_, s| Ok(s.short_name().to_string()));

        fields.document("The full name of the type");
        fields.add_field_method_get("type_name", |_, s| Ok(s.type_name()));
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

pub type LuaWorld = ScriptWorld;

impl_tealr_type!(LuaWorld);

impl TealData for LuaWorld {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.document_type("Represents the bevy world all scripts live in.");
        methods.document_type("Provides ways to interact with and modify the world.");

        methods.add_meta_method(tealr::mlu::mlua::MetaMethod::ToString, |_, s, ()| {
            Ok(format!("{s:?}"))
        });

        methods.document("Retrieves type information given either a short (`MyType`) or fully qualified rust type name (`MyModule::MyType`).");
        methods.document(
            "Returns `nil` if no such type exists or if one wasn't registered on the rust side.",
        );
        methods.document("\n");
        methods.document("This is used extensively in [`LuaWorld`]");
        methods.add_method("get_type_by_name", |_, world, type_name: String| {
            let w = world.read();

            let registry: &AppTypeRegistry = w.get_resource().unwrap();

            let registry = registry.read();

            Ok(registry
                .get_with_short_type_path(&type_name)
                .or_else(|| registry.get_with_type_path(&type_name))
                .map(|registration| LuaTypeRegistration::new(Arc::new(registration.clone()))))
        });

        methods.document("Inserts a component of the given type to the given entity by instantiating a default version of it.");
        methods.document("The component can then be modified using field access.");
        methods.add_method(
            "add_default_component",
            |_, world, (entity, comp_type): (LuaEntity, LuaTypeRegistration)| {
                world
                    .add_default_component(entity.inner()?, comp_type)
                    .map_err(|e| mlua::Error::RuntimeError(e.to_string()))
            },
        );

        methods.document("Retrieves a component of the given type from the given entity.");
        methods.document("If such a component does not exist returns `nil`.");
        methods.add_method(
            "get_component",
            |_, world, (entity, comp_type): (LuaEntity, LuaTypeRegistration)| {
                world
                    .get_component(entity.inner()?, comp_type)
                    .map_err(|e| mlua::Error::RuntimeError(e.to_string()))
            },
        );

        methods
            .document("Returns `true` if the given entity contains a component of the given type.");
        methods.add_method(
            "has_component",
            |_, world, (entity, comp_type): (LuaEntity, LuaTypeRegistration)| {
                world
                    .has_component(entity.inner()?, comp_type)
                    .map_err(|e| mlua::Error::RuntimeError(e.to_string()))
            },
        );

        methods.document("Removes the given component from the given entity, does nothing if it doesn't exist on the entity.");
        methods.add_method_mut(
            "remove_component",
            |_, world, (entity, comp_type): (LuaEntity, LuaTypeRegistration)| {
                world
                    .remove_component(entity.inner()?, comp_type)
                    .map_err(|e| mlua::Error::RuntimeError(e.to_string()))
            },
        );

        methods.document("Retrieves a resource of the given type from the world.");
        methods.document("If such a resource does not exist returns `nil`.");
        methods.add_method("get_resource", |_, world, res_type: LuaTypeRegistration| {
            world
                .get_resource(res_type)
                .map_err(|e| mlua::Error::RuntimeError(e.to_string()))
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

        methods.document("Retrieves children entities of the parent entity if it has any.");
        methods.add_method("get_children", |_, world, parent: LuaEntity| {
            Ok(world
                .get_children(parent.inner()?)
                .into_iter()
                .map(LuaEntity::new)
                .collect::<Vec<LuaEntity>>())
        });

        methods.document("Retrieves the parent entity of the given entity if it has any.");
        methods.add_method("get_parent", |_, world, parent: LuaEntity| {
            Ok(world.get_parent(parent.inner()?).map(LuaEntity::new))
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
        methods.add_method_mut(
            "push_child",
            |_, world, (parent, child): (LuaEntity, LuaEntity)| {
                world.push_child(parent.inner()?, child.inner()?);
                Ok(())
            },
        );

        methods.document("Removes children entities from the given parent entity.");
        methods.add_method(
            "remove_children",
            |_, world, (parent, children): (LuaEntity, Vec<LuaEntity>)| {
                let children = children
                    .iter()
                    .map(|e| e.inner())
                    .collect::<Result<Vec<_>, _>>()?;

                world.remove_children(parent.inner()?, &children);
                Ok(())
            },
        );

        methods.document("Removes child entity from the given parent entity.");
        methods.add_method(
            "remove_child",
            |_, world, (parent, child): (LuaEntity, LuaEntity)| {
                world.remove_children(parent.inner()?, &[child.inner()?]);
                Ok(())
            },
        );

        methods
            .document("Inserts children entities to the given parent entity at the given index.");
        methods.add_method(
            "insert_children",
            |_, world, (parent, index, children): (LuaEntity, LuaIndex, Vec<LuaEntity>)| {
                let children = children
                    .iter()
                    .map(|e| e.inner())
                    .collect::<Result<Vec<_>, _>>()?;

                world.insert_children(parent.inner()?, *index, &children);
                Ok(())
            },
        );

        methods.document("Inserts child entity to the given parent entity at the given index.");
        methods.add_method(
            "insert_child",
            |_, world, (parent, index, child): (LuaEntity, LuaIndex, LuaEntity)| {
                world.insert_children(parent.inner()?, *index, &[child.inner()?]);
                Ok(())
            },
        );

        methods.document("Despawns the given entity's children recursively");
        methods.add_method(
            "despawn_children_recursive",
            |_, world, entity: LuaEntity| {
                world.despawn_children_recursive(entity.inner()?);
                Ok(())
            },
        );

        methods.document("Despawns the given entity and the entity's children recursively");
        methods.add_method("despawn_recursive", |_, world, entity: LuaEntity| {
            world.despawn_recursive(entity.inner()?);
            Ok(())
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
