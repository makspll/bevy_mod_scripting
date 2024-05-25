use std::{ops::Deref, sync::Arc};

use bevy::reflect::TypeRegistration;

/// A wrapper around a `TypeRegistration` that provides additional information about the type.
///
/// This is used as a hook to a rust type from a scripting language. We should be able to easily convert between a type name and a [`ScriptTypeRegistration`].
#[derive(Clone)]
pub struct ScriptTypeRegistration(pub(crate) Arc<TypeRegistration>);

impl ScriptTypeRegistration {
    pub fn new(arc: Arc<TypeRegistration>) -> Self {
        Self(arc)
    }

    #[inline(always)]
    pub fn short_name(&self) -> &str {
        self.0.type_info().type_path_table().short_path()
    }

    #[inline(always)]
    pub fn type_name(&self) -> &'static str {
        self.0.type_info().type_path_table().path()
    }
}

impl std::fmt::Debug for ScriptTypeRegistration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ScriptTypeRegistration")
            .field(&self.0.type_info().type_path())
            .finish()
    }
}

impl std::fmt::Display for ScriptTypeRegistration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.type_info().type_path())
    }
}

impl Deref for ScriptTypeRegistration {
    type Target = Arc<TypeRegistration>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
