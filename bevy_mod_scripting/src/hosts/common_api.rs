use std::{fmt,sync::Arc, cell::UnsafeCell, ops::Deref};
use bevy::reflect::*;
use anyhow::{anyhow,Result};
use rlua::Debug;

/// Represents a rust type but stored in a script. We try to store only references to actual rust data
/// to make data transfer cheaper, however sometimes lua has to semantically 'own' some stuff.
#[derive(Clone)]
pub enum ScriptReflectVal {
    /// A rust object living in the bevy world, or alternatively
    /// a reference to a subfield of a lua owned value
    Ref(*mut (dyn Reflect + 'static)), 
    /// A rust object living in the lua world, not in the bevy world
    /// It's not "concrete" since lua should be able to create anything 
    Owned(Arc<UnsafeCell<dyn Reflect + 'static>>),
}


pub struct PrintableReflect<'a>(pub &'a dyn Reflect);

impl fmt::Debug for PrintableReflect<'_>{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.print(f)
    }
}



pub trait PrintReflect {
    fn print(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

macro_rules! impl_downcast_print_cases {
    ($v:ident,$fmt:ident,$f:ty,$($x:ty),*) => {
        {   

            if let Some(i) = $v.downcast_ref::<$f>(){
                write!($fmt,"({:#?}){}",$v.type_name(),i)?;
            }
            $(
            else if let Some(i) = $v.downcast_ref::<$x>() {
                write!($fmt,"({:#?}){}",$v.type_name(),i)?;
            }
            )*
            else {
                write!($fmt,"({:#?})",$v.type_name())?;
            }
        }
    };
}

impl <T : Reflect + ?Sized> PrintReflect for &T {
    fn print(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.reflect_ref() {
            ReflectRef::Struct(s) => {
                (0..s.field_len()).fold(f.debug_struct(s.type_name()), |mut b, i| {
                    b.field(
                        s.name_at(i).unwrap(),
                        &PrintableReflect(s.field_at(i).unwrap()));
                    b
                }).finish()
            },
            ReflectRef::Map(m) => {
                m.iter().fold(f.debug_map(), |mut b, (k,v)| {
                    b.entry(
                        &PrintableReflect(k),
                        &PrintableReflect(v));
                    b
                }).finish()
            },
            ReflectRef::TupleStruct(ts) => {
                ts.iter_fields().fold(f.debug_tuple(ts.type_name()),|mut b, i|{
                    b.field(&PrintableReflect(i));
                    b
                }).finish()
            },
            ReflectRef::Tuple(t) => {
                t.iter_fields().fold(f.debug_tuple(""),|mut b, i|{
                    b.field(&PrintableReflect(i));
                    b
                }).finish()
            },
            ReflectRef::List(l) => {
                l.iter().fold(f.debug_list(), |mut b, i|{
                    b.entry(&PrintableReflect(i));
                    b
                }).finish()
            },
            ReflectRef::Value(v) => {
                impl_downcast_print_cases!(v,f,
                    usize,
                    isize,
                    u128,
                    i128,
                    u64,
                    i64,
                    u32,
                    i32,
                    u16,
                    i16,
                    u8,
                    i8,
                    f32,
                    f64,
                    String);
                Ok(())
            },
        }

    }
} 



impl fmt::Debug for ScriptReflectVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.ref_immut().print(f)
    }
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

        let re = ref_mut.path(path).map_err(|e| anyhow!("Cannot access field `{}`",path))?;
        Ok(Self::Ref(re as *const dyn Reflect as *mut dyn Reflect))
    }

    pub fn path_set(&mut self, path: &str, val : Box<dyn Reflect>) -> Result<()> {
        let ref_mut = self.ref_mut();

        match ref_mut.path_mut(path){
            Ok(f) => {
                f.apply(val.as_ref());
                Ok(())

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
