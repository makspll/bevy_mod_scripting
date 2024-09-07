use std::marker::PhantomData;

use bevy::reflect::{FromReflect, GetTypeRegistration, TypePath};

use crate::{error::ReflectionError, ReflectReference, ValueIndex};

pub struct ScriptVec<T> {
    pub(crate) ref_: ReflectReference,
    _ph: PhantomData<T>,
}

impl<T> Clone for ScriptVec<T> {
    fn clone(&self) -> Self {
        Self {
            ref_: self.ref_.clone(),
            _ph: PhantomData,
        }
    }
}

impl<T> std::fmt::Debug for ScriptVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScriptVec")
            .field("ref_", &self.ref_)
            .finish()
    }
}

impl<T: std::fmt::Display + FromReflect + GetTypeRegistration + TypePath> std::fmt::Display
    for ScriptVec<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = self
            .ref_
            .get_typed(|s: &Vec<T>| s.iter().map(|v| format!("{v}")).collect::<Vec<_>>())
            .map_err(|_| std::fmt::Error)?
            .join(",");
        f.write_str("[")?;
        f.write_str(&str)?;
        f.write_str("]")
    }
}

impl<T: FromReflect + TypePath + GetTypeRegistration> ScriptVec<T> {
    pub fn new_ref(ref_: ReflectReference) -> Self {
        Self {
            ref_,
            _ph: PhantomData,
        }
    }

    pub fn is_empty(&self) -> Result<bool, ReflectionError> {
        Ok(self.len()? == 0)
    }

    pub fn len(&self) -> Result<usize, ReflectionError> {
        self.ref_.get_typed(|s: &Vec<T>| s.len())
    }

    pub fn push(&mut self, val: T) -> Result<(), ReflectionError> {
        self.ref_.get_mut_typed(|s: &mut Vec<T>| {
            s.push(val);
            Ok(())
        })?
    }

    pub fn pop(&mut self) -> Result<Option<T>, ReflectionError> {
        self.ref_.get_mut_typed(|s: &mut Vec<T>| s.pop())
    }

    pub fn clear(&mut self) -> Result<(), ReflectionError> {
        self.ref_.get_mut_typed(|s: &mut Vec<T>| {
            s.clear();
            Ok(())
        })?
    }

    pub fn insert(&mut self, idx: usize, val: T) -> Result<(), ReflectionError> {
        self.ref_.get_mut_typed(|s: &mut Vec<T>| {
            s.insert(idx, val);
            Ok(())
        })?
    }

    pub fn remove(&mut self, idx: usize) -> Result<T, ReflectionError> {
        self.ref_
            .get_mut_typed(|s: &mut Vec<T>| Ok(s.remove(idx)))?
    }
}

impl<T> ValueIndex<usize> for ScriptVec<T> {
    type Output = ReflectReference;

    fn index(&self, index: usize) -> Self::Output {
        self.ref_.index(index)
    }
}

impl<T> From<ScriptVec<T>> for ReflectReference {
    fn from(v: ScriptVec<T>) -> Self {
        v.ref_
    }
}

pub struct ScriptVecIterator<T> {
    current: usize,
    len: usize,
    base: ScriptVec<T>,
}

impl<T: FromReflect> Iterator for ScriptVecIterator<T> {
    type Item = ReflectReference;

    fn next(&mut self) -> Option<Self::Item> {
        let nxt = (self.current < self.len).then(|| self.base.index(self.current));
        self.current += 1;
        nxt
    }
}

impl<T: FromReflect + TypePath + GetTypeRegistration> IntoIterator for ScriptVec<T> {
    type Item = ReflectReference;

    type IntoIter = ScriptVecIterator<T>;

    /// Converts the vector into an iterator over references
    ///
    /// # Panics
    /// will panic if the base reference is invalid or mutably locked
    fn into_iter(self) -> Self::IntoIter {
        ScriptVecIterator {
            current: 0,
            // TODO?: end used to be an Option, and this check moved into the next method but
            // I am not sure if this will ever realistically fail, so if you do get this exception happening
            // hit me with an issue
            // if len > 0, subtract 1, otherwise set to 0
            len: self.len().expect("Failed to get length of ScriptVec"),
            base: self,
        }
    }
}
