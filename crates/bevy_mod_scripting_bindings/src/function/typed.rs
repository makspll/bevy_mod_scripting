//! Values whose type in scripts is more precise than how they are converted: functions with a signature, enums passed by
//! name, and the conversions of `#[derive(ScriptData)]`.
//!
//! A type is either reflected, which scripts hold references to, or plain data deriving `ScriptData`, never both:
//!
//! ```
//! # use bevy_mod_scripting_derive::ScriptData;
//! #[derive(ScriptData)]
//! #[script_data(bms_bindings_path = "bevy_mod_scripting_bindings")]
//! struct Options {
//!     name: String,
//! }
//! ```
//!
//! ```compile_fail,E0119
//! # use bevy_mod_scripting_derive::ScriptData;
//! #[derive(bevy_reflect::Reflect, ScriptData)]
//! #[script_data(bms_bindings_path = "bevy_mod_scripting_bindings")]
//! struct Options {
//!     name: String,
//! }
//! ```

use std::{any::TypeId, marker::PhantomData, ops::Deref};

use bevy_mod_scripting_world::{CallScope, WorldGuard};
use bevy_reflect::{
    FromReflect, GetTypeRegistration, TypeInfo, TypeRegistry, Typed,
    enums::{DynamicEnum, DynamicVariant, Enum},
};

use crate::{
    ScriptValue,
    docgen::{
        info::GetFunctionInfo,
        typed_through::{ThroughTypeInfo, TypedThrough},
    },
    error::InteropError,
    function::{
        arg_meta::ArgMeta,
        from::FromScript,
        from::V,
        into::IntoScript,
        namespace::FunctionScope,
        script_function::DynamicScriptFunction,
        type_dependencies::{GetFunctionTypeDependencies, GetTypeDependencies},
    },
};

/// A function, documented with the types of its arguments and of what it returns, i.e. `FunctionOf<fn(V<Entity>) -> bool>`
/// is `fun(arg1: Entity): boolean` in Lua. It converts like a [`DynamicScriptFunction`]:
///
/// ```rust,ignore
/// fn on_hit(callback: FunctionOf<fn(V<Entity>, f32) -> ()>) { .. }
/// ```
///
/// The signature only documents the function, calls aren't checked against it.
pub struct FunctionOf<F>(pub DynamicScriptFunction, PhantomData<fn() -> F>);

impl<F> FunctionOf<F> {
    /// Wrap a function.
    pub fn new(function: DynamicScriptFunction) -> Self {
        FunctionOf(function, PhantomData)
    }

    /// Unwrap the function.
    pub fn into_inner(self) -> DynamicScriptFunction {
        self.0
    }
}

impl<F> Clone for FunctionOf<F> {
    fn clone(&self) -> Self {
        FunctionOf::new(self.0.clone())
    }
}

impl<F> Deref for FunctionOf<F> {
    type Target = DynamicScriptFunction;

    fn deref(&self) -> &DynamicScriptFunction {
        &self.0
    }
}

impl<F> FromScript for FunctionOf<F> {
    type This<'w> = Self;

    fn from_script(value: ScriptValue, scope: &CallScope) -> Result<Self, InteropError> {
        DynamicScriptFunction::from_script(value, scope).map(FunctionOf::new)
    }

    fn could_convert(value: &ScriptValue) -> bool {
        DynamicScriptFunction::could_convert(value)
    }
}

impl<F> IntoScript for FunctionOf<F> {
    fn into_script(self, world: WorldGuard) -> Result<ScriptValue, InteropError> {
        self.0.into_script(world)
    }
}

impl<F> ArgMeta for FunctionOf<F> {}

// a signature `fn(A) -> R` is itself a function, described by the machinery of registered functions
impl<F: GetFunctionInfo<F>> TypedThrough for FunctionOf<F> {
    fn through_type_info() -> ThroughTypeInfo {
        let info = F::function_info("".into(), FunctionScope::Builtin);
        let described = |type_info: Option<ThroughTypeInfo>| {
            type_info.unwrap_or(ThroughTypeInfo::Primitive(
                bevy_mod_scripting_bindings_domain::ReflectionPrimitiveKind::ScriptValue,
            ))
        };
        ThroughTypeInfo::Function {
            params: info
                .arg_info
                .into_iter()
                .map(|arg| described(arg.type_info))
                .collect(),
            returns: Box::new(described(info.return_info.type_info)),
        }
    }
}

impl<F: GetFunctionTypeDependencies<F>> GetTypeDependencies for FunctionOf<F> {
    type Underlying = DynamicScriptFunction;

    fn register_type_dependencies(registry: &mut TypeRegistry) {
        F::register_type_dependencies(registry)
    }
}

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

/// The fields of a data struct, taken from the map scripts pass one by one, see `#[derive(ScriptData)]`.
///
/// A field converts like an argument of a function: [`FromScript`] converts it, and a missing field is its
/// [`ArgMeta::default_value`], so `Option<T>` fields can be left out and others can't.
pub struct StructFields {
    of: &'static str,
    fields: bevy_platform::collections::HashMap<String, ScriptValue>,
}

impl StructFields {
    /// The fields in the given map, of the struct named `of`. An empty list is an empty map, as Lua can't tell them apart.
    pub fn new(of: &'static str, value: ScriptValue) -> Result<Self, InteropError> {
        let fields = match value {
            ScriptValue::Map(fields) => fields,
            ScriptValue::List(list) if list.is_empty() => Default::default(),
            value => return Err(InteropError::value_mismatch(TypeId::of::<Self>(), value)),
        };
        Ok(StructFields { of, fields })
    }

    /// Whether the value could be the fields of a struct, see [`FromScript::could_convert`].
    pub fn could_convert(value: &ScriptValue) -> bool {
        match value {
            ScriptValue::Map(_) => true,
            ScriptValue::List(list) => list.is_empty(),
            _ => false,
        }
    }

    /// Converts the field with the given name, or its default if it is missing.
    pub fn take<'s, T: FromScript + ArgMeta>(
        &mut self,
        name: &str,
        scope: &'s CallScope<'_>,
    ) -> Result<T::This<'s>, InteropError> {
        let value = match self.fields.remove(name) {
            Some(value) => value,
            None => T::default_value().ok_or_else(|| {
                InteropError::string(format!("missing field `{name}` of `{}`", self.of))
            })?,
        };
        T::from_script(value, scope)
            .map_err(|e| InteropError::string(format!("field `{name}` of `{}`: {e}", self.of)))
    }

    /// Fails if fields other than the ones taken were given, naming the ones the struct has.
    pub fn finish(self, names: &[&str]) -> Result<(), InteropError> {
        match self.fields.keys().next() {
            Some(unknown) => Err(InteropError::string(format!(
                "unknown field `{unknown}` of `{}`, the fields are: {}",
                self.of,
                names.join(", ")
            ))),
            None => Ok(()),
        }
    }
}

/// The map of fields a data struct is returned as, see `#[derive(ScriptData)]`.
#[derive(Default)]
pub struct StructMap(Vec<(String, ScriptValue)>);

impl StructMap {
    /// Adds a field, unless it converts to `nil`, which is what a missing field is.
    pub fn push<T: IntoScript>(
        &mut self,
        name: &str,
        value: T,
        world: WorldGuard,
    ) -> Result<(), InteropError> {
        match value.into_script(world)? {
            ScriptValue::Unit => {}
            value => self.0.push((name.to_owned(), value)),
        }
        Ok(())
    }

    /// The map.
    pub fn into_value(self) -> ScriptValue {
        ScriptValue::Map(self.0.into_iter().collect())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{CurrentScriptAttachment, ScriptDataInfo, WorldExtensions};
    use bevy_ecs::world::World;
    use bevy_mod_scripting_bindings_domain::ReflectionPrimitiveKind;
    use bevy_mod_scripting_derive::ScriptData;
    use bevy_mod_scripting_world::WorldAccessGuard;
    use bevy_reflect::Reflect;

    #[derive(Reflect, Debug, PartialEq)]
    enum Direction {
        Row,
        ColumnReverse,
        Custom(f32),
    }

    #[derive(ScriptData)]
    #[script_data(bms_bindings_path = "crate")]
    enum Size {
        Named(ByName<Direction>),
        Pixels(f64),
        Text(String),
    }

    /// The options of a test
    #[derive(ScriptData)]
    #[script_data(bms_bindings_path = "crate", namespace = "test")]
    struct Options {
        name: String,
        size: Option<Size>,
        on_done: Option<FunctionOf<fn(f64) -> bool>>,
    }

    fn table(entries: &[(&str, ScriptValue)]) -> ScriptValue {
        ScriptValue::Map(
            entries
                .iter()
                .map(|(k, v)| (k.to_string(), v.clone()))
                .collect(),
        )
    }

    #[test]
    fn structs_unions_and_names_convert_and_describe_themselves() {
        let mut world = World::default();
        let cache = WorldAccessGuard::setup_cache(&world, CurrentScriptAttachment::empty());
        let guard = WorldAccessGuard::new_exclusive(&mut world, cache);
        let text = |s: &str| ScriptValue::String(s.to_owned().into());
        guard.with_call_scope(|scope| {
            let options =
                |entries: &[(&str, ScriptValue)]| Options::from_script(table(entries), scope);
            let size = |value| match options(&[("name", text("a")), ("size", value)]) {
                Ok(Options {
                    size: Some(size), ..
                }) => size,
                _ => panic!("no size"),
            };

            let converted = options(&[("name", text("a"))]).expect("converts");
            assert_eq!(converted.name, "a");
            assert!(converted.size.is_none() && converted.on_done.is_none());
            // the first type which converts it, names before text
            assert!(matches!(size(ScriptValue::Integer(5)), Size::Pixels(5.0)));
            assert!(matches!(
                size(text("Row")),
                Size::Named(ByName(Direction::Row))
            ));
            assert!(matches!(
                size(text("ColumnReverse")),
                Size::Named(ByName(Direction::ColumnReverse))
            ));
            // names are exact, anything else is text
            assert!(matches!(size(text("row")), Size::Text(t) if t == "row"));
            // variants with fields can't be named, the name is text then
            assert!(matches!(size(text("Custom")), Size::Text(t) if t == "Custom"));
            assert!(Size::from_script(ScriptValue::Unit, scope).is_err());

            let error = |entries: &[(&str, ScriptValue)]| {
                options(entries)
                    .err()
                    .map(|e| e.to_string())
                    .unwrap_or_default()
            };
            assert!(error(&[("size", ScriptValue::Integer(1))]).contains("missing field `name`"));
            let unknown = error(&[("name", text("a")), ("nmae", text("b"))]);
            assert!(
                unknown.contains("unknown field `nmae`") && unknown.contains("name, size, on_done"),
                "{unknown}"
            );
            assert!(
                error(&[("name", ScriptValue::Integer(1))]).contains("field `name` of `Options`")
            );
            assert!(
                ByName::<Direction>::from_script(text("rows"), scope)
                    .err()
                    .is_some_and(|e| e.to_string().contains("\"Row\", \"ColumnReverse\""))
            );

            // nil fields are left out
            let back = options(&[("name", text("a")), ("size", text("ColumnReverse"))])
                .and_then(|o| o.into_script(guard.clone()));
            let Ok(ScriptValue::Map(map)) = back else {
                panic!("not a table")
            };
            assert!(
                matches!(map.get("size"), Some(ScriptValue::String(s)) if s == "ColumnReverse")
            );
            assert!(map.contains_key("name") && !map.contains_key("on_done"));
        });

        let ThroughTypeInfo::Data(info) = Options::through_type_info() else {
            panic!("not a table")
        };
        let ScriptDataInfo {
            identifier,
            namespace,
            docs,
            fields,
            ..
        } = info;
        assert_eq!(
            (*identifier, *namespace, *docs),
            ("Options", Some("test"), Some("The options of a test"))
        );
        // optional by their type: `Option<T>` defaults to `nil`
        assert_eq!(
            fields
                .iter()
                .map(|f| (f.name, (f.default)().is_some()))
                .collect::<Vec<_>>(),
            [("name", false), ("size", true), ("on_done", true)]
        );
        let ThroughTypeInfo::Option(size) = (fields[1].type_info)() else {
            panic!()
        };
        let ThroughTypeInfo::Union(size) = *size else {
            panic!()
        };
        // one union, the names included
        assert!(matches!(&size[..], [
            ThroughTypeInfo::Literal(a),
            ThroughTypeInfo::Literal(b),
            ThroughTypeInfo::Primitive(ReflectionPrimitiveKind::F64),
            ThroughTypeInfo::Primitive(ReflectionPrimitiveKind::String),
        ] if a == "Row" && b == "ColumnReverse"));
        let ThroughTypeInfo::Option(on_done) = (fields[2].type_info)() else {
            panic!()
        };
        let ThroughTypeInfo::Function { params, returns } = *on_done else {
            panic!()
        };
        assert!(matches!(
            &params[..],
            [ThroughTypeInfo::Primitive(ReflectionPrimitiveKind::F64)]
        ));
        assert!(matches!(
            *returns,
            ThroughTypeInfo::Primitive(ReflectionPrimitiveKind::Bool)
        ));
    }
}
