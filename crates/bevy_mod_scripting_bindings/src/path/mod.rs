//! BMS native reflection paths

use std::{borrow::Cow, fmt::Display};

use bevy_mod_scripting_asset::Language;
use bevy_mod_scripting_derive::DebugWithTypeInfo;
use bevy_mod_scripting_display::{DisplayWithTypeInfo, GetTypeInfo};
use bevy_reflect::{PartialReflect, ReflectMut, ReflectRef, TypeInfo, TypeRegistry};

use crate::{ScriptValue, WorldGuard, convert};

/// A key referencing into a `Reflect` supporting trait object.
#[derive(DebugWithTypeInfo)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
pub enum ReferencePart {
    /// A string labelled reference
    StringAccess(Cow<'static, str>),
    /// An integer labelled reference, with optional indexing correction
    IntegerAccess(i64, bool),
    /// A key to a map or set,
    MapAccess(Box<dyn PartialReflect>),
}

impl Clone for ReferencePart {
    fn clone(&self) -> Self {
        match self {
            Self::StringAccess(arg0) => Self::StringAccess(arg0.clone()),
            Self::IntegerAccess(arg0, arg1) => Self::IntegerAccess(*arg0, *arg1),
            Self::MapAccess(arg0) => Self::MapAccess(match arg0.reflect_clone() {
                Ok(c) => c,
                Err(_) => arg0.to_dynamic(), // this is okay, because we need to call FromReflect on the map side anyway
            }),
        }
    }
}

impl PartialEq for ReferencePart {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::StringAccess(l0), Self::StringAccess(r0)) => l0 == r0,
            (Self::IntegerAccess(l0, l1), Self::IntegerAccess(r0, r1)) => l0 == r0 && l1 == r1,
            (Self::MapAccess(l0), Self::MapAccess(r0)) => {
                l0.reflect_partial_eq(r0.as_ref()).unwrap_or(false)
            }
            _ => false,
        }
    }
}

impl Display for ReferencePart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReferencePart::StringAccess(cow) => {
                f.write_str("[\"")?;
                f.write_str(cow)?;
                f.write_str("\"]")
            }
            ReferencePart::IntegerAccess(int, correction) => {
                f.write_str("[")?;
                f.write_str(&if *correction { *int - 1 } else { *int }.to_string())?;
                f.write_str("]")
            }
            ReferencePart::MapAccess(partial_reflect) => {
                f.write_str("[")?;
                partial_reflect.debug(f)?;
                f.write_str("]")
            }
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
    pub fn expect_integer(&self, correct_indexing: bool) -> Result<i64, ()> {
        match self {
            ReferencePart::IntegerAccess(index, correction) => {
                Ok(if *correction && correct_indexing {
                    *index - 1
                } else {
                    *index
                })
            }
            _ => Err(()),
        }
    }

    /// Casts the keys into a partial reflect value regardless of type of access
    pub fn with_any<O, F: FnOnce(&dyn PartialReflect) -> O>(
        &self,
        correct_indexing: bool,
        f: F,
    ) -> O {
        match self {
            ReferencePart::StringAccess(cow) => f(cow),
            ReferencePart::IntegerAccess(index, correction) => {
                f(&(if *correction && correct_indexing {
                    *index - 1
                } else {
                    *index
                }))
            }
            ReferencePart::MapAccess(partial_reflect) => f(partial_reflect.as_ref()),
        }
    }

    /// Converts a [`ScriptValue`] to a indexing corrected reference part.
    ///
    /// If given a world guard also supports arbitrary references as keys
    pub fn new_from_script_val(
        value: ScriptValue,
        language: Language,
        world: Option<WorldGuard>,
    ) -> Result<Self, ScriptValue> {
        Ok(match value {
            ScriptValue::Integer(v) => Self::IntegerAccess(v, language.one_indexed()),
            ScriptValue::Float(v) => Self::IntegerAccess(
                v.max(i64::MIN as f64).min(i64::MAX as f64).round() as i64,
                language.one_indexed(),
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
        one_indexed: bool,
    ) -> Result<Option<&'a dyn PartialReflect>, ()> {
        Ok(match elem.reflect_ref() {
            ReflectRef::Struct(x) => x.field(self.expect_string()?),
            ReflectRef::TupleStruct(x) => x.field(self.expect_integer(one_indexed)? as usize),
            ReflectRef::Tuple(x) => x.field(self.expect_integer(one_indexed)? as usize),
            ReflectRef::List(x) => x.get(self.expect_integer(one_indexed)? as usize),
            ReflectRef::Array(x) => x.get(self.expect_integer(one_indexed)? as usize),
            ReflectRef::Enum(x) => match x.variant_type() {
                bevy_reflect::VariantType::Struct => x.field(self.expect_string()?),
                bevy_reflect::VariantType::Tuple => {
                    x.field_at(self.expect_integer(one_indexed)? as usize)
                }
                bevy_reflect::VariantType::Unit => return Err(()),
            },
            ReflectRef::Map(x) => {
                let id = x.get_represented_map_info().ok_or(())?.key_ty().id();
                self.with_any(one_indexed, |key| {
                    let coerced = convert(key, id).ok_or(())?;
                    Ok(x.get(coerced.as_ref()))
                })?
            }
            ReflectRef::Set(x) => {
                let id = match x.get_represented_type_info().ok_or(())? {
                    TypeInfo::Set(set_info) => set_info.value_ty().id(),
                    _ => unreachable!("impossible"),
                };
                self.with_any(one_indexed, |key| {
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
        one_indexed: bool,
    ) -> Result<Option<&'a mut dyn PartialReflect>, ()> {
        Ok(match elem.reflect_mut() {
            ReflectMut::Struct(x) => x.field_mut(self.expect_string()?),
            ReflectMut::TupleStruct(x) => x.field_mut(self.expect_integer(one_indexed)? as usize),
            ReflectMut::Tuple(x) => x.field_mut(self.expect_integer(one_indexed)? as usize),
            ReflectMut::List(x) => x.get_mut(self.expect_integer(one_indexed)? as usize),
            ReflectMut::Array(x) => x.get_mut(self.expect_integer(one_indexed)? as usize),
            ReflectMut::Enum(x) => match x.variant_type() {
                bevy_reflect::VariantType::Struct => x.field_mut(self.expect_string()?),
                bevy_reflect::VariantType::Tuple => {
                    x.field_at_mut(self.expect_integer(one_indexed)? as usize)
                }
                bevy_reflect::VariantType::Unit => return Err(()),
            },
            ReflectMut::Map(x) => {
                let id = x.get_represented_map_info().ok_or(())?.key_ty().id();
                self.with_any(one_indexed, |key| {
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
    one_indexed: bool,
    path: Vec<ReferencePart>,
}

impl ReferencePath {
    /// Sets the indexing mode on the path
    pub fn set_is_one_indexed(&mut self, is_one_indexed: bool) {
        self.one_indexed = is_one_indexed;
    }

    /// Traverses the reference path from the given root object.
    pub fn reflect_element<'a>(
        &self,
        val: &'a dyn PartialReflect,
        type_registry: &TypeRegistry,
    ) -> Result<Option<&'a dyn PartialReflect>, ReferencePathError> {
        let mut next: &'a dyn PartialReflect = val;
        for i in &self.path {
            next = match i.reflect_element(next, type_registry, self.one_indexed) {
                Ok(None) => return Ok(None),
                Ok(Some(v)) => v,
                Err(_) => {
                    return Err(ReferencePathError {
                        val: next.get_represented_type_info(),
                        part: i.clone(),
                    });
                }
            };
        }
        Ok(Some(next))
    }

    /// Traverses the reference path from the given root object.
    pub fn reflect_element_mut<'a>(
        &self,
        val: &'a mut dyn PartialReflect,
        type_registry: &TypeRegistry,
    ) -> Result<Option<&'a mut dyn PartialReflect>, ReferencePathError> {
        let mut next: &'a mut dyn PartialReflect = val;
        for i in &self.path {
            let type_info_current = next.get_represented_type_info();
            next = match i.reflect_element_mut(next, type_registry, self.one_indexed) {
                Ok(None) => return Ok(None),
                Ok(Some(v)) => v,
                Err(_) => {
                    return Err(ReferencePathError {
                        val: type_info_current,
                        part: i.clone(),
                    });
                }
            };
        }
        Ok(Some(next))
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

impl Display for ReferencePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in &self.path {
            std::fmt::Display::fmt(i, f)?
        }
        Ok(())
    }
}

impl DisplayWithTypeInfo for ReferencePath {
    fn display_with_type_info(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        _type_info_provider: Option<&dyn GetTypeInfo>,
    ) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

/// Errors to do with custom BMS reference path resolution.
pub struct ReferencePathError {
    val: Option<&'static TypeInfo>,
    part: ReferencePart,
}

impl Display for ReferencePathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Cannot reflect into ")?;
        f.write_str(match self.val {
            Some(TypeInfo::Struct(_)) => "struct",
            Some(TypeInfo::TupleStruct(_)) => "tuple truct",
            Some(TypeInfo::Tuple(_)) => "tuple",
            Some(TypeInfo::List(_)) => "list",
            Some(TypeInfo::Array(_)) => "array",
            Some(TypeInfo::Map(_)) => "map",
            Some(TypeInfo::Set(_)) => "set",
            Some(TypeInfo::Enum(_)) => "enum",
            Some(TypeInfo::Opaque(_)) => "opaque type",
            None => "unknown type",
        })?;

        f.write_str(" of type: ")?;
        f.write_str(
            self.val
                .map(|t| t.type_path_table().path())
                .unwrap_or("unknown type"),
        )?;

        f.write_str(" with ")?;

        f.write_str(match &self.part {
            ReferencePart::StringAccess(_) => "string key",
            ReferencePart::IntegerAccess(_, _) => "integer key",
            ReferencePart::MapAccess(_) => "map key",
        })?;

        f.write_str(": `")?;
        std::fmt::Display::fmt(&self.part, f)?;
        f.write_str("`")?;

        Ok(())
    }
}
