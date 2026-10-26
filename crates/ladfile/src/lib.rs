//! Parsing definitions for the LAD (Language Agnostic Declaration) file format.
//!
//! The main ideals behind the format are:
//! - Centralization, we want to centralize as much of the "documentation" logic in the building of this format. For example, instead of letting each backend parse argument docstrings from the function docstring, we can do this here, and let the backends concentrate on pure generation.
//! - Rust centric, the format describes bindings from the Rust side, so we generate rust centric declarations. These can then freely be converted into whatever representaion necessary.
//!
//! # Writing a backend
//!
//! A backend turns a [`LadFile`] into something else, i.e. Lua language server declarations or mdbook pages.
//! - [`LadFile::types`] are the types scripts see, keyed by [`LadTypeId`]; [`LadTypeDefinition::layout`] describes their fields and variants.
//! - [`LadFile::functions`] are keyed by [`LadFunctionId`], [`LadFunction::namespace`] says whether they are global, on a type or on an enum variant.
//! - Every argument, field and return value has a [`LadTypeKind`]. Render it with an [`ArgumentVisitor`]:
//!   pick an `Output` (i.e. a type in your language, or `()` when writing into a buffer) and handle each container kind.
//!   To only find what a kind contains, use [`LadTypeKind::for_each`].
//!
//! The builder (`ladfile_builder`) does the work which would otherwise be repeated by each backend:
//! - arguments are always named, unnamed ones are `arg1`, `arg2`...
//! - arguments scripts never pass, like the function call context, are left out
//! - [`LadFunction::receiver`] says if the first argument is the value the function is called on (`v:length()` in Lua)
//! - [`LadFunctionMetadata::operator`] says which operator a function implements
//!
//! The expected outputs of the backends' tests are generated from `test_assets/test.lad.json`, run `cargo xtask bless-lad` to update all of them.

use std::{borrow::Cow, collections::HashMap};

pub use bevy_mod_scripting_bindings_domain::{
    GenericBound, ReflectionPrimitiveKind, ScriptOperatorNames,
}; // re-export the things we use
use indexmap::IndexMap;
mod plugin;
pub use plugin::*;

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

    /// The script namespaces, i.e. what `import_namespace("math")` returns.
    #[serde(skip_serializing_if = "IndexMap::is_empty", default)]
    pub namespaces: IndexMap<Cow<'static, str>, LadNamespace>,

    /// The types defined in the LAD file.
    pub types: IndexMap<LadTypeId, LadTypeDefinition>,

    /// The generic types which types in the LAD file are instantiations of, i.e. `Handle` for `Handle<Image>`.
    ///
    /// Only the generic types a LAD file builder was told about are listed, other generic types
    /// are only known by their instantiations.
    #[serde(skip_serializing_if = "IndexMap::is_empty", default)]
    pub generic_types: IndexMap<LadTypeId, LadGenericType>,

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
            namespaces: IndexMap::new(),
            types: IndexMap::new(),
            generic_types: IndexMap::new(),
            functions: IndexMap::new(),
            description: None,
        }
    }

    /// The generic type and the arguments of a type which is an instantiation of one, i.e. `Handle` and `[Image]` for `Handle<Image>`.
    pub fn get_generic_instance(
        &self,
        type_id: &LadTypeId,
    ) -> Option<(&LadGenericType, &[LadGeneric])> {
        let lad_type = self.types.get(type_id)?;
        let generic_type = self.generic_types.get(lad_type.generic_type.as_ref()?)?;
        Some((generic_type, &lad_type.generics))
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
            .or_else(|| {
                self.generic_types
                    .get(type_id)
                    .and_then(|t| t.documentation.as_deref())
            })
    }

    /// Retrieves the metadata section for the given lad type id.
    /// Primitives don't contain metadata
    pub fn get_type_metadata(&self, type_id: &LadTypeId) -> Option<&LadTypeMetadata> {
        self.types.get(type_id).map(|t| &t.metadata)
    }

    /// The types documented on their own, i.e. without the instantiations of generic types, which are documented by the generic type.
    pub fn documented_types(&self) -> impl Iterator<Item = (&LadTypeId, &LadTypeDefinition)> {
        self.types
            .iter()
            .filter(|(_, lad_type)| lad_type.generic_type.is_none())
    }

    /// The builtins, the functions every script sees as globals, i.e. `import_namespace`.
    pub fn builtin_functions(&self) -> impl Iterator<Item = (&LadFunctionId, &LadFunction)> {
        self.functions
            .iter()
            .filter(|(_, function)| function.namespace.is_global())
    }

    /// The functions on the variants of the given enum
    pub fn variant_functions<'a>(
        &'a self,
        enum_id: &'a LadTypeId,
    ) -> impl Iterator<Item = (&'a str, &'a LadFunction)> {
        self.functions
            .values()
            .filter_map(move |function| match &function.namespace {
                LadFunctionNamespace::Variant { type_id, variant } if type_id == enum_id => {
                    Some((variant.as_ref(), function))
                }
                _ => None,
            })
    }
}

/// A script namespace, what `import_namespace` returns for it.
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub struct LadNamespace {
    /// The types in the namespace by the name scripts use for them, i.e. `Vec3` in `import_namespace("math").Vec3`
    #[serde(skip_serializing_if = "IndexMap::is_empty", default)]
    pub types: IndexMap<Cow<'static, str>, LadTypeId>,
    /// The functions in the namespace by name
    #[serde(skip_serializing_if = "IndexMap::is_empty", default)]
    pub functions: IndexMap<Cow<'static, str>, LadFunctionId>,
    /// The values in the namespace by name
    #[serde(skip_serializing_if = "IndexMap::is_empty", default)]
    pub values: IndexMap<Cow<'static, str>, LadInstance>,
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
    pub type_kind: LadTypeKind,

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
    /// Whether the function is called on a value, which is its first argument, i.e. `v:length()` in Lua.
    #[serde(skip_serializing_if = "is_false", default)]
    pub receiver: bool,
    /// The generic parameter the signature mentions, if any, see [`LadTypeKind::Param`].
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub generic: Option<LadGenericParam>,
}

/// The generic parameter of a function, i.e. `A: Component` for a function returning a value of the component type it is given.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LadGenericParam {
    /// The name of the parameter, i.e. `A`
    pub name: String,
    /// What the types it stands for must be, i.e. components. A type satisfies all of them.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub bounds: Vec<GenericBound>,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
/// Additional data relevant to functions
pub struct LadFunctionMetadata {
    /// The operator the function implements, if any, i.e. `add` for `+`
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub operator: Option<ScriptOperatorNames>,
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
    /// A builtin, one of the few functions every script has as a global, i.e. `import_namespace`
    Global,
    /// A function in a script namespace, i.e. `import_namespace("bms").log_info`
    Namespace {
        /// The namespace
        namespace: Cow<'static, str>,
    },
    /// A function on a variant of an enum, i.e. `MyEnum.Variant:new{...}`.
    ///
    /// These are the functions on `EnumVariantRef` whose signatures depend on the variant, bound to it.
    Variant {
        /// The enum
        #[serde(rename = "type")]
        type_id: LadTypeId,
        /// The name of the variant
        variant: Cow<'static, str>,
    },
}

impl LadFunctionNamespace {
    /// Retruns true if the namespace is the global namespace
    pub fn is_global(&self) -> bool {
        matches!(self, LadFunctionNamespace::Global)
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
/// An argument definition used in a LAD file.
pub struct LadArgument {
    /// The kind and type of argument
    pub kind: LadTypeKind,

    /// The provided documentation for this argument. Normally derived from the function docstring.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub documentation: Option<Cow<'static, str>>,

    /// The name of the argument, always set for function arguments (`arg1`, `arg2`... if the function doesn't name them)
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
pub enum LadTypeKind {
    /// a `Ref` wrapped argument
    Ref(LadTypeId),
    /// a `Mut` wrapped argument
    Mut(LadTypeId),
    /// a `Val` wrapped argument
    Val(LadTypeId),
    /// an `Option` wrapped argument
    Option(Box<LadTypeKind>),
    /// a `Vec`
    Vec(Box<LadTypeKind>),
    /// a `HashMap`
    HashMap(Box<LadTypeKind>, Box<LadTypeKind>),
    /// a `HashSet`
    HashSet(Box<LadTypeKind>),
    /// A `InteropResult`
    InteropResult(Box<LadTypeKind>),
    /// A tuple of arguments
    Tuple(Vec<LadTypeKind>),
    /// A variadic tuple
    UntypedTuple,
    /// A list of values of the given kinds followed by any number of values of one kind
    TupleWithRest {
        /// The kinds of the first values
        head: Vec<LadTypeKind>,
        /// The kind of the values after them
        rest: Box<LadTypeKind>,
    },
    /// An array
    Array(Box<LadTypeKind>, usize),
    /// A primitive type, implementing `IntoScript` and `FromScript` natively in BMS.
    Primitive(ReflectionPrimitiveKind),
    /// A union of two or more types
    Union(Vec<LadTypeKind>),
    /// An arbitrary type which is either unsupported, doesn't contain type information, or is generally unknown.
    ///
    /// This will be the variant used for external primitives as well.
    Unknown(LadTypeId),
    /// A table of the fields of a type, keyed by field name or index, i.e. the payload of a constructor.
    ///
    /// The fields themselves are described by the type's layout.
    Fields {
        /// The type the fields belong to: a value, or a generic parameter
        of: Box<LadTypeKind>,
        /// For enums, the variant the fields belong to
        #[serde(skip_serializing_if = "Option::is_none", default)]
        variant: Option<Cow<'static, str>>,
    },
    /// A value of the type the generic parameter of the function stands for, see [`LadFunction::generic`].
    Param(String),
    /// A type itself rather than a value of it, i.e. `import_namespace("math").Vec3`, usually of a generic parameter.
    TypeOf(Box<LadTypeKind>),
    /// A generic type applied to arguments, i.e. `Handle<A>`.
    Generic {
        /// The generic type, see [`LadFile::generic_types`]
        generic_type: LadTypeId,
        /// Its arguments
        args: Vec<LadTypeKind>,
    },
}

impl LadTypeKind {
    /// The kinds this one contains, i.e. `T` in `Vec<T>`.
    pub fn children(&self) -> Vec<&LadTypeKind> {
        match self {
            LadTypeKind::Option(inner)
            | LadTypeKind::Vec(inner)
            | LadTypeKind::HashSet(inner)
            | LadTypeKind::InteropResult(inner)
            | LadTypeKind::TypeOf(inner)
            | LadTypeKind::Array(inner, _) => vec![&**inner],
            LadTypeKind::HashMap(key, value) => vec![&**key, &**value],
            LadTypeKind::Tuple(inner) | LadTypeKind::Union(inner) => inner.iter().collect(),
            LadTypeKind::TupleWithRest { head, rest } => {
                head.iter().chain(std::iter::once(&**rest)).collect()
            }
            LadTypeKind::Generic { args, .. } => args.iter().collect(),
            LadTypeKind::Fields { of, .. } => vec![&**of],
            LadTypeKind::Ref(_)
            | LadTypeKind::Mut(_)
            | LadTypeKind::Val(_)
            | LadTypeKind::Unknown(_)
            | LadTypeKind::Primitive(_)
            | LadTypeKind::UntypedTuple
            | LadTypeKind::Param(_) => vec![],
        }
    }

    /// Calls `f` on this kind and every kind nested in it, outermost first.
    pub fn for_each(&self, f: &mut impl FnMut(&LadTypeKind)) {
        f(self);
        for child in self.children() {
            child.for_each(f);
        }
    }
}

/// Something an [`ArgumentVisitor`] can visit, i.e. a type kind, or a type id on its own
#[cfg(feature = "visitor")]
pub trait LadVisitable {
    /// Runs the visitor on this
    fn accept<V: ArgumentVisitor>(&self, visitor: &mut V) -> V::Output;
}

#[cfg(feature = "visitor")]
impl LadVisitable for LadTypeId {
    fn accept<V: ArgumentVisitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit_type_id(self)
    }
}

#[cfg(feature = "visitor")]
impl LadVisitable for LadTypeKind {
    fn accept<V: ArgumentVisitor>(&self, visitor: &mut V) -> V::Output {
        visitor.visit(self)
    }
}

/// A visitor over the type of an argument, field or return value, i.e. to print it in the syntax of a language.
///
/// [`Self::visit`] dispatches on the [`LadTypeKind`], containers are handed their inner kinds to [`Self::visit`] in turn,
/// so a visitor can build its output bottom up (i.e. `Output = LuaType`), or write into a buffer (`Output = ()`).
/// Every container must be handled, so a new kind is a compile error in each visitor rather than a silent default.
///
/// To only find the type ids or kinds contained in a type, use [`LadTypeKind::for_each`] instead.
#[cfg(feature = "visitor")]
pub trait ArgumentVisitor {
    /// What visiting a type produces
    type Output;

    /// A type by its id. Ids of primitives are visited here too, check [`LadFile::primitive_kind`] to tell them apart.
    fn visit_type_id(&mut self, type_id: &LadTypeId) -> Self::Output;

    /// A primitive type
    fn visit_primitive(&mut self, primitive_kind: &ReflectionPrimitiveKind) -> Self::Output;

    /// A reference to a value of the type, by default [`Self::visit_type_id`]
    fn visit_ref(&mut self, type_id: &LadTypeId) -> Self::Output {
        self.visit_type_id(type_id)
    }

    /// A mutable reference to a value of the type, by default [`Self::visit_type_id`]
    fn visit_mut(&mut self, type_id: &LadTypeId) -> Self::Output {
        self.visit_type_id(type_id)
    }

    /// A value of the type, by default [`Self::visit_type_id`]
    fn visit_val(&mut self, type_id: &LadTypeId) -> Self::Output {
        self.visit_type_id(type_id)
    }

    /// A type converted in a way the LAD file can't describe, by default [`Self::visit_type_id`]
    fn visit_unknown(&mut self, type_id: &LadTypeId) -> Self::Output {
        self.visit_type_id(type_id)
    }

    /// A value which might also be an error, by default the value itself
    fn visit_interop_result(&mut self, inner: &LadTypeKind) -> Self::Output {
        self.visit(inner)
    }

    /// An optional value
    fn visit_option(&mut self, inner: &LadTypeKind) -> Self::Output;

    /// A list
    fn visit_vec(&mut self, inner: &LadTypeKind) -> Self::Output;

    /// A fixed size array
    fn visit_array(&mut self, inner: &LadTypeKind, size: usize) -> Self::Output;

    /// A map
    fn visit_hash_map(&mut self, key: &LadTypeKind, value: &LadTypeKind) -> Self::Output;

    /// A set
    fn visit_hash_set(&mut self, inner: &LadTypeKind) -> Self::Output;

    /// A tuple
    fn visit_tuple(&mut self, inner: &[LadTypeKind]) -> Self::Output;

    /// Any number of values of any type
    fn visit_untyped_tuple(&mut self) -> Self::Output;

    /// Values of the given kinds followed by any number of values of the `rest` kind
    fn visit_tuple_with_rest(&mut self, head: &[LadTypeKind], rest: &LadTypeKind) -> Self::Output;

    /// One of many types
    fn visit_union(&mut self, inner: &[LadTypeKind]) -> Self::Output;

    /// A table of the fields of a type (a value or a generic parameter), or of a variant of an enum
    fn visit_fields(&mut self, of: &LadTypeKind, variant: Option<&str>) -> Self::Output;

    /// A value of the type a generic parameter stands for
    fn visit_param(&mut self, param: &str) -> Self::Output;

    /// A type itself, i.e. the type of a generic parameter
    fn visit_type_of(&mut self, of: &LadTypeKind) -> Self::Output;

    /// A generic type applied to arguments
    fn visit_generic(&mut self, generic_type: &LadTypeId, args: &[LadTypeKind]) -> Self::Output;

    /// Visits a type, dispatching to the method for its kind.
    fn visit(&mut self, kind: &LadTypeKind) -> Self::Output {
        match kind {
            LadTypeKind::Ref(type_id) => self.visit_ref(type_id),
            LadTypeKind::Mut(type_id) => self.visit_mut(type_id),
            LadTypeKind::Val(type_id) => self.visit_val(type_id),
            LadTypeKind::Unknown(type_id) => self.visit_unknown(type_id),
            LadTypeKind::Primitive(primitive_kind) => self.visit_primitive(primitive_kind),
            LadTypeKind::Option(inner) => self.visit_option(inner),
            LadTypeKind::Vec(inner) => self.visit_vec(inner),
            LadTypeKind::Array(inner, size) => self.visit_array(inner, *size),
            LadTypeKind::HashMap(key, value) => self.visit_hash_map(key, value),
            LadTypeKind::HashSet(inner) => self.visit_hash_set(inner),
            LadTypeKind::InteropResult(inner) => self.visit_interop_result(inner),
            LadTypeKind::Tuple(inner) => self.visit_tuple(inner),
            LadTypeKind::UntypedTuple => self.visit_untyped_tuple(),
            LadTypeKind::TupleWithRest { head, rest } => self.visit_tuple_with_rest(head, rest),
            LadTypeKind::Union(inner) => self.visit_union(inner),
            LadTypeKind::Fields { of, variant } => self.visit_fields(of, variant.as_deref()),
            LadTypeKind::Param(param) => self.visit_param(param),
            LadTypeKind::TypeOf(of) => self.visit_type_of(of),
            LadTypeKind::Generic { generic_type, args } => self.visit_generic(generic_type, args),
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

    /// The script namespace the type lives in, i.e. what scripts pass to `import_namespace` to get at the type.
    ///
    /// This is the crate defining the type unless overridden. Empty for the root namespace, which holds types without a crate such as primitives.
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub namespace: String,

    /// The generic parameters of the type.
    ///
    /// Generics are always monomorphized in the LAD file format.
    /// Meaning that they are always assigned a concrete type.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub generics: Vec<LadGeneric>,

    /// The generic type this type is an instantiation of, see [`LadFile::generic_types`].
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub generic_type: Option<LadTypeId>,

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
    /// True if the type is a resource according to the type registry
    pub is_resource: bool,
    /// True if the type is a bundle according to the type registry
    #[serde(skip_serializing_if = "is_false", default)]
    pub is_bundle: bool,
    /// True if the type is an asset according to the type registry
    #[serde(skip_serializing_if = "is_false", default)]
    pub is_asset: bool,

    /// True if the type actually implements reflect, some types can be
    /// added as namespaces without actually implementing the Reflect trait
    pub is_reflect: bool,

    /// Set to a primitive kind if this type is mapped to a primitive
    pub mapped_to_primitive_kind: Option<ReflectionPrimitiveKind>,

    /// Set if the type has a registered `Default` implementation.
    ///
    /// Constructors use it to fill in fields missing from their payload.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub default: Option<LadDefault>,

    /// Extra metadata sections that plugins can use to serialize other information
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub misc: HashMap<String, String>,
}

/// Describes the registered `Default` implementation of a type.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LadDefault {
    /// For enums, the variant the default value is in.
    ///
    /// The default only provides fields when constructing this variant.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub variant: Option<Cow<'static, str>>,
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
    pub type_: LadTypeKind,
    /// True if the type of the field has a registered `Default` implementation.
    #[serde(skip_serializing_if = "is_false", default)]
    pub has_default: bool,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
/// A named field definition used in a LAD file.
pub struct LadNamedField {
    /// The name of the field.
    pub name: String,
    #[serde(rename = "type")]
    /// The type of the field.
    pub type_: LadTypeKind,
    /// True if the type of the field has a registered `Default` implementation.
    #[serde(skip_serializing_if = "is_false", default)]
    pub has_default: bool,
}

fn is_false(value: &bool) -> bool {
    !value
}

impl LadTypeDefinition {
    /// Whether a constructor can leave out the given field of the given variant, i.e. a default fills it in.
    ///
    /// This is the case if the field's type has a default, or if the type has a default in the same variant.
    /// `variant` is `None` for types which are not enums.
    pub fn field_has_default(&self, variant: Option<&str>, field_has_default: bool) -> bool {
        field_has_default
            || self
                .metadata
                .default
                .as_ref()
                .is_some_and(|default| default.variant.as_deref() == variant)
    }
}

/// A generic type used in a LAD file, i.e. `Handle<A>`.
///
/// Its instantiations are types of their own, which point back to it, see [`LadTypeDefinition::generic_type`].
/// The layout of the type is not described, backends treat it as opaque.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LadGenericType {
    /// The identifier of the type, without generics, i.e. `Handle`.
    pub identifier: String,

    /// The source crate of the type
    #[serde(rename = "crate", skip_serializing_if = "Option::is_none", default)]
    pub crate_: Option<String>,

    /// The full path of the type, without generics, i.e. `bevy_asset::handle::Handle`.
    pub path: String,

    /// The script namespace the type lives in, see [`LadTypeDefinition::namespace`].
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub namespace: String,

    /// The names of the generic parameters, i.e. `["A"]`.
    pub params: Vec<String>,

    /// The documentation describing the type.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub documentation: Option<String>,
}

impl LadGenericType {
    /// The type with its parameters, i.e. `Handle<A>`
    pub fn name_with_params(&self) -> String {
        format!("{}<{}>", self.identifier, self.params.join(", "))
    }
}

/// The generic argument of an instantiation of a generic type.
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
