pub mod bevy_types;
pub mod wrappers;
pub mod generated;

use std::{ops::DerefMut,sync::Weak};
use parking_lot::{RwLock};
use bevy::{
    prelude::*,
    reflect::{ReflectRef, TypeRegistry, GetPath}, ecs::component::ComponentId,
};
use std::{
    cell::Ref,
    fmt,
};

use crate::{PrintableReflect};
use anyhow::Result;
use tealr::mlu::{mlua,mlua::{prelude::*,Value,UserData,MetaMethod}, TealData};
pub use {generated::*,bevy_types::*, wrappers::*,generated::LuaBevyAPIProvider };

#[reflect_trait]
pub trait CustomUserData {
    /// a version of `mlua::to_lua` which does not consume the object
    fn ref_to_lua<'lua>(&self, lua: &'lua Lua) -> mlua::Result<Value<'lua>>;

    fn apply_lua<'lua>(&mut self, lua: &'lua Lua, new_val: Value<'lua>) -> mlua::Result<()>;
}

impl<T: Clone + UserData + Send + 'static> CustomUserData for T {
    fn ref_to_lua<'lua>(&self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        Ok(Value::UserData(lua.create_userdata(self.clone())?))
    }

    fn apply_lua<'lua>(&mut self, _lua: &'lua Lua, new_val: Value<'lua>) -> mlua::Result<()> {
        if let Value::UserData(v) = new_val {
            let s: Ref<T> = v.borrow::<T>()?;
            *self = s.clone();
            Ok(())
        } else {
            Err(mlua::Error::RuntimeError(
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
    fn to_lua(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> where {
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
        id: ComponentId,
        entity: Entity,
        world: Weak<RwLock<World>>,
    },
    /// A lua owned reflect type (for example a vector constructed in lua)
    /// These can be de-allocated whenever the lua gc picks them up, so every lua owned object
    /// has safety features.
    /// 
    /// It's extremely important that the userdata aliasing rules are upheld.
    /// this is protected in  rust -> lua accesses using the valid pointer. on the lua side,
    /// we handle references directly which are safe. If those accesses are ever mixed, one must be extremely careful!
    LuaOwned{
        /// We use the rwlock to validate reads and writes
        /// When a lua value goes out of scope, it checks there are no strong references
        /// to this value, if there are it panicks,
        /// so being able to acquire a read/write lock is enough to validate the reference!
        valid: Weak<RwLock<()>>
    }
}

impl fmt::Debug for LuaRefBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Component { entity, world , ..} => f.debug_struct("Component").field("entity", entity).field("world", world).finish(),
            Self::LuaOwned {..} => write!(f, "LuaOwned"),
        }
    }
}


#[derive(Clone,Debug)]
pub enum ReflectPtr {
    Const(*const dyn Reflect),
    Mut(*mut dyn Reflect),
}

/// safe since Reflect values have to be Send 
unsafe impl Send for ReflectPtr {}
/// safe since Reflect values have to be Sync
unsafe impl Sync for ReflectPtr {}

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

    // A ptr caching the last reflection
    // it's only safe to deref when we have appropriate world access
    r: ReflectPtr
}


impl fmt::Display for LuaRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}


impl LuaRef {
    /// Checks that the cached pointer is valid 
    /// by checking that the root reference is valid
    fn is_valid(&self) -> bool {
        match &self.root {
            LuaRefBase::Component { comp, entity, world, .. } => {
                if world.strong_count() == 0 {
                    return false;
                }

                let g = world.upgrade().expect("Trying to access cached value from previous frame");

                let g = g.try_read().expect("Rust safety violation: attempted to borrow data immutably while it was already mutably borrowed");

                comp.reflect_component(&g,*entity).is_some()
            },
            LuaRefBase::LuaOwned { valid } => valid.strong_count() > 0,
        }
    }

    /// Retrieves the underlying `dyn Reflect` reference and applies function which can retrieve a value.
    /// Panics if world/value is already borrowed mutably
    /// # Safety
    /// The caller must ensure the root reference has not been deleted or moved 
    pub unsafe fn get_unsafe<O,F>(&self, f: F) -> O  where 
        F : FnOnce(&dyn Reflect, &LuaRef) -> O 
    {
        match &self.root {
            LuaRefBase::Component { world , ..} => {
                let g = world.upgrade()
                    .expect("Trying to access cached value from previous frame");

                let g = g.try_read().expect("Rust safety violation: attempted to borrow world while it was already mutably borrowed");

                // unsafe since pointer may be dangling
                let dyn_ref = match self.r {
                    ReflectPtr::Const(r) => &*r,
                    ReflectPtr::Mut(r) => &*r,
                };

                let o = f(dyn_ref,self);

                drop(g);

                o
            },
            LuaRefBase::LuaOwned { valid } => {
                // in this case we don't allocate the whole value but a valid bit
                // nonetheless we can use it to uphold borrow rules
                let g = valid.upgrade()
                    .expect("Trying to access cached value from previous frame");

                let g = g.try_read().expect("Rust safety violation: attempted to borrow value {self:?} while it was already mutably borrowed");

                let dyn_ref = match self.r {
                    ReflectPtr::Const(r) => &*r,
                    ReflectPtr::Mut(r) => &*r,
                };

                let o = f(dyn_ref,self);

                // important
                drop(g);

                o
            },
        }
    }

    /// Retrieves the underlying `dyn Reflect` reference and applies function which can retrieve a value.
    /// Panics if the reference is invalid or world is already borrowed mutably.
    #[inline(always)]    
    pub fn get<O,F>(&self, f: F) -> O  where 
        F : FnOnce(&dyn Reflect, &LuaRef) -> O
    {

        // check the cached pointer is dangling
        if !self.is_valid(){
            panic!("reference {self:?} is invalid")
        }
        // safety: we know the pointer is valid
        unsafe {
            self.get_unsafe(f)
        }
    }

    /// Retrieves the underlying `dyn Reflect` reference and applies function which can retrieve a value.
    /// If this is a component it is marked as changed.
    /// Panics if the world/value is already borrowed or if r is not a mutable pointer.
    /// # Safety
    /// The caller must ensure the root reference has not been deleted or moved     
    pub unsafe fn get_mut_unsafe<O,F>(&mut self, f: F) -> O  
    where 
        F : FnOnce(&mut dyn Reflect, &mut LuaRef) -> O 
    {
        match &self.root {
            LuaRefBase::Component { comp, entity, world, .. } => {
                
                let g = world.upgrade()
                                                            .expect("Trying to access cached value from previous frame");

                let mut g = g.try_write().expect("Rust safety violation: attempted to mutably borrow world while it was already borrowed");

                // check we cached the mutable reference already
                // safety: if we have mutable access to the world, no other reference to this value or world exists
                match self.r {
                    ReflectPtr::Const(_) => {},
                    ReflectPtr::Mut(r) => {
                        
                        // make sure that if this was cached, we also mark the component as changed appropriately
                        // this is necessary if we decide to allow to hold LuaRefs for more than one frame!
                        comp.reflect_component_mut(&mut g, *entity)
                            .unwrap()
                            .set_changed();

                        // this is safe since &mut g is now out of scope
                        // the lock itself is not an active &mut reference
                        let o = f(&mut *r,self);
                        drop(g);
                        return o
                    },
                }

                let mut ref_mut = comp.reflect_component_mut(&mut g, *entity)
                    .unwrap();

                let dyn_ref = ref_mut
                    .path_mut(&self.path.as_ref().expect("No reflection path available"))
                    .unwrap();

                // cache this pointer for future use
                self.r = ReflectPtr::Mut(dyn_ref);

                let o = f(dyn_ref,self);
                drop(g);
                o
            },
            LuaRefBase::LuaOwned{ valid } => {
                let g = valid.upgrade().expect("Trying to access cached value from previous frame");
                let g = g.try_write().expect("Rust safety violation: attempted to borrow value {self:?} while it was already borrowed");
           
                let dyn_ref = match self.r {
                    ReflectPtr::Const(_) => panic!("Mutable pointer not available!"),
                    ReflectPtr::Mut(r) => &mut *r,
                };

                let o = f(dyn_ref,self);
                
                // important
                drop(g);

                o
            },
        }    
    }

    /// Retrieves the underlying `dyn Reflect` reference and applies function which can retrieve a value.
    /// If this is a component it is marked as changed.
    /// Panics if the reference is invalid or if the world/value is already borrowed or if r is not a mutable pointer.
    #[inline(always)]
    pub fn get_mut<O,F>(&mut self, f: F) -> O  where 
        F : FnOnce(&mut dyn Reflect, &mut LuaRef) -> O
    {
        if !self.is_valid(){
            panic!("reference {self:?} is invalid")
        }

        unsafe {
            self.get_mut_unsafe(f)
        }
    }

    /// Accesses a subfield of the underlying reflect value given a string path using the `bevy_reflect::path::GetPath` trait
    pub fn path_ref(&self, path: &str) -> Result<Self,mlua::Error> {
        self.get(|s,_| {
            let subfield = s.path(path)
                .map_err(|_e| mlua::Error::RuntimeError(format!("Cannot access field `{}`", path)))?;

            Ok(LuaRef {
                root: self.root.clone(),
                path: Some(format!("{}{}",self.path.as_ref().unwrap(), path)),
                r: ReflectPtr::Const(subfield),
            })
        })


    }

    /// Accesses a subfield of the underlying reflect value given a lua value index/path using the `bevy_reflect::path::GetPath` trait.
    /// Accepts both array expressions and integer indices as well as string paths.
    pub fn path_ref_lua(&self, path: Value) -> Result<Self,mlua::Error> {
        self.get(|s,_| {
            let r = s.reflect_ref();

            let (path,subfield) = match path {
                Value::Integer(idx) => {
                    let idx = idx as usize - 1;
                    let path_str = format!("[{idx}]");
                    let field = match r {
                        ReflectRef::Tuple(v) => v.field(idx).unwrap(),
                        ReflectRef::TupleStruct(v) => v.field(idx).unwrap(),
                        ReflectRef::List(v) => v.get(idx).unwrap(),
                        ReflectRef::Map(v) => v.get(&(idx)).unwrap(),
                        _ => return Err(mlua::Error::RuntimeError(format!("Tried to index a primitive rust type {:#?}", self))),
                    };
    
                    (path_str,field)
                }
                Value::String(field) => {
                    let path_str = field.to_str().unwrap().to_string();
                    let field = match r {
                        ReflectRef::Map(v) => v.get(&path_str.to_owned()).unwrap(),
                        ReflectRef::Struct(v) => v.field(&path_str).unwrap(),
                        _ => return Err(mlua::Error::RuntimeError(format!("Tried to index a primitive rust type {:#?}", self))),
                    };
    
                    (path_str,field)
                }
                _ => return Err(mlua::Error::RuntimeError(format!("Cannot index a rust object with {:?}", path))),
            };
    
            Ok(LuaRef{ 
                root: self.root.clone(), 
                path: Some(format!("{}{}",self.path.as_ref().unwrap(),path)), 
                r: ReflectPtr::Const(subfield)
            })
        })
    
    }

    /// Converts the given lua reference to an appropriate lua value.
    /// The actual lua value depends on the underlying type of the field pointed to by the reference:
    /// - If the type is one of supported standard bevy types (such as Vec2), and is in the `bevy_mod_scripting::BEVY_TO_LUA` array, 
    ///     it is converted to a wrapper type which supports a subset of the original methods.
    /// - If the type is registered with the `TypeRegistry` and implemnets `CustomUserData` it is converted using the `CustomUserData::ref_to_lua` function
    /// - If the type is not any of the above, it's converted to lua as a plain `LuaRef` UserData  
    pub fn convert_to_lua<'lua>(self, ctx: &'lua Lua) -> Result<Value<'lua>,mlua::Error> {
        self.get(|s,_| {

            if let Some(f) = BEVY_TO_LUA.get(s.type_name()){
                Ok(f(&self,ctx))
            } else {

                let luaworld = ctx.globals()
                    .get::<_, LuaWorld>("world")
                    .unwrap();

                let world = luaworld.upgrade().unwrap();
                let world = &mut world.read();
    
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

    /// applies another luaref to self by carefuly acquiring locks and cloning.
    /// This is semantically equivalent to the `bevy_reflect::Reflect::apply` method. 
    pub fn apply_luaref(&mut self, o : &LuaRef){
        // sadly apply already performs a clone for value types, so this incurs
        // a double clone in some cases TODO: is there another way ?
        // can we avoid the box ?
        let cloned = o.get(|s,_| s.clone_value());

        // safety: we already called get so reference must be valid
        unsafe {
            self.get_mut_unsafe(|s,_| s.apply(&*cloned));
        }
    }

    /// Applies a lua value to self by carefuly acquiring locks and cloning if necessary.
    /// This is semantically equivalent to the `bevy_reflect::Reflect::apply` method. 
    pub fn apply_lua<'lua>(&mut self, ctx: &'lua Lua, v: Value<'lua>) -> Result<(),mlua::Error> {
    
        let type_name = self.get(|s,_| s.type_name().to_owned());

        if let Some(f) = APPLY_LUA_TO_BEVY.get(&type_name) {
            return f(self,ctx,v)
        };
            

        let luaworld = ctx.globals()
                        .get::<_, LuaWorld>("world")
                        .unwrap();

        // remove typedata from the world to be able to manipulate world 
        let typedata = {
            let world = luaworld.upgrade().unwrap();
            let world = &mut world.write();
            world.remove_resource::<TypeRegistry>().unwrap()
        };

        let g = typedata.read();

        // safety we already called `get` so reference must be valid
        let v = unsafe {
            match self.get_mut_unsafe(|mut s,_| {
                if let Some(ud) = g.get_type_data::<ReflectCustomUserData>(s.type_id()){

                    ud.get_mut(s.deref_mut())
                        .unwrap()
                        .apply_lua(ctx, v)
                        .unwrap();
                    Ok(Ok(()))
                } else {
                    Err(v)
                }
        })
         {
            Ok(o) => {
                drop(g);

                let world = luaworld.upgrade().unwrap();
                let world = &mut world.write();
                world.insert_resource(typedata);

                return o
            },
            Err(o) => o,
        }};

        drop(g);
        let world = luaworld.upgrade().unwrap();
        let world = &mut world.write();
        world.insert_resource(typedata);

        

        if let Value::UserData(v) = v{
            if v.is::<LuaRef>() {
                let b = v.borrow_mut::<LuaRef>().unwrap();
                self.apply_luaref(&b);
                return Ok(())
            }
        }

        Err(mlua::Error::RuntimeError("Invalid assignment".to_owned()))
        
    }
}



/// Plain reference based UserData, the default lua representation of non-supported types
impl UserData for LuaRef {
    fn add_methods<'lua, T: mlua::UserDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_meta_method(MetaMethod::ToString, |_, val, ()| {
            val.get(|s,_| 
                Ok(format!("{:#?}", PrintableReflect(s))
                ))
        });

        methods.add_meta_method_mut(MetaMethod::Index, |ctx, val, field: Value| {
            let r = val.path_ref_lua(field).unwrap();
            Ok(r.convert_to_lua(ctx).unwrap())
        });

        methods.add_meta_method_mut(
            MetaMethod::NewIndex,
            |ctx, val, (field, new_val): (Value, Value)| {
                val.path_ref_lua(field).unwrap().apply_lua(ctx, new_val).unwrap();
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


