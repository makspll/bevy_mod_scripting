//! Parsing definitions for the LAD (Language Agnostic Decleration) file format.
//!
//! The main ideals behind the format are:
//! - Centralization, we want to centralize as much of the "documentation" logic in the building of this format. For example, instead of letting each backend parse argument docstrings from the function docstring, we can do this here, and let the backends concentrate on pure generation.
//! - Rust centric, the format describes bindings from the Rust side, so we generate rust centric declarations. These can then freely be converted into whatever representaion necessary.

use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};

pub use bevy_mod_scripting_bindings_domain::ReflectionPrimitiveKind; // re-export the thing we use
use indexmap::IndexMap;

/// The current version of the LAD_VERSION format supported by this library.
/// Earlier versions are not guaranteed to be supported.
pub const LAD_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The included example LAD file for testing purposes.
#[cfg(feature = "testfile")]
pub const EXAMPLE_LADFILE: &str = include_str!("../test_assets/test.lad.json");

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
/// A Language Agnostic Declaration (LAD) file.
pub struct LadFile {
    /// The version of the LAD file format used.
    pub version: Cow<'static, str>,

    /// The global instances defined in the LAD file.
    pub globals: IndexMap<Cow<'static, str>, LadInstance>,

    /// The types defined in the LAD file.
    pub types: IndexMap<LadTypeId, LadTypeDefinition>,

    /// The functions defined in the LAD file.
    pub functions: IndexMap<LadFunctionId, LadFunction>,

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
            description: None,
        }
    }

    /// Retrieves the best type identifier suitable for a type id.
    pub fn get_type_identifier(
        &self,
        type_id: &LadTypeId,
        raw_type_id_replacement: Option<&'static str>,
    ) -> Cow<'static, str> {
        self.types
            .get(type_id)
            .map(|t| t.identifier.clone().into())
            .unwrap_or_else(|| {
                if let Some(replacement) = raw_type_id_replacement {
                    replacement.into()
                } else {
                    type_id.0.clone()
                }
            })
    }

    /// Retrieves true if the type id corresponds to a primitive type.
    pub fn primitive_kind(&self, type_id: &LadTypeId) -> Option<&ReflectionPrimitiveKind> {
        self.types
            .get(type_id)
            .and_then(|t| t.metadata.mapped_to_primitive_kind.as_ref())
    }

    /// Retrieves the generics of a type id if it is a generic type.
    pub fn get_type_generics(&self, type_id: &LadTypeId) -> Option<&[LadGeneric]> {
        self.types
            .get(type_id)
            .and_then(|t| (!t.generics.is_empty()).then_some(t.generics.as_slice()))
    }

    /// Retrieves the documentation of a type id if it is a defined type and has documentation.
    pub fn get_type_documentation(&self, type_id: &LadTypeId) -> Option<&str> {
        self.types
            .get(type_id)
            .and_then(|t| t.documentation.as_deref())
    }

    /// Retrieves the metadata section for the given lad type id.
    /// Primitives don't contain metadata
    pub fn get_type_metadata(&self, type_id: &LadTypeId) -> Option<&LadTypeMetadata> {
        self.types.get(type_id).map(|t| &t.metadata)
    }

    /// Retrieves all unique types, then groups them by their generics arity,
    /// this grouping represents types as expected to be seen in rust source code.
    ///
    /// For example `Vec<T>` and `Vec<i32>` will be grouped together as `Vec` with arity 1.
    pub fn polymorphizied_types(&self) -> IndexMap<PolymorphicTypeKey, HashSet<&LadTypeId>> {
        let mut types_by_identifier_and_arity: IndexMap<PolymorphicTypeKey, HashSet<&LadTypeId>> =
            IndexMap::<PolymorphicTypeKey, HashSet<&LadTypeId>>::new();
        for type_id in self.types.keys() {
            let arity = self.get_type_arity(type_id);
            let identifier = self.get_type_identifier(type_id, None);
            types_by_identifier_and_arity
                .entry(PolymorphicTypeKey { identifier, arity })
                .or_default()
                .insert(type_id);
        }

        types_by_identifier_and_arity
    }

    /// Returns the arity of a type, which is the number of generic parameters it has.
    /// Types without generics will return 0, meaning they can be identified uniquely by their identifier.
    pub fn get_type_arity(&self, type_id: &LadTypeId) -> usize {
        self.types
            .get(type_id)
            .map(|t| t.generics.len())
            .unwrap_or(0) // primitives have no generics currently
    }
}

/// A key for polymorphic types, used to group types by their identifier and arity.
///
/// Each key would correspond to a unique rust type, such as `Vec<T>` or `HashMap<K,V>`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PolymorphicTypeKey {
    /// The type identifier
    pub identifier: Cow<'static, str>,
    /// The arity of the type
    pub arity: usize,
}

impl Default for LadFile {
    fn default() -> Self {
        Self::new()
    }
}

/// A LAD global instance
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LadInstance {
    /// The kind of the instance
    pub type_kind: LadFieldOrVariableKind,

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
    /// If `Some`, signifies the function is an overload of another function
    pub overload_index: Option<usize>,
    /// The argument information for the function.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub arguments: Vec<LadArgument>,
    /// The return type of the function.
    pub return_type: LadArgument,
    /// The documentation describing the function.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub documentation: Option<Cow<'static, str>>,
    /// Function metadata
    pub metadata: LadFunctionMetadata,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
/// Additional data relevant to functions
pub struct LadFunctionMetadata {
    /// True if the function represents a supported binary or unary operation
    pub is_operator: bool,
    /// Extra metadata to be populated by external plugins
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub misc: HashMap<String, String>,
}

impl LadFunction {
    /// Checks if the function is an overload, and if so parses the overload number and the true name.
    pub fn as_overload(&self) -> Option<(Cow<'static, str>, usize)> {
        self.overload_index
            .map(|index| (self.identifier.clone(), index))
    }

    /// Qualifies the identifier of this function with its overload number.
    /// Use this instead of the identifier if you require a unique name per function.
    pub fn identifier_with_overload(&self) -> Cow<'static, str> {
        if let Some(index) = self.overload_index {
            format!("{}-{index}", self.identifier).into()
        } else {
            self.identifier.clone()
        }
    }
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
    pub kind: LadFieldOrVariableKind,

    /// The provided documentation for this argument. Normally derived from the function docstring.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub documentation: Option<Cow<'static, str>>,

    /// The name of the argument
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub name: Option<Cow<'static, str>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
/// The kind of field or variable/parameter in a LAD file.
/// There is a distinction between the "core" definition/identity of a type
/// and how it's used in various contexts.
///
/// for example:
/// - `Vec<T>` is a list of `T`'s
/// - `T` IS T
///
/// In generating documents, it's convenient to distinguish a few core "containers" to provide useful information.
#[serde(rename_all = "camelCase")]
pub enum LadFieldOrVariableKind {
    /// a `Ref` wrapped argument
    Ref(LadTypeId),
    /// a `Mut` wrapped argument
    Mut(LadTypeId),
    /// a `Val` wrapped argument
    Val(LadTypeId),
    /// an `Option` wrapped argument
    Option(Box<LadFieldOrVariableKind>),
    /// a `Vec`
    Vec(Box<LadFieldOrVariableKind>),
    /// a `HashMap`
    HashMap(Box<LadFieldOrVariableKind>, Box<LadFieldOrVariableKind>),
    /// a `HashSet`
    HashSet(Box<LadFieldOrVariableKind>),
    /// A `InteropResult`
    InteropResult(Box<LadFieldOrVariableKind>),
    /// A tuple of arguments
    Tuple(Vec<LadFieldOrVariableKind>),
    /// An array
    Array(Box<LadFieldOrVariableKind>, usize),
    /// A primitive type, implementing `IntoScript` and `FromScript` natively in BMS.
    Primitive(ReflectionPrimitiveKind),
    /// A union of two or more types
    Union(Vec<LadFieldOrVariableKind>),
    /// An arbitrary type which is either unsupported, doesn't contain type information, or is generally unknown.
    ///
    /// This will be the variant used for external primitives as well.
    Unknown(LadTypeId),
}

/// Utility for dispatching visitors on dyn traits
pub trait LadVisitable {
    /// Runs the visitor on the given node
    fn accept(&self, visitor: &mut dyn ArgumentVisitor);
}

impl LadVisitable for LadTypeId {
    fn accept(&self, visitor: &mut dyn ArgumentVisitor) {
        visitor.visit_lad_type_id(self);
    }
}

impl LadVisitable for LadFieldOrVariableKind {
    fn accept(&self, visitor: &mut dyn ArgumentVisitor) {
        visitor.visit(self);
    }
}
/// A visitor pattern for running arbitrary logic on the hierarchy of arguments.
///
/// Use cases are mostly to do with printing the arguments in a human readable format.
#[allow(unused_variables)]
#[cfg(feature = "visitor")]
pub trait ArgumentVisitor {
    /// perform an action on a `LadTypeId`, by default noop
    fn visit_lad_type_id(&mut self, type_id: &LadTypeId);

    /// perform an action on a `LadBMSPrimitiveKind`, by default visits the type id of the primitive kind
    fn visit_lad_bms_primitive_kind(&mut self, primitive_kind: &ReflectionPrimitiveKind);

    /// perform an action on a `Unknown`, by default visits the type id of the unknown type
    fn visit_unknown(&mut self, type_id: &LadTypeId) {
        self.visit_lad_type_id(type_id);
    }

    /// walks the lad type_id structure, by default simply visits type_id's
    /// Can be used to dispatch to primitives instead of the type maps to these
    fn walk_lad_type_id(&mut self, type_id: &LadTypeId) {
        self.visit_lad_type_id(type_id);
    }

    /// traverse a `Ref` wrapped argument, by default calls `visit` on the inner argument
    fn walk_ref(&mut self, type_id: &LadTypeId) {
        self.visit_lad_type_id(type_id);
    }

    /// traverse a `Mut` wrapped argument, by default calls `visit` on the inner argument
    fn walk_mut(&mut self, type_id: &LadTypeId) {
        self.visit_lad_type_id(type_id);
    }

    /// traverse a `Val` wrapped argument, by default calls `visit` on the inner argument
    fn walk_val(&mut self, type_id: &LadTypeId) {
        self.visit_lad_type_id(type_id);
    }

    /// traverse an `Option` wrapped argument, by default calls `visit` on the inner argument
    fn walk_option(&mut self, inner: &LadFieldOrVariableKind) {
        self.visit(inner);
    }

    /// traverse a `Vec` wrapped argument, by default calls `visit` on the inner argument
    fn walk_vec(&mut self, inner: &LadFieldOrVariableKind) {
        self.visit(inner);
    }

    /// traverse a `HashMap` wrapped argument, by default calls `visit` on the key and value
    fn walk_hash_map(&mut self, key: &LadFieldOrVariableKind, value: &LadFieldOrVariableKind) {
        self.visit(key);
        self.visit(value);
    }

    /// traverse a `HashMap` wrapped argument, by default calls `visit` on the key and value
    fn walk_hash_set(&mut self, key: &LadFieldOrVariableKind) {
        self.visit(key);
    }

    /// traverse an `InteropResult` wrapped argument, by default calls `visit` on the inner argument
    fn walk_interop_result(&mut self, inner: &LadFieldOrVariableKind) {
        self.visit(inner);
    }

    /// traverse a tuple of arguments, by default calls `visit` on each argument
    fn walk_tuple(&mut self, inner: &[LadFieldOrVariableKind]) {
        for arg in inner {
            self.visit(arg);
        }
    }

    /// traverse an array of arguments, by default calls `visit` on the inner argument
    fn walk_array(&mut self, inner: &LadFieldOrVariableKind, size: usize) {
        self.visit(inner);
    }

    /// traverse a primitive argument, by default calls `visit` on the primitive kind
    fn walk_primitive(&mut self, primitive_kind: &ReflectionPrimitiveKind) {
        self.visit_lad_bms_primitive_kind(primitive_kind);
    }

    /// traverse a union of arguments, by default calls `visit` on each argument
    fn walk_union(&mut self, inner: &[LadFieldOrVariableKind]) {
        for arg in inner {
            self.visit(arg);
        }
    }

    /// traverse an unknown argument, by default calls `visit` on the type id
    fn walk_unknown(&mut self, type_id: &LadTypeId) {
        self.visit_unknown(type_id);
    }

    /// Visit an argument kind, by default calls the appropriate walk method on each enum variant.
    ///
    /// Each walk variant will walk over nested kinds, and visit the leaf types.
    ///
    /// If you want to do something with the parent types, you WILL have to override each individual walk method.
    fn visit(&mut self, kind: &LadFieldOrVariableKind) {
        match kind {
            LadFieldOrVariableKind::Ref(type_id) => self.walk_ref(type_id),
            LadFieldOrVariableKind::Mut(type_id) => self.walk_mut(type_id),
            LadFieldOrVariableKind::Val(type_id) => self.walk_val(type_id),
            LadFieldOrVariableKind::Option(inner) => self.walk_option(inner),
            LadFieldOrVariableKind::Vec(inner) => self.walk_vec(inner),
            LadFieldOrVariableKind::HashMap(key, value) => self.walk_hash_map(key, value),
            LadFieldOrVariableKind::HashSet(key) => self.walk_hash_set(key),
            LadFieldOrVariableKind::InteropResult(inner) => self.walk_interop_result(inner),
            LadFieldOrVariableKind::Tuple(inner) => self.walk_tuple(inner),
            LadFieldOrVariableKind::Array(inner, size) => self.walk_array(inner, *size),
            LadFieldOrVariableKind::Primitive(primitive_kind) => {
                self.walk_primitive(primitive_kind)
            }
            LadFieldOrVariableKind::Union(inner) => self.walk_union(inner),
            LadFieldOrVariableKind::Unknown(type_id) => self.walk_unknown(type_id),
        }
    }
}

// #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
// /// A BMS primitive definition
// pub struct LadBMSPrimitiveType {
//     /// The kind of primitive
//     pub kind: ReflectionPrimitiveKind,
//     /// The documentation describing the primitive
//     pub documentation: Cow<'static, str>,
// }

// impl LadBMSPrimitiveType {
//     /// Get the corresponding type id for a primitive kind.
//     pub fn lad_type_id(kind: ReflectionPrimitiveKind) -> LadTypeId {
//         match kind {
//             ReflectionPrimitiveKind::Bool => LadTypeId::new_string_id("bool".into()),
//             ReflectionPrimitiveKind::Isize => LadTypeId::new_string_id("isize".into()),
//             ReflectionPrimitiveKind::I8 => LadTypeId::new_string_id("i8".into()),
//             ReflectionPrimitiveKind::I16 => LadTypeId::new_string_id("i16".into()),
//             ReflectionPrimitiveKind::I32 => LadTypeId::new_string_id("i32".into()),
//             ReflectionPrimitiveKind::I64 => LadTypeId::new_string_id("i64".into()),
//             ReflectionPrimitiveKind::I128 => LadTypeId::new_string_id("i128".into()),
//             ReflectionPrimitiveKind::Usize => LadTypeId::new_string_id("usize".into()),
//             ReflectionPrimitiveKind::U8 => LadTypeId::new_string_id("u8".into()),
//             ReflectionPrimitiveKind::U16 => LadTypeId::new_string_id("u16".into()),
//             ReflectionPrimitiveKind::U32 => LadTypeId::new_string_id("u32".into()),
//             ReflectionPrimitiveKind::U64 => LadTypeId::new_string_id("u64".into()),
//             ReflectionPrimitiveKind::U128 => LadTypeId::new_string_id("u128".into()),
//             ReflectionPrimitiveKind::F32 => LadTypeId::new_string_id("f32".into()),
//             ReflectionPrimitiveKind::F64 => LadTypeId::new_string_id("f64".into()),
//             ReflectionPrimitiveKind::Char => LadTypeId::new_string_id("char".into()),
//             ReflectionPrimitiveKind::Str => LadTypeId::new_string_id("str".into()),
//             ReflectionPrimitiveKind::String => LadTypeId::new_string_id("String".into()),
//             ReflectionPrimitiveKind::OsString => LadTypeId::new_string_id("OsString".into()),
//             ReflectionPrimitiveKind::PathBuf => LadTypeId::new_string_id("PathBuf".into()),
//             ReflectionPrimitiveKind::FunctionCallContext => {
//                 LadTypeId::new_string_id("FunctionCallContext".into())
//             }
//             ReflectionPrimitiveKind::DynamicFunction => {
//                 LadTypeId::new_string_id("DynamicFunction".into())
//             }
//             ReflectionPrimitiveKind::DynamicFunctionMut => {
//                 LadTypeId::new_string_id("DynamicFunctionMut".into())
//             }
//             ReflectionPrimitiveKind::ReflectReference => {
//                 LadTypeId::new_string_id("ReflectReference".into())
//             }
//             ReflectionPrimitiveKind::ScriptValue => LadTypeId::new_string_id("ScriptValue".into()),
//             ReflectionPrimitiveKind::External(external) => {
//                 LadTypeId::new_string_id(external.into())
//             }
//         }
//     }
// }

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
pub struct LadTypeDefinition {
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

    /// If a type is marked as auto generated. Auto generated types might be treated differently by
    /// backends which generate documentation or other files. For example they might be hidden or put in a separate section.
    #[serde(default)]
    pub generated: bool,

    /// An "importance" value. By default all types get a value of 1000.
    /// A lower insignificance means the type is more important.
    ///
    /// Backends can use this value to determine the order in which types are displayed.
    #[serde(default = "default_importance")]
    pub insignificance: usize,

    /// Additional metadata about the type.
    pub metadata: LadTypeMetadata,
}

/// Metadata either calculated from the type registry or added by plugins
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LadTypeMetadata {
    /// True if the type is a component according to the type registry
    pub is_component: bool,
    /// Returns true if the type is a component according to the type registry
    pub is_resource: bool,

    /// True if the type actually implements reflect, some types can be
    /// added as namespaces without actually implementing the Reflect trait
    pub is_reflect: bool,

    /// Set to a primitive kind if this type is mapped to a primitive
    pub mapped_to_primitive_kind: Option<ReflectionPrimitiveKind>,

    /// Extra metadata sections that plugins can use to serialize other information
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub misc: HashMap<String, String>,
}

/// The default importance value for a type.
pub fn default_importance() -> usize {
    1000
}

// /// A type importance value.
// pub struct Importance(pub usize)

// impl Default for Importance {

// }

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
    pub type_: LadFieldOrVariableKind,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
/// A named field definition used in a LAD file.
pub struct LadNamedField {
    /// The name of the field.
    pub name: String,
    #[serde(rename = "type")]
    /// The type of the field.
    pub type_: LadFieldOrVariableKind,
}

/// A generic type definition used in a LAD file.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LadGeneric {
    /// The id of the type assigned to this generic.
    pub type_id: LadTypeId,
    /// The name of the generic
    pub name: String,
}

/// Parses a json string into a LAD file.
pub fn parse_lad_file(toml: &str) -> Result<LadFile, serde_json::Error> {
    serde_json::from_str(toml)
}

/// Serializes a LAD file into a json file.
pub fn serialize_lad_file(lad_file: &LadFile, pretty: bool) -> Result<String, serde_json::Error> {
    if pretty {
        serde_json::to_string_pretty(lad_file)
    } else {
        serde_json::to_string(lad_file)
    }
}
