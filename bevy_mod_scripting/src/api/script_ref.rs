use crate::{impl_tealr_type, SubReflect, IdentitySubReflect, CompositeSubReflect};
use anyhow::Result;
use tealr::{mlu::{mlua,mlua::{prelude::*,Value,UserData,MetaMethod}, TealData, TealDataMethods}, TypeName};
use std::{fmt::Debug, cell::{RefCell, Cell}};

use std::{ops::{Deref,DerefMut, Index},sync::Weak, borrow::Cow, marker::PhantomData};
use parking_lot::{RwLock};
use bevy::{
    prelude::*,
    reflect::{ReflectRef, TypeRegistry, GetPath, TypeData}, ecs::component::ComponentId,
};
use std::{
    sync::Arc,
    cell::Ref,
    fmt,
};


/// A reference to a rust type available from some script language.
/// References can be either to rust or script managed values (created either on the bevy or script side).
/// but also to any subfield of those values (All pointed to values must support `reflect`).
/// Each reference holds a reflection path from the root.
/// 
/// Automatically converts to most convenient lua representation.
/// See [`ScriptRef::to_lua`]
#[derive(Clone)]
pub struct ScriptRef{
    /// The underlying top-level value 
    /// one of:
    /// - Component
    /// - Resource
    /// - Script owned value
    pub(crate) root: ScriptRefBase,

    /// A ptr caching the last reflection
    /// it's only safe to deref when we have appropriate world access..
    /// 
    /// The script ref will prefer to use this pointer if one of appropriate type is available,
    /// if a mut pointer is necessary and only a const one is available, depending on the reference base,
    /// the script ref will attempt to either retrieve a new reference from the root if it's a component or resource,
    /// or simply panick if there is no way to do that safely.
    pub(crate) r: Cell<ReflectPtr>,

    /// The reflection path from the root
    pub(crate) path: Option<String>,

    /// A recipe to access some sub-component of the pointed to "object".
    /// 
    /// Allows us to do more than the standard Reflect API allows!
    sub_reflect: CompositeSubReflect
}



impl ScriptRef {

    /// Creates a new script reference
    /// This is unsafe since it's entirely possible to create an invalid reference which does not satisfy invariants.
    /// 
    /// # Safety: 
    /// The caller must ensure that:
    /// - In the case of a component/resource, the ptr points to an actual component/resource in the same world as the ReflectComponent/Resource and Entity
    /// - In the case of ScriptOwned values, the valid pointer is a part of the pointed to struct which implements Drop and corectly drops that pointer.
    /// - In the case of stack values, the safety guarantees of normally safe functions don't hold, as no safety checks are performed, it's up to you to ensure
    ///   correctness of what you're doing
    /// - In all cases the ptr must point to a Reflect implementing value.
    pub unsafe fn new(root: ScriptRefBase, path: Option<String>, ptr : ReflectPtr) -> Self {
        Self {
            root,
            path,
            r: ptr.into(),
            sub_reflect: CompositeSubReflect::default(),
        }
    }

    pub fn sub_ref(&self, get: fn(&dyn Reflect) -> &dyn Reflect,get_mut: fn(&mut dyn Reflect) -> &mut dyn Reflect) -> ScriptRef {
        Self {
            sub_reflect: self.sub_reflect.new_sub(get,get_mut),
            ..self.clone()
        }
    }

    fn get_sub_reflect<'a>(&self, ref_: &'a dyn Reflect) -> &'a dyn Reflect{
        self.sub_reflect.sub_ref(ref_)
    }
    fn get_sub_reflect_mut<'a>(&self, ref_: &'a mut dyn Reflect) -> &'a mut dyn Reflect {
        self.sub_reflect.sub_ref_mut(ref_)
    }

    /// Checks that the cached pointer is valid 
    /// by checking that the root reference is valid
    pub fn is_valid(&self) -> bool {
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
            ScriptRefBase::Stack => true,
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
                let g = g. try_read().expect("Rust safety violation: attempted to borrow world while it was already mutably borrowed");
                
                // unsafe since pointer may be dangling
                let dyn_ref = self.get_sub_reflect(self.r.get().const_ref());

                let o = f(dyn_ref);

                drop(g);

                o

            },
            ScriptRefBase::Component { world , ..} => {
                let g = world.upgrade()
                    .expect("Trying to access cached value from previous frame");
                let g = g.try_read().expect("Rust safety violation: attempted to borrow world while it was already mutably borrowed");

                // unsafe since pointer may be dangling
                let dyn_ref = self.get_sub_reflect(self.r.get().const_ref());

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

                let dyn_ref = self.get_sub_reflect(self.r.get().const_ref());

                let o = f(dyn_ref);

                // important
                drop(g);

                o
            },
            ScriptRefBase::Stack => {
                let dyn_ref = self.get_sub_reflect(self.r.get().const_ref());
                f(dyn_ref)
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

    pub unsafe fn get_unsafe_typed<T,O,F>(&self, f: F) -> O  where 
        F : FnOnce(&T) -> O,
        T : Reflect
    {
        self.get_unsafe(|reflect| (f)(reflect.downcast_ref().unwrap()))
    }

    pub fn get_typed<T,O,F>(&self, f: F) -> O  where 
        F : FnOnce(&T) -> O,
        T : Reflect 
    {
        self.get(|reflect| (f)(reflect.downcast_ref::<T>().unwrap()))
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
                let ptr = self.r.get();
                if ptr.is_mut{
                    // make sure that if this was cached, we also mark the component as changed appropriately
                    // this is necessary if we decide to allow to hold LuaRefs for more than one frame!
                    res.reflect_mut(&mut g)
                        .unwrap()
                        .set_changed();

                    // this is safe since &mut g is now out of scope
                    // the lock itself is not an active &mut reference
                    let o = f(self.get_sub_reflect_mut(ptr.mut_ref().unwrap()),self);
                    drop(g);
                    return o
                }

                let mut ref_mut = res.reflect_mut(&mut g)
                    .unwrap();

                let dyn_ref = ref_mut
                    .path_mut(&self.path.as_ref().expect("No reflection path available"))
                    .unwrap();

                // cache this pointer for future use
                self.r.set((dyn_ref as *mut dyn Reflect).into());

                let o = f(self.get_sub_reflect_mut(dyn_ref),self);
                drop(g);
                o
            },
            ScriptRefBase::Component { comp, entity, world, .. } => {
                
                let g = world.upgrade()
                                                            .expect("Trying to access cached value from previous frame");

                let mut g = g.try_write().expect("Rust safety violation: attempted to mutably borrow world while it was already borrowed");

                // check we cached the mutable reference already
                // safety: if we have mutable access to the world, no other reference to this value or world exists
                let ptr = self.r.get();
                if ptr.is_mut{
                    // make sure that if this was cached, we also mark the component as changed appropriately
                    // this is necessary if we decide to allow to hold LuaRefs for more than one frame!
                    comp.reflect_mut(&mut g, *entity)
                        .unwrap()
                        .set_changed();

                    // this is safe since &mut g is now out of scope
                    // the lock itself is not an active &mut reference
                    let o = f(self.get_sub_reflect_mut(ptr.mut_ref().unwrap()),self);
                    drop(g);
                    return o
                }

                let mut ref_mut = comp.reflect_mut(&mut g, *entity)
                    .unwrap();

                let dyn_ref = ref_mut
                    .path_mut(&self.path.as_ref().expect("No reflection path available"))
                    .unwrap();

                // cache this pointer for future use
                self.r.set((dyn_ref as *mut dyn Reflect).into());

                let o = f(self.get_sub_reflect_mut(dyn_ref),self);
                drop(g);
                o
            },
            ScriptRefBase::ScriptOwned{ valid } => {
                let g = valid.upgrade().expect("Trying to access cached value from previous frame");
                let g = g.try_write().expect("Rust safety violation: attempted to borrow value {self:?} while it was already borrowed");
           
                let dyn_ref = self.r.get().mut_ref().expect("Mutable pointer not available!");

                let o = f(self.get_sub_reflect_mut(dyn_ref),self);
                
                // important
                drop(g);

                o
            },
            ScriptRefBase::Stack => {
                let dyn_ref = self.r.get().mut_ref().expect("Mutable pointer not available!");
                f(self.get_sub_reflect_mut(dyn_ref),self)
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


    pub unsafe fn get_mut_unsafe_typed<T,O,F>(&mut self, f: F) -> O  where 
        F : FnOnce(&mut T, &mut ScriptRef) -> O,
        T : Reflect
    {
        self.get_mut_unsafe(|reflect,ref_| (f)(reflect.downcast_mut().unwrap(),ref_))
    }

    pub fn get_mut_typed<T,O,F>(&mut self, f: F) -> O  where 
        F : FnOnce(&mut T, &mut ScriptRef) -> O,
        T : Reflect
    {
        self.get_mut(|reflect,ref_| (f)(reflect.downcast_mut().unwrap(),ref_))
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

    /// Unlike apply this method expects the other type to be identical. Does not allocate so is likely to be faster than apply, uses direct assignment.
    /// If you have a concrete value use [`Self::set_val`](TypedScriptRef) unstead
    pub fn set<T>(&mut self, other : &Self) where T : Reflect + Clone {
        let other : T = other.get_typed(|s : &T| s.clone());
        self.get_mut_typed(|s,_| *s = other);
    }

    /// Version of [`Self::set`](TypedScriptRef) which directly accepts a `T` value
    pub fn set_val<T>(&mut self, other : T) where T : Reflect  {
        self.get_mut_typed(|s,_| *s = other);
    }

}

impl Debug for ScriptRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ScriptRef").field("root", &self.root).field("path", &self.path).field("r", &self.r).finish()
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


impl ValueIndex<Cow<'static,str>> for ScriptRef {
    type Output=Result<Self,mlua::Error>;

    fn index(&self, index: Cow<'static,str>) -> Self::Output {
        self.get(|s| {
            let field = match s.reflect_ref() {
                ReflectRef::Map(v) => v.get(&index).unwrap(),
                ReflectRef::Struct(v) => v.field(&index).unwrap(),
                _ => return Err(mlua::Error::RuntimeError(format!("Tried to index a primitive rust type {:#?}", self))),
            };

            Ok(unsafe{
                ScriptRef::new(
                    self.root.clone(), 
                    Some(format!("{}.{index}",self.path.as_ref().unwrap())), 
                    (field as *const dyn Reflect).into())
            })
        })
    }
}

/// A version of index for returning values instead of references
pub trait ValueIndex<Idx> {
    type Output;

    fn index(&self, index: Idx) -> Self::Output;
}

impl ValueIndex<usize> for ScriptRef{
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

            Ok(unsafe{ 
                ScriptRef::new( 
                    self.root.clone(), 
                    Some(format!("{}[{index}]",self.path.as_ref().unwrap())), 
                    (field as *const dyn Reflect).into()
            )})
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



/// The base of a reference, i.e. the top-level object or source from which the reference stems
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
    },

    /// This is sort of reference is very short lived, and dangerous to use.
    /// Only use if you know what you're doing.
    /// 
    /// The purpose of this is to allow using temporary locals as ScriptRefs, in places where only those are accepted.
    /// There is absolutely no safety guarantees here, since we rely on the stack frame being in place and the variable not moving.
    Stack
}

impl fmt::Debug for ScriptRefBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Component { entity, world , ..} => f.debug_struct("Component").field("entity", entity).field("world", world).finish(),
            Self::ScriptOwned {..} => write!(f, "ScriptOwned"),
            Self::Resource {  world , ..} => f.debug_struct("Resource").field("world",world).finish(),
            Self::Stack => f.debug_struct("Stack").finish()
        }
    }
}


/// A pointer wrapper with some extra safety information about mutability.
#[derive(Clone,Copy,Debug)]
pub struct ReflectPtr {
   /// the pointer to the data
   ptr: *const dyn Reflect,
   /// a safety bit, if false, cannot cast as mutable pointer
   is_mut: bool,
}

impl From<*const dyn Reflect> for ReflectPtr {
    fn from(ptr: *const dyn Reflect) -> Self {
        Self { ptr, is_mut: false }
    }
}

impl From<*mut dyn Reflect> for ReflectPtr {
    fn from(ptr: *mut dyn Reflect) -> Self {
        Self { ptr, is_mut: true }
    }
}

impl ReflectPtr {
    /// dereference the pointer as an immutable reference.
    /// The caller must ensure the pointer is valid. 
    pub unsafe fn const_ref<'a>(self) -> &'a dyn Reflect {
        &*self.ptr
    }

    /// Dereference the pointer as a mutable reference,
    /// 
    /// The caller must ensure the pointer is valid. Returns None if the underlying pointer is const
    pub unsafe fn mut_ref<'a>(self) -> Option<&'a mut dyn Reflect> {
        if self.is_mut {
            Some(&mut *(self.ptr as *mut dyn Reflect))
        } else {
            None
        }
    }
}

/// safe since Reflect values have to be Send 
unsafe impl Send for ReflectPtr {}
/// safe since Reflect values have to be Sync
unsafe impl Sync for ReflectPtr {}


/// A value representing a type which has no special UserData implementation,
/// It exposes the much less convenient reflect interface of the underlying type.
#[derive(Clone)]
pub struct ReflectedValue {
    pub(crate) ref_: ScriptRef
}

impl Into<ScriptRef> for ReflectedValue {
    fn into(self) -> ScriptRef {
        self.ref_
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
            let ptr : ReflectPtr = (refl_ref as *const dyn Reflect).into();

            component_ref1 =unsafe{
                ScriptRef::new(
                    ScriptRefBase::Component{ 
                        comp: refl, 
                        entity,
                        world: Arc::downgrade(&world_arc),
                    }, 
                    Some("".to_string()),
                    ptr.into())
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
        
        let ptr : ReflectPtr = (mat.col_mut(0) as *mut dyn Reflect).into();
        let valid = Arc::new(RwLock::new(()));

        let mut ref1 = unsafe{ ScriptRef::new(
            ScriptRefBase::ScriptOwned{valid:Arc::downgrade(&valid)},
            None,
            ptr.into() 
        )};
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