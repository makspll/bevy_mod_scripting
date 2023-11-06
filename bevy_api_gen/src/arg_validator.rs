use rustdoc_types::{GenericArg, GenericArgs, Type};

/// A representation of valid argument types
#[derive(Debug, Clone)]
pub enum ArgType {
    /// The primary identifier of the type
    ///
    /// Valid types right now follow the following syntax:
    /// `(&)? (mut)? ident:ident`
    Self_ {
        in_receiver_position: bool,
    },
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
            ArgType::Self_ {
                in_receiver_position,
            } => {
                if *in_receiver_position {
                    f.write_str("self")
                } else {
                    f.write_str("Self")
                }
            }
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

impl TryFrom<(bool, &Type)> for ArgType {
    type Error = String;

    fn try_from((is_receiver, value): (bool, &Type)) -> Result<Self, Self::Error> {
        match value {
            Type::ResolvedPath(path) => {
                let mut processed_args = Vec::default();

                for a in &path.args {
                    if let GenericArgs::AngleBracketed { args, bindings } = a.as_ref() {
                        for generic in args {
                            match generic {
                                GenericArg::Type(type_) => {
                                    processed_args.push((is_receiver, type_).try_into()?)
                                }
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
                let base = (is_receiver, &Type::Primitive(path.name.to_string())).try_into()?;
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
                if name == "Self" || is_receiver {
                    Ok(Self::Self_ {
                        in_receiver_position: is_receiver,
                    })
                } else {
                    Ok(Self::Base(name.split("::").last().unwrap().to_owned()))
                }
            }
            Type::BorrowedRef { mutable, type_, .. } => Ok(Self::Ref {
                is_mut: *mutable,
                ref_: Box::new((is_receiver, type_.as_ref()).try_into()?),
            }),
            _ => Err("ArgType is not supported".to_owned()),
        }
    }
}

impl ArgType {
    /// Modify base and return modified self
    pub fn map_base_mut<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut String),
    {
        match self {
            ArgType::Base(b) => f(b),
            ArgType::Ref { is_mut: _, ref_ } => _ = ref_.map_base_mut(f),
            ArgType::Generic { base, .. } => _ = base.map_base_mut(f),
            _ => (),
        };
        self
    }

    pub fn is_receiver(&self) -> bool {
        match self {
            ArgType::Self_ {
                in_receiver_position,
            } => *in_receiver_position,
            ArgType::Generic { base, .. } => base.is_receiver(),
            ArgType::Ref { ref_, .. } => ref_.is_receiver(),
            _ => false,
        }
    }

    pub fn is_contextual(&self) -> bool {
        match self {
            ArgType::Self_ { .. } => true,
            ArgType::Generic { base, .. } => base.is_receiver(),
            ArgType::Ref { ref_, .. } => ref_.is_receiver(),
            _ => false,
        }
    }

    /// Retrieves the base ident if this type is resolved otherwise returns None (i.e. in the case of a self receiver)
    pub fn base_ident(&self) -> Option<&str> {
        match self {
            ArgType::Base(b) => Some(b),
            ArgType::Ref { is_mut: _, ref_ } => ref_.base_ident(),
            ArgType::Self_ { .. } => None,
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
            .is_receiver()
            .then_some(ArgWrapperType::None)
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
