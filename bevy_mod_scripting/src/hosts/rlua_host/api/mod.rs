pub mod bevy_types;

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
        let refl = unsafe { self.val.get() } ;
        let usrdata = self.refl.get(refl);
        match usrdata {
            Some(v) => v.ref_to_lua(lua),
            None => todo!(),
        }
    }
}


#[derive(Clone)]
pub struct LuaRef(pub(crate) *mut (dyn Reflect + 'static ));


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

    pub fn get(&self) -> &dyn Reflect {
        unsafe {
            println!("Getting luaref: {:p}, {:?}", self.0, self.0.as_ref().expect("").type_name());
            self.0.as_ref().expect("Invalid pointer")
        }
    }

    pub fn get_mut(&mut self) -> &mut dyn Reflect {
        unsafe {
            println!("Getting luaref mut: {:p}, {:?}", self.0, self.0.as_ref().expect("").type_name());
            self.0.as_mut().expect("Invalid pointer")
        }
    }

    pub unsafe fn path_ref(&mut self, path: &str) -> Result<Self,rlua::Error> {
        println!("Indexing luaref {:p} with {}", self.get(),path);
        let ref_mut = self.get_mut();

        let re = ref_mut
            .path_mut(path)
            .map_err(|_e| rlua::Error::RuntimeError(format!("Cannot access field `{}`", path)))?;
        Ok(Self(re as *mut dyn Reflect))
    }

    pub unsafe fn path_lua_val_ref(&mut self, path: Value) -> Result<Self,rlua::Error> {
        println!("Indexing luaref {:p} with {:?}", self.get(),path);

        let r = self.get_mut().reflect_mut();

        match path {
            Value::Integer(idx) => {
                let idx = idx as usize - 1;
                match r {
                    ReflectMut::Tuple(v) => Ok(v.field_mut(idx).unwrap()),
                    ReflectMut::TupleStruct(v) => Ok(v.field_mut(idx).unwrap()),
                    ReflectMut::List(v) => Ok(v.get_mut(idx).unwrap()),
                    ReflectMut::Map(v) => Ok(v.get_mut(&(idx)).unwrap()),
                    _ => Err(rlua::Error::RuntimeError(format!("Tried to index a primitive rust type {:#?}", self))),
                }
            }
            Value::String(field) => {
                let path = field.to_str().unwrap();
                match r {
                    ReflectMut::Map(v) => Ok(v.get_mut(&path.to_owned()).unwrap()),
                    ReflectMut::Struct(v) => Ok(v.field_mut(path).unwrap()),
                    _ => Err(rlua::Error::RuntimeError(format!("Tried to index a primitive rust type {:#?}", self))),
                }
            }
            _ => Err(rlua::Error::RuntimeError(format!("Cannot index a rust object with {:?}", path))),
        }
        .map(|v| LuaRef(v as *mut dyn Reflect))
    }

    pub unsafe fn convert_to_lua<'lua>(mut self, ctx: Context<'lua>) -> Result<Value<'lua>,rlua::Error> {
        println!("Converting luaref {self} {:p}", self.get());

        if let Some(f) = BEVY_TO_LUA.get(self.get().type_name()){
            Ok(f(self.get_mut(),ctx))
        } else {
            let w = unsafe { &mut *(ctx.globals().get::<_, LuaWorld>("world").unwrap()).0 };
            let typedata = w.resource::<TypeRegistry>();
            let g = typedata.read();

            if let Some(v) = g.get_type_data::<ReflectCustomUserData>(self.get().type_id()) {
                Ok(v.get(self.get()).unwrap().ref_to_lua(ctx).unwrap())
            } else {
                Ok(Value::UserData(ctx.create_userdata(self).unwrap()))
            }
        } 
        // another case for an enumeration of 
    }

    pub unsafe fn apply_lua<'lua>(&mut self, ctx: Context<'lua>, v: Value<'lua>) -> Result<(),rlua::Error> {
        println!("Applying lua to luaref {:p} with {:?}", self.get(), v);
        if let Some(f) = APPLY_LUA_TO_BEVY.get(self.get().type_name()) {
            return f(self.get_mut(),ctx,v)
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


