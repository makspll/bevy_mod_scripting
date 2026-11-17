//! Values whose type in scripts is more precise than how they are converted: functions with a signature, and enums passed by name.

use std::{any::TypeId, marker::PhantomData, ops::Deref};

use bevy_mod_scripting_world::{CallScope, WorldGuard};
use bevy_reflect::{
    FromReflect, GetTypeRegistration, TypeInfo, TypeRegistry, Typed,
    enums::{DynamicEnum, DynamicVariant, Enum},
};

use crate::{
    ScriptValue,
    docgen::typed_through::{ThroughTypeInfo, TypedThrough},
    error::InteropError,
    function::{
        arg_meta::ArgMeta, from::FromScript, from::V, into::IntoScript,
        script_function::DynamicScriptFunction, type_dependencies::GetTypeDependencies,
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

macro_rules! impl_function_of {
    ($($param:ident),*) => {
        impl<$($param: TypedThrough,)* Ret: TypedThrough> TypedThrough for FunctionOf<fn($($param),*) -> Ret> {
            fn through_type_info() -> ThroughTypeInfo {
                ThroughTypeInfo::Function {
                    params: vec![$($param::through_type_info()),*],
                    returns: Box::new(Ret::through_type_info()),
                }
            }
        }

        impl<$($param: GetTypeDependencies,)* Ret: GetTypeDependencies> GetTypeDependencies
            for FunctionOf<fn($($param),*) -> Ret>
        {
            type Underlying = DynamicScriptFunction;

            fn register_type_dependencies(_registry: &mut TypeRegistry) {
                $($param::register_type_dependencies(_registry);)*
                Ret::register_type_dependencies(_registry);
            }
        }
    };
}

variadics_please::all_tuples!(impl_function_of, 0, 13, P);

/// A unit variant of the reflected enum `E`, passed by its name in snake case, i.e. `"flex_start"` for `JustifyContent::FlexStart`.
///
/// It is documented as the names, i.e. `"row" | "column"` in Lua, and converts from a name or from a reference to a value of `E`.
/// Names are the variant names with an `_` before each upper case letter but the first, all in lower case.
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

/// The name a variant is passed by, see [`ByName`].
pub fn snake_case(name: &str) -> String {
    let mut out = String::with_capacity(name.len() + 4);
    for (i, c) in name.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            out.push('_');
        }
        out.extend(c.to_lowercase());
    }
    out
}

/// Whether `snake` is the [`snake_case`] of `name`, without allocating.
fn is_snake_case_of(name: &str, snake: &str) -> bool {
    let mut snake = snake.chars();
    for (i, c) in name.chars().enumerate() {
        if c.is_uppercase() && i > 0 && snake.next() != Some('_') {
            return false;
        }
        for lower in c.to_lowercase() {
            if snake.next() != Some(lower) {
                return false;
            }
        }
    }
    snake.next().is_none()
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
        let variant =
            unit_variants(E::type_info()).find(|variant| is_snake_case_of(variant, &name));
        let Some(variant) = variant else {
            let names = unit_variants(E::type_info())
                .map(|variant| format!("\"{}\"", snake_case(variant)))
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
        Ok(ScriptValue::String(
            snake_case(self.0.variant_name()).into(),
        ))
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
            .map(|variant| ThroughTypeInfo::Literal(snake_case(variant)))
            .collect::<Vec<_>>();
        match names.len() {
            1 => names.remove(0),
            _ => ThroughTypeInfo::Union(names),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{CurrentScriptAttachment, ScriptStructInfo, WorldExtensions};
    use bevy_ecs::world::World;
    use bevy_mod_scripting_bindings_domain::ReflectionPrimitiveKind;
    use bevy_mod_scripting_derive::{ScriptStruct, ScriptUnion};
    use bevy_mod_scripting_world::WorldAccessGuard;
    use bevy_reflect::Reflect;

    #[derive(Reflect, Debug, PartialEq)]
    enum Direction {
        Row,
        ColumnReverse,
        Custom(f32),
    }

    #[derive(ScriptUnion)]
    #[script_union(bms_bindings_path = "crate")]
    enum Size {
        Named(ByName<Direction>),
        Pixels(f64),
        Text(String),
    }

    /// The options of a test
    #[derive(ScriptStruct)]
    #[script_struct(bms_bindings_path = "crate", namespace = "test")]
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
                size(text("row")),
                Size::Named(ByName(Direction::Row))
            ));
            assert!(matches!(
                size(text("column_reverse")),
                Size::Named(ByName(Direction::ColumnReverse))
            ));
            // variants with fields can't be named, the name is text then
            assert!(matches!(size(text("custom")), Size::Text(t) if t == "custom"));
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
                    .is_some_and(|e| e.to_string().contains("\"row\", \"column_reverse\""))
            );

            // nil fields are left out
            let back = options(&[("name", text("a")), ("size", text("row"))])
                .and_then(|o| o.into_script(guard.clone()));
            let Ok(ScriptValue::Map(map)) = back else {
                panic!("not a table")
            };
            assert!(matches!(map.get("size"), Some(ScriptValue::String(s)) if s == "row"));
            assert!(map.contains_key("name") && !map.contains_key("on_done"));
        });

        let ThroughTypeInfo::Struct(info) = Options::through_type_info() else {
            panic!("not a table")
        };
        let ScriptStructInfo {
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
        assert_eq!(
            fields
                .iter()
                .map(|f| (f.name, f.optional))
                .collect::<Vec<_>>(),
            [("name", false), ("size", true), ("on_done", true)]
        );
        let ThroughTypeInfo::Option(size) = (fields[1].type_info)() else {
            panic!()
        };
        let ThroughTypeInfo::Union(size) = *size else {
            panic!()
        };
        assert!(matches!(&size[..], [
            ThroughTypeInfo::Union(names),
            ThroughTypeInfo::Primitive(ReflectionPrimitiveKind::F64),
            ThroughTypeInfo::Primitive(ReflectionPrimitiveKind::String),
        ] if matches!(&names[..], [ThroughTypeInfo::Literal(a), ThroughTypeInfo::Literal(b)] if a == "row" && b == "column_reverse")));
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

    #[test]
    fn names_are_snake_case() {
        for (name, snake) in [
            ("Row", "row"),
            ("FlexStart", "flex_start"),
            ("SpaceBetween", "space_between"),
            ("Px2", "px2"),
        ] {
            assert_eq!(snake_case(name), snake);
            assert!(is_snake_case_of(name, snake), "{name} {snake}");
        }
        assert!(!is_snake_case_of("FlexStart", "flexstart"));
        assert!(!is_snake_case_of("FlexStart", "flex_start_"));
        assert!(!is_snake_case_of("Row", "ro"));
    }
}
