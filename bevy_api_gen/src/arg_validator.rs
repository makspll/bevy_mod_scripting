use rustdoc_types::{GenericArg, GenericArgs, Type};

/// A representation of valid argument types
#[derive(Debug)]
pub enum ArgType {
    /// The primary identifier of the type
    ///
    /// Valid types right now follow the following syntax:
    /// `(&)? (mut)? ident:ident`
    Self_,
    Base(String),
    Generic {
        base: Box<ArgType>,
        args: Vec<ArgType>,
    },
    Ref {
        is_mut: bool,
        ref_: Box<ArgType>,
    },
}

impl fmt::Display for ArgType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArgType::Base(b) => {
                if b == "Self" {
                    f.write_str("self")
                } else {
                    f.write_str(b)
                }
            }
            ArgType::Ref { is_mut, ref_ } => {
                if *is_mut {
                    f.write_str("&mut ")?;
                } else {
                    f.write_str("&")?;
                }

                ref_.fmt(f)
            }
            ArgType::Self_ => f.write_str("self"),
            ArgType::Generic { base, args } => {
                base.fmt(f)?;
                f.write_str("<")?;
                for (a, i) in args.iter().zip(1..) {
                    a.fmt(f)?;
                    if i != args.len() {
                        f.write_str(",")?;
                    }
                }
                f.write_str(">")
            }
        }
    }
}

impl TryFrom<Type> for ArgType {
    type Error = String;

    fn try_from(value: Type) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&Type> for ArgType {
    type Error = String;

    fn try_from(value: &Type) -> Result<Self, Self::Error> {
        match value {
            Type::ResolvedPath (path) => {
                let mut processed_args = Vec::default();

                for a in &path.args {
                    if let GenericArgs::AngleBracketed { args, bindings } = a.as_ref() {
                        for generic in args {
                            match generic {
                                GenericArg::Type(type_) => processed_args.push(type_.try_into()?),
                                _ => {
                                    return Err(
                                        "Only types are allowed as generic arguments".to_owned()
                                    )
                                }
                            }
                        }
                        if !bindings.is_empty() {
                            return Err("Type bindings are not supported".to_owned());
                        }
                    } else {
                        return Err("Parenthesised generics are not supported".to_owned());
                    }
                }
                let base = Type::Primitive(path.name.to_string()).try_into()?;
                if let base @ ArgType::Base(_) = base {
                    if !processed_args.is_empty() {
                        Ok(Self::Generic {
                            base: Box::new(base),
                            args: processed_args,
                        })
                    } else {
                        Ok(base)
                    }
                } else {
                    Err("Base is invalid".to_owned())
                }
            }
            Type::Primitive(name) | Type::Generic(name) => {
                if name == "Self" {
                    Ok(Self::Self_)
                } else {
                    Ok(Self::Base(name.split("::").last().unwrap().to_owned()))
                }
            }
            Type::BorrowedRef { mutable, type_, .. } => Ok(Self::Ref {
                is_mut: *mutable,
                ref_: Box::new(type_.as_ref().try_into()?),
            }),
            _ => Err("".to_owned()),
        }
    }
}

impl ArgType {
    /// Produce an arbitrary output given the base identifier of this type or err if this is the base is a self receiver
    pub fn map_base<F, O>(&self, f: F) -> O
    where
        F: FnOnce(Result<&String, ()>) -> O,
    {
        match self {
            ArgType::Base(b) => f(Ok(b)),
            ArgType::Ref { is_mut: _, ref_ } => ref_.map_base(f),
            ArgType::Self_ => f(Err(())),
            ArgType::Generic { base, .. } => base.map_base(f),
        }
    }

    /// Produce an arbitrary output given the base identifier of this type, and optionally modify it
    pub fn map_base_mut<F, O>(&mut self, f: F) -> O
    where
        F: FnOnce(Result<&mut String, ()>) -> O,
    {
        match self {
            ArgType::Base(b) => f(Ok(b)),
            ArgType::Ref { is_mut: _, ref_ } => ref_.map_base_mut(f),
            ArgType::Self_ => f(Err(())),
            ArgType::Generic { base, .. } => base.map_base_mut(f),
        }
    }

    pub fn is_self(&self) -> bool {
        self.map_base(|b| b.is_err())
    }

    /// Retrieves the base ident if this type is resolved otherwise returns None (i.e. in the case of a self receiver)
    pub fn base_ident(&self) -> Option<&str> {
        match self {
            ArgType::Base(b) => Some(b),
            ArgType::Ref { is_mut: _, ref_ } => ref_.base_ident(),
            ArgType::Self_ => None,
            ArgType::Generic { base, .. } => base.base_ident(),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum ArgWrapperType {
    Raw,
    Wrapped,
    /// in case of `self` argument
    None,
}

impl ArgWrapperType {
    pub fn with_config(self_type: &str, type_: &ArgType, config: &Config) -> Option<Self> {
        let base_ident = type_.base_ident().unwrap_or(self_type);
        type_
            .is_self()
            .then(|| ArgWrapperType::None)
            .or_else(|| {
                config
                    .primitives
                    .contains(base_ident)
                    .then_some(ArgWrapperType::Raw)
            })
            .or_else(|| {
                config
                    .types
                    .contains_key(base_ident)
                    .then_some(ArgWrapperType::Wrapped)
            })
    }
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

pub struct Arg {
    pub type_: ArgType,
    pub wrapper: ArgWrapperType,
}

impl Arg {
    pub fn new(type_: ArgType, wrapper: ArgWrapperType) -> Self {
        Self { type_, wrapper }
    }
}

use std::fmt;

use crate::Config;
impl fmt::Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = self.type_.to_string();

        match self.wrapper {
            ArgWrapperType::Raw | ArgWrapperType::Wrapped => {
                write!(f, "{}({inner})", self.wrapper)
            }
            ArgWrapperType::None => f.write_str(&inner),
        }
    }
}
