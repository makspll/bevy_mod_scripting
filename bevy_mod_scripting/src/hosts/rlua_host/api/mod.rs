pub mod primitives;
pub mod base;

use bevy::{prelude::*, reflect::{DynamicStruct, TypeRegistry, TypeData, ReflectRef}};
use rlua::{UserData, MetaMethod, Value, FromLuaMulti,ToLuaMulti};
use rlua::prelude::LuaError;
use std::{sync::Arc,ops::{Deref,DerefMut}, cell::UnsafeCell, fmt};

use crate::{base::LuaRef, base::PrintableReflect, ScriptCollection, LuaFile, Script};
use anyhow::{anyhow,Result};

pub use {primitives::*,base::*};


pub fn apply_lua_to_reflect(v :Value, dest : &mut dyn Reflect) -> Result<()>{

    match v {
        Value::Boolean(b) => LuaNumber::Usize(b as usize).reflect_apply(dest),
        Value::Integer(i) => LuaNumber::I64(i).reflect_apply(dest),
        Value::Number(n) => LuaNumber::F64(n).reflect_apply(dest),
        Value::String(v) => {dest.apply(&v.to_str().unwrap().to_owned()); Ok(()) },
        Value::UserData(v) => {
                // can only set a field to another field or primitive
                if v.is::<LuaRef>(){
                    let mut b = v.borrow_mut::<LuaRef>().unwrap();
                    dest.apply(b.get());
                    Ok(())
                } else {
                    return Err(anyhow!(""))
                }
            },
        _ => return Err(anyhow!("Type not supported")),

    }
}

pub fn reflect_to_lua<'s,'lua>(v : &'s dyn Reflect, ctx : &mut rlua::Context<'lua> ) -> Result<Value<'lua>>{


    match v.reflect_ref() {
        ReflectRef::List(l) => {
            let i = l.iter()
                .map(|v| reflect_to_lua(v,ctx))
                .collect::<Result<Vec<Value<'lua>>>>()?
                .into_iter()
                .enumerate();

            ctx.create_table_from(i)
                .map(|v| Value::Table(v))
                .map_err(|e| anyhow!(e.to_string()))
        },
        ReflectRef::Value(v) => {
            if let Some(c) = REFLECT_TO_LUA_CONVERSIONS.get(v.type_name()){
                return Ok(c(v,ctx))
            } else if let Some(v) = v.downcast_ref::<String>(){
                return ctx.create_string(v)
                            .map(|v| Value::String(v)).map_err(|e| anyhow!("{}",e))
            } else {
                return Err(anyhow!("This type cannot be converted to a lua value: {:#?}",PrintableReflect(v)))
            }
        },
        ReflectRef::Struct(s) => {
            Ok(Value::Nil)
        },
        ReflectRef::TupleStruct(ts) => todo!(),
        ReflectRef::Tuple(t) => todo!(),
        ReflectRef::Map(m) => todo!(),
        ReflectRef::Array(_) => todo!(),
        
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
        methods.add_method("add_component", |_,w,(entity, comp_name ): (LuaEntity,String) |{
            let w = unsafe { &mut *w.0 };

            let refl : ReflectComponent = get_type_data(w,&comp_name).unwrap();
            let def = get_type_data::<ReflectDefault>(w,&comp_name).unwrap();
            let s = def.default();
            refl.add_component(w, *entity, s.as_ref());

            Ok(LuaComponent {
                comp: LuaRef(refl.reflect_component(w, *entity).unwrap() as *const dyn Reflect as *mut dyn Reflect),
                refl
            })
        });

        methods.add_method::<_,(LuaEntity,String),_,_>("get_component", |_,w,(entity, comp_name)|{
            let w = unsafe { &mut *w.0 };
            
            let refl : ReflectComponent = get_type_data(w,&comp_name).unwrap();

            let dyn_comp = refl.reflect_component(w, *entity)
                .ok_or(LuaError::RuntimeError("Component not part of entity".to_string())).unwrap();

            Ok(LuaComponent{
                comp: LuaRef(dyn_comp as *const dyn Reflect as *mut dyn Reflect),
                refl,
            })
        });

        methods.add_method("new_script_entity", |_,w,name : String|{
            let w = unsafe { &mut *w.0 };

            w.resource_scope(|w,r : Mut<AssetServer>|{
                let handle = r.load::<LuaFile,_>(&name);
                Ok(LuaEntity(w.spawn()
                    .insert(ScriptCollection::<crate::LuaFile>{
                        scripts: vec![
                            Script::<LuaFile>::new(name,handle)
                        ],
                    }).id()))
            })
        });

        methods.add_method("spawn", |_,w,()|{
            let w = unsafe { &mut *w.0 };        
            Ok(LuaEntity(w.spawn().id()))
        });
    }
}

#[derive(Clone)]
pub struct LuaComponent{
    comp: LuaRef,
    refl: ReflectComponent
}

impl fmt::Debug for LuaComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LuaComponent").field("comp", &self.comp).finish()
    }
}


impl UserData for LuaComponent {
    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_meta_method(MetaMethod::ToString, |_, val, a : Value| {
            Ok(format!("{:#?}",PrintableReflect(val.comp.get())))
        });

        methods.add_meta_method(MetaMethod::Index, |_,val,field : String|{ 
    
            Ok(val.comp.path_ref(&field).unwrap())
        });

        methods.add_meta_method_mut(MetaMethod::NewIndex, |_,val,(field,new_val) : (Value,Value)|{  
            apply_lua_to_reflect(new_val,val.comp.path_lua_val_ref(field).unwrap().get_mut()).unwrap();
            Ok(())
        })
    }

}

pub struct LuaResource {
    res: LuaRef 
}

impl UserData for LuaResource {
    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(_methods: &mut T) {

    }

}


impl UserData for LuaRef {
    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_meta_method(MetaMethod::ToString, |_, val, ()| {
            Ok(format!("{:#?}",PrintableReflect(val.get())))
        });

        methods.add_meta_method(MetaMethod::Index, |ctx,val,field : Value|{    
                let r = val.path_lua_val_ref(field).unwrap();

                if let Some(c) =  REFLECT_TO_LUA_CONVERSIONS.get(r.get().type_name()){
                    Ok(c(r.get(),&ctx))
                } else {
                    Ok(Value::UserData(ctx.create_userdata(r).unwrap()))
                }
        });

        methods.add_meta_method_mut(MetaMethod::NewIndex, |_,val,(field,new_val): (Value,Value)|{
            apply_lua_to_reflect(new_val,val.path_lua_val_ref(field).unwrap().get_mut()).unwrap();
            Ok(())
        });

        methods.add_meta_method(MetaMethod::Len, |_,val,()|{    
            let r =  val.get().reflect_ref();
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

        methods.add_method("val",|mut ctx,val,()|{
            Ok(reflect_to_lua(val.get(),&mut ctx).map_err(|e| LuaError::RuntimeError(e.to_string())).unwrap())
        });
    }
}





