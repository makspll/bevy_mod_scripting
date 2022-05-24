use bevy::reflect::*;
use rlua::{Context, Value};
use std::fmt;

use phf::{phf_map, Map};

use crate::{LuaWorld, ReflectCustomUserData, PrintReflect, APPLY_LUA_TO_BEVY, BEVY_TO_LUA};

/// A rust type representation in lua
#[derive(Clone)]
pub struct LuaRef(pub(crate) *mut (dyn Reflect + 'static));

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
        assert!(!self.0.is_null());
        unsafe { &*self.0 }
    }

    pub fn get_mut(&mut self) -> &mut dyn Reflect {
        assert!(!self.0.is_null());
        unsafe { &mut *self.0 }
    }

    pub fn path_ref(&self, path: &str) -> Result<Self,rlua::Error> {
        let ref_mut = self.get();

        let re = ref_mut
            .path(path)
            .map_err(|_e| rlua::Error::RuntimeError(format!("Cannot access field `{}`", path)))?;
        Ok(Self(re as *const dyn Reflect as *mut dyn Reflect))
    }

    pub fn path_lua_val_ref(&self, path: Value) -> Result<Self,rlua::Error> {
        let r = self.get().reflect_ref();

        match path {
            Value::Integer(idx) => {
                let idx = idx as usize - 1;
                match r {
                    ReflectRef::Tuple(v) => Ok(v.field(idx).unwrap()),
                    ReflectRef::TupleStruct(v) => Ok(v.field(idx).unwrap()),
                    ReflectRef::List(v) => Ok(v.get(idx).unwrap()),
                    ReflectRef::Map(v) => Ok(v.get(&(idx)).unwrap()),
                    _ => Err(rlua::Error::RuntimeError(format!("Tried to index a primitive rust type {:#?}", self))),
                }
            }
            Value::String(field) => {
                let path = field.to_str().unwrap();
                match r {
                    ReflectRef::Map(v) => Ok(v.get(&path.to_owned()).unwrap()),
                    ReflectRef::Struct(v) => Ok(v.field(path).unwrap()),
                    _ => Err(rlua::Error::RuntimeError(format!("Tried to index a primitive rust type {:#?}", self))),
                }
            }
            _ => Err(rlua::Error::RuntimeError(format!("Cannot index a rust object with {:?}", path))),
        }
        .map(|v| LuaRef(v as *const dyn Reflect as *mut dyn Reflect))
    }

    pub fn convert_to_lua<'lua>(self, ctx: Context<'lua>) -> Result<Value<'lua>,rlua::Error> {
        
        if let Some(f) = BEVY_TO_LUA.get(self.get().type_name()){
            Ok(f(self.get(),ctx))
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

    pub fn apply_lua<'lua>(&mut self, ctx: Context<'lua>, v: Value<'lua>) -> Result<(),rlua::Error> {

        if let Some(f) = APPLY_LUA_TO_BEVY.get(self.get().type_name()) {
            println!("{}",self.get().type_name());
            return f(self.get_mut(),ctx,v)
        } else {
            let w = unsafe { &mut *(ctx.globals().get::<_, LuaWorld>("world").unwrap()).0 };
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

unsafe impl Send for LuaRef {}



