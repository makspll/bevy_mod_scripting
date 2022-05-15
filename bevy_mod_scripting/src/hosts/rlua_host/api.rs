
use bevy::{prelude::*, reflect::{DynamicStruct, TypeRegistry, TypeData},ecs::reflect::*};
use rlua::{UserData, MetaMethod, Value};
use rlua::prelude::LuaError;
use std::{sync::Arc,ops::{Deref,DerefMut}, cell::UnsafeCell, fmt};

use crate::{ScriptReflectVal};
use anyhow::{anyhow,Result};



pub fn lua_to_reflect(v :Value) -> Result<Box<dyn Reflect + 'static>>{
    Ok(match v {
        Value::Boolean(v) => Box::new(v),
        Value::Integer(v) => Box::new(v),
        Value::Number(v) => Box::new(v),
        Value::String(v) => Box::new(v.to_str().unwrap().to_owned()),
        Value::UserData(v) => {
            // can only set a field to another field
            if v.is::<ScriptReflectVal>(){
                let mut b = v.borrow_mut::<ScriptReflectVal>().unwrap();

                b.deref_mut().to_owned()?
            } else {
                return Err(anyhow!(""))
            }

        },
        Value::Function(_) => return Err(anyhow!("Type not supported")),
        Value::Table(_) => return Err(anyhow!("Type not supported")),
        Value::Error(_) => return Err(anyhow!("Type not supported")),
        Value::Thread(_) => return Err(anyhow!("Type not supported")),
        Value::Nil => return Err(anyhow!("Type not supported")),
        Value::LightUserData(_) => return Err(anyhow!("Type not supported")),

    })
}

pub fn reflect_to_lua<'s,'lua>(v : &'s dyn Reflect, ctx : &mut rlua::Context<'lua> ) -> Result<Value<'lua>>{
    if let Some(v) = v.downcast_ref::<bool>(){
        return Ok(Value::Boolean(*v))
    } else if let Some(v) = v.downcast_ref::<i64>() {
        return Ok(Value::Integer(*v))
    } else if let Some(v) = v.downcast_ref::<f64>() {
        return Ok(Value::Number(*v))
    } else if let Some(v) = v.downcast_ref::<String>(){
        return ctx.create_string(v).map(|v| Value::String(v)).map_err(|e| anyhow!("{}",e))
    } else {
        return Err(anyhow!("Object was not a primitive"))
    }
}


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

        methods.add_method("bits", |_,e,()| {
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
#[derive(Clone)]
pub struct LuaWorld(pub *mut World);

unsafe impl Send for LuaWorld {}



pub fn get_type_data<T : TypeData + ToOwned<Owned=T>>(w : &mut World, name : &str) -> Result<T>{
    let registry: &TypeRegistry = w.get_resource().unwrap();

    let registry = registry.read();


    let reg = 
    registry.get_with_short_name(&name)
            .or(registry.get_with_name(&name))
            .ok_or(LuaError::RuntimeError(format!("Invalid component name {name}"))).unwrap();

    info!("{},{}",reg.name(),reg.short_name());
    let refl : T = reg.data::<T>()
                                    .ok_or(LuaError::RuntimeError(format!("Invalid component name {name}"))).unwrap()
                                    .to_owned();

    Ok(refl)
}


impl UserData for LuaWorld {
    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_method("new_component", |_,w,comp_name : String |{
            let w = unsafe { &mut *w.0 };

            let refl = get_type_data(w,&comp_name).map_err(|e| LuaError::RuntimeError(e.to_string()))?;

            let mut s = DynamicStruct::default();
            s.set_name(comp_name);

            Ok(LuaComponent {
                comp: ScriptReflectVal::Owned(Arc::new(UnsafeCell::new(s))),
                refl
            })
        });

        methods.add_method::<_,(LuaEntity,String),_,_>("get_component", |_,w,(entity, comp_name)|{
            let w = unsafe { &mut *w.0 };
            
            let refl : ReflectComponent = get_type_data(w,&comp_name).map_err(|e| LuaError::RuntimeError(e.to_string())).unwrap();

            let dyn_comp = refl.reflect_component(w, *entity)
                .ok_or(LuaError::RuntimeError("Component not part of entity".to_string())).unwrap();

            Ok(LuaComponent{
                comp: ScriptReflectVal::Ref(dyn_comp as *const dyn Reflect as *mut dyn Reflect),
                refl,
            })
        });

        methods.add_method("spawn", |_,w,components : Vec<LuaComponent> |{
            
            let w = unsafe { &mut *w.0 };

            let e = w.spawn().id();

            components.iter().for_each(|c| {
                c.refl.add_component(w, e, c.comp.ref_immut())
            });
        
            Ok(LuaEntity(e))
        });
    }
}

#[derive(Clone)]
pub struct LuaComponent{
    comp: ScriptReflectVal,
    refl: ReflectComponent
}

impl fmt::Debug for LuaComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LuaComponent").field("comp", &self.comp).finish()
    }
}


impl UserData for LuaComponent {
    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_meta_method(MetaMethod::Index, |_,val,field : String|{
            Ok(val.comp.path_ref(&field).unwrap())
        });

        methods.add_meta_method_mut::<(String,Value),_,_>(MetaMethod::NewIndex, |a,val,(field,new_val)|{
            val.comp.path_set(&field,lua_to_reflect(new_val).unwrap()).unwrap();
            Ok(())
        })
    }

}

pub struct LuaResource {
    res: ScriptReflectVal 
}

impl UserData for LuaResource {
    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(_methods: &mut T) {

    }

}


impl UserData for ScriptReflectVal {
    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_meta_method(MetaMethod::Index, |_,val,field : String|{
            Ok(val.path_ref(&field).unwrap())
        });

        methods.add_meta_method_mut::<(String,Value),_,_>(MetaMethod::NewIndex, |_,val,(field,new_val)|{
            val.path_set(&field, lua_to_reflect(new_val).unwrap()).unwrap();
            Ok(())
        });

        methods.add_method("val",|mut ctx,val,()|{
            reflect_to_lua(val.ref_immut(),&mut ctx).map_err(|e| LuaError::RuntimeError(e.to_string()))
        });
    }
}

