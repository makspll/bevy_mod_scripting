use std::{error::Error, fmt};

use rustdoc_types::{GenericArg, GenericArgs, Type};

/// A representation of valid argument types
#[derive(Debug, Clone)]
pub enum ArgType {
    /// The primary identifier of the type
    ///
    /// Valid types right now follow the following syntax:
    /// `(&)? (mut)? ident:ident`
    AssociatedType {
        on: Box<ArgType>,
        name: String,
    },
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
            ArgType::AssociatedType { on, name } => {
                f.write_str("<")?;
                on.fmt(f)?;
                f.write_str("::")?;
                f.write_str(name)
            }
        }
    }
}

impl ArgType {
    pub fn try_new(is_receiver: bool, value: &Type) -> Result<Self, String> {
        match value {
            Type::ResolvedPath(path) => {
                let mut processed_args = Vec::default();

                for a in &path.args {
                    if let GenericArgs::AngleBracketed { args, bindings } = a.as_ref() {
                        for generic in args {
                            match generic {
                                GenericArg::Type(type_) => {
                                    processed_args.push(ArgType::try_new(is_receiver, type_)?)
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
                let base = ArgType::try_new(is_receiver, &Type::Primitive(path.name.to_string()))?;
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
                ref_: Box::new(ArgType::try_new(is_receiver, type_.as_ref())?),
            }),
            Type::QualifiedPath {
                name, self_type, ..
            } => Ok(Self::AssociatedType {
                on: Box::new(ArgType::try_new(is_receiver, self_type.as_ref())?),
                name: name.to_owned(),
            }),
            _ => Err(format!("ArgType is not supported: `{:?}`", value)),
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

    /// Returns true for Self/self arguments both in receiver and non receiver positions
    pub fn is_contextual(&self) -> bool {
        match self {
            ArgType::Self_ { .. } => true,
            ArgType::Generic { base, .. } => base.is_contextual(),
            ArgType::Ref { ref_, .. } => ref_.is_contextual(),
            _ => false,
        }
    }

    pub fn is_associated_type(&self) -> bool {
        match self {
            ArgType::AssociatedType { on, name } => true,
            ArgType::Self_ {
                in_receiver_position,
            } => false,
            ArgType::Base(_) => false,
            ArgType::Generic { base, args } => base.is_associated_type(),
            ArgType::Ref { is_mut, ref_ } => ref_.is_associated_type(),
        }
    }

    /// Retrieves the base ident if this type is resolved otherwise returns None (i.e. in the case of a self receiver or an associated type)
    pub fn base_ident(&self) -> Option<&str> {
        match self {
            ArgType::Base(b) => Some(b),
            ArgType::Ref { is_mut: _, ref_ } => ref_.base_ident(),
            ArgType::Self_ { .. } => None,
            ArgType::Generic { base, .. } => base.base_ident(),
            ArgType::AssociatedType { .. } => None,
        }
    }

    pub fn has_outer_ref(&self) -> bool {
        matches!(self, ArgType::Ref { .. })
    }

    pub fn map_associated_types<F>(self, f: &F) -> Self
    where
        F: Fn(Box<ArgType>, String) -> Option<Self>,
    {
        match self {
            ArgType::AssociatedType { on, name } => {
                f(on.clone(), name.clone()).unwrap_or(ArgType::AssociatedType { on, name })
            }
            ArgType::Self_ { .. } => self,
            ArgType::Base(_) => self,
            ArgType::Generic { base, args } => ArgType::Generic {
                base: Box::new(base.map_associated_types(f)),
                args: args
                    .into_iter()
                    .map(|a| a.map_associated_types(f))
                    .collect(),
            },
            ArgType::Ref { ref_, is_mut } => ArgType::Ref {
                ref_: Box::new(ref_.map_associated_types(f)),
                is_mut,
            },
        }
    }
}
