use std::{sync::{RwLock,Weak,Arc}, ffi::c_void};

use bevy::{prelude::*, reflect::GetPath};
use rlua::{UserData, LightUserData, MetaMethod, Value, FromLuaMulti};
use rlua::prelude::LuaError;
use std::ops::{Deref,DerefMut};

use crate::{ScriptCollection, LuaFile};
use anyhow::{anyhow,Result};

/// A lua representation of an entity reference
#[derive(Clone)]
pub struct LuaEntity(pub Entity);

impl UserData for LuaEntity {
    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_method("id", |_,e,()| {
            Ok(e.0.id())
        });

        methods.add_method("generation", |_,e,()| {
            Ok(e.0.generation())
        });

        methods.add_method("to_bits", |_,e,()| {
            Ok(e.0.to_bits())
        });
    }
}
impl Deref for LuaEntity {
    type Target=Entity;

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
pub struct LuaWorld(pub *mut World);

unsafe impl Send for LuaWorld {}

impl Deref for LuaWorld {
    type Target=*mut World;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaWorld {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl UserData for LuaWorld {
    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_method("get_script_component", |ctx,world,entity: LuaEntity |{
            let w = unsafe {&mut *world.0};

            let c = w.get_mut::<ScriptCollection<LuaFile>>(*entity);

            match c {
                Some(mut c) => {
                    let ptr = c.deref_mut() as *mut ScriptCollection<LuaFile>;
                    Ok(LuaComponent { comp: LuaReflect::Ref(ptr)})
                },
                None => Err(rlua::Error::RuntimeError("No script component found".to_string())),
            }
        });
    }
}

#[derive(Debug)]
pub struct LuaComponent{
    comp: LuaReflect
}

impl UserData for LuaComponent {
    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_meta_method_mut(MetaMethod::Index, |_,val,field : String|{
            let sub_field = val.comp.path_ref(&field).unwrap();
            Ok(sub_field)
        });

        methods.add_meta_method_mut::<(String,Value),_,_>(MetaMethod::NewIndex, |a,val,(field,new_val)|{
            val.comp.path_set(&field,new_val).unwrap();
            Ok(())
        })
    }

}

pub struct LuaResource {
    res: LuaReflect 
}

impl UserData for LuaResource {
    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(_methods: &mut T) {

    }

}


/// Represents a rust type but stored in lua. We try to store only references to actual rust data
/// to make data transfer cheaper, however sometimes lua has to 'own' some stuff, not in the 
/// 'i allocate this memory and handle' it sense, but rather the 'i requested this memory but i am handling it currently but with rust's permission'
/// 
/// We subdivide our lua types to those which own their content, and those which refer to owned content 
#[derive(Debug)]
pub enum LuaReflect {
    /// A rust object living in the bevy world, or alternatively
    /// a reference to a subfield of a lua owned value
    Ref(*mut (dyn Reflect + 'static)), 
    /// A rust object living in the lua world, not in the bevy world
    Owned(Box<dyn Reflect + 'static>),
}


impl UserData for LuaReflect {
    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_meta_method_mut(MetaMethod::Index, |_,val,field : String|{
            let sub_field = val.path_ref(&field).unwrap();
            Ok(sub_field)
        });

        methods.add_meta_method_mut::<(String,Value),_,_>(MetaMethod::NewIndex, |a,val,(field,new_val)|{
            val.path_set(&field, new_val).unwrap();
            Ok(())
        })
    }
}




impl LuaReflect {

    pub fn val(&mut self) -> Result<Box<dyn Reflect + 'static>> {
        match self {
            LuaReflect::Ref(r) => Ok(unsafe{ Box::from_raw(*r) }),
            LuaReflect::Owned(r) => Err(anyhow!("")),
        }
    }

    pub fn path_ref(&mut self, path: &str) -> Result<Self> {


        let ref_mut = match self {
            LuaReflect::Ref(r) =>{ 

                let r = unsafe{  &mut **r };
                info!("{},{}, {:?}",r.type_name(),path, self);

                r
            
            },
            LuaReflect::Owned(r) => r.as_mut()
        };

        let re = ref_mut.path_mut(path).map_err(|e| anyhow!(e.to_string()))?;
        Ok(Self::Ref(re as *mut dyn Reflect))
    }

    pub fn path_set(&mut self, path: &str, val : Value) -> Result<()> {
        let ref_mut = match self {
            LuaReflect::Ref(r) => unsafe{  &mut **r },
            LuaReflect::Owned(r) => r.as_mut()
        };

        let dyn_val : Box<dyn Reflect + 'static> = match val {
            Value::Boolean(v) => Box::new(v),
            Value::Integer(v) => Box::new(v),
            Value::Number(v) => Box::new(v),
            Value::String(v) => Box::new(v.to_str()?.to_owned()),
            Value::UserData(v) => {
                // can only set a field to another field
                if v.is::<LuaReflect>(){
                    let mut b = v.borrow_mut::<LuaReflect>().unwrap();

                    b.val()?
                } else {
                    return Err(anyhow!(""))
                }

            },
            Value::Function(_) => return Err(anyhow!("")),
            Value::Table(_) => return Err(anyhow!("")),
            Value::Error(_) => return Err(anyhow!("")),
            Value::Thread(_) => return Err(anyhow!("")),
            Value::Nil => return Err(anyhow!("")),
            Value::LightUserData(_) => return Err(anyhow!("")),

        };



        ref_mut.path_mut(path)
            .map_err(|e| anyhow!(""))?
            .set(dyn_val)
            .map_err(|e| anyhow!(""))
    }
}

unsafe impl Send for LuaReflect {}
