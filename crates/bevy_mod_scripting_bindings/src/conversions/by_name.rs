//! Reflected enums passed by the names of their variants, see [`ByName`].

use std::{any::TypeId, ops::Deref};

use bevy_mod_scripting_world::{CallScope, WorldGuard};
use bevy_reflect::{
    FromReflect, GetTypeRegistration, TypeInfo, TypeRegistry, Typed,
    enums::{DynamicEnum, DynamicVariant, Enum},
};

use crate::{
    ScriptValue,
    conversions::{
        from::{FromScript, V},
        into::IntoScript,
    },
    docgen::typed_through::{ThroughTypeInfo, TypedThrough},
    error::InteropError,
    function::{arg_meta::ArgMeta, type_dependencies::GetTypeDependencies},
};

/// A unit variant of the reflected enum `E`, passed by its name as in Rust, i.e. `"FlexStart"` for `JustifyContent::FlexStart`.
///
/// It is documented as the names, i.e. `"Row" | "Column"` in Lua, and converts from a name or from a reference to a value of `E`.
/// Variants with fields can't be named.
pub struct ByName<E>(pub E);

impl<E> ByName<E> {
    /// Unwrap the value.
    pub fn into_inner(self) -> E {
        self.0
    }
}

impl<E> Deref for ByName<E> {
    type Target = E;

    fn deref(&self) -> &E {
        &self.0
    }
}

/// The variants of `E` which can be named, see [`ByName`].
fn unit_variants(type_info: &'static TypeInfo) -> impl Iterator<Item = &'static str> {
    type_info.as_enum().into_iter().flat_map(|info| {
        info.iter()
            .filter(|variant| variant.as_unit_variant().is_ok())
            .map(|variant| variant.name())
    })
}

impl<E: Enum + FromReflect + Typed> FromScript for ByName<E> {
    type This<'w> = Self;

    fn from_script(value: ScriptValue, scope: &CallScope) -> Result<Self, InteropError> {
        let name = match value {
            ScriptValue::String(name) => name,
            ScriptValue::Reference(_) => {
                return V::<E>::from_script(value, scope).map(|v| ByName(v.into_inner()));
            }
            value => return Err(InteropError::value_mismatch(TypeId::of::<E>(), value)),
        };
        let variant = unit_variants(E::type_info()).find(|variant| *variant == name);
        let Some(variant) = variant else {
            let names = unit_variants(E::type_info())
                .map(|variant| format!("\"{variant}\""))
                .collect::<Vec<_>>()
                .join(", ");
            return Err(InteropError::string(format!(
                "`{name}` is not a name of `{}`, use one of: {names}",
                E::type_path()
            )));
        };
        E::from_reflect(&DynamicEnum::new(variant, DynamicVariant::Unit))
            .map(ByName)
            .ok_or_else(|| {
                InteropError::string(format!(
                    "the `{variant}` variant of `{}` could not be built",
                    E::type_path()
                ))
            })
    }

    fn could_convert(value: &ScriptValue) -> bool {
        matches!(value, ScriptValue::String(_) | ScriptValue::Reference(_))
    }
}

impl<E: Enum> IntoScript for ByName<E> {
    fn into_script(self, _world: WorldGuard) -> Result<ScriptValue, InteropError> {
        Ok(ScriptValue::String(self.0.variant_name().to_owned().into()))
    }
}

impl<E> ArgMeta for ByName<E> {}

impl<E: GetTypeRegistration> GetTypeDependencies for ByName<E> {
    type Underlying = E;

    fn register_type_dependencies(registry: &mut TypeRegistry) {
        registry.register::<E>();
    }
}

impl<E: Typed> TypedThrough for ByName<E> {
    fn through_type_info() -> ThroughTypeInfo {
        let mut names = unit_variants(E::type_info())
            .map(|variant| ThroughTypeInfo::Literal(variant.to_owned()))
            .collect::<Vec<_>>();
        match names.len() {
            1 => names.remove(0),
            _ => ThroughTypeInfo::Union(names),
        }
    }
}
