use std::ops::{Deref, DerefMut};

use bevy_mod_scripting_core::{prelude::*, world::WorldPointer};
use bevy_mod_scripting_rhai::{rhai as rhai, prelude::*};
use rhai::plugin::*;

#[derive(Clone)]
pub struct RhaiWorld(WorldPointer);

impl Deref for RhaiWorld {
    type Target=WorldPointer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RhaiWorld {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


impl AsRef<WorldPointer> for RhaiWorld {
    fn as_ref(&self) -> &WorldPointer {
        &self.0
    }
}

impl RhaiWorld {
    pub fn new(w : WorldPointer) -> Self {
        Self(w)
    }
}

#[rhai::export_module]
pub mod bevy_plugin{
    pub type World = RhaiWorld;
}


pub struct RhaiBevyAPIProvider;

impl APIProvider for RhaiBevyAPIProvider {
    type APITarget=Engine;
    type ScriptContext=RhaiContext;
    type DocTarget=RhaiDocFragment;

    fn attach_api(&mut self, engine: &mut Self::APITarget) -> Result<(), ScriptError> {
        engine.register_global_module(exported_module!(bevy_plugin).into());
        Ok(())
    }
}