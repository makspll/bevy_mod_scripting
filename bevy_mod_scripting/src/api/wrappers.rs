use bevy::reflect::Reflect;
use parking_lot::RwLock;

use std::{sync::Arc, fmt::{Debug,Display, Formatter}, cell::UnsafeCell};
use crate::{ScriptRef, ScriptRefBase, ReflectPtr, api::FromLua};


/// Script representable type with pass-by-value semantics
pub trait ScriptValue : Reflect + Clone {}
impl <T : Reflect + Clone> ScriptValue for T {}

/// Script representable type with pass-by-reference semantics
pub trait ScriptReference : Reflect {}
impl <T : Reflect> ScriptReference for T {}


#[derive(Debug)]
/// A wrapper for lua pass-by-value types possibly owned by lua itself
pub enum LuaWrapper<T : ScriptReference> { 
    Owned(UnsafeCell<T>,Arc<RwLock<()>>),
    Ref(ScriptRef)
}


impl <T : ScriptValue> Clone for LuaWrapper<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Owned(arg0, arg1) => Self::Owned(
                // Safety: fine since we are explicitly not derefing into &mut but into &
                UnsafeCell::new(unsafe {&*(arg0.get() as *const T)}.clone()), 
                arg1.clone()
            ),
            Self::Ref(arg0) => Self::Ref(arg0.clone()),
        }
    }
}



// TODO: look at this when rust gets better
// Oh boy, there is no way in current rust to implement this
// We need trait specialization.
// This isn't even possible if implemented without generics since then 
// we get a compile error from mlua about how `Clone` may be implemented on the wrapped type in the feature
// :C
// impl <'lua, T : ScriptReference + !Clone> FromLua<'lua> for LuaWrapper<T> {
//     fn from_lua(lua_value: tealr::mlu::mlua::Value<'lua>, lua: &'lua tealr::mlu::mlua::Lua) -> tealr::mlu::mlua::Result<Self> {
//         match lua_value {
//             tealr::mlu::mlua::Value::UserData(ud) => {
                
//             match ud.borrow::<LuaWrapper<T>>()?{
//                 // here we need to move out of the value in the lua world
//                 LuaWrapper::Owned(_, _) => ud.take(),
//                 // we can copy fine here
//                 LuaWrapper::Ref(ref_) => Ok(LuaWrapper::new_ref(&ref_)),
//             }
//         }
//             _ => Err(tealr::mlu::mlua::Error::FromLuaConversionError {
//                 from: lua_value.type_name(),
//                 to: "userdata",
//                 message: None,
//             })
//         }
//     }
// }



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

    /// Creates a script reference pointing to this wrapper.
    /// 
    /// Depending on this value it may be a lua owned or reflect relative reference
    pub fn script_ref(&self) -> ScriptRef {
        match self {
            LuaWrapper::Owned(val, valid) => {
                ScriptRef {
                    root: ScriptRefBase::ScriptOwned { valid: Arc::downgrade(valid) },
                    path: None,
                    r: ReflectPtr::Mut(val.get()),
                }
            },
            LuaWrapper::Ref(ref_) => {
                ref_.clone()
            },
        }
    }

    pub fn new(b : T) -> Self 
    where 
        T : ScriptValue
    {
        Self::Owned(UnsafeCell::new(b),Arc::new(RwLock::new(())))
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
                let o = accessor(unsafe{&*(v.get() as *const T)});
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
                let o = accessor(v.get_mut());
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
            //no need to lock here
            Self::Owned(ref v, ..) => unsafe{&*(v.get() as *const T)}.clone(),
            Self::Ref(v) => {
                v.get(|s,_| s.downcast_ref::<T>().unwrap().clone())
            },
        }
    }


    /// Converts a ScriptRef to Self
    pub fn base_to_self(b: &ScriptRef) -> Self {
        Self::new_ref(b)
    }

    /// Applies Self to another ScriptRef.
    /// may require a write lock on the world
    pub fn apply_self_to_base(&self, other: &mut ScriptRef)
    {
        match self {
            Self::Owned(v, ..) => {
                // if we own the value, we are not borrowing from the world
                // we're good to just apply, yeet
                // TODO: we use apply here due to the fact we implement `Drop`
                // if we didn't or if ScriptRef itself owned the value we could just consume the cell and assign
                other.get_mut(|other,_| other.apply(unsafe {&*(v.get() as *const T)}))
            },
            Self::Ref(v) => {
                // if we are a ScriptRef, we have to be careful with borrows
                // to avoid deadlock
                // we take advantage of the fact we know the expected type
                other.apply(v)
            }
        }
    }

    /// Applies Self to another ScriptRef.
    /// may require a write lock on the world
    pub fn apply_self_to_base_typed(&self, other: &mut ScriptRef) -> Result<(),tealr::mlu::mlua::Error>
    where 
        T : ScriptValue
    {
        match self {
            Self::Owned(v, ..) => {
                // if we own the value, we are not borrowing from the world
                // we're good to just apply, yeet
                // TODO: we use apply here due to the fact we implement `Drop`
                // if we didn't or if ScriptRef itself owned the value we could just consume the cell and assign
                other.get_mut(|other,_| other.apply(unsafe {&*(v.get() as *const T)}))
            },
            Self::Ref(v) => {
                // if we are a ScriptRef, we have to be careful with borrows
                // to avoid deadlock
                // we take advantage of the fact we know the expected type
                other.apply_typed::<T>(v)?
            }
        };

        Ok(())
    }

}

