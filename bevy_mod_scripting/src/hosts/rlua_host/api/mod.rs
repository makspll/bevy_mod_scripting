pub mod bevy_types;
use std::{ops::DerefMut,sync::Weak};
use parking_lot::{RwLock};
use bevy::{
    prelude::*,
    reflect::{ReflectRef, TypeRegistry, GetPath},
};
use rlua::{Context, MetaMethod, ToLua, UserData, Value};
use std::{
    cell::Ref,
    fmt,
};

use crate::{PrintableReflect};
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
        self.val.get(|s,_| {
            let usrdata = self.refl.get(s);
            match usrdata {
                Some(v) => v.ref_to_lua(lua),
                None => todo!(),
            }
        })
    }
}




/// The base of a reference, i.e. the top-level object which owns the underlying value
#[derive(Clone)]
pub enum LuaRefBase {
    /// A bevy component reference
    Component{
        comp: ReflectComponent,
        entity: Entity,
        world: Weak<RwLock<World>>,
    },
    // A lua owned reflect type (for example a vector constructed in lua)
    LuaOwned
}

impl fmt::Debug for LuaRefBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Component { comp, entity, world } => f.debug_struct("Component").field("entity", entity).field("world", world).finish(),
            Self::LuaOwned => write!(f, "LuaOwned"),
        }
    }
}


// #[derive(Clone,Debug)]
// pub enum LuaPtr {
//     Const(*const dyn Reflect),
//     Mut(*mut dyn Reflect)
// }

/// A reference to a rust type available from lua.
/// References can be either to rust or lua managed values (created either on the bevy or script side).
/// but also to any subfield of those values (All pointed to values must support `reflect`).
/// Each reference holds a reflection path from the root.
#[derive(Clone,Debug)]
pub struct LuaRef{
    /// The underlying top-level value 
    root: LuaRefBase,

    /// The reflection path from the root
    path: Option<String>,

    // A read-only 'current' pointer pointing to the reflected field,
    // the purpose of this pointer is to avoid reflection through the path when not necessary
    // and to perform type checking at each field access
    // r: LuaPtr
}




// impl fmt::Debug for LuaRef {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // self.get().print(f)

//     }
// }

impl fmt::Display for LuaRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}


impl LuaRef {

    
    pub fn get<O,F>(&self, f: F) -> O  where 
        F : FnOnce(&dyn Reflect, &LuaRef) -> O
    {
        match &self.root {
            LuaRefBase::Component { comp, entity, world } => {
                
                let g = world.upgrade().unwrap();
                let g = g.read();

                let dyn_ref = comp.reflect_component(&g, *entity)
                    .unwrap()
                    .path(&self.path.as_ref().expect("No reflection path available"))
                    .unwrap();

                f(dyn_ref,self)

            },
            LuaRefBase::LuaOwned => todo!(),
        }
    }

    pub fn get_mut<O,F>(&mut self, f: F) -> O  where 
        F : FnOnce(&mut dyn Reflect, &mut LuaRef) -> O
    {
        match &self.root {
            LuaRefBase::Component { comp, entity, world } => {
                
                let g = world.upgrade().unwrap();
                let mut g = g.write();

                let mut ref_mut = comp.reflect_component_mut(&mut g, *entity)
                    .unwrap();

                let dyn_ref = ref_mut
                    .path_mut(&self.path.as_ref().expect("No reflection path available"))
                    .unwrap();

                f(dyn_ref,self)
            },
            LuaRefBase::LuaOwned => todo!(),
        }    
    }

    pub fn path_ref(&self, path: &str) -> Result<Self,rlua::Error> {
        self.get(|s,_| {
            s.path(path)
            .map_err(|_e| rlua::Error::RuntimeError(format!("Cannot access field `{}`", path)))?;

            Ok(LuaRef {
                root: self.root.clone(),
                path: Some(format!("{}{}",self.path.as_ref().unwrap(), path)),
                // r: LuaPtr::Const(re),
            })
        })


    }

    pub fn path_lua_val_ref(&self, path: Value) -> Result<Self,rlua::Error> {
        self.get(|s,_| {
            let r = s.reflect_ref();

            let (path,_) = match path {
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
                // r: LuaPtr::Const(v)
            })
        })
    
    }

    pub fn convert_to_lua<'lua>(self, ctx: Context<'lua>) -> Result<Value<'lua>,rlua::Error> {
        self.get(|s,_| {
            if let Some(f) = BEVY_TO_LUA.get(s.type_name()){
                Ok(f(&self,ctx))
            } else {
                let luaworld = ctx.globals()
                    .get::<_, LuaWorld>("world")
                    .unwrap();

                let world = luaworld.0.read();
    
                let typedata = world.resource::<TypeRegistry>();
                let g = typedata.read();
    
                if let Some(v) = g.get_type_data::<ReflectCustomUserData>(s.type_id()) {
                    Ok(v.get(s).unwrap().ref_to_lua(ctx).unwrap())
                } else {
                    Ok(Value::UserData(ctx.create_userdata(self.clone()).unwrap()))
                }
            } 
        })
    }

    /// applies another luaref by carefuly acquiring locks and cloning
    pub fn apply_luaref(&mut self, o : &LuaRef){


        let cloned = o.get(|s,_| s.clone_value());
        // sadly apply already performs a clone for value types, so this incurs
        // a double clone in some cases TODO: is there another way ?
        // can we avoid the box ?
        self.get_mut(|s,_| s.apply(&*cloned));
    }

    pub fn apply_lua<'lua>(&mut self, ctx: Context<'lua>, v: Value<'lua>) -> Result<(),rlua::Error> {


        let type_name = self.get_mut(|s,_| s.type_name().to_owned());

        if let Some(f) = APPLY_LUA_TO_BEVY.get(&type_name) {
            return f(self,ctx,v)
        };
            

        let luaworld = ctx.globals()
                                    .get::<_, LuaWorld>("world")
                                    .unwrap();

        // remove typedata from the world to be able to manipulate world 
        let typedata = {
            luaworld.0.write().remove_resource::<TypeRegistry>().unwrap()
        };

        let g = typedata.read();

        let v = match self.get_mut(|mut s,_| {
            if let Some(ud) = g.get_type_data::<ReflectCustomUserData>(s.type_id()){

                ud.get_mut(s.deref_mut())
                    .unwrap()
                    .apply_lua(ctx, v)
                    .unwrap();
                Ok(Ok(()))
            } else {
                Err(v)
            }
        }) {
            Ok(o) => {
                drop(g);
                luaworld.0.write().insert_resource(typedata);

                return o
            },
            Err(o) => o,
        };

        drop(g);
        luaworld.0.write().insert_resource(typedata);

        

        if let Value::UserData(v) = v{
            if v.is::<LuaRef>() {
                let b = v.borrow_mut::<LuaRef>().unwrap();
                self.apply_luaref(&b);
                return Ok(())
            }
        }

        Err(rlua::Error::RuntimeError("Invalid assignment".to_owned()))
        
    }
}



impl UserData for LuaRef {
    fn add_methods<'lua, T: rlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_meta_method(MetaMethod::ToString, |_, val, ()| {
            val.get(|s,_| 
                Ok(format!("{:#?}", PrintableReflect(s))
                ))
        });

        methods.add_meta_method_mut(MetaMethod::Index, |ctx, val, field: Value| {
            let r = val.path_lua_val_ref(field).unwrap();
            Ok(r.convert_to_lua(ctx).unwrap())
        });

        methods.add_meta_method_mut(
            MetaMethod::NewIndex,
            |ctx, val, (field, new_val): (Value, Value)| {
                val.path_lua_val_ref(field).unwrap().apply_lua(ctx, new_val).unwrap();
                Ok(())
            },
        );

        methods.add_meta_method(MetaMethod::Len, |_, val, ()| {
            val.get(|s,_| {
                let r = s.reflect_ref();
                if let ReflectRef::List(v) = r {
                    Ok(v.len())
                } else if let ReflectRef::Map(v) = r {
                    Ok(v.len())
                } else if let ReflectRef::Tuple(v) = r {
                    Ok(v.field_len())
                } else {
                    panic!("Hello");
                }
            })
        });

    }
}


