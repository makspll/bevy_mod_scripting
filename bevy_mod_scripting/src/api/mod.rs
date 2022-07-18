use crate::{impl_tealr_type};
use anyhow::Result;
use tealr::{mlu::{mlua,mlua::{prelude::*,Value,UserData,MetaMethod}, TealData, TealDataMethods}, TypeName};

use std::{ops::{Deref,DerefMut, Index},sync::Weak, borrow::Cow, marker::PhantomData};
use parking_lot::{RwLock};
use bevy::{
    prelude::*,
    reflect::{ReflectRef, TypeRegistry, GetPath, TypeData}, ecs::component::ComponentId,
};
use std::{
    cell::Ref,
    fmt,
};

pub mod generated;
pub mod wrappers;
pub mod lua;



pub use {wrappers::*,lua::*};


/// The base of a reference, i.e. the top-level object which owns the underlying value
#[derive(Clone)]
pub enum ScriptRefBase {
    /// A bevy component reference
    Component{
        comp: ReflectComponent,
        entity: Entity,
        world: Weak<RwLock<World>>,
    },
    /// A bevy resource reference
    Resource{
        res: ReflectResource,
        world: Weak<RwLock<World>>
    },


    /// A script owned reflect type (for example a vector constructed in lua)
    /// These can be de-allocated whenever the script gc picks them up, so every script owned object
    /// has safety features.
    /// 
    /// It's extremely important that the userdata aliasing rules are upheld.
    /// this is protected in  rust -> lua accesses using the valid pointer. on the lua side,
    /// we handle references directly which are safe. If those accesses are ever mixed, one must be extremely careful!
    ScriptOwned{
        /// We use the rwlock to validate reads and writes
        /// When a script value goes out of scope, it checks there are no strong references
        /// to this value, if there are it panicks,
        /// so being able to acquire a read/write lock is enough to validate the reference!
        valid: Weak<RwLock<()>>
    }
}

impl fmt::Debug for ScriptRefBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Component { entity, world , ..} => f.debug_struct("Component").field("entity", entity).field("world", world).finish(),
            Self::ScriptOwned {..} => write!(f, "ScriptOwned"),
            Self::Resource {  world , ..} => f.debug_struct("Resource").field("world",world).finish(),
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


/// A value representing a type which has no special UserData implementation,
/// It exposes the much less convenient reflect interface of the underlying type.
#[derive(Clone)]
pub struct ReflectedValue {
    ref_: ScriptRef
}

impl Into<ScriptRef> for ReflectedValue {
    fn into(self) -> ScriptRef {
        self.ref_
    }
}

impl_tealr_type!(ReflectedValue);

impl TealData for ReflectedValue {
    fn add_methods<'lua, T: TealDataMethods<'lua, Self>>(methods: &mut T) {
        methods.add_meta_method(MetaMethod::ToString, |_, val, ()| {
            val.ref_.get(|s| 
                Ok(format!("{:#?}", &s)
                ))
        });

        methods.add_meta_method_mut(MetaMethod::Index, |_, val, field: Value| {
            let r = val.ref_.index(field).unwrap();
            Ok(r)
        });

        methods.add_meta_method_mut(
            MetaMethod::NewIndex,
            |ctx, val, (field, new_val): (Value, Value)| {
                val.ref_.index(field).unwrap().apply_lua(ctx, new_val).unwrap();
                Ok(())
            },
        );

        methods.add_meta_method(MetaMethod::Len, |_, val, ()| {
            val.ref_.get(|s| {
                let r = s.reflect_ref();
                if let ReflectRef::List(v) = r {
                    Ok(v.len())
                } else if let ReflectRef::Map(v) = r {
                    Ok(v.len())
                } else if let ReflectRef::Tuple(v) = r {
                    Ok(v.field_len())
                } else {
                    panic!("No length on this type");
                }
            })
        });

    }

}


/// A typed version of ScriptRef, each function automatically downcasts to `T`, due to necessity with the Reflection API, T is selected on a per function basis
/// TODO: If we ever get generic closures use this wherever possible and enforce T at type level,
/// 
/// panics if it ever doesn't, and exposes a more convenient interface
pub trait TypedScriptRef{

     /// Typed version of [`ScriptRef::get_unsafe`](ScriptRef)
    unsafe fn get_unsafe_typed<T,O,F>(&self, f: F) -> O  where 
        F : FnOnce(&T) -> O,
        T : ScriptReference;

    /// Typed version of [`ScriptRef::get`](ScriptRef)
    fn get_typed<T,O,F>(&self, f: F) -> O  where 
        F : FnOnce(&T) -> O,
        T : ScriptReference;

    /// Typed version of [`ScriptRef::get_mut_unsafe`](ScriptRef)
    unsafe fn get_mut_unsafe_typed<T,O,F>(&mut self, f: F) -> O  where 
        F : FnOnce(&mut T, &mut ScriptRef) -> O,
        T : ScriptReference;

    /// Typed version of [`ScriptRef::get_mut`](ScriptRef)
    fn get_mut_typed<T,O,F>(&mut self, f: F) -> O  where 
        F : FnOnce(&mut T, &mut ScriptRef) -> O,
        T : ScriptReference;

    /// Unlike apply this method expects the other type to be identical. Does not allocate so is likely to be faster than apply, uses direct assignment.
    /// If you have a concrete value use [`Self::set_val`](TypedScriptRef) unstead
    fn set<T>(&mut self, other : &Self) where T : ScriptValue;

    /// Version of [`Self::set`](TypedScriptRef) which directly accepts a `T` value
    fn set_val<T>(&mut self, other : T) where T : ScriptReference;

}

impl TypedScriptRef for ScriptRef {

    /// Typed version of [`ScriptRef::get_unsafe`](ScriptRef)
    unsafe fn get_unsafe_typed<T,O,F>(&self, f: F) -> O  where 
        F : FnOnce(&T) -> O,
        T : ScriptReference
    {
        self.get_unsafe(|reflect| (f)(reflect.downcast_ref().unwrap()))
    }

    /// Typed version of [`ScriptRef::get`](ScriptRef)
    fn get_typed<T,O,F>(&self, f: F) -> O  where 
        F : FnOnce(&T) -> O,
        T : ScriptReference 
    {
        self.get(|reflect| (f)(reflect.downcast_ref::<T>().unwrap()))
    }

    /// Typed version of [`ScriptRef::get_mut_unsafe`](ScriptRef)
    unsafe fn get_mut_unsafe_typed<T,O,F>(&mut self, f: F) -> O  where 
        F : FnOnce(&mut T, &mut ScriptRef) -> O,
        T : ScriptReference
    {
        self.get_mut_unsafe(|reflect,ref_| (f)(reflect.downcast_mut().unwrap(),ref_))
    }

    /// Typed version of [`ScriptRef::get_mut`](ScriptRef)
    fn get_mut_typed<T,O,F>(&mut self, f: F) -> O  where 
        F : FnOnce(&mut T, &mut ScriptRef) -> O,
        T : ScriptReference
    {
        self.get_mut(|reflect,ref_| (f)(reflect.downcast_mut().unwrap(),ref_))
    }

    /// Unlike apply this method expects the other type to be identical. Does not allocate so is likely to be faster than apply, uses direct assignment.
    /// If you have a concrete value use [`Self::set_val`](TypedScriptRef) unstead
    fn set<T>(&mut self, other : &Self) where T : ScriptValue {
        let other : T = other.get_typed(|s : &T| s.clone());
        self.get_mut_typed(|s,_| *s = other);
    }

    /// Version of [`Self::set`](TypedScriptRef) which directly accepts a `T` value
    fn set_val<T>(&mut self, other : T) where T : ScriptReference  {
        self.get_mut_typed(|s,_| *s = other);
    }

}
 

/// A reference to a rust type available from some script language.
/// References can be either to rust or script managed values (created either on the bevy or script side).
/// but also to any subfield of those values (All pointed to values must support `reflect`).
/// Each reference holds a reflection path from the root.
/// 
/// Automatically converts to most convenient lua representation.
/// See [`ScriptRef::to_lua`]
#[derive(Clone,Debug)]
pub struct ScriptRef{
    /// The underlying top-level value 
    /// one of:
    /// - Component
    /// - Resource
    /// - Script owned value
    root: ScriptRefBase,

    /// The reflection path from the root
    path: Option<String>,

    // A ptr caching the last reflection
    // it's only safe to deref when we have appropriate world access
    r: ReflectPtr
}

impl ScriptRef {
    /// Checks that the cached pointer is valid 
    /// by checking that the root reference is valid
    fn is_valid(&self) -> bool {
        match &self.root {
            ScriptRefBase::Resource { res, world } => {
                if world.strong_count() == 0 {
                    return false;
                }

                let g = world.upgrade().expect("Trying to access cached value from previous frame");
                let g = g.try_read().expect("Rust safety violation: attempted to borrow data immutably while it was already mutably borrowed");
                
                res.reflect(&g).is_some()
            },
            ScriptRefBase::Component { comp, entity, world, .. } => {
                if world.strong_count() == 0 {
                    return false;
                }

                let g = world.upgrade().expect("Trying to access cached value from previous frame");

                let g = g.try_read().expect("Rust safety violation: attempted to borrow data immutably while it was already mutably borrowed");

                comp.reflect(&g,*entity).is_some()
            },
            ScriptRefBase::ScriptOwned { valid } => valid.strong_count() > 0,
            
        }
    }

    /// Retrieves the underlying `dyn Reflect` reference and applies function which can retrieve a value.
    /// Panics if world/value is already borrowed mutably
    /// # Safety
    /// The caller must ensure the root reference has not been deleted or moved 
    pub unsafe fn get_unsafe<O,F>(&self, f: F) -> O  where 
        F : FnOnce(&dyn Reflect) -> O
    {
        match &self.root {
            ScriptRefBase::Resource { res: _, world } => {
                let g = world.upgrade()
                .expect("Trying to access cached value from previous frame");
                let g = g.try_read().expect("Rust safety violation: attempted to borrow world while it was already mutably borrowed");
                
                // unsafe since pointer may be dangling
                let dyn_ref = match self.r {
                    ReflectPtr::Const(r) => &*r,
                    ReflectPtr::Mut(r) => &*r,
                };

                let o = f(dyn_ref);

                drop(g);

                o

            },
            ScriptRefBase::Component { world , ..} => {
                let g = world.upgrade()
                    .expect("Trying to access cached value from previous frame");
                let g = g.try_read().expect("Rust safety violation: attempted to borrow world while it was already mutably borrowed");

                // unsafe since pointer may be dangling
                let dyn_ref = match self.r {
                    ReflectPtr::Const(r) => &*r,
                    ReflectPtr::Mut(r) => &*r,
                };

                let o = f(dyn_ref);

                drop(g);

                o
            },
            ScriptRefBase::ScriptOwned { valid } => {
                // in this case we don't allocate the whole value but a valid bit
                // nonetheless we can use it to uphold borrow rules
                let g = valid.upgrade()
                    .expect("Trying to access cached value from previous frame");

                let g = g.try_read().expect("Rust safety violation: attempted to borrow value {self:?} while it was already mutably borrowed");

                let dyn_ref = match self.r {
                    ReflectPtr::Const(r) => &*r,
                    ReflectPtr::Mut(r) => &*r,
                };

                let o = f(dyn_ref);

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
        F : FnOnce(&dyn Reflect) -> O,
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
        F : FnOnce(&mut dyn Reflect, &mut ScriptRef) -> O 
    {
        match &self.root {
            ScriptRefBase::Resource { res, world } => {
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
                        res.reflect_mut(&mut g)
                            .unwrap()
                            .set_changed();

                        // this is safe since &mut g is now out of scope
                        // the lock itself is not an active &mut reference
                        let o = f(&mut *r,self);
                        drop(g);
                        return o
                    },
                }

                let mut ref_mut = res.reflect_mut(&mut g)
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
            ScriptRefBase::Component { comp, entity, world, .. } => {
                
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
                        comp.reflect_mut(&mut g, *entity)
                            .unwrap()
                            .set_changed();

                        // this is safe since &mut g is now out of scope
                        // the lock itself is not an active &mut reference
                        let o = f(&mut *r,self);
                        drop(g);
                        return o
                    },
                }

                let mut ref_mut = comp.reflect_mut(&mut g, *entity)
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
            ScriptRefBase::ScriptOwned{ valid } => {
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
        F : FnOnce(&mut dyn Reflect, &mut ScriptRef) -> O
    {
        if !self.is_valid(){
            panic!("reference {self:?} is invalid")
        }

        unsafe {
            self.get_mut_unsafe(f)
        }
    }


    /// applies another [`ScriptRef`] to self by carefuly acquiring locks and cloning if necessary.
    /// 
    /// This is semantically equivalent to the [`Reflect::apply`] method.
    /// If you know the type of this value use [`Self::apply_luaref_typed`] since it avoids double cloning and allocating
    pub fn apply(&mut self, other : &ScriptRef){
        // sadly apply already performs a clone for value types, so this incurs
        // a double clone in some cases TODO: is there another way ?
        // can we avoid the box ?
        let cloned = other.get(|s| s.clone_value());

        // safety: we already called `get` so reference must be valid
        unsafe {
            self.get_mut_unsafe(|s,_| 
                s.apply(&*cloned)
            )
        }
    }

}



impl TypeName for ScriptRef {
    /// We represent LuaRef types as `any` wildcards, since they can convert to literally anything 
    fn get_type_parts() -> std::borrow::Cow<'static, [tealr::NamePart]> {
        std::borrow::Cow::Borrowed(&[tealr::NamePart::Type(tealr::TealType {
            name: std::borrow::Cow::Borrowed("any"),
            generics: None,
            type_kind: tealr::KindOfType::Builtin,
        })])     
    }
}




/// A version of index for returning values instead of references
pub trait ValueIndex<Idx> {
    type Output;

    fn index(&self, index: Idx) -> Self::Output;
}

impl ValueIndex<Cow<'static,str>> for ScriptRef {
    type Output=Result<Self,mlua::Error>;

    fn index(&self, index: Cow<'static,str>) -> Self::Output {
        self.get(|s| {
            let field = match s.reflect_ref() {
                ReflectRef::Map(v) => v.get(&index).unwrap(),
                ReflectRef::Struct(v) => v.field(&index).unwrap(),
                _ => return Err(mlua::Error::RuntimeError(format!("Tried to index a primitive rust type {:#?}", self))),
            };

            Ok(ScriptRef{ 
                root: self.root.clone(), 
                path: Some(format!("{}.{index}",self.path.as_ref().unwrap())), 
                r: ReflectPtr::Const(field)
            })
        })
    }
}

impl ValueIndex<usize> for ScriptRef {
    type Output=Result<Self,mlua::Error>;

    fn index(&self, index: usize) -> Self::Output {
        self.get(|s| {
            let field = match s.reflect_ref() {
                ReflectRef::Tuple(v) => v.field(index).unwrap(),
                ReflectRef::TupleStruct(v) => v.field(index).unwrap(),
                ReflectRef::List(v) => v.get(index).unwrap(),
                ReflectRef::Map(v) => v.get(&(index)).unwrap(),
                _ => return Err(mlua::Error::RuntimeError(format!("Tried to index a primitive rust type {:#?}", self))),
            };

            Ok(ScriptRef{ 
                root: self.root.clone(), 
                path: Some(format!("{}[{index}]",self.path.as_ref().unwrap())), 
                r: ReflectPtr::Const(field)
            })
        })
    }
}

impl ValueIndex<Value<'_>> for ScriptRef {
    type Output=Result<Self,mlua::Error>;

    fn index(&self, index: Value<'_>) -> Self::Output {
        match index {
            Value::Integer(idx) => {
                self.index(idx  as usize)
            }
            Value::String(field) => {
                let str_ = field.to_str()?.to_string();
                // TODO: hopefully possible to use a &'_ str here
                // but this requires Reflect implementation for &str 
                <Self as ValueIndex<Cow<'static,str>>>::index(self,str_.into())
            }
            _ => return Err(mlua::Error::RuntimeError(format!("Cannot index a rust object with {:?}", index))),
        }
    }
}



#[cfg(test)]

mod test {
    use crate::{langs::mlu::{mlua,mlua::prelude::*},api::lua::LuaEntity, ScriptRef, ScriptRefBase, ReflectPtr};
    use bevy::{prelude::*,reflect::TypeRegistryArc};
    use std::{sync::Arc};
    use parking_lot::RwLock;

    #[derive(Clone)]
    struct TestArg(LuaEntity);

    impl <'lua>ToLua<'lua> for TestArg {
        fn to_lua(self, ctx: &'lua Lua) -> Result<LuaValue<'lua>, mlua::Error> { 
            self.0.to_lua(ctx) 
        }
    }

    #[derive(Component,Reflect,Default)]
    #[reflect(Component)]
    struct TestComponent{
        mat3: Mat3,
    }

    #[test]
    #[should_panic]
    fn miri_test_components(){
        let world_arc = Arc::new(RwLock::new(World::new()));

        let mut component_ref1;
        let mut component_ref2;

        {
            let world = &mut world_arc.write();

            world.init_resource::<TypeRegistryArc>();
            let registry = world.resource_mut::<TypeRegistryArc>();
            registry.write().register::<TestComponent>();

            let tst_comp = TestComponent{
                mat3: Mat3::from_cols(Vec3::new(1.0,2.0,3.0),
                                    Vec3::new(4.0,5.0,6.0),
                                    Vec3::new(7.0,8.0,9.0))
            };

            let refl = registry.read()
                .get_with_short_name("TestComponent")
                .and_then(|registration| registration.data::<ReflectComponent>())
                .unwrap()
                .clone();
                
            let entity = world.spawn()
                            .insert(tst_comp)
                            .id();


            let refl_ref = refl.reflect(world,entity).unwrap();
            let ptr : ReflectPtr = ReflectPtr::Const(refl_ref);

            component_ref1 = ScriptRef{
                r: ptr,
                root: ScriptRefBase::Component{ 
                    comp: refl, 
                    entity,
                    world: Arc::downgrade(&world_arc),
                }, 
                path: Some("".to_string()), 
            };
            component_ref2 = component_ref1.clone();
        }

        component_ref1.get(|r1| {
            component_ref2.get(|r2|{
                let _ = r1.downcast_ref::<TestComponent>().unwrap().mat3 + r2.downcast_ref::<TestComponent>().unwrap().mat3;
            })
        });

        component_ref1.get_mut(|r1,_| {
            let _ = r1.downcast_ref::<TestComponent>().unwrap().mat3 * 2.0;
        });

        component_ref2.get_mut(|r2,_|{
            let _ = r2.downcast_ref::<TestComponent>().unwrap().mat3 * 2.0;
        });

        // invalid should panic here
        component_ref1.get_mut(|r1,_| {
            component_ref2.get(|r2|{
                r1.downcast_mut::<TestComponent>().unwrap().mat3 = r2.downcast_ref::<TestComponent>().unwrap().mat3;
            })
        });    
    }

    #[test]
    #[should_panic]
    fn miri_test_owned(){
       
        let mut mat = Mat3::from_cols(Vec3::new(1.0,2.0,3.0),
                                Vec3::new(4.0,5.0,6.0),
                                Vec3::new(7.0,8.0,9.0));
        
        let ptr : ReflectPtr = ReflectPtr::Mut(mat.col_mut(0));
        let valid = Arc::new(RwLock::new(()));

        let mut ref1 = ScriptRef{
            r: ptr,
            root: ScriptRefBase::ScriptOwned{valid:Arc::downgrade(&valid)},
            path: None, 
        };
        let mut ref2 = ref1.clone();

        ref1.get(|r1| {
            ref2.get(|r2|{
                let _ = *r1.downcast_ref::<Vec3>().unwrap() + *r2.downcast_ref::<Vec3>().unwrap();
            })
        });

        ref1.get_mut(|r1,_| {
            let _ = *r1.downcast_ref::<Vec3>().unwrap() * 2.0;
        });

        ref2.get_mut(|r2,_|{
            let _ = *r2.downcast_ref::<Vec3>().unwrap() * 2.0;
        });

        drop(valid);
        drop(mat);

        // should panic since original value dropped
        ref1.get_mut(|r1,_| r1.downcast_mut::<Vec3>().unwrap()[1] = 2.0);
    }

}