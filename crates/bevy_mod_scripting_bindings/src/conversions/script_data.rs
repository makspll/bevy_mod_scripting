//! The conversions of `#[derive(ScriptData)]`: plain data, copied to and from scripts, see [`StructFields`] and
//! [`StructMap`].
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

use std::any::TypeId;

use bevy_mod_scripting_world::{CallScope, WorldGuard};

use crate::{
    ScriptValue,
    conversions::{from::FromScript, into::IntoScript},
    error::InteropError,
    function::arg_meta::ArgMeta,
};

/// The fields of a data struct, from the map scripts pass, see `#[derive(ScriptData)]`.
///
/// The derive converts the fields given in one pass, so its cost is in the fields given, not in the fields the struct
/// has. A field converts like an argument of a function: [`FromScript`] converts it, and a missing field is its
/// [`ArgMeta::default_value`], so `Option<T>` fields can be left out and others can't.
pub struct StructFields;

impl StructFields {
    /// The fields given, by name. An empty list is an empty map, as Lua can't tell them apart.
    pub fn given(
        value: ScriptValue,
    ) -> Result<bevy_platform::collections::HashMap<String, ScriptValue>, InteropError> {
        match value {
            ScriptValue::Map(fields) => Ok(fields),
            ScriptValue::List(list) if list.is_empty() => Ok(Default::default()),
            value => Err(InteropError::value_mismatch(TypeId::of::<Self>(), value)),
        }
    }

    /// Whether the value could be the fields of a struct, see [`FromScript::could_convert`].
    pub fn could_convert(value: &ScriptValue) -> bool {
        match value {
            ScriptValue::Map(_) => true,
            ScriptValue::List(list) => list.is_empty(),
            _ => false,
        }
    }

    /// Converts a field given to the struct named `of`, see [`Self::given`].
    pub fn convert<'s, T: FromScript>(
        of: &str,
        name: &str,
        value: ScriptValue,
        scope: &'s CallScope<'_>,
    ) -> Result<T::This<'s>, InteropError> {
        T::from_script(value, scope)
            .map_err(|e| InteropError::string(format!("field `{name}` of `{of}`: {e}")))
    }

    /// The value of a field which wasn't given: its [`ArgMeta::default_value`], so `Option<T>` fields can be left out.
    pub fn missing<'s, T: FromScript + ArgMeta>(
        of: &str,
        name: &str,
        scope: &'s CallScope<'_>,
    ) -> Result<T::This<'s>, InteropError> {
        let value = T::default_value()
            .ok_or_else(|| InteropError::string(format!("missing field `{name}` of `{of}`")))?;
        Self::convert::<T>(of, name, value, scope)
    }

    /// The error of a field the struct named `of` doesn't have, naming the ones it has.
    pub fn unknown(of: &str, name: &str, names: &[&str]) -> InteropError {
        InteropError::string(format!(
            "unknown field `{name}` of `{of}`, the fields are: {}",
            names.join(", ")
        ))
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
    use crate::{
        ByName, CurrentScriptAttachment, FunctionOf, GetTypeDependencies, ScriptDataInfo,
        ScriptDataShape, ThroughTypeInfo, TypedThrough, WorldExtensions,
    };
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

    /// A size
    #[derive(ScriptData)]
    #[script_data(bms_bindings_path = "crate", namespace = "test")]
    enum Size {
        Named(ByName<Direction>),
        Pixels(f64),
        Text(String),
    }

    /// Holds itself
    #[derive(ScriptData)]
    #[script_data(bms_bindings_path = "crate")]
    enum Tree {
        Leaf(f64),
        Branch(Vec<Tree>),
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

            // a type holding itself converts, describes itself and registers its dependencies without recursing
            let tree = ScriptValue::List(
                [ScriptValue::Float(1.0), ScriptValue::List([ScriptValue::Float(2.0)].into())].into(),
            );
            let Ok(Tree::Branch(children)) = Tree::from_script(tree, scope) else {
                panic!("not a branch")
            };
            assert!(matches!(&children[..], [Tree::Leaf(1.0), Tree::Branch(inner)] if matches!(&inner[..], [Tree::Leaf(2.0)])));
        });
        let ThroughTypeInfo::Data(ScriptDataInfo {
            shape: ScriptDataShape::Union(variants),
            ..
        }) = Tree::through_type_info()
        else {
            panic!("not a named union")
        };
        let ThroughTypeInfo::Vec(branch) = variants[1]() else {
            panic!("not a list")
        };
        assert!(matches!(*branch, ThroughTypeInfo::Data(info) if info.identifier == "Tree"));
        Tree::register_type_dependencies(&mut crate::__derive::TypeRegistry::default());

        let ThroughTypeInfo::Data(info) = Options::through_type_info() else {
            panic!("not a table")
        };
        let ScriptDataInfo {
            identifier,
            namespace,
            docs,
            shape: ScriptDataShape::Fields(fields),
            ..
        } = info
        else {
            panic!("not a struct")
        };
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
        // an enum is a named data type, a union of its variants' types
        let ThroughTypeInfo::Option(size) = (fields[1].type_info)() else {
            panic!()
        };
        let ThroughTypeInfo::Data(ScriptDataInfo {
            identifier: "Size",
            namespace: Some("test"),
            docs: Some("A size"),
            shape: ScriptDataShape::Union(variants),
            ..
        }) = *size
        else {
            panic!("not a named union: {size:?}")
        };
        let variants = variants.iter().map(|variant| variant()).collect::<Vec<_>>();
        assert!(matches!(&variants[..], [
            ThroughTypeInfo::Union(names),
            ThroughTypeInfo::Primitive(ReflectionPrimitiveKind::F64),
            ThroughTypeInfo::Primitive(ReflectionPrimitiveKind::String),
        ] if matches!(&names[..], [ThroughTypeInfo::Literal(a), ThroughTypeInfo::Literal(b)] if a == "Row" && b == "ColumnReverse")));
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
