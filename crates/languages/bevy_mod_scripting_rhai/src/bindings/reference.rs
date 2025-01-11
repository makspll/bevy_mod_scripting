use std::ops::{Deref, DerefMut};

use bevy_mod_scripting_core::bindings::{ReflectReference, ThreadWorldContainer, WorldContainer};
use rhai::{CustomType, Dynamic};

#[derive(Clone, Debug, PartialEq)]
pub struct RhaiReflectReference(pub ReflectReference);

impl AsRef<ReflectReference> for RhaiReflectReference {
    fn as_ref(&self) -> &ReflectReference {
        &self.0
    }
}

impl From<ReflectReference> for RhaiReflectReference {
    fn from(value: ReflectReference) -> Self {
        RhaiReflectReference(value)
    }
}

impl From<RhaiReflectReference> for ReflectReference {
    fn from(value: RhaiReflectReference) -> Self {
        value.0
    }
}

impl Deref for RhaiReflectReference {
    type Target = ReflectReference;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RhaiReflectReference {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl CustomType for RhaiReflectReference {
    fn build(mut builder: rhai::TypeBuilder<Self>) {
        builder
            .with_name(std::any::type_name::<ReflectReference>())
            .with_indexer_get(|_obj: &mut Self, _index: Dynamic| {
                let _world = ThreadWorldContainer.get_world();
            });
    }
}
