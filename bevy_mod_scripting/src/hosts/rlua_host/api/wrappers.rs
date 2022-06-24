use bevy::reflect::Reflect;
use parking_lot::RwLock;
use tealr::{mlu::{TealData,mlua}, TypeBody};
use std::{ops::{Deref,DerefMut},sync::Arc, fmt::{Debug,Display, Formatter}};
use crate::LuaRef;

pub trait LuaWrappable : Reflect + Clone {}

impl <T : Reflect + Clone> LuaWrappable for T {}

#[derive(Debug,Clone)]
/// A lua wrapper for reflectable types
pub enum LuaWrapper<T : LuaWrappable> { 
    Owned(T,Arc<RwLock<()>>),
    Ref(LuaRef)
}

#[derive(Clone)]
/// A lua wrapper for any type, these wrappers cannot be assigned to component fields
pub struct AnyLuaWrapper<T>(T);


impl <T>AnyLuaWrapper<T> {
    pub fn new(i : T) -> Self{
        Self(i)
    }
}

impl<T>AsRef<T> for AnyLuaWrapper<T>{
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl <T>Deref for AnyLuaWrapper<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl <T>DerefMut for AnyLuaWrapper<T>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl <T : Debug> Debug for AnyLuaWrapper<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("AnyLuaWrapper").field(&self.0).finish()
    }
}


impl <T : LuaWrappable>Drop for LuaWrapper<T> {
    fn drop(&mut self) {
        match self {
            Self::Owned(_,valid) => {
                if valid.is_locked() {
                    panic!("Something is referencing a lua value and it's about to go out of scope!");
                }
            },
            Self::Ref(_) => {},
        }
    }
}

impl <T : LuaWrappable + Display> Display for LuaWrapper<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        write!(f,"{}", self)
    }
}

impl <T : LuaWrappable>LuaWrapper<T> {

    pub fn new(b : T) -> Self {
        Self::Owned(b,Arc::new(RwLock::new(())))
    }

    pub fn new_ref(b : &LuaRef) -> Self {
        Self::Ref(b.clone())
    }

    /// Perform an operation on the base type and optionally retrieve something by value
    /// may require a read lock on the world in case this is a reference
    pub fn val<G,F>(&self, accessor: F) -> G
        where 
        F: FnOnce(&T) -> G
    {
        match self {
            Self::Owned(ref v, valid) => {
                // we lock here in case the accessor has a luaref holding reference to us
                let lock = valid.read();
                let o = accessor(v);
                drop(lock);

                o
            },
            Self::Ref(v) => {
                v.get(|s,_| accessor(s.downcast_ref::<T>().unwrap()))
            },
        }
    }

    pub fn val_mut<G,F>(&mut self, accessor: F) -> G
        where 
        F: FnOnce(&mut T) -> G
    {
        match self {
            Self::Owned(ref mut v, valid) => {
                let lock = valid.read();
                let o = accessor(v);
                drop(lock);

                o
            },
            Self::Ref(v) => {
                v.get_mut(|s,_| accessor(s.downcast_mut::<T>().unwrap()))
            },
        }
    }

    /// returns wrapped value by value, 
    /// may require a read lock on the world in case this is a reference
    pub fn inner(&self) -> T
    {
        match self {
            Self::Owned(ref v, ..) => v.clone(),//no need to lock here
            Self::Ref(v) => {
                v.get(|s,_| s.downcast_ref::<T>().unwrap().clone())
            },
        }
    }

    /// Converts a LuaRef to Self
    pub fn base_to_self(b: &LuaRef) -> Self {
        Self::Ref(b.clone())
    }

    /// Applies Self to a LuaRef.
    /// may require a write lock on the world
    pub fn apply_self_to_base(&self, b: &mut LuaRef){

        match self {
            Self::Owned(ref v, ..) => {
                // if we own the value, we are not borrowing from the world
                // we're good to just apply, yeet
                b.get_mut(|b,_| b.apply(v))
            },
            Self::Ref(v) => {
                // if we are a luaref, we have to be careful with borrows
                b.apply_luaref(v)
            }
        }
    }
}


