use std::{marker::PhantomData, borrow::Cow, sync::Weak};

use bevy::{reflect::{Reflect, GetPath, ReflectRef, ReflectMut}, prelude::{ReflectComponent, Entity, World, ReflectResource}};
use parking_lot::RwLock;

use std::fmt;

pub type SubReflectGet = fn(&dyn Reflect) -> &dyn Reflect;
pub type SubReflectGetMut = fn(&mut dyn Reflect) -> &mut dyn Reflect;
pub type SubReflectIndexedGet = fn(usize,&dyn Reflect) -> &dyn Reflect;
pub type SubReflectIndexedGetMut = fn(usize,&mut dyn Reflect) -> &mut dyn Reflect;



/// The base of a reflect path, i.e. the top-level object or source. Reflections paths are always relative to some reflect base
#[derive(Clone)]
pub enum ReflectBase {
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
        ptr: ReflectPtr,
        /// We use the rwlock to validate reads and writes
        /// When a script value goes out of scope, it checks there are no strong references
        /// to this value, if there are it panicks,
        /// so being able to acquire a read/write lock is enough to validate the reference!
        valid: Weak<RwLock<()>>
    },
}

impl fmt::Debug for ReflectBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Component { entity, world , ..} => f.debug_struct("Component").field("entity", entity).field("world", world).finish(),
            Self::ScriptOwned {..} => write!(f, "ScriptOwned"),
            Self::Resource {  world , ..} => f.debug_struct("Resource").field("world",world).finish(),
        }
    }
}



/// Stores the path of reflection + sub reflection from a root reflect reference.
/// 
/// Also allows accessing elements beyond reach of the normal reflect API
#[derive(Clone)]
pub enum ReflectPathElem {
    SubReflection{
        label: &'static str,
        get: SubReflectGet,
        get_mut: SubReflectGetMut
    },
    IndexedSubReflection{
        label: &'static str,
        index: usize,
        get: SubReflectIndexedGet,
        get_mut: SubReflectIndexedGetMut
    },
    /// Access to a struct field
    FieldAccess(Cow<'static,str>),
    /// Access to a TupleStruct, Tuple, List or Array element
    IndexAccess(usize)
    // TODO: Map access
}
use std::fmt::{Debug,Display};

use crate::ReflectPtr;

impl Debug for ReflectPathElem{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SubReflection { label, .. } => f.debug_struct("SubReflection").field("label", label).finish(),
            Self::IndexedSubReflection { label, index , .. } => f.debug_struct("SubReflection").field("label", label).field("index", index).finish(),
            Self::FieldAccess(arg0) => f.debug_tuple("FieldAccess").field(arg0).finish(),
            Self::IndexAccess(arg0) => f.debug_tuple("IndexAccess").field(arg0).finish(),
            
        }
    }
}

impl Display for ReflectPathElem{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReflectPathElem::SubReflection { label,.. } => {
                f.write_str(".")?;
                f.write_str(label)?;
                f.write_str("()")
            },
            ReflectPathElem::IndexedSubReflection { label, index, .. } => {
                f.write_str(".")?;
                f.write_str(label)?;
                f.write_str("(")?;
                f.write_str(&index.to_string())?;
                f.write_str(")")
            },
            ReflectPathElem::FieldAccess(s) => {
                f.write_str(".")?;
                f.write_str(s)
            },
            ReflectPathElem::IndexAccess(i) => {
                f.write_str("[")?;
                f.write_str(&i.to_string())?;
                f.write_str("]")
            }, 
        }
    }
}

impl ReflectPathElem{
    pub fn sub_ref<'a>(&self, base: &'a dyn Reflect) -> &'a dyn Reflect {
        match self {
            ReflectPathElem::SubReflection { get, .. } => get(base),
            ReflectPathElem::IndexedSubReflection { get, index, .. } => get(*index ,base),
            ReflectPathElem::FieldAccess(field) => match base.reflect_ref() {
                ReflectRef::Struct(s) => s.field(field).unwrap(),
                _ => panic!("Invalid indexing")
            },
            ReflectPathElem::IndexAccess(index) => match base.reflect_ref(){
                ReflectRef::TupleStruct(s) => s.field(*index).unwrap(),
                ReflectRef::Tuple(s) => s.field(*index).unwrap(),
                ReflectRef::List(s) => s.get(*index).unwrap(),
                ReflectRef::Array(s) => s.get(*index).unwrap(),
                _ => panic!("Invalid Indexing")
            },
        }
    }

    pub fn sub_ref_mut<'a>(&self, base: &'a mut dyn Reflect) -> &'a mut dyn Reflect {
        match self {
            ReflectPathElem::SubReflection { get_mut, .. } => get_mut(base),
            ReflectPathElem::IndexedSubReflection { get_mut, index, .. } => get_mut(*index ,base),
            ReflectPathElem::FieldAccess(field) => match base.reflect_mut() {
                ReflectMut::Struct(s) => s.field_mut(field).unwrap(),
                _ => panic!("Invalid indexing")
            },
            ReflectPathElem::IndexAccess(index) => match base.reflect_mut(){
                ReflectMut::TupleStruct(s) => s.field_mut(*index).unwrap(),
                ReflectMut::Tuple(s) => s.field_mut(*index).unwrap(),
                ReflectMut::List(s) => s.get_mut(*index).unwrap(),
                ReflectMut::Array(s) => s.get_mut(*index).unwrap(),
                _ => panic!("Invalid Indexing")
            },
        }
    }
    
}


#[derive(Clone,Debug)]
pub struct ReflectPath {
    base: ReflectBase,
    // most of these will be very short, people don't make many nested hashmaps vecs etc.
    accesses: Vec<ReflectPathElem>
}

impl Display for ReflectPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("(Base)");
        for access in &self.accesses {
            f.write_str(&access.to_string())?
        }
        Ok(())
    }
}


impl ReflectPath {

    pub fn new(base : ReflectBase) -> Self{
        Self{
            base,
            accesses: Vec::default(),
        }
    }

    /// pushes another sub reflect level access to the end of this access. 
    /// 
    /// The most recent sub access added will be executed last.
    pub fn push(&mut self, elem: ReflectPathElem){
        self.accesses.push(elem);
    }

    /// Creates a new composite sub reflect
    pub fn new_sub(&self, elem : ReflectPathElem) -> Self{
        let mut accesses = self.accesses.clone();

        accesses.push(elem);

        Self{
            accesses,
            ..self.clone()
        }
    }


    /// Walks the path with the given reference as the base
    fn walk_path<'a>(&self, ref_ : &'a dyn Reflect) -> &'a dyn Reflect { 
        let first = self.accesses.first().map(|s| s.sub_ref(ref_));

        if let Some(first) = first {
            if self.accesses.len() > 1{
                self.accesses[1..].iter().fold(first, |a,access| {
                    access.sub_ref(a)
                }) 
            } else {
                first
            }
        } else {
            ref_
        }

    }

    /// Walks the path with the given mutable reference as the base.
    fn walk_path_mut<'a>(&self, ref_ : &'a mut dyn Reflect) -> &'a mut dyn Reflect {
        if let Some(first) = self.accesses.first(){
            if self.accesses.len() > 1{
                return self.accesses[1..].iter().fold(first.sub_ref_mut(ref_), |a,access| {
                    access.sub_ref_mut(a)
                }) 
            } else {
                first.sub_ref_mut(ref_)
            }
        } else {
            ref_
        }
    }

    pub fn len(&self) -> u8 {
        self.accesses.len() as u8
    }


    pub fn get<O,F>(&self, f: F) -> O  where 
        F : FnOnce(&dyn Reflect) -> O
    {
        match &self.base {
            ReflectBase::Component { comp, entity, world } => {
                let g = world.upgrade()
                .expect("Trying to access cached value from previous frame");
                let g = g.try_read().expect("Rust safety violation: attempted to borrow world while it was already mutably borrowed");
                
                let ref_ = self.walk_path(comp.reflect(&g, *entity).unwrap());
                // unsafe since pointer may be dangling
                let o = f(ref_);
                drop(g);
                o
            },
            ReflectBase::Resource { res, world } => {
                let g = world.upgrade()
                .expect("Trying to access cached value from previous frame");
                let g = g.try_read().expect("Rust safety violation: attempted to borrow world while it was already mutably borrowed");
                
                let ref_ = self.walk_path(res.reflect(&g).unwrap());
                // unsafe since pointer may be dangling
                let o = f(ref_);
                drop(g);
                o
            },
            ReflectBase::ScriptOwned { ptr, valid } => {
                let g = valid.upgrade()
                    .expect("Trying to access cached value from previous frame");

                let g = g.try_read().expect("Rust safety violation: attempted to borrow value {self:?} while it was already mutably borrowed");
                
                let ref_ = self.walk_path(unsafe{ptr.const_ref()});
                let o = f(ref_);
                drop(g);
                o
            },
        }
    }

    pub fn get_mut<O,F>(&mut self, f: F) -> O  
    where 
        F : FnOnce(&mut dyn Reflect) -> O 
    {
        match &self.base {
            ReflectBase::Component { comp, entity, world } => {
                let g = world.upgrade()
                .expect("Trying to access cached value from previous frame");
                let mut g = g.try_write().expect("Rust safety violation: attempted to borrow world while it was already mutably borrowed");
                
                let ref_ = self.walk_path_mut(comp.reflect_mut(&mut g, *entity).unwrap().into_inner());
                // unsafe since pointer may be dangling
                let o = f(ref_);
                drop(g);
                o
            },
            ReflectBase::Resource { res, world } => {
                let g = world.upgrade()
                .expect("Trying to access cached value from previous frame");
                let mut g = g.try_write().expect("Rust safety violation: attempted to borrow world while it was already mutably borrowed");
                
                let ref_ = self.walk_path_mut(res.reflect_mut(&mut g).unwrap().into_inner());
                // unsafe since pointer may be dangling
                let o = f(ref_);
                drop(g);
                o
            },
            ReflectBase::ScriptOwned { ptr, valid } => {
                let g = valid.upgrade()
                    .expect("Trying to access cached value from previous frame");

                let g = g.try_write().expect("Rust safety violation: attempted to borrow value {self:?} while it was already mutably borrowed");
                
                let ref_ = self.walk_path_mut(unsafe{ptr.mut_ref().expect("Expected mutable pointer")});
                let o = f(ref_);
                drop(g);
                o
            },
        }
    }

}
