use parking_lot::RwLock;
use std::fmt;
use std::fmt::{Debug, Display};
use std::ops::{Deref, DerefMut};
use std::{borrow::Cow, sync::Weak};

use bevy::{
    prelude::{Entity, ReflectComponent, ReflectResource},
    reflect::{Reflect, ReflectMut, ReflectRef},
};

use crate::error::ReflectionError;
use bevy_mod_scripting_core::world::WorldPointer;

/// The base of a reflect path, i.e. the top-level object or source. Reflections paths are always relative to some reflect base
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
    /// These can be de-allocated whenever the script gc picks them up, so every script owned object
    /// has safety features.
    ///
    /// It's extremely important that the userdata aliasing rules are upheld.
    /// this is protected in  rust -> lua accesses using the valid pointer. on the lua side,
    /// we handle references directly which are safe. If those accesses are ever mixed, one must be extremely careful!
    /// Safety:
    /// this is only accessible within the crate but internally, as long as the pointer points to a value whose lifetime is guarded by the
    /// valid pointer, we are okay to dereference the pointer. Script owned values are of course owned by us so we can get (well mlua)
    /// both a mutable and non mutable reference no problem assuming mlua does not do anything unexpected TODO: double check :) or box
    ScriptOwned {
        /// We use the rwlock to validate reads and writes
        /// When a script value goes out of scope, it checks there are no strong references
        /// to this value, if there are it panicks,
        /// so being able to acquire a read/write lock is enough to validate the lifetime
        val: Weak<RwLock<dyn Reflect>>,
    },
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

pub type Get = fn(&dyn Reflect) -> Result<&dyn Reflect, ReflectionError>;
pub type GetMut = fn(&mut dyn Reflect) -> Result<&mut dyn Reflect, ReflectionError>;

pub type IndexedGet = fn(usize, &dyn Reflect) -> Result<&dyn Reflect, ReflectionError>;
pub type IndexedGetMut = fn(usize, &mut dyn Reflect) -> Result<&mut dyn Reflect, ReflectionError>;

/// Stores the path of reflection + sub reflection from a root reflect reference.
///
/// Also allows accessing elements beyond reach of the normal reflect API
#[derive(Clone)]
pub(crate) enum ReflectPathElem {
    SubReflection {
        label: &'static str,
        get: Get,
        get_mut: GetMut,
    },
    SubReflectionIndexed {
        label: &'static str,
        index: usize,
        get: IndexedGet,
        get_mut: IndexedGetMut,
    },
    /// Access to a struct field
    FieldAccess(Cow<'static, str>),
    /// Access to a TupleStruct, Tuple, List or Array element
    IndexAccess(usize), // TODO: Map access
}

impl Debug for ReflectPathElem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SubReflection { label, .. } => f
                .debug_struct("SubReflection")
                .field("label", label)
                .finish(),
            Self::SubReflectionIndexed { label, index, .. } => f
                .debug_struct("SubReflection")
                .field("label", label)
                .field("index", index)
                .finish(),
            Self::FieldAccess(arg0) => f.debug_tuple("FieldAccess").field(arg0).finish(),
            Self::IndexAccess(arg0) => f.debug_tuple("IndexAccess").field(arg0).finish(),
        }
    }
}

impl Display for ReflectPathElem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReflectPathElem::SubReflection { label, .. } => {
                f.write_str(".")?;
                f.write_str(label)?;
                f.write_str("()")
            }
            ReflectPathElem::SubReflectionIndexed { label, index, .. } => {
                f.write_str(".")?;
                f.write_str(label)?;
                f.write_str("(")?;
                f.write_str(&index.to_string())?;
                f.write_str(")")
            }
            ReflectPathElem::FieldAccess(s) => {
                f.write_str(".")?;
                f.write_str(s)
            }
            ReflectPathElem::IndexAccess(i) => {
                f.write_str("[")?;
                f.write_str(&i.to_string())?;
                f.write_str("]")
            }
        }
    }
}

impl ReflectPathElem {
    pub(crate) fn sub_ref<'a>(
        &self,
        base: &'a dyn Reflect,
    ) -> Result<&'a dyn Reflect, ReflectionError> {
        match self {
            ReflectPathElem::SubReflection { get, .. } => get(base),
            ReflectPathElem::SubReflectionIndexed { get, index, .. } => get(*index, base),
            ReflectPathElem::FieldAccess(field) => match base.reflect_ref() {
                ReflectRef::Struct(s) => {
                    s.field(field)
                        .ok_or_else(|| ReflectionError::InvalidReflectionPath {
                            path: self.to_string(),
                            msg: "No such field".to_owned(),
                        })
                }
                _ => Err(ReflectionError::InvalidReflectionPath {
                    path: self.to_string(),
                    msg: "No such field".to_owned(),
                }),
            },
            ReflectPathElem::IndexAccess(index) => match base.reflect_ref() {
                ReflectRef::TupleStruct(s) => {
                    s.field(*index)
                        .ok_or_else(|| ReflectionError::InvalidReflectionPath {
                            path: self.to_string(),
                            msg: "No such element".to_owned(),
                        })
                }
                ReflectRef::Tuple(s) => {
                    s.field(*index)
                        .ok_or_else(|| ReflectionError::InvalidReflectionPath {
                            path: self.to_string(),
                            msg: "No such element".to_owned(),
                        })
                }
                ReflectRef::List(s) => {
                    s.get(*index)
                        .ok_or_else(|| ReflectionError::InvalidReflectionPath {
                            path: self.to_string(),
                            msg: "No such element".to_owned(),
                        })
                }
                ReflectRef::Array(s) => {
                    s.get(*index)
                        .ok_or_else(|| ReflectionError::InvalidReflectionPath {
                            path: self.to_string(),
                            msg: "No such element".to_owned(),
                        })
                }
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
            ReflectPathElem::SubReflection { get_mut, .. } => get_mut(base),
            ReflectPathElem::SubReflectionIndexed { get_mut, index, .. } => get_mut(*index, base),
            ReflectPathElem::FieldAccess(field) => match base.reflect_mut() {
                ReflectMut::Struct(s) => {
                    s.field_mut(field)
                        .ok_or_else(|| ReflectionError::InvalidReflectionPath {
                            path: self.to_string(),
                            msg: "No such field".to_owned(),
                        })
                }
                _ => Err(ReflectionError::InvalidReflectionPath {
                    path: self.to_string(),
                    msg: "No such field".to_owned(),
                }),
            },
            ReflectPathElem::IndexAccess(index) => match base.reflect_mut() {
                ReflectMut::TupleStruct(s) => {
                    s.field_mut(*index)
                        .ok_or_else(|| ReflectionError::InvalidReflectionPath {
                            path: self.to_string(),
                            msg: "No such element".to_owned(),
                        })
                }
                ReflectMut::Tuple(s) => {
                    s.field_mut(*index)
                        .ok_or_else(|| ReflectionError::InvalidReflectionPath {
                            path: self.to_string(),
                            msg: "No such element".to_owned(),
                        })
                }
                ReflectMut::List(s) => {
                    s.get_mut(*index)
                        .ok_or_else(|| ReflectionError::InvalidReflectionPath {
                            path: self.to_string(),
                            msg: "No such element".to_owned(),
                        })
                }
                ReflectMut::Array(s) => {
                    s.get_mut(*index)
                        .ok_or_else(|| ReflectionError::InvalidReflectionPath {
                            path: self.to_string(),
                            msg: "No such element".to_owned(),
                        })
                }
                _ => Err(ReflectionError::InvalidReflectionPath {
                    path: self.to_string(),
                    msg: "No such element".to_owned(),
                }),
            },
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ReflectPath {
    base: ReflectBase,
    // most of these will be very short, people don't make many nested hashmaps vecs etc.
    accesses: Vec<ReflectPathElem>,
}

impl Display for ReflectPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.base.to_string())?;
        for access in &self.accesses {
            f.write_str(&access.to_string())?
        }
        Ok(())
    }
}

impl ReflectPath {
    pub fn new(base: ReflectBase) -> Self {
        Self {
            base,
            accesses: Vec::default(),
        }
    }

    /// pushes another sub reflect level access to the end of this access.
    ///
    /// The most recent sub access added will be executed last.
    pub fn push(&mut self, elem: ReflectPathElem) {
        self.accesses.push(elem);
    }

    /// Creates a new composite sub reflect
    pub fn new_sub(&self, elem: ReflectPathElem) -> Self {
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

    pub fn len(&self) -> u8 {
        self.accesses.len() as u8
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
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
                        .ok_or_else(|| ReflectionError::InvalidBaseReference {
                            base: self.base.to_string(),
                            reason: "This entity does not exist".to_owned(),
                        })?;

                let ref_ = self.walk_path(comp.reflect(entity_ref).ok_or_else(|| {
                    ReflectionError::InvalidBaseReference {
                        base: self.base.to_string(),
                        reason: "Given component does not exist on this entity".to_owned(),
                    }
                })?)?;
                // unsafe since pointer may be dangling
                let o = f(ref_);
                drop(g);
                Ok(o)
            }
            ReflectBase::Resource { res } => {
                let g = world_ptr.read();

                let ref_ = self.walk_path(res.reflect(&g).ok_or_else(|| {
                    ReflectionError::InvalidBaseReference {
                        base: self.base.to_string(),
                        reason: "Given resource does not exist in this world".to_owned(),
                    }
                })?)?;
                // unsafe since pointer may be dangling
                let o = f(ref_);
                drop(g);
                Ok(o)
            }
            ReflectBase::ScriptOwned { val } => {
                let g = val
                    .upgrade()
                    .expect("Trying to access cached value from previous frame");

                let g = g.try_read().expect("Rust safety violation: attempted to borrow value {self:?} while it was already mutably borrowed");
                // Safety: we know the pointer points to a valid value and is live, no other value has mutable access to it either
                let ref_ = self.walk_path(&*g)?;
                let o = f(ref_);
                drop(g);
                Ok(o)
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
                // unsafe since pointer may be dangling
                let o = f(ref_);
                drop(g);
                Ok(o)
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
                // unsafe since pointer may be dangling
                let o = f(ref_);
                drop(g);
                Ok(o)
            }
            ReflectBase::ScriptOwned { val } => {
                let g = val
                    .upgrade()
                    .expect("Trying to access cached value from previous frame");

                let mut g = g.try_write().expect("Rust safety violation: attempted to borrow value {self:?} while it was already mutably borrowed");
                // Safety: we know the pointer points to a valid value and is live, no other value has access to it either
                let ref_ = self.walk_path_mut(&mut *g)?;
                let o = f(ref_);
                drop(g);
                Ok(o)
            }
        }
    }
}
