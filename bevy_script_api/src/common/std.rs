use std::{
    marker::PhantomData,
    ops::{Index},
};

use bevy::reflect::FromReflect;

use crate::{error::ReflectionError, ScriptRef, ValueIndex};

pub struct ScriptVec<T> {
    pub(crate) ref_: ScriptRef,
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

impl<T: std::fmt::Display + FromReflect> std::fmt::Display for ScriptVec<T> {
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

impl<T: FromReflect> ScriptVec<T> {
    pub fn new_ref(ref_: ScriptRef) -> Self {
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
    type Output = ScriptRef;

    fn index(&self, index: usize) -> Self::Output {
        self.ref_.index(index)
    }
}

impl<T> From<ScriptVec<T>> for ScriptRef {
    fn from(v: ScriptVec<T>) -> Self {
        v.ref_
    }
}

pub struct ScriptVecIterator<T> {
    current: usize,
    end: usize,
    base: ScriptVec<T>,
}

impl<T: FromReflect> Iterator for ScriptVecIterator<T> {
    type Item = ScriptRef;

    fn next(&mut self) -> Option<Self::Item> {
        let nxt = (self.current <= self.end).then(|| self.base.index(self.current));
        self.current += 1;
        nxt
    }
}

impl<T: FromReflect> IntoIterator for ScriptVec<T> {
    type Item = ScriptRef;

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
            end: self
                .len()
                .map(|v| v - 1)
                .expect("Could not access length when creating iterator"),
            base: self,
        }
    }
}
