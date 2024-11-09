use bevy::reflect::PartialReflect;
use parking_lot::RwLock;
use std::fmt;
use std::fmt::{Debug, Display};
use std::sync::Arc;
use std::{borrow::Cow, sync::Weak};

use bevy::{
    prelude::{Entity, ReflectComponent, ReflectResource},
    reflect::{Reflect, ReflectMut, ReflectRef},
};

use crate::error::ReflectionError;
use bevy_mod_scripting_core::world::WorldPointer;

/// The base of a reflect path, i.e. the top-level object or source.
/// Reflections paths are always relative to some reflect base.
///
/// If the reflection base and reflection path are both valid we can use them to traverse reflect types
#[derive(Clone)]
pub(crate) enum ReflectBase {
    /// A bevy component reference
    Component {
        comp: ReflectComponent,
        entity: Entity,
    },
    /// A bevy resource reference
    Resource { res: ReflectResource },

    /// A script owned reflect type (for example a vector constructed in lua)
    ScriptOwned { val: Weak<RwLock<dyn Reflect>> },
}

/// Safety: we can safely send this value across thread boundaries
/// the pointer variant is always accessed with the
unsafe impl Send for ReflectBase {}
/// Safety: todo!()
unsafe impl Sync for ReflectBase {}

impl fmt::Debug for ReflectBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Component { entity, .. } => {
                f.debug_struct("Component").field("entity", entity).finish()
            }
            Self::ScriptOwned { .. } => write!(f, "ScriptOwned"),
            Self::Resource { .. } => f.debug_struct("Resource").finish(),
        }
    }
}

impl fmt::Display for ReflectBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReflectBase::Component { entity, .. } => {
                f.write_str("(Component on ")?;
                f.write_str(&entity.index().to_string())?;
                f.write_str(")")
            }
            ReflectBase::Resource { .. } => f.write_str("(Resource)"),
            ReflectBase::ScriptOwned { .. } => f.write_str("(ScriptOwned)"),
        }
    }
}

pub type Get = dyn Fn(&dyn Reflect) -> Result<&dyn Reflect, ReflectionError>;
pub type GetMut = dyn Fn(&mut dyn Reflect) -> Result<&mut dyn Reflect, ReflectionError>;

/// Stores a part of the path of reflection + sub reflection from a root reflect reference.
/// Sub reflection allows us to access values unreachable by standard reflection.
#[derive(Clone)]
pub enum ReflectionPathElement {
    SubReflection {
        label: &'static str,
        get: Arc<Get>,
        get_mut: Arc<GetMut>,
    },
    /// Access to a struct field
    FieldAccess(Cow<'static, str>),
    /// Access to a TupleStruct, Tuple, List or Array element
    IndexAccess(usize), // TODO: Map access
}

impl Debug for ReflectionPathElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SubReflection { label, .. } => f
                .debug_struct("SubReflection")
                .field("label", label)
                .finish(),
            Self::FieldAccess(arg0) => f.debug_tuple("FieldAccess").field(arg0).finish(),
            Self::IndexAccess(arg0) => f.debug_tuple("IndexAccess").field(arg0).finish(),
        }
    }
}

impl Display for ReflectionPathElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReflectionPathElement::SubReflection { label, .. } => {
                f.write_str(".")?;
                f.write_str(label)?;
                f.write_str("()")
            }
            ReflectionPathElement::FieldAccess(s) => {
                f.write_str(".")?;
                f.write_str(s)
            }
            ReflectionPathElement::IndexAccess(i) => {
                f.write_str("[")?;
                f.write_str(&i.to_string())?;
                f.write_str("]")
            }
        }
    }
}

impl ReflectionPathElement {
    pub(crate) fn sub_ref<'a>(
        &self,
        base: &'a dyn Reflect,
    ) -> Result<&'a dyn Reflect, ReflectionError> {
        match self {
            ReflectionPathElement::SubReflection { get, .. } => get(base),
            ReflectionPathElement::FieldAccess(field) => match base.reflect_ref() {
                ReflectRef::Struct(s) => s
                    .field(field)
                    .and_then(PartialReflect::try_as_reflect)
                    .ok_or_else(|| ReflectionError::InvalidReflectionPath {
                        path: self.to_string(),
                        msg: "No such field".to_owned(),
                    }),
                _ => Err(ReflectionError::InvalidReflectionPath {
                    path: self.to_string(),
                    msg: "No such field".to_owned(),
                }),
            },
            ReflectionPathElement::IndexAccess(index) => match base.reflect_ref() {
                ReflectRef::TupleStruct(s) => s
                    .field(*index)
                    .and_then(PartialReflect::try_as_reflect)
                    .ok_or_else(|| ReflectionError::InvalidReflectionPath {
                        path: self.to_string(),
                        msg: "No such element".to_owned(),
                    }),
                ReflectRef::Tuple(s) => s
                    .field(*index)
                    .and_then(PartialReflect::try_as_reflect)
                    .ok_or_else(|| ReflectionError::InvalidReflectionPath {
                        path: self.to_string(),
                        msg: "No such element".to_owned(),
                    }),
                ReflectRef::List(s) => s
                    .get(*index)
                    .and_then(PartialReflect::try_as_reflect)
                    .ok_or_else(|| ReflectionError::InvalidReflectionPath {
                        path: self.to_string(),
                        msg: "No such element".to_owned(),
                    }),
                ReflectRef::Array(s) => s
                    .get(*index)
                    .and_then(PartialReflect::try_as_reflect)
                    .ok_or_else(|| ReflectionError::InvalidReflectionPath {
                        path: self.to_string(),
                        msg: "No such element".to_owned(),
                    }),
                _ => Err(ReflectionError::InvalidReflectionPath {
                    path: self.to_string(),
                    msg: "No such element".to_owned(),
                }),
            },
        }
    }

    pub(crate) fn sub_ref_mut<'a>(
        &self,
        base: &'a mut dyn Reflect,
    ) -> Result<&'a mut dyn Reflect, ReflectionError> {
        match self {
            ReflectionPathElement::SubReflection { get_mut, .. } => get_mut(base),
            ReflectionPathElement::FieldAccess(field) => match base.reflect_mut() {
                ReflectMut::Struct(s) => s
                    .field_mut(field)
                    .and_then(PartialReflect::try_as_reflect_mut)
                    .ok_or_else(|| ReflectionError::InvalidReflectionPath {
                        path: self.to_string(),
                        msg: "No such field".to_owned(),
                    }),
                _ => Err(ReflectionError::InvalidReflectionPath {
                    path: self.to_string(),
                    msg: "No such field".to_owned(),
                }),
            },
            ReflectionPathElement::IndexAccess(index) => match base.reflect_mut() {
                ReflectMut::TupleStruct(s) => s
                    .field_mut(*index)
                    .and_then(PartialReflect::try_as_reflect_mut)
                    .ok_or_else(|| ReflectionError::InvalidReflectionPath {
                        path: self.to_string(),
                        msg: "No such element".to_owned(),
                    }),
                ReflectMut::Tuple(s) => s
                    .field_mut(*index)
                    .and_then(PartialReflect::try_as_reflect_mut)
                    .ok_or_else(|| ReflectionError::InvalidReflectionPath {
                        path: self.to_string(),
                        msg: "No such element".to_owned(),
                    }),
                ReflectMut::List(s) => s
                    .get_mut(*index)
                    .and_then(PartialReflect::try_as_reflect_mut)
                    .ok_or_else(|| ReflectionError::InvalidReflectionPath {
                        path: self.to_string(),
                        msg: "No such element".to_owned(),
                    }),
                ReflectMut::Array(s) => s
                    .get_mut(*index)
                    .and_then(PartialReflect::try_as_reflect_mut)
                    .ok_or_else(|| ReflectionError::InvalidReflectionPath {
                        path: self.to_string(),
                        msg: "No such element".to_owned(),
                    }),
                _ => Err(ReflectionError::InvalidReflectionPath {
                    path: self.to_string(),
                    msg: "No such element".to_owned(),
                }),
            },
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ReflectionPath {
    base: ReflectBase,
    // most of these will be very short, people don't make many nested hashmaps vecs etc.
    accesses: Vec<ReflectionPathElement>,
}

impl Display for ReflectionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.base.to_string())?;
        for access in &self.accesses {
            f.write_str(&access.to_string())?
        }
        Ok(())
    }
}

impl ReflectionPath {
    pub fn new(base: ReflectBase) -> Self {
        Self {
            base,
            accesses: Vec::default(),
        }
    }

    /// Creates a new composite sub reflect
    pub fn new_sub(&self, elem: ReflectionPathElement) -> Self {
        let mut accesses = self.accesses.clone();

        accesses.push(elem);

        Self {
            accesses,
            ..self.clone()
        }
    }

    /// Walks the path with the given reference as the base
    fn walk_path<'a>(&self, ref_: &'a dyn Reflect) -> Result<&'a dyn Reflect, ReflectionError> {
        let first = self.accesses.first().map(|s| s.sub_ref(ref_));

        if let Some(first) = first {
            if self.accesses.len() > 1 {
                self.accesses[1..]
                    .iter()
                    .try_fold(first?, |a, access| access.sub_ref(a))
            } else {
                first
            }
        } else {
            Ok(ref_)
        }
    }

    /// Walks the path with the given mutable reference as the base.
    fn walk_path_mut<'a>(
        &self,
        ref_: &'a mut dyn Reflect,
    ) -> Result<&'a mut dyn Reflect, ReflectionError> {
        if let Some(first) = self.accesses.first() {
            if self.accesses.len() > 1 {
                return self.accesses[1..]
                    .iter()
                    .try_fold(first.sub_ref_mut(ref_)?, |a, access| access.sub_ref_mut(a));
            } else {
                first.sub_ref_mut(ref_)
            }
        } else {
            Ok(ref_)
        }
    }

    pub fn get<O, F>(&self, world_ptr: WorldPointer, f: F) -> Result<O, ReflectionError>
    where
        F: FnOnce(&dyn Reflect) -> O,
    {
        match &self.base {
            ReflectBase::Component { comp, entity } => {
                let g = world_ptr.read();

                let entity_ref =
                    g.get_entity(*entity)
                        .map_err(|e| ReflectionError::InvalidBaseReference {
                            base: self.base.to_string(),
                            reason: format!("This entity could not be retrieved. {e}"),
                        })?;

                let ref_ = self.walk_path(comp.reflect(entity_ref).ok_or_else(|| {
                    ReflectionError::InvalidBaseReference {
                        base: self.base.to_string(),
                        reason: "Given component does not exist on this entity".to_owned(),
                    }
                })?)?;
                Ok(f(ref_))
            }
            ReflectBase::Resource { res } => {
                let g = world_ptr.read();

                let ref_ = self.walk_path(res.reflect(&g).ok_or_else(|| {
                    ReflectionError::InvalidBaseReference {
                        base: self.base.to_string(),
                        reason: "Given resource does not exist in this world".to_owned(),
                    }
                })?)?;
                Ok(f(ref_))
            }
            ReflectBase::ScriptOwned { val } => {
                let g = val
                    .upgrade()
                    .expect("Trying to access cached value from previous frame");

                let g = g.try_read().expect("Rust safety violation: attempted to borrow value {self:?} while it was already mutably borrowed");
                Ok(f(self.walk_path(&*g)?))
            }
        }
    }

    pub fn get_mut<O, F>(&mut self, world_ptr: WorldPointer, f: F) -> Result<O, ReflectionError>
    where
        F: FnOnce(&mut dyn Reflect) -> O,
    {
        match &self.base {
            ReflectBase::Component { comp, entity } => {
                let mut g = world_ptr.write();

                let mut e = g.entity_mut(*entity);
                let ref_ = self.walk_path_mut(
                    comp.reflect_mut(&mut e)
                        .ok_or_else(|| ReflectionError::InvalidBaseReference {
                            base: self.base.to_string(),
                            reason: "Given component does not exist on this entity".to_owned(),
                        })?
                        .into_inner(),
                )?;
                Ok(f(ref_))
            }
            ReflectBase::Resource { res } => {
                let mut g = world_ptr.write();

                let ref_ = self.walk_path_mut(
                    res.reflect_mut(&mut g)
                        .ok_or_else(|| ReflectionError::InvalidBaseReference {
                            base: self.base.to_string(),
                            reason: "Given resource does not exist in this world".to_owned(),
                        })?
                        .into_inner(),
                )?;
                Ok(f(ref_))
            }
            ReflectBase::ScriptOwned { val } => {
                let g = val
                    .upgrade()
                    .expect("Trying to access cached value from previous frame");
                let mut g = g.try_write().expect("Rust safety violation: attempted to borrow value {self:?} while it was already mutably borrowed");
                Ok(f(self.walk_path_mut(&mut *g)?))
            }
        }
    }
}
