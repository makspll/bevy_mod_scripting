use std::{fmt,sync::Arc, cell::UnsafeCell, ops::Deref};
use bevy::reflect::*;
use anyhow::{anyhow,Result};

/// Represents a rust type but stored in a script. We try to store only references to actual rust data
/// to make data transfer cheaper, however sometimes lua has to semantically 'own' some stuff.
#[derive(Debug,Clone)]
pub enum ScriptReflectVal {
    /// A rust object living in the bevy world, or alternatively
    /// a reference to a subfield of a lua owned value
    Ref(*mut (dyn Reflect + 'static)), 
    /// A rust object living in the lua world, not in the bevy world
    /// It's not "concrete" since lua should be able to create anything 
    Owned(Arc<UnsafeCell<dyn Reflect + 'static>>),
}



impl fmt::Display for ScriptReflectVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{:?}",self)
    }
}


impl ScriptReflectVal {

    pub fn ref_immut(&self) -> &dyn Reflect {
        match self {
            ScriptReflectVal::Ref(r) => unsafe{  &**r },
            ScriptReflectVal::Owned(r) => unsafe{&*r.deref().get() }  
        }    
    }

    pub fn ref_mut(&mut self) -> &mut dyn Reflect {
        match self {
            ScriptReflectVal::Ref(r) => unsafe{  &mut **r },
            ScriptReflectVal::Owned(r) => unsafe{&mut *r.deref().get() } 
        }
    }

    pub fn to_owned(&mut self) -> Result<Box<dyn Reflect + 'static>> {
        match self {
            ScriptReflectVal::Ref(r) => Ok(unsafe{ Box::from_raw(*r) }),
            ScriptReflectVal::Owned(r) => Err(anyhow!("")),
        }
    }

    pub fn path_ref(&self, path: &str) -> Result<Self> {
        let ref_mut = self.ref_immut();

        let re = ref_mut.path(path).map_err(|e| anyhow!(e.to_string()))?;
        Ok(Self::Ref(re as *const dyn Reflect as *mut dyn Reflect))
    }

    pub fn path_set(&mut self, path: &str, val : Box<dyn Reflect>) -> Result<()> {
        let ref_mut = self.ref_mut();

        match ref_mut.path_mut(path){
            Ok(f) => {
                f.set(val)
                .map_err(|_| anyhow!("No field named {} exists", path))
            },
            Err(e) => {
                // we check if we are a dynamic struct/enum since then we can add the field
                // right now only structs are supported by bevy_reflect TODO
                let struc = ref_mut.downcast_mut::<DynamicStruct>().ok_or(anyhow!(e.to_string()))?;
                
                Ok(struc.insert_boxed(path, val))
            },
        }
        

    }
}

unsafe impl Send for ScriptReflectVal {}
