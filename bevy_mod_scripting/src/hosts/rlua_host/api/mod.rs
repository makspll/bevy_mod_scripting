pub mod base;
pub mod primitives;

use bevy::{
    prelude::*,
    reflect::{DynamicStruct, ReflectRef, TypeData, TypeRegistry},
};
use rlua::prelude::LuaError;
use rlua::{Context, FromLuaMulti, MetaMethod, ToLua, ToLuaMulti, UserData, Value};
use std::{
    cell::{Ref, UnsafeCell},
    fmt,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use crate::{base::LuaRef, base::PrintableReflect, LuaFile, Script, ScriptCollection};
use anyhow::{anyhow, Result};

pub use {base::*, primitives::*};

#[reflect_trait]
pub trait CustomUserData {
    /// a version of `rlua::to_lua` which does not consume the object
    fn ref_to_lua<'lua>(&self, lua: Context<'lua>) -> rlua::Result<Value<'lua>>;

    fn apply_lua<'lua>(&mut self, lua: Context<'lua>, new_val: Value<'lua>) -> rlua::Result<()>;
}

impl<T: Clone + UserData + Send + 'static> CustomUserData for T {
    fn ref_to_lua<'lua>(&self, lua: Context<'lua>) -> rlua::Result<Value<'lua>> {
        Ok(Value::UserData(lua.create_userdata(self.clone())?))
    }

    fn apply_lua<'lua>(&mut self, _lua: Context<'lua>, new_val: Value<'lua>) -> rlua::Result<()> {
        if let Value::UserData(v) = new_val {
            let s: Ref<T> = v.borrow::<T>()?;
            *self = s.clone();
            Ok(())
        } else {
            Err(rlua::Error::RuntimeError(
                "Error in assigning to custom user data".to_owned(),
            ))
        }
    }
}

pub struct LuaCustomUserData {
    val: LuaRef,
    refl: ReflectCustomUserData,
}

impl<'lua> ToLua<'lua> for LuaCustomUserData {
    fn to_lua(self, lua: Context<'lua>) -> rlua::Result<Value<'lua>> where {
        let refl = self.val.get();
        let usrdata = self.refl.get(refl);
        match usrdata {
            Some(v) => v.ref_to_lua(lua),
            None => todo!(),
        }
    }
}

/// A lua representation of an entity reference
#[derive(Clone)]
pub struct LuaEntity(pub Entity);

impl UserData for LuaEntity {
    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_method("id", |_, e, ()| Ok(e.0.id()));

        methods.add_method("generation", |_, e, ()| Ok(e.0.generation()));

        methods.add_method("bits", |_, e, ()| Ok(e.0.to_bits()));
    }
}

impl Deref for LuaEntity {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaEntity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A lua representation of a world reference.
#[derive(Clone)]
pub struct LuaWorld(pub *mut World);

unsafe impl Send for LuaWorld {}

pub fn get_type_data<T: TypeData + ToOwned<Owned = T>>(w: &mut World, name: &str) -> Result<T> {
    let registry: &TypeRegistry = w.get_resource().unwrap();

    let registry = registry.read();

    let reg = registry
        .get_with_short_name(&name)
        .or(registry.get_with_name(&name))
        .ok_or(LuaError::RuntimeError(format!(
            "Invalid component name {name}"
        )))
        .unwrap();

    let refl: T = reg
        .data::<T>()
        .ok_or(LuaError::RuntimeError(format!(
            "Invalid component name {name}"
        )))
        .unwrap()
        .to_owned();

    Ok(refl)
}

impl UserData for LuaWorld {
    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_method(
            "add_component",
            |_, w, (entity, comp_name): (LuaEntity, String)| {
                let w = unsafe { &mut *w.0 };

                let refl: ReflectComponent = get_type_data(w, &comp_name).unwrap();
                let def = get_type_data::<ReflectDefault>(w, &comp_name).unwrap();
                let s = def.default();
                refl.add_component(w, *entity, s.as_ref());

                Ok(LuaComponent {
                    comp: LuaRef(
                        refl.reflect_component(w, *entity).unwrap() as *const dyn Reflect
                            as *mut dyn Reflect,
                    ),
                    refl,
                })
            },
        );

        methods.add_method::<_, (LuaEntity, String), _, _>(
            "get_component",
            |_, w, (entity, comp_name)| {
                let w = unsafe { &mut *w.0 };

                let refl: ReflectComponent = get_type_data(w, &comp_name).unwrap();

                let dyn_comp = refl
                    .reflect_component(w, *entity)
                    .ok_or(LuaError::RuntimeError(
                        "Component not part of entity".to_string(),
                    ))
                    .unwrap();

                Ok(LuaComponent {
                    comp: LuaRef(dyn_comp as *const dyn Reflect as *mut dyn Reflect),
                    refl,
                })
            },
        );

        methods.add_method("new_script_entity", |_, w, name: String| {
            let w = unsafe { &mut *w.0 };

            w.resource_scope(|w, r: Mut<AssetServer>| {
                let handle = r.load::<LuaFile, _>(&name);
                Ok(LuaEntity(
                    w.spawn()
                        .insert(ScriptCollection::<crate::LuaFile> {
                            scripts: vec![Script::<LuaFile>::new(name, handle)],
                        })
                        .id(),
                ))
            })
        });

        methods.add_method("spawn", |_, w, ()| {
            let w = unsafe { &mut *w.0 };
            Ok(LuaEntity(w.spawn().id()))
        });
    }
}

#[derive(Clone)]
pub struct LuaComponent {
    comp: LuaRef,
    refl: ReflectComponent,
}

impl fmt::Debug for LuaComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LuaComponent")
            .field("comp", &self.comp)
            .finish()
    }
}

impl UserData for LuaComponent {
    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_meta_method(MetaMethod::ToString, |_, val, a: Value| {
            Ok(format!("{:#?}", PrintableReflect(val.comp.get())))
        });

        methods.add_meta_method(MetaMethod::Index, |ctx, val, field: String| {
            let r = val.comp.path_ref(&field).unwrap();
            Ok(r.convert_to_lua(ctx).unwrap())
        });

        methods.add_meta_method_mut(
            MetaMethod::NewIndex,
            |ctx, val, (field, new_val): (Value, Value)| {
                val.comp
                    .path_lua_val_ref(field)
                    .unwrap()
                    .apply_lua(ctx, new_val);
                Ok(())
            },
        )
    }
}

pub struct LuaResource {
    res: LuaRef,
}

impl UserData for LuaResource {
    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(_methods: &mut T) {}
}

impl UserData for LuaRef {
    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_meta_method(MetaMethod::ToString, |_, val, ()| {
            Ok(format!("{:#?}", PrintableReflect(val.get())))
        });

        methods.add_meta_method(MetaMethod::Index, |ctx, val, field: Value| {
            let r = val.path_lua_val_ref(field).unwrap();
            Ok(r.convert_to_lua(ctx).unwrap())
        });

        methods.add_meta_method_mut(
            MetaMethod::NewIndex,
            |ctx, val, (field, new_val): (Value, Value)| {
                val.path_lua_val_ref(field).unwrap().apply_lua(ctx, new_val);
                Ok(())
            },
        );

        methods.add_meta_method(MetaMethod::Len, |_, val, ()| {
            let r = val.get().reflect_ref();
            if let ReflectRef::List(v) = r {
                Ok(v.len())
            } else if let ReflectRef::Map(v) = r {
                Ok(v.len())
            } else if let ReflectRef::Tuple(v) = r {
                Ok(v.field_len())
            } else {
                panic!("Hello");
            }
        });

        methods.add_method("val", |mut ctx, val, ()| {
            Ok(reflect_to_lua(val.get(), ctx)
                .map_err(|e| LuaError::RuntimeError(e.to_string()))
                .unwrap())
        });
    }
}
