use std::{
    marker::PhantomData,
    ops::{Index, Range},
};

use bevy::reflect::{FromReflect, Reflect};
use bevy_mod_scripting_core::prelude::Script;
use bevy_mod_scripting_rhai::rhai::EvalAltResult;

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
    /// why is this an option? well we lazily initialize this field
    /// we don't want to store the length in the ScriptVec itself
    /// but if the element is about to be accessed it makes sense to throw an error there
    end: usize,
    base: ScriptVec<T>,
}

impl<T: FromReflect> Iterator for ScriptVecIterator<T> {
    type Item = ScriptRef;

    fn next(&mut self) -> Option<Self::Item> {
        (self.current <= self.end).then(|| self.base.index(self.current))
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
            end: self
                .len()
                .expect("Could not access length when creating iterator"),
            base: self,
        }
    }
}
