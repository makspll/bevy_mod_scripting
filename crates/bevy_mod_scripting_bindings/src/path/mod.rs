//! BMS native reflection paths

use std::{borrow::Cow, fmt::Display};

use bevy_mod_scripting_asset::Language;
use bevy_mod_scripting_derive::DebugWithTypeInfo;
use bevy_mod_scripting_display::DisplayWithTypeInfo;
use bevy_mod_scripting_world::WorldGuard;
use bevy_reflect::{
    PartialReflect, ReflectMut, ReflectRef, TypeInfo, TypeRegistry, enums::VariantType,
};

use crate::{ScriptValue, convert};

/// A key referencing into a `Reflect` supporting trait object.
#[derive(DebugWithTypeInfo)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
pub enum ReferencePart {
    /// A string labelled reference
    StringAccess(Cow<'static, str>),
    /// An integer labelled reference, always 0-based, see [`ReferencePart::new_from_script_val`]
    IntegerAccess(i64),
    /// A key to a map or set,
    MapAccess(Box<dyn PartialReflect>),
}

impl Clone for ReferencePart {
    #[allow(
        clippy::expect_used,
        reason = "`Clone` is infallible, and map keys which can neither be cloned nor converted to a dynamic value cannot be used for access anyway"
    )]
    fn clone(&self) -> Self {
        match self {
            Self::StringAccess(arg0) => Self::StringAccess(arg0.clone()),
            Self::IntegerAccess(arg0) => Self::IntegerAccess(*arg0),
            Self::MapAccess(arg0) => Self::MapAccess(match arg0.reflect_clone() {
                Ok(c) => c,
                Err(_) => arg0.to_dynamic().expect("map key must be cloneable"), // this is okay, because we need to call FromReflect on the map side anyway
            }),
        }
    }
}

impl PartialEq for ReferencePart {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::StringAccess(l0), Self::StringAccess(r0)) => l0 == r0,
            (Self::IntegerAccess(l0), Self::IntegerAccess(r0)) => l0 == r0,
            (Self::MapAccess(l0), Self::MapAccess(r0)) => {
                l0.reflect_partial_eq(r0.as_ref()).unwrap_or(false)
            }
            _ => false,
        }
    }
}

#[allow(clippy::result_unit_err, reason = "it works better")]
impl ReferencePart {
    /// Expect this reference to be a string, and return it if it is
    pub fn expect_string(&self) -> Result<&str, ()> {
        match self {
            ReferencePart::StringAccess(cow) => Ok(cow.as_ref()),
            _ => Err(()),
        }
    }

    /// Expect this reference to be an integer, and return it if it is
    pub fn expect_integer(&self) -> Result<i64, ()> {
        match self {
            ReferencePart::IntegerAccess(index) => Ok(*index),
            _ => Err(()),
        }
    }

    /// Casts the keys into a partial reflect value regardless of type of access
    pub fn with_any<O, F: FnOnce(&dyn PartialReflect) -> O>(&self, f: F) -> O {
        match self {
            ReferencePart::StringAccess(cow) => f(cow),
            ReferencePart::IntegerAccess(index) => f(index),
            ReferencePart::MapAccess(partial_reflect) => f(partial_reflect.as_ref()),
        }
    }

    /// Converts a [`ScriptValue`] to a reference part, integers are converted to 0-based indices for one-indexed languages.
    ///
    /// If given a world guard also supports arbitrary references as keys
    pub fn new_from_script_val(
        value: ScriptValue,
        language: Language,
        world: Option<WorldGuard>,
    ) -> Result<Self, ScriptValue> {
        let correction = language.one_indexed() as i64;
        Ok(match value {
            ScriptValue::Integer(v) => Self::IntegerAccess(v.saturating_sub(correction)),
            ScriptValue::Float(v) => Self::IntegerAccess(
                (v.max(i64::MIN as f64).min(i64::MAX as f64).round() as i64)
                    .saturating_sub(correction),
            ),
            ScriptValue::String(cow) => Self::StringAccess(cow),
            ScriptValue::Reference(_ref) => world
                .and_then(|world| Some(Self::MapAccess(_ref.to_owned_value(world).ok()?)))
                .ok_or_else(|| ScriptValue::Reference(_ref))?,
            _ => return Err(value),
        })
    }

    /// Tries to reference into the given root object with the current reference part
    pub fn reflect_element<'a>(
        &self,
        elem: &'a dyn PartialReflect,
        _type_registry: &TypeRegistry,
    ) -> Result<Option<&'a dyn PartialReflect>, ()> {
        Ok(match elem.reflect_ref() {
            ReflectRef::Struct(x) => x.field(self.expect_string()?),
            ReflectRef::TupleStruct(x) => x.field(self.expect_integer()? as usize),
            ReflectRef::Tuple(x) => x.field(self.expect_integer()? as usize),
            ReflectRef::List(x) => x.get(self.expect_integer()? as usize),
            ReflectRef::Array(x) => x.get(self.expect_integer()? as usize),
            ReflectRef::Enum(x) => match x.variant_type() {
                VariantType::Struct => x.field(self.expect_string()?),
                VariantType::Tuple => x.field_at(self.expect_integer()? as usize),
                VariantType::Unit => return Err(()),
            },
            ReflectRef::Map(x) => {
                let id = x.get_represented_map_info().ok_or(())?.key_ty().id();
                self.with_any(|key| {
                    let coerced = convert(key, id).ok_or(())?;
                    Ok(x.get(coerced.as_ref()))
                })?
            }
            ReflectRef::Set(x) => {
                let id = match x.get_represented_type_info().ok_or(())? {
                    TypeInfo::Set(set_info) => set_info.value_ty().id(),
                    _ => unreachable!("impossible"),
                };
                self.with_any(|key| {
                    let coerced = convert(key, id).ok_or(())?;
                    Ok(x.get(coerced.as_ref()))
                })?
            }
            _ => return Err(()),
        })
    }

    /// Tries to reference into the given root object with the current reference part
    pub fn reflect_element_mut<'a>(
        &self,
        elem: &'a mut dyn PartialReflect,
        _type_registry: &TypeRegistry,
    ) -> Result<Option<&'a mut dyn PartialReflect>, ()> {
        Ok(match elem.reflect_mut() {
            ReflectMut::Struct(x) => x.field_mut(self.expect_string()?),
            ReflectMut::TupleStruct(x) => x.field_mut(self.expect_integer()? as usize),
            ReflectMut::Tuple(x) => x.field_mut(self.expect_integer()? as usize),
            ReflectMut::List(x) => x.get_mut(self.expect_integer()? as usize),
            ReflectMut::Array(x) => x.get_mut(self.expect_integer()? as usize),
            ReflectMut::Enum(x) => match x.variant_type() {
                VariantType::Struct => x.field_mut(self.expect_string()?),
                VariantType::Tuple => x.field_at_mut(self.expect_integer()? as usize),
                VariantType::Unit => return Err(()),
            },
            ReflectMut::Map(x) => {
                let id = x.get_represented_map_info().ok_or(())?.key_ty().id();
                self.with_any(|key| {
                    let coerced = convert(key, id).ok_or(())?;
                    Ok(x.get_mut(coerced.as_ref()))
                })?
            }
            // ReflectMut::Set(x) => {} // no get_mut is available
            _ => return Err(()),
        })
    }
}

/// A collection of references into a `Reflect` supporting trait object in series.
#[derive(DebugWithTypeInfo, Clone, PartialEq, Default)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
pub struct ReferencePath {
    path: Vec<ReferencePart>,
}

impl Display for ReferencePart {
    /// Displays the part as it appears in a path: `.field`, `[index]` (0-based) or `[key]`
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReferencePart::StringAccess(cow) => write!(f, ".{cow}"),
            ReferencePart::IntegerAccess(int) => write!(f, "[{int}]"),
            ReferencePart::MapAccess(key) => {
                f.write_str("[")?;
                // string keys without debug quotes
                if let Some(s) = key.try_downcast_ref::<String>() {
                    f.write_str(s)?;
                } else if let Some(s) = key.try_downcast_ref::<Cow<'static, str>>() {
                    f.write_str(s)?;
                } else if let Some(s) = key.try_downcast_ref::<&'static str>() {
                    f.write_str(s)?;
                } else {
                    key.debug(f)?;
                }
                f.write_str("]")
            }
        }
    }
}

impl Display for ReferencePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.path.iter().try_for_each(|p| p.fmt(f))
    }
}

impl DisplayWithTypeInfo for ReferencePath {
    fn display_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _type_info_provider: Option<&WorldGuard>,
    ) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl ReferencePath {
    /// Traverses the reference path from the given root object.
    pub fn reflect_element<'a>(
        &self,
        val: &'a dyn PartialReflect,
        type_registry: &TypeRegistry,
    ) -> Result<Option<&'a dyn PartialReflect>, ReferencePathError> {
        let mut current_root: &'a dyn PartialReflect = val;
        for (i, p) in self.path.iter().enumerate() {
            current_root = match p.reflect_element(current_root, type_registry) {
                Ok(None) => return Ok(None),
                Ok(Some(v)) => v,
                Err(_) => {
                    return Err(ReferencePathError {
                        prior_reflected_element: current_root.get_represented_type_info(),
                        path: self.clone(),
                        index: i,
                    });
                }
            };
        }
        Ok(Some(current_root))
    }

    /// Traverses the reference path from the given root object.
    pub fn reflect_element_mut<'a>(
        &self,
        val: &'a mut dyn PartialReflect,
        type_registry: &TypeRegistry,
    ) -> Result<Option<&'a mut dyn PartialReflect>, ReferencePathError> {
        let mut current_root: &'a mut dyn PartialReflect = val;
        for (i, p) in self.path.iter().enumerate() {
            let previous_type_info = current_root.get_represented_type_info();
            current_root = match p.reflect_element_mut(current_root, type_registry) {
                Ok(None) => return Ok(None),
                Ok(Some(v)) => v,
                Err(_) => {
                    return Err(ReferencePathError {
                        prior_reflected_element: previous_type_info,
                        path: self.clone(),
                        index: i,
                    });
                }
            };
        }
        Ok(Some(current_root))
    }

    /// Returns true if the underlying path has no elements
    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }

    /// Pushes a new reference at the end of the current reflection path.
    pub fn push(&mut self, part: ReferencePart) {
        self.path.push(part)
    }

    /// Pushes a new reference at the end of the current reflection path.
    pub fn extend(&mut self, part: impl Iterator<Item = ReferencePart>) {
        self.path.extend(part)
    }
}

/// Errors to do with custom BMS reference path resolution.
pub struct ReferencePathError {
    prior_reflected_element: Option<&'static TypeInfo>,
    path: ReferencePath,
    index: usize,
}

impl Display for ReferencePathError {
    /// e.g.
    /// ```text
    /// Cannot access field `x` on opaque type `u8`, at:
    ///   [1][1][0].x
    ///            ^^
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Some(part) = self.path.path.get(self.index) else {
            return write!(f, "Cannot reflect into path: `{}`", self.path);
        };
        match part {
            ReferencePart::StringAccess(s) => write!(f, "Cannot access field `{s}`")?,
            ReferencePart::IntegerAccess(i) => write!(f, "Cannot access index `{i}` (0-based)")?,
            ReferencePart::MapAccess(_) => write!(f, "Cannot access key `{part}`")?,
        }
        let kind = match self.prior_reflected_element {
            Some(TypeInfo::Struct(_)) => "struct",
            Some(TypeInfo::TupleStruct(_)) => "tuple struct",
            Some(TypeInfo::Tuple(_)) => "tuple",
            Some(TypeInfo::List(_)) => "list",
            Some(TypeInfo::Array(_)) => "array",
            Some(TypeInfo::Map(_)) => "map",
            Some(TypeInfo::Set(_)) => "set",
            Some(TypeInfo::Enum(_)) => "enum",
            Some(TypeInfo::Opaque(_)) => "opaque type",
            None => "value of unknown type",
        };
        f.write_str(" on ")?;
        f.write_str(kind)?;
        if let Some(info) = self.prior_reflected_element {
            write!(f, " `{}`", info.type_path_table().path())?;
        }

        // the path with the failing part underlined
        let before: String = self.path.path[..self.index]
            .iter()
            .map(ToString::to_string)
            .collect();
        let failed = part.to_string();
        let after: String = self.path.path[self.index + 1..]
            .iter()
            .map(ToString::to_string)
            .collect();
        write!(
            f,
            ", at:\n  {before}{failed}{after}\n  {}{}",
            " ".repeat(before.chars().count()),
            "^".repeat(failed.chars().count().max(1)),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn script_indices_are_corrected_once_per_part() {
        let registry = TypeRegistry::default();
        let mut value: Vec<Vec<Option<u8>>> = vec![vec![], vec![Some(1), Some(2)]];
        let script_index = |i: i64, language: Language| match ReferencePart::new_from_script_val(
            ScriptValue::Integer(i),
            language,
            None,
        ) {
            Ok(part) => part,
            Err(_) => panic!("integer key not converted"),
        };
        // indexed from Lua, then further from a 0-indexed language, then into `Some` by the host
        let mut path = ReferencePath::default();
        path.push(script_index(2, Language::Lua));
        path.push(script_index(1, Language::Rune));
        path.push(ReferencePart::IntegerAccess(0));

        let Ok(Some(v)) = path.reflect_element(&value, &registry) else {
            panic!("element not found")
        };
        assert_eq!(v.try_downcast_ref::<u8>(), Some(&2));
        // displayed as the host index
        assert_eq!(path.to_string(), "[1][1][0]");

        let Ok(Some(v)) = path.reflect_element_mut(&mut value, &registry) else {
            panic!("element not found")
        };
        *v.try_downcast_mut::<u8>().unwrap() = 5;
        assert_eq!(value[1][1], Some(5));

        // the correction saturates instead of overflowing
        assert!(script_index(i64::MIN, Language::Lua) == ReferencePart::IntegerAccess(i64::MIN));

        // errors show the whole path and underline the failing part, string keys without quotes
        path.push(ReferencePart::StringAccess("x".into()));
        path.push(ReferencePart::MapAccess(Box::new("k".to_owned())));
        let Err(err) = path.reflect_element(&value, &registry) else {
            panic!("string key into u8 resolved")
        };
        assert_eq!(
            err.to_string(),
            "Cannot access field `x` on opaque type `u8`, at:\n  [1][1][0].x[k]\n           ^^"
        );
    }
}
