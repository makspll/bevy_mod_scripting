use bevy::reflect::Reflect;
use parking_lot::RwLock;

use std::{sync::Arc, fmt::{Debug,Display, Formatter}};
use crate::ScriptRef;


/// Script representable type with pass-by-value semantics
pub trait ScriptValue : Reflect + Clone {}
impl <T : Reflect + Clone> ScriptValue for T {}

/// Script representable type with pass-by-reference semantics
pub trait ScriptReference : Reflect {}
impl <T : Reflect> ScriptReference for T {}


#[derive(Debug,Clone)]
/// A wrapper for lua pass-by-value types possibly owned by lua itself
pub enum LuaWrapper<T : ScriptReference> { 
    Owned(T,Arc<RwLock<()>>),
    Ref(ScriptRef)
}


impl <T : ScriptReference>Drop for LuaWrapper<T> {
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

impl <T : ScriptReference + Display> Display for LuaWrapper<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        write!(f,"{}", self)
    }
}

impl <T : ScriptReference>LuaWrapper<T> {

    pub fn new(b : T) -> Self 
    where 
        T : ScriptValue
    {
        Self::Owned(b,Arc::new(RwLock::new(())))
    }

    pub fn new_ref(b : &ScriptRef) -> Self {
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

    /// retrieves the underlying value by cloning it 
    pub fn clone(&self) -> T
    where 
        T : ScriptValue
    {
        match self {
            Self::Owned(ref v, ..) => v.clone(),//no need to lock here
            Self::Ref(v) => {
                v.get(|s,_| s.downcast_ref::<T>().unwrap().clone())
            },
        }
    }


    /// Converts a LuaRef to Self
    pub fn base_to_self(b: &ScriptRef) -> Self {
        Self::new_ref(b)
    }

    /// Applies Self to a LuaRef.
    /// may require a write lock on the world
    pub fn apply_self_to_base(&self, b: &mut ScriptRef){

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

