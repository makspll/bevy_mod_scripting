use super::script_value::FromDynamic;
use bevy_mod_scripting_core::bindings::{
    function::script_function::DynamicScriptFunction, script_value::ScriptValue, ReflectReference,
    ThreadWorldContainer, WorldContainer, WorldGuard,
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

#[allow(dead_code)]
fn lookup_dynamic_function(
    _world: WorldGuard,
    _name: &str,
    _on: TypeId,
) -> Option<DynamicScriptFunction> {
    // let registry = world.with_resource(|registry: &AppScriptFunctionRegistry| registry.clone());
    // let registry = registry.read();

    // registry
    //     .get_namespaced_function(name.to_string(), Namespace::OnType(on))
    //     .cloned()
    todo!()
}

impl CustomType for RhaiReflectReference {
    fn build(mut builder: rhai::TypeBuilder<Self>) {
        builder
            .with_name(std::any::type_name::<ReflectReference>())
            .with_indexer_get(|_obj: &mut Self, _index: Dynamic| {
                let _world = ThreadWorldContainer.get_world();
                let key: ScriptValue = ScriptValue::from_dynamic(_index)?;
                if let ScriptValue::String(_key) = key {
                    // lookup function
                    todo!()
                }
                Ok::<_, Box<EvalAltResult>>(())
            });
    }
}
