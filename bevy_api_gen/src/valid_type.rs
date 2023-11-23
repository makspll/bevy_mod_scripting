use std::{error::Error, fmt};

use rustdoc_types::{GenericArg, GenericArgs, Type};

/// A representation of valid argument types
#[derive(Debug, Clone)]
pub enum ValidType {
    /// The primary identifier of the type
    ///
    /// Valid types right now follow the following syntax:
    /// `(&)? (mut)? ident:ident`
    AssociatedType {
        on: Box<ValidType>,
        name: String,
    },
    Self_ {
        in_receiver_position: bool,
    },
    Base(String),
    Generic {
        base: Box<ValidType>,
        args: Vec<ValidType>,
    },
    Ref {
        is_mut: bool,
        ref_: Box<ValidType>,
    },
}

impl fmt::Display for ValidType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidType::Base(b) => {
                if b == "Self" {
                    f.write_str("self")
                } else {
                    f.write_str(b)
                }
            }
            ValidType::Ref { is_mut, ref_ } => {
                if *is_mut {
                    f.write_str("&mut ")?;
                } else {
                    f.write_str("&")?;
                }

                ref_.fmt(f)
            }
            ValidType::Self_ {
                in_receiver_position,
            } => {
                if *in_receiver_position {
                    f.write_str("self")
                } else {
                    f.write_str("Self")
                }
            }
            ValidType::Generic { base, args } => {
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
            ValidType::AssociatedType { on, name } => {
                f.write_str("<")?;
                on.fmt(f)?;
                f.write_str("::")?;
                f.write_str(name)
            }
        }
    }
}

impl ValidType {
    pub fn try_new(is_receiver: bool, value: &Type) -> Result<Self, String> {
        match value {
            Type::ResolvedPath(path) => {
                let mut processed_args = Vec::default();

                for a in &path.args {
                    if let GenericArgs::AngleBracketed { args, bindings } = a.as_ref() {
                        for generic in args {
                            match generic {
                                GenericArg::Type(type_) => {
                                    processed_args.push(ValidType::try_new(is_receiver, type_)?)
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
                let base =
                    ValidType::try_new(is_receiver, &Type::Primitive(path.name.to_string()))?;
                if let base @ ValidType::Base(_) = base {
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
                ref_: Box::new(ValidType::try_new(is_receiver, type_.as_ref())?),
            }),
            Type::QualifiedPath {
                name, self_type, ..
            } => Ok(Self::AssociatedType {
                on: Box::new(ValidType::try_new(is_receiver, self_type.as_ref())?),
                name: name.to_owned(),
            }),
            _ => Err(format!("ArgType is not supported: `{:?}`", value)),
        }
    }
}

impl ValidType {
    /// Modify base and return modified self
    pub fn map_base_mut<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut String),
    {
        match self {
            ValidType::Base(b) => f(b),
            ValidType::Ref { is_mut: _, ref_ } => _ = ref_.map_base_mut(f),
            ValidType::Generic { base, .. } => _ = base.map_base_mut(f),
            _ => (),
        };
        self
    }

    pub fn is_receiver(&self) -> bool {
        match self {
            ValidType::Self_ {
                in_receiver_position,
            } => *in_receiver_position,
            ValidType::Generic { base, .. } => base.is_receiver(),
            ValidType::Ref { ref_, .. } => ref_.is_receiver(),
            _ => false,
        }
    }

    /// Returns true for Self/self arguments both in receiver and non receiver positions
    pub fn is_contextual(&self) -> bool {
        match self {
            ValidType::Self_ { .. } => true,
            ValidType::Generic { base, .. } => base.is_contextual(),
            ValidType::Ref { ref_, .. } => ref_.is_contextual(),
            _ => false,
        }
    }

    pub fn is_associated_type(&self) -> bool {
        match self {
            ValidType::AssociatedType { on, name } => true,
            ValidType::Self_ {
                in_receiver_position,
            } => false,
            ValidType::Base(_) => false,
            ValidType::Generic { base, args } => base.is_associated_type(),
            ValidType::Ref { is_mut, ref_ } => ref_.is_associated_type(),
        }
    }

    /// Retrieves the base ident if this type is resolved otherwise returns None (i.e. in the case of a self receiver or an associated type)
    pub fn base_ident(&self) -> Option<&str> {
        match self {
            ValidType::Base(b) => Some(b),
            ValidType::Ref { is_mut: _, ref_ } => ref_.base_ident(),
            ValidType::Self_ { .. } => None,
            ValidType::Generic { base, .. } => base.base_ident(),
            ValidType::AssociatedType { .. } => None,
        }
    }

    pub fn has_outer_ref(&self) -> bool {
        matches!(self, ValidType::Ref { .. })
    }

    /// Replaces every receiver node with the given type
    pub fn resolve_receivers_with(&mut self, other: &Self) -> &mut Self {
        match self {
            ValidType::Self_ { .. } => *self = other.clone(),
            ValidType::AssociatedType { on, .. } => _ = on.resolve_receivers_with(other),
            ValidType::Base(_) => (),
            ValidType::Generic { base, args } => {
                base.resolve_receivers_with(other);
                args.iter_mut()
                    .for_each(|a| _ = a.resolve_receivers_with(other));
            }
            ValidType::Ref { ref_, .. } => _ = ref_.resolve_receivers_with(other),
        };
        self
    }

    /// Runs a function on every node of AssociatedType in this type, if the function returns a value it is used to
    /// replace that node
    pub fn map_associated_types<F>(&mut self, f: &F) -> &mut Self
    where
        F: Fn(&Box<ValidType>, &String) -> Option<Self>,
    {
        match self {
            ValidType::AssociatedType { on, name } => {
                if let Some(new) = f(on, name) {
                    *self = new
                };
            }
            ValidType::Self_ { .. } => (),
            ValidType::Base(_) => (),
            ValidType::Generic {
                ref mut base,
                ref mut args,
            } => {
                base.map_associated_types(f);
                args.iter_mut()
                    .for_each(|arg| _ = arg.map_associated_types(f));
            }
            ValidType::Ref { ref_, .. } => _ = ref_.map_associated_types(f),
        };
        self
    }
}
