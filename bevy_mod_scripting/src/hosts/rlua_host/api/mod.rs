pub mod rust_primitives;

use bevy::{prelude::*, reflect::{DynamicStruct, TypeRegistry, TypeData, ReflectRef}};
use rlua::{UserData, MetaMethod, Value};
use rlua::prelude::LuaError;
use std::{sync::Arc,ops::{Deref,DerefMut}, cell::UnsafeCell, fmt};

use crate::{ScriptReflectVal, PrintableReflect, rust_primitives::LuaNumber};
use anyhow::{anyhow,Result};

pub use rust_primitives::*;

pub fn lua_to_reflect(v :Value) -> Result<Box<dyn Reflect + 'static>>{
    Ok(match v {
        Value::Boolean(v) => Box::new(v),
        Value::Integer(v) => Box::new(v),
        Value::Number(v) => Box::new(v),
        Value::String(v) => Box::new(v.to_str().unwrap().to_owned()),
        Value::UserData(v) => {
            // can only set a field to another field or primitive
            if v.is::<ScriptReflectVal>(){
                let mut b = v.borrow_mut::<ScriptReflectVal>().unwrap();

                b.deref_mut().to_owned()?
            } else if v.is::<LuaNumber>(){
                let mut b = v.borrow_mut::<LuaNumber>().unwrap();

                b.deref_mut().to_reflect()
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


    match v.reflect_ref() {
        bevy::reflect::ReflectRef::List(l) => {
            let i = l.iter()
                .map(|v| reflect_to_lua(v,ctx))
                .collect::<Result<Vec<Value<'lua>>>>()?
                .into_iter()
                .enumerate();

            ctx.create_table_from(i)
                .map(|v| Value::Table(v))
                .map_err(|e| anyhow!(e.to_string()))
        },
        bevy::reflect::ReflectRef::Struct(s) => {

            
            Ok(Value::Nil)
        },
        bevy::reflect::ReflectRef::TupleStruct(ts) => todo!(),
        bevy::reflect::ReflectRef::Tuple(t) => todo!(),

        bevy::reflect::ReflectRef::Map(m) => todo!(),
        bevy::reflect::ReflectRef::Value(v) => {
            if let Some(v) = v.downcast_ref::<bool>(){
                return Ok(Value::Boolean(*v))
            } else if let Some(v) = v.downcast_ref::<u32>() {
                return Ok(Value::Integer((*v) as i64))
            } else if let Some(v) = v.downcast_ref::<i64>() {
                return Ok(Value::Integer(*v))
            } else if let Some(v) = v.downcast_ref::<f64>() {
                return Ok(Value::Number(*v))
            } else if let Some(v) = v.downcast_ref::<String>(){
                return ctx.create_string(v)
                            .map(|v| Value::String(v)).map_err(|e| anyhow!("{}",e))
            } else {
                return Err(anyhow!("This type cannot be converted to a lua value: {:#?}",PrintableReflect(v)))
            }
        },
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
        methods.add_meta_method(MetaMethod::ToString, |_, val, ()| {
            Ok(format!("{:#?}",PrintableReflect(val.ref_immut())))
        });

        methods.add_meta_method(MetaMethod::Index, |_,val,field : Value|{    

            let r = val.ref_immut().reflect_ref();

            let r = match field {
                Value::Integer(idx) => {
                    let idx = idx as usize - 1;
                    match r {
                        ReflectRef::Tuple(v) => Ok(v.field(idx).unwrap()),
                        ReflectRef::TupleStruct(v) => Ok(v.field(idx).unwrap()),
                        ReflectRef::List(v) => Ok(v.get(idx).unwrap()),
                        ReflectRef::Map(v) => Ok(v.get(&(idx)).unwrap()),
                        _ => Err(LuaError::RuntimeError("".to_string()))
                    }
                },
                Value::String(field) => {
                    let path = field.to_str().unwrap();
                    match r {
                        ReflectRef::Map(v) => Ok(v.get(&path.to_owned()).unwrap()),
                        ReflectRef::Struct(v) => Ok(v.field(path).unwrap()),
                        _ => Err(LuaError::RuntimeError("".to_string()))
                    }

                },
                _ => panic!("ASDASD")
            }?;

            Ok(ScriptReflectVal::Ref(r as *const dyn Reflect as *mut dyn Reflect))
        });

        methods.add_meta_method(MetaMethod::Len, |_,val,()|{    
            let r =  val.ref_immut().reflect_ref();
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

        methods.add_meta_method_mut::<(String,Value),_,_>(MetaMethod::NewIndex, |_,val,(field,new_val)|{
            val.path_set(&field, lua_to_reflect(new_val).unwrap()).unwrap();
            Ok(())
        });
        

        methods.add_method("val",|mut ctx,val,()|{
            Ok(reflect_to_lua(val.ref_immut(),&mut ctx).map_err(|e| LuaError::RuntimeError(e.to_string())).unwrap())
        });

        methods.add_meta_method(MetaMethod::Add, |_,val,o : Value|{
            let l = LuaNumber::from_reflect(val.ref_immut()).unwrap();
            let r = LuaNumber::from_lua(o,l.type_name()).unwrap();
            Ok(l.add(r).unwrap())
        })
    }
}





