pub mod bevy_types;
use std::{sync::{Arc,Mutex}, cell::UnsafeCell};
use bevy::{
    prelude::*,
    reflect::{ReflectRef, TypeRegistry, GetPath, ReflectMut},
};
use rlua::{Context, MetaMethod, ToLua, UserData, Value};
use std::{
    cell::Ref,
    fmt,
};

use crate::{PrintableReflect, PrintReflect};
use anyhow::Result;

pub use bevy_types::*;

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
        let refl = self.val.get() ;
        let usrdata = self.refl.get(refl);
        match usrdata {
            Some(v) => v.ref_to_lua(lua),
            None => todo!(),
        }
    }
}




/// The base of a reference, i.e. the top-level object which owns the underlying value
#[derive(Clone)]
pub enum LuaRefBase {
    /// A bevy component reference
    Component{
        comp: ReflectComponent,
        entity: Entity,
        world: *mut World,
    },
    // A lua owned reflect type (for example a vector constructed in lua)
    LuaOwned
}


#[derive(Clone)]
pub enum LuaPtr {
    Const(*const (dyn Reflect + 'static)),
    Mut(*mut (dyn Reflect + 'static))
}

/// A reference to a rust type available from lua.
/// References can be either to rust or lua managed values (created either on the bevy or script side).
/// but also to any subfield of those values (All pointed to values must support `reflect`).
/// Each reference holds a reflection path from the root.
#[derive(Clone)]
pub struct LuaRef{
    /// The underlying top-level value 
    root: LuaRefBase,

    /// The reflection path from the root
    path: Option<String>,

    /// A read-only 'current' pointer pointing to the reflected field,
    /// the purpose of this pointer is to avoid reflection through the path when not necessary
    /// and to perform type checking at each field access
    r: LuaPtr
}



unsafe impl Send for LuaRef {}

impl fmt::Debug for LuaRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().print(f)
    }
}

impl fmt::Display for LuaRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl LuaRef {

    pub fn get(&self) -> &(dyn Reflect + 'static) {
        unsafe {
            match self.r {
                LuaPtr::Const(r) => r.as_ref().expect("Invalid pointer"),
                LuaPtr::Mut(r) => r.as_ref().expect("Invalid pointer"),
            }
        }
    }

    pub fn get_mut(&mut self) -> &mut (dyn Reflect + 'static) {
        unsafe {
            match &self.root {
                LuaRefBase::Component { comp, entity, world } => 
                    comp.reflect_component_mut(world.as_mut().expect("Invalid pointer"), *entity)
                        .unwrap()
                        .into_inner()
                        .path_mut(&self.path.as_ref().expect("No reflection path available"))
                        .unwrap(),
                LuaRefBase::LuaOwned => match self.r {
                    LuaPtr::Mut(ptr) => ptr.as_mut().unwrap(),
                    _ => panic!("Cannot get_mut without pointer")
                }
            }
        }
    }

    pub unsafe fn path_ref(&self, path: &str) -> Result<Self,rlua::Error> {
        let ref_mut = self.get();

        let re = ref_mut
            .path(path)
            .map_err(|_e| rlua::Error::RuntimeError(format!("Cannot access field `{}`", path)))?;

        Ok(LuaRef {
            root: self.root.clone(),
            path: Some(format!("{}{}",self.path.as_ref().unwrap(), path)),
            r: LuaPtr::Const(re),
        })
    }

    pub unsafe fn path_lua_val_ref(&self, path: Value) -> Result<Self,rlua::Error> {
        let r = self.get().reflect_ref();

        let (path,v) = match path {
            Value::Integer(idx) => {
                let idx = idx as usize - 1;
                let path_str = format!("[{idx}]");
                let field = match r {
                    ReflectRef::Tuple(v) => v.field(idx).unwrap(),
                    ReflectRef::TupleStruct(v) => v.field(idx).unwrap(),
                    ReflectRef::List(v) => v.get(idx).unwrap(),
                    ReflectRef::Map(v) => v.get(&(idx)).unwrap(),
                    _ => return Err(rlua::Error::RuntimeError(format!("Tried to index a primitive rust type {:#?}", self))),
                };

                (path_str,field)
            }
            Value::String(field) => {
                let path_str = field.to_str().unwrap().to_string();
                let field = match r {
                    ReflectRef::Map(v) => v.get(&path_str.to_owned()).unwrap(),
                    ReflectRef::Struct(v) => v.field(&path_str).unwrap(),
                    _ => return Err(rlua::Error::RuntimeError(format!("Tried to index a primitive rust type {:#?}", self))),
                };

                (path_str,field)
            }
            _ => return Err(rlua::Error::RuntimeError(format!("Cannot index a rust object with {:?}", path))),
        };

        Ok(LuaRef{ 
            root: self.root.clone(), 
            path: Some(format!("{}{}",self.path.as_ref().unwrap(),path)), 
            r: LuaPtr::Const(v)
        })
    }

    pub unsafe fn convert_to_lua<'lua>(self, ctx: Context<'lua>) -> Result<Value<'lua>,rlua::Error> {
        println!("Converting luaref {self} {:p}", self.get());

        if let Some(f) = BEVY_TO_LUA.get(self.get().type_name()){
            Ok(f(&self,ctx))
        } else {
            let w = &mut *(ctx.globals().get::<_, LuaWorld>("world").unwrap()).0;
            let typedata = w.resource::<TypeRegistry>();
            let g = typedata.read();

            if let Some(v) = g.get_type_data::<ReflectCustomUserData>(self.get().type_id()) {
                Ok(v.get(self.get()).unwrap().ref_to_lua(ctx).unwrap())
            } else {
                Ok(Value::UserData(ctx.create_userdata(self).unwrap()))
            }
        } 
    }

    pub unsafe fn apply_lua<'lua>(&mut self, ctx: Context<'lua>, v: Value<'lua>) -> Result<(),rlua::Error> {
        println!("Applying lua to luaref {:p} with {:?}", self.get(), v);
        if let Some(f) = APPLY_LUA_TO_BEVY.get(self.get().type_name()) {
            return f(self,ctx,v)
        } else {
            let w = &mut *(ctx.globals().get::<_, LuaWorld>("world").unwrap()).0 ;
            let typedata = w.resource::<TypeRegistry>();
            let g = typedata.read();
    
            if let Some(ud) = g.get_type_data::<ReflectCustomUserData>(self.get().type_id()){

                ud.get_mut(self.get_mut())
                    .unwrap()
                    .apply_lua(ctx, v)
                    .unwrap();
                return Ok(())
            } else if let Value::UserData(v) = v {
                if v.is::<LuaRef>() {
                    let b = v.borrow_mut::<LuaRef>().unwrap();
                    self.get_mut().apply(b.get());
                    return Ok(())
                }
            } 
        }; 

        Err(rlua::Error::RuntimeError("Invalid assignment".to_owned()))
        
    }
}



impl UserData for LuaRef {
    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_meta_method(MetaMethod::ToString, |_, val, ()| {
            Ok(format!("{:#?}", PrintableReflect(unsafe{val.get()})))
        });

        methods.add_meta_method_mut(MetaMethod::Index, |ctx, val, field: Value| {
            let r = unsafe { val.path_lua_val_ref(field) }.unwrap();
            Ok(unsafe{ r.convert_to_lua(ctx) } .unwrap())
        });

        methods.add_meta_method_mut(
            MetaMethod::NewIndex,
            |ctx, val, (field, new_val): (Value, Value)| {
                unsafe { val.path_lua_val_ref(field).unwrap().apply_lua(ctx, new_val) };
                Ok(())
            },
        );

        methods.add_meta_method(MetaMethod::Len, |_, val, ()| {
            let r = unsafe{ val.get() }.reflect_ref();
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

    }
}


