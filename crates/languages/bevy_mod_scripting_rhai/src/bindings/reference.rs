use super::script_value::{FromDynamic, IntoDynamic, RHAI_CALLER_CONTEXT};
use bevy_mod_scripting_core::{
    bindings::{
        pretty_print::DisplayWithWorld, script_value::ScriptValue, ReflectReference,
        ThreadWorldContainer, WorldContainer,
    },
    error::InteropError,
    reflection_extensions::TypeIdExtensions,
};
use rhai::{CustomType, Dynamic, EvalAltResult};
use std::{
    any::TypeId,
    ops::{Deref, DerefMut},
};

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
            .with_indexer_get(|self_: &mut Self, _index: Dynamic| {
                let world = ThreadWorldContainer.try_get_world()?;
                let self_ = &self_.0;
                let type_id = self_.tail_type_id(world.clone())?.or_fake_id();

                let key: ScriptValue = ScriptValue::from_dynamic(_index)?;
                let key = match key.as_string() {
                    Ok(string) => {
                        match world
                            .lookup_function([type_id, TypeId::of::<ReflectReference>()], string)
                        {
                            Ok(func) => return ScriptValue::Function(func).into_dynamic(),
                            Err(string) => ScriptValue::String(string),
                        }
                    }
                    Err(key) => key,
                };

                let func = world
                    .lookup_function([type_id, TypeId::of::<ReflectReference>()], "get")
                    .map_err(|_| InteropError::missing_function(type_id, "get".to_owned()))?;

                let out = func.call(
                    vec![ScriptValue::Reference(self_.clone()), key],
                    RHAI_CALLER_CONTEXT,
                )?;

                Ok::<_, Box<EvalAltResult>>(Dynamic::from(out))
            });
    }
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct RhaiStaticReflectReference(pub TypeId);

impl CustomType for RhaiStaticReflectReference {
    fn build(mut builder: rhai::TypeBuilder<Self>) {
        builder
            .with_name(std::any::type_name::<RhaiStaticReflectReference>())
            .with_indexer_get(|self_: &mut Self, index: Dynamic| {
                let world = ThreadWorldContainer.try_get_world()?;
                let type_id = self_.0;
                let key: ScriptValue = ScriptValue::from_dynamic(index)?;

                let key = match key.as_string() {
                    Ok(name) => match world.lookup_function([type_id], name) {
                        Ok(func) => return ScriptValue::Function(func).into_dynamic(),
                        Err(key) => ScriptValue::String(key),
                    },
                    Err(key) => key,
                };

                Err::<_, Box<EvalAltResult>>(
                    InteropError::missing_function(type_id, key.display_with_world(world.clone()))
                        .into(),
                )
            });
    }
}
