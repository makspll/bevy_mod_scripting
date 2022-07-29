use rustdoc_types::{Type};






/// A representation of valid argument types
#[derive(Debug)]
pub enum ArgType{
    /// The primary identifier of the type 
    /// 
    /// Valid types right now follow the following syntax:
    /// `(&)? (mut)? ident:ident`
    Self_,
    Base(String),
    Ref{
        is_mut: bool,
        ref_ : Box<ArgType>
    }
}

impl fmt::Display for ArgType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgType::Base(b) => {
                if b == "Self"{
                    f.write_str("self")
                } else {
                    f.write_str(b)
                }
            },
            ArgType::Ref { is_mut, ref_ } => {
                if *is_mut{
                    f.write_str("&mut ")?;
                }else {
                    f.write_str("&")?;
                }

                ref_.fmt(f)
            },
            ArgType::Self_ => f.write_str("self"),
            
        }
    }
}


impl TryFrom<Type> for ArgType {
    type Error=String;

    fn try_from(value: Type) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&Type> for ArgType {
    type Error=String;

    fn try_from(value: &Type) -> Result<Self, Self::Error> {
        match value {
            Type::Primitive(name) |
            Type::Generic(name) |
            Type::ResolvedPath { name, .. } => {
                if name == "Self" {
                    Ok(Self::Self_)
                } else {
                    Ok(Self::Base(name.split("::").last().unwrap().to_owned()))
                }
            },
            Type::BorrowedRef { mutable, type_, .. } => Ok(Self::Ref { is_mut: *mutable, ref_: Box::new(type_.as_ref().try_into()?) }),
            _ => Err("".to_owned())
        }
    }
}

impl ArgType {
    /// Produce an arbitrary output given the base identifier of this type or err if this is the base is a self receiver
    pub fn map_base<F,O>(&self,f : F) -> O where 
        F : FnOnce(Result<&String,()>) -> O,
    {
        match self 
        {
            ArgType::Base(b) => f(Ok(b)),
            ArgType::Ref { is_mut: _, ref_ } => ref_.map_base(f),
            ArgType::Self_ => f(Err(())),
            
        }
    }

    /// Produce an arbitrary output given the base identifier of this type, and optionally modify it
    pub fn map_base_mut<F,O>(&mut self,f : F) -> O where 
    F : FnOnce(Result<&mut String,()>) -> O
    {
        match self 
        {
            ArgType::Base(b) => f(Ok(b)),
            ArgType::Ref { is_mut: _, ref_ } => ref_.map_base_mut(f),
            ArgType::Self_ => f(Err(())),
            
        }
    }

    pub fn is_self(&self) -> bool {
        self.map_base(|b| b.is_err())
    }

    /// Retrieves the base ident if this type is resolved otherwise returns Err(()) (i.e. in the case of a self receiver)
    pub fn base_ident(&self) -> Result<&str,()> {
        match self {
            ArgType::Base(b) => Ok(b),
            ArgType::Ref { is_mut: _, ref_ } => ref_.base_ident(),
            ArgType::Self_ => Err(()),
        }
    }
}

#[derive(PartialEq,Eq)]
pub enum ArgWrapperType{
    Raw,
    Wrapped,
    /// in case of `self` argument
    None
}


impl fmt::Display for ArgWrapperType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgWrapperType::Raw => f.write_str("Raw"),
            ArgWrapperType::Wrapped => f.write_str("Wrapped"),
            ArgWrapperType::None => f.write_str("None"),
        }
    }
}


pub struct Arg{
    pub type_ : ArgType,
    pub wrapper: ArgWrapperType
}

impl Arg {
    pub fn new(type_: ArgType, wrapper: ArgWrapperType) -> Self{
        Self {
            type_,
            wrapper,
        }
    }

}

use std::fmt;
impl fmt::Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        let inner = self.type_.to_string();

        match self.wrapper {
            ArgWrapperType::Raw |
            ArgWrapperType::Wrapped => write!(f,"{}({inner})",self.wrapper.to_string()),
            ArgWrapperType::None => f.write_str(&inner),
        }
    }
}