//! Parsing definitions for the LAD (Language Agnostic Decleration) file format.

use indexmap::IndexMap;
use std::borrow::Cow;

/// The current version of the LAD_VERSION format supported by this library.
/// Earlier versions are not guaranteed to be supported.
pub const LAD_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
/// A Language Agnostic Declaration (LAD) file.
pub struct LadFile {
    /// The version of the LAD file format used.
    pub version: Cow<'static, str>,

    /// The global instances defined in the LAD file.
    pub globals: IndexMap<Cow<'static, str>, LadInstance>,

    /// The types defined in the LAD file.
    pub types: IndexMap<LadTypeId, LadType>,

    /// The functions defined in the LAD file.
    pub functions: IndexMap<LadFunctionId, LadFunction>,

    /// A mapping from type ids to primitive types
    pub primitives: IndexMap<LadTypeId, LadBMSPrimitiveType>,

    /// A description of the LAD file and its contents in markdown
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,
}

impl LadFile {
    /// Create a new empty LAD file.
    pub fn new() -> Self {
        Self {
            version: LAD_VERSION.into(),
            globals: IndexMap::new(),
            types: IndexMap::new(),
            functions: IndexMap::new(),
            primitives: IndexMap::new(),
            description: None,
        }
    }
}

impl Default for LadFile {
    fn default() -> Self {
        Self::new()
    }
}

/// A LAD global instance
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LadInstance {
    /// The type of the instance
    pub type_id: LadTypeId,

    /// whether the instance is static or not
    ///
    /// static instances do not support method call syntax on them. I.e. only functions without a self parameter can be called on them.
    /// They also do not support field access syntax.
    pub is_static: bool,
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
/// A unique identifier for a function in a LAD file.
/// Only unique within the LAD file.
pub struct LadFunctionId(String);

impl std::fmt::Display for LadFunctionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl LadFunctionId {
    /// Create a new LAD function id with a string.
    pub fn new_string_id(function_id: String) -> Self {
        LadFunctionId(function_id)
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
/// A function definition used in a LAD file.
pub struct LadFunction {
    /// The namespace of the function.
    pub namespace: LadFunctionNamespace,
    /// The identifier or name of the function.
    pub identifier: Cow<'static, str>,
    /// The argument information for the function.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub arguments: Vec<LadArgument>,
    /// The return type of the function.
    pub return_type: LadTypeId,
    /// The documentation describing the function.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub documentation: Option<Cow<'static, str>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
/// A function namespace used in a LAD file.
pub enum LadFunctionNamespace {
    /// A function in a type's namespace
    Type(LadTypeId),
    /// A global function
    Global,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
/// An argument definition used in a LAD file.
pub struct LadArgument {
    /// The kind and type of argument
    pub kind: LadArgumentKind,
    /// The name of the argument
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub name: Option<Cow<'static, str>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
/// The kind of an argument in a LAD file.
#[serde(rename_all = "camelCase")]
pub enum LadArgumentKind {
    /// a `Ref` wrapped argument
    Ref(LadTypeId),
    /// a `Mut` wrapped argument
    Mut(LadTypeId),
    /// a `Val` wrapped argument
    Val(LadTypeId),
    /// an `Option` wrapped argument
    Option(Box<LadArgumentKind>),
    /// a `Vec`
    Vec(Box<LadArgumentKind>),
    /// a `HashMap`
    HashMap(Box<LadArgumentKind>, Box<LadArgumentKind>),
    /// A `InteropResult`
    InteropResult(Box<LadArgumentKind>),
    /// A tuple of arguments
    Tuple(Vec<LadArgumentKind>),
    /// An array
    Array(Box<LadArgumentKind>, usize),
    /// A primitive type, implementing `IntoScript` and `FromScript` natively in BMS.
    Primitive(LadBMSPrimitiveKind),
    /// An arbitrary type which is either unsupported, doesn't contain type information, or is generally unknown.
    ///
    /// This will be the variant used for external primitives as well.
    Unknown(LadTypeId),
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
/// A BMS primitive definition
pub struct LadBMSPrimitiveType {
    /// The kind of primitive
    pub kind: LadBMSPrimitiveKind,
    /// The documentation describing the primitive
    pub documentation: Cow<'static, str>,
}

/// A primitive type kind in the LAD file format.
///
/// The docstrings on variants corresponding to Reflect types, are used to generate documentation for these primitives.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)]
pub enum LadBMSPrimitiveKind {
    Bool,
    Isize,
    I8,
    I16,
    I32,
    I64,
    I128,
    Usize,
    U8,
    U16,
    U32,
    U64,
    U128,
    F32,
    F64,
    Char,
    Str,
    String,
    OsString,
    PathBuf,
    FunctionCallContext,
    DynamicFunction,
    DynamicFunctionMut,
    ReflectReference,
}

impl LadBMSPrimitiveKind {
    /// Get the corresponding type id for a primitive kind.
    pub fn lad_type_id(self) -> LadTypeId {
        match self {
            LadBMSPrimitiveKind::Bool => LadTypeId::new_string_id("bool".into()),
            LadBMSPrimitiveKind::Isize => LadTypeId::new_string_id("isize".into()),
            LadBMSPrimitiveKind::I8 => LadTypeId::new_string_id("i8".into()),
            LadBMSPrimitiveKind::I16 => LadTypeId::new_string_id("i16".into()),
            LadBMSPrimitiveKind::I32 => LadTypeId::new_string_id("i32".into()),
            LadBMSPrimitiveKind::I64 => LadTypeId::new_string_id("i64".into()),
            LadBMSPrimitiveKind::I128 => LadTypeId::new_string_id("i128".into()),
            LadBMSPrimitiveKind::Usize => LadTypeId::new_string_id("usize".into()),
            LadBMSPrimitiveKind::U8 => LadTypeId::new_string_id("u8".into()),
            LadBMSPrimitiveKind::U16 => LadTypeId::new_string_id("u16".into()),
            LadBMSPrimitiveKind::U32 => LadTypeId::new_string_id("u32".into()),
            LadBMSPrimitiveKind::U64 => LadTypeId::new_string_id("u64".into()),
            LadBMSPrimitiveKind::U128 => LadTypeId::new_string_id("u128".into()),
            LadBMSPrimitiveKind::F32 => LadTypeId::new_string_id("f32".into()),
            LadBMSPrimitiveKind::F64 => LadTypeId::new_string_id("f64".into()),
            LadBMSPrimitiveKind::Char => LadTypeId::new_string_id("char".into()),
            LadBMSPrimitiveKind::Str => LadTypeId::new_string_id("str".into()),
            LadBMSPrimitiveKind::String => LadTypeId::new_string_id("String".into()),
            LadBMSPrimitiveKind::OsString => LadTypeId::new_string_id("OsString".into()),
            LadBMSPrimitiveKind::PathBuf => LadTypeId::new_string_id("PathBuf".into()),
            LadBMSPrimitiveKind::FunctionCallContext => {
                LadTypeId::new_string_id("FunctionCallContext".into())
            }
            LadBMSPrimitiveKind::DynamicFunction => {
                LadTypeId::new_string_id("DynamicFunction".into())
            }
            LadBMSPrimitiveKind::DynamicFunctionMut => {
                LadTypeId::new_string_id("DynamicFunctionMut".into())
            }
            LadBMSPrimitiveKind::ReflectReference => {
                LadTypeId::new_string_id("ReflectReference".into())
            }
        }
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
/// A unique identifier for a type in a LAD file.
///
/// Only guaranteed to be unique within the LAD file.
/// It *might* be unique across LAD files, but this is not guaranteed and depends on the type itself.
pub struct LadTypeId(Cow<'static, str>);

impl std::fmt::Display for LadTypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl LadTypeId {
    /// Create a new LAD type id with a specific index.
    pub fn new_string_id(type_id: Cow<'static, str>) -> Self {
        LadTypeId(type_id)
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
/// A type definition used in a LAD file.
pub struct LadType {
    /// The identifier or name of the type.
    pub identifier: String,

    /// The source crate of the type
    #[serde(rename = "crate", skip_serializing_if = "Option::is_none", default)]
    pub crate_: Option<String>,

    /// The full path of the type
    pub path: String,

    /// The generic parameters of the type.
    ///
    /// Generics are always monomorphized in the LAD file format.
    /// Meaning that they are always assigned a concrete type.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub generics: Vec<LadGeneric>,

    /// The documentation describing the type.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub documentation: Option<String>,

    /// Functions which are "associated" with this type.
    /// I.e. those which are either methods or static functions of this type.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub associated_functions: Vec<LadFunctionId>,

    /// The layout or kind of the type.
    pub layout: LadTypeLayout,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
/// Description of a type layout in a LAD file.
pub enum LadTypeLayout {
    /// A type with hidden layout
    Opaque,
    /// A type with at least one variant
    MonoVariant(LadVariant),
    /// A type with multiple variants
    Enum(Vec<LadVariant>),
}

impl LadTypeLayout {
    /// Traverses the layout in a depth-first manner and calls the provided function on each variant in order of appearance.
    /// Calls the function with the variant and its index in the layout starting from 0.
    ///
    /// If the layout is opaque, Some with the provided default is returned
    pub fn for_each_variant<F: FnMut(&LadVariant, usize), D>(
        &self,
        mut f: F,
        default: D,
    ) -> Option<D> {
        match self {
            LadTypeLayout::Opaque => Some(default),
            LadTypeLayout::MonoVariant(variant) => {
                f(variant, 0);
                None
            }
            LadTypeLayout::Enum(variants) => {
                for (i, variant) in variants.iter().enumerate() {
                    f(variant, i);
                }
                None
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind")]
/// A variant definition used in a LAD file.
pub enum LadVariant {
    /// A tuple struct variant i.e. a struct with unnamed fields.
    TupleStruct {
        /// The name of the variant.
        ///
        /// For types which are not Enums, this will simply be the name of the type or its path if no identifier is present.
        name: Cow<'static, str>,

        /// The fields of the tuple struct variant.
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        fields: Vec<LadField>,
    },
    /// A struct variant i.e. a struct with named fields.
    Struct {
        /// The name of the variant.
        ///
        /// For types which are not Enums, this will simply be the name of the type or its path if no identifier is present.
        name: Cow<'static, str>,

        /// The fields of the struct variant.
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        fields: Vec<LadNamedField>,
    },
    /// A unit variant i.e. a type with no fields
    Unit {
        /// The name of the variant.
        ///
        /// For types which are not Enums, this will simply be the name of the type or its path if no identifier is present.
        name: Cow<'static, str>,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
/// A field definition used in a LAD file.
pub struct LadField {
    /// The type of the field.
    #[serde(rename = "type")]
    pub type_: LadTypeId,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
/// A named field definition used in a LAD file.
pub struct LadNamedField {
    /// The name of the field.
    pub name: String,
    #[serde(rename = "type")]
    /// The type of the field.
    pub type_: LadTypeId,
}

/// A generic type definition used in a LAD file.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LadGeneric {
    /// The id of the type assigned to this generic.
    pub type_id: LadTypeId,
    /// The name of the generic
    pub name: String,
}

/// Parses a toml string into a LAD file.
pub fn parse_lad_file(toml: &str) -> Result<LadFile, serde_json::Error> {
    serde_json::from_str(toml)
}

/// Serializes a LAD file into a toml file.
pub fn serialize_lad_file(lad_file: &LadFile, pretty: bool) -> Result<String, serde_json::Error> {
    if pretty {
        serde_json::to_string_pretty(lad_file)
    } else {
        serde_json::to_string(lad_file)
    }
}
