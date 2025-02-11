//! Parsing definitions for the LAD (Language Agnostic Decleration) file format.

use bevy_mod_scripting_core::{
    bindings::{function::script_function::FunctionCallContext, ReflectReference},
    docgen::{
        info::FunctionInfo,
        typed_through::{ThroughTypeInfo, TypedWrapperKind, UntypedWrapperKind},
    },
    match_by_type,
};
use bevy_reflect::{
    func::{DynamicFunction, DynamicFunctionMut},
    NamedField, Reflect, TypeInfo, TypeRegistry, Typed, UnnamedField,
};
use indexmap::IndexMap;
use std::{any::TypeId, borrow::Cow, collections::HashMap, ffi::OsString, path::PathBuf};

const LAD_VERSION: &str = "0.1.0";

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
/// A Language Agnostic Declaration (LAD) file.
pub struct LadFile {
    /// The version of the LAD file format used.
    pub version: Cow<'static, str>,

    /// The types defined in the LAD file.
    pub types: IndexMap<LadTypeId, LadType>,

    /// The functions defined in the LAD file.
    pub functions: IndexMap<LadFunctionId, LadFunction>,

    /// A mapping from type ids to primitive types
    pub primitives: IndexMap<LadTypeId, LadBMSPrimitiveType>,
}

impl LadFile {
    /// Create a new empty LAD file.
    pub fn new() -> Self {
        Self {
            version: LAD_VERSION.into(),
            types: IndexMap::new(),
            functions: IndexMap::new(),
            primitives: IndexMap::new(),
        }
    }
}

impl Default for LadFile {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
/// A unique identifier for a function in a LAD file.
/// Only unique within the LAD file.
pub struct LadFunctionId(String);

impl LadFunctionId {
    /// Create a new LAD function id with a string.
    pub fn new_string_id(function_id: String) -> Self {
        LadFunctionId(function_id)
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
/// A function definition used in a LAD file.
pub struct LadFunction {
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
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, Reflect)]
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

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
/// A unique identifier for a type in a LAD file.
///
/// Only guaranteed to be unique within the LAD file.
/// It *might* be unique across LAD files, but this is not guaranteed and depends on the type itself.
pub struct LadTypeId(Cow<'static, str>);

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
    #[serde(rename = "type")]
    type_: LadTypeId,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
/// A named field definition used in a LAD file.
pub struct LadNamedField {
    name: String,
    #[serde(rename = "type")]
    type_: LadTypeId,
}

/// A generic type definition used in a LAD file.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LadGeneric {
    /// The id of the type assigned to this generic.
    pub type_id: LadTypeId,
    /// The name of the generic
    pub name: String,
}

/// A builder for constructing LAD files.
/// This should be your preferred way of constructing LAD files.
pub struct LadFileBuilder<'t> {
    file: LadFile,
    type_id_mapping: HashMap<TypeId, LadTypeId>,
    type_registry: &'t TypeRegistry,
    sorted: bool,
}

impl<'t> LadFileBuilder<'t> {
    /// Create a new LAD file builder loaded with primitives.
    pub fn new(type_registry: &'t TypeRegistry) -> Self {
        let mut builder = Self {
            file: LadFile::new(),
            type_id_mapping: HashMap::new(),
            type_registry,
            sorted: false,
        };

        builder
            .add_bms_primitive::<bool>("A boolean value")
            .add_bms_primitive::<isize>("A signed pointer-sized integer")
            .add_bms_primitive::<i8>("A signed 8-bit integer")
            .add_bms_primitive::<i16>("A signed 16-bit integer")
            .add_bms_primitive::<i32>("A signed 32-bit integer")
            .add_bms_primitive::<i64>("A signed 64-bit integer")
            .add_bms_primitive::<i128>("A signed 128-bit integer")
            .add_bms_primitive::<usize>("An unsigned pointer-sized integer")
            .add_bms_primitive::<u8>("An unsigned 8-bit integer")
            .add_bms_primitive::<u16>("An unsigned 16-bit integer")
            .add_bms_primitive::<u32>("An unsigned 32-bit integer")
            .add_bms_primitive::<u64>("An unsigned 64-bit integer")
            .add_bms_primitive::<u128>("An unsigned 128-bit integer")
            .add_bms_primitive::<f32>("A 32-bit floating point number")
            .add_bms_primitive::<f64>("A 64-bit floating point number")
            .add_bms_primitive::<char>("An 8-bit character")
            .add_bms_primitive::<&'static str>("A static string slice")
            .add_bms_primitive::<String>("A heap allocated string")
            .add_bms_primitive::<OsString>("A heap allocated OS string")
            .add_bms_primitive::<PathBuf>("A heap allocated file path")
            .add_bms_primitive::<FunctionCallContext>("Function call context, if accepted by a function, means the function can access the world in arbitrary ways.")
            .add_bms_primitive::<DynamicFunction>("A callable dynamic function")
            .add_bms_primitive::<DynamicFunctionMut>("A stateful and callable dynamic function")
            .add_bms_primitive::<ReflectReference>("A reference to a reflectable type");

        builder
    }

    /// Set whether the LAD file should be sorted at build time.
    pub fn set_sorted(&mut self, sorted: bool) -> &mut Self {
        self.sorted = sorted;
        self
    }

    /// Add a BMS primitive to the LAD file.
    /// Will do nothing if the type is not a BMS primitive.
    pub fn add_bms_primitive<T: 'static>(
        &mut self,
        docs: impl Into<Cow<'static, str>>,
    ) -> &mut Self {
        let type_id = self.lad_id_from_type_id(TypeId::of::<T>());
        let kind = match Self::lad_primitive_type_from_type_id(TypeId::of::<T>()) {
            Some(primitive) => primitive,
            None => return self,
        };
        self.file.primitives.insert(
            type_id,
            LadBMSPrimitiveType {
                kind,
                documentation: docs.into(),
            },
        );
        self
    }

    /// Add a type definition to the LAD file.
    ///
    /// Equivalent to calling [`Self::add_type_info`] with `T::type_info()`.
    pub fn add_type<T: Typed>(&mut self) -> &mut Self {
        self.add_type_info(T::type_info());
        self
    }

    /// Add a type definition to the LAD file.
    /// Will overwrite any existing type definitions with the same type id.
    pub fn add_type_info(&mut self, type_info: &TypeInfo) -> &mut Self {
        let type_id = self.lad_id_from_type_id(type_info.type_id());
        let lad_type = LadType {
            identifier: type_info
                .type_path_table()
                .ident()
                .unwrap_or_default()
                .to_string(),
            generics: type_info
                .generics()
                .iter()
                .map(|param| LadGeneric {
                    type_id: self.lad_id_from_type_id(param.type_id()),
                    name: param.name().to_string(),
                })
                .collect(),
            documentation: type_info.docs().map(|s| s.to_string()),
            associated_functions: Vec::new(),
            crate_: type_info
                .type_path_table()
                .crate_name()
                .map(|s| s.to_owned()),
            path: type_info.type_path_table().path().to_owned(),
            layout: self.lad_layout_from_type_info(type_info),
        };
        self.file.types.insert(type_id, lad_type);
        self
    }

    /// Add a function definition to the LAD file.
    /// Will overwrite any existing function definitions with the same function id.
    pub fn add_function_info(&mut self, function_info: FunctionInfo) -> &mut Self {
        let function_id = self.lad_function_id_from_info(&function_info);
        let lad_function = LadFunction {
            identifier: function_info.name,
            arguments: function_info
                .arg_info
                .into_iter()
                .map(|arg| {
                    let kind = match &arg.type_info {
                        Some(through_type) => {
                            self.lad_argument_type_from_through_type(through_type)
                        }
                        None => LadArgumentKind::Unknown(self.lad_id_from_type_id(arg.type_id)),
                    };
                    LadArgument {
                        kind,
                        name: arg.name,
                    }
                })
                .collect(),
            return_type: self.lad_id_from_type_id(function_info.return_info.type_id),
            documentation: function_info.docs,
        };
        self.file.functions.insert(function_id, lad_function);
        self
    }

    /// Build the finalized and optimized LAD file.
    pub fn build(&mut self) -> LadFile {
        let mut file = std::mem::replace(&mut self.file, LadFile::new());
        if self.sorted {
            file.types.sort_keys();
            file.functions.sort_keys();
            file.primitives.sort_keys();
        }

        file
    }

    fn variant_identifier_for_non_enum(type_info: &TypeInfo) -> Cow<'static, str> {
        type_info
            .type_path_table()
            .ident()
            .unwrap_or_else(|| type_info.type_path_table().path())
            .into()
    }

    fn struct_variant_from_named_fields<'a, I: Iterator<Item = &'a NamedField>>(
        &mut self,
        name: Cow<'static, str>,
        fields: I,
    ) -> LadVariant {
        LadVariant::Struct {
            name,
            fields: fields
                .map(|field| LadNamedField {
                    name: field.name().to_string(),
                    type_: self.lad_id_from_type_id(field.type_id()),
                })
                .collect(),
        }
    }

    fn tuple_struct_variant_from_fields<'a, I: Iterator<Item = &'a UnnamedField>>(
        &mut self,
        name: Cow<'static, str>,
        fields: I,
    ) -> LadVariant {
        LadVariant::TupleStruct {
            name,
            fields: fields
                .map(|field| LadField {
                    type_: self.lad_id_from_type_id(field.type_id()),
                })
                .collect(),
        }
    }

    fn lad_layout_from_type_info(&mut self, type_info: &TypeInfo) -> LadTypeLayout {
        match type_info {
            TypeInfo::Struct(struct_info) => {
                let fields = (0..struct_info.field_len()).filter_map(|i| struct_info.field_at(i));

                LadTypeLayout::MonoVariant(self.struct_variant_from_named_fields(
                    Self::variant_identifier_for_non_enum(type_info),
                    fields,
                ))
            }
            TypeInfo::TupleStruct(tuple_struct_info) => {
                let fields = (0..tuple_struct_info.field_len())
                    .filter_map(|i| tuple_struct_info.field_at(i));

                LadTypeLayout::MonoVariant(self.tuple_struct_variant_from_fields(
                    Self::variant_identifier_for_non_enum(type_info),
                    fields,
                ))
            }
            TypeInfo::Enum(enum_info) => {
                let mut variants = Vec::new();
                for i in 0..enum_info.variant_len() {
                    if let Some(variant) = enum_info.variant_at(i) {
                        let variant_name = variant.name();
                        let variant = match variant {
                            bevy_reflect::VariantInfo::Struct(struct_variant_info) => {
                                let fields = (0..struct_variant_info.field_len())
                                    .filter_map(|i| struct_variant_info.field_at(i));

                                self.struct_variant_from_named_fields(variant_name.into(), fields)
                            }
                            bevy_reflect::VariantInfo::Tuple(tuple_variant_info) => {
                                let fields = (0..tuple_variant_info.field_len())
                                    .filter_map(|i| tuple_variant_info.field_at(i));

                                self.tuple_struct_variant_from_fields(variant_name.into(), fields)
                            }
                            bevy_reflect::VariantInfo::Unit(_) => LadVariant::Unit {
                                name: variant_name.into(),
                            },
                        };
                        variants.push(variant);
                    }
                }
                LadTypeLayout::Enum(variants)
            }
            _ => LadTypeLayout::Opaque,
        }
    }

    fn lad_id_from_type_id(&mut self, type_id: TypeId) -> LadTypeId {
        if let Some(lad_id) = self.type_id_mapping.get(&type_id) {
            return lad_id.clone();
        }
        let new_id = match self.type_registry.get_type_info(type_id) {
            Some(info) => info.type_path_table().path().to_owned(),
            None => format!("{type_id:?}"),
        };

        let lad_id = LadTypeId::new_string_id(new_id.into());
        self.type_id_mapping.insert(type_id, lad_id.clone());
        lad_id
    }

    fn lad_function_id_from_info(&mut self, function_info: &FunctionInfo) -> LadFunctionId {
        let namespace_string = match function_info.namespace {
            bevy_mod_scripting_core::bindings::function::namespace::Namespace::Global => {
                "".to_string()
            }
            bevy_mod_scripting_core::bindings::function::namespace::Namespace::OnType(type_id) => {
                self.lad_id_from_type_id(type_id).0.to_string()
            }
        };

        LadFunctionId::new_string_id(format!("{}::{}", namespace_string, function_info.name))
    }

    /// We can assume that the types here will be either primitives
    /// or reflect types, as the rest will be covered by typed wrappers
    /// so just check
    fn lad_primitive_type_from_type_id(type_id: TypeId) -> Option<LadBMSPrimitiveKind> {
        match_by_type!(match type_id {
            i: bool => return Some(LadBMSPrimitiveKind::Bool),
            i: isize => return Some(LadBMSPrimitiveKind::Isize),
            i: i8 => return Some(LadBMSPrimitiveKind::I8),
            i: i16 => return Some(LadBMSPrimitiveKind::I16),
            i: i32 => return Some(LadBMSPrimitiveKind::I32),
            i: i64 => return Some(LadBMSPrimitiveKind::I64),
            i: i128 => return Some(LadBMSPrimitiveKind::I128),
            i: usize => return Some(LadBMSPrimitiveKind::Usize),
            i: u8 => return Some(LadBMSPrimitiveKind::U8),
            i: u16 => return Some(LadBMSPrimitiveKind::U16),
            i: u32 => return Some(LadBMSPrimitiveKind::U32),
            i: u64 => return Some(LadBMSPrimitiveKind::U64),
            i: u128 => return Some(LadBMSPrimitiveKind::U128),
            i: f32 => return Some(LadBMSPrimitiveKind::F32),
            i: f64 => return Some(LadBMSPrimitiveKind::F64),
            i: char => return Some(LadBMSPrimitiveKind::Char),
            i: &'static str => return Some(LadBMSPrimitiveKind::Str),
            i: String => return Some(LadBMSPrimitiveKind::String),
            i: OsString => return Some(LadBMSPrimitiveKind::OsString),
            i: PathBuf => return Some(LadBMSPrimitiveKind::PathBuf),
            i: FunctionCallContext => return Some(LadBMSPrimitiveKind::FunctionCallContext),
            i: DynamicFunction => return Some(LadBMSPrimitiveKind::DynamicFunction),
            i: DynamicFunctionMut => return Some(LadBMSPrimitiveKind::DynamicFunctionMut),
            i: ReflectReference => return Some(LadBMSPrimitiveKind::ReflectReference)
        });

        None
    }

    fn lad_argument_type_from_through_type(
        &mut self,
        through_type: &ThroughTypeInfo,
    ) -> LadArgumentKind {
        match through_type {
            ThroughTypeInfo::UntypedWrapper {
                through_type,
                wrapper_kind,
                ..
            } => match wrapper_kind {
                UntypedWrapperKind::Ref => {
                    LadArgumentKind::Ref(self.lad_id_from_type_id(through_type.type_id()))
                }
                UntypedWrapperKind::Mut => {
                    LadArgumentKind::Mut(self.lad_id_from_type_id(through_type.type_id()))
                }
                UntypedWrapperKind::Val => {
                    LadArgumentKind::Val(self.lad_id_from_type_id(through_type.type_id()))
                }
            },
            ThroughTypeInfo::TypedWrapper(typed_wrapper_kind) => match typed_wrapper_kind {
                TypedWrapperKind::Vec(through_type_info) => LadArgumentKind::Vec(Box::new(
                    self.lad_argument_type_from_through_type(through_type_info),
                )),
                TypedWrapperKind::HashMap(through_type_info, through_type_info1) => {
                    LadArgumentKind::HashMap(
                        Box::new(self.lad_argument_type_from_through_type(through_type_info)),
                        Box::new(self.lad_argument_type_from_through_type(through_type_info1)),
                    )
                }
                TypedWrapperKind::Array(through_type_info, size) => LadArgumentKind::Array(
                    Box::new(self.lad_argument_type_from_through_type(through_type_info)),
                    *size,
                ),
                TypedWrapperKind::Option(through_type_info) => LadArgumentKind::Option(Box::new(
                    self.lad_argument_type_from_through_type(through_type_info),
                )),
                TypedWrapperKind::InteropResult(through_type_info) => {
                    LadArgumentKind::InteropResult(Box::new(
                        self.lad_argument_type_from_through_type(through_type_info),
                    ))
                }
                TypedWrapperKind::Tuple(through_type_infos) => LadArgumentKind::Tuple(
                    through_type_infos
                        .iter()
                        .map(|through_type_info| {
                            self.lad_argument_type_from_through_type(through_type_info)
                        })
                        .collect(),
                ),
            },
            ThroughTypeInfo::TypeInfo(type_info) => {
                match Self::lad_primitive_type_from_type_id(type_info.type_id()) {
                    Some(primitive) => LadArgumentKind::Primitive(primitive),
                    None => LadArgumentKind::Unknown(self.lad_id_from_type_id(type_info.type_id())),
                }
            }
        }
    }
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

#[cfg(test)]
mod test {
    use bevy_mod_scripting_core::{
        bindings::function::{
            from::Ref,
            namespace::{GlobalNamespace, IntoNamespace},
        },
        docgen::info::GetFunctionInfo,
    };
    use bevy_reflect::Typed;

    use super::*;

    /// Set to true to put output into test_assets.
    const BLESS_TEST_FILE: bool = true;

    #[test]
    fn test_empty_lad_file_serializes_correctly() {
        let lad_file = LadFile::new();
        let serialized = serialize_lad_file(&lad_file, false).unwrap();
        let deserialized = parse_lad_file(&serialized).unwrap();
        assert_eq!(lad_file, deserialized);
        assert_eq!(deserialized.version, LAD_VERSION);
    }

    #[test]
    fn test_serializes_as_expected() {
        let mut type_registry = TypeRegistry::default();
        type_registry.register::<ReflectReference>();
        type_registry.register::<FunctionCallContext>();

        #[derive(Reflect)]
        /// I am a struct
        struct StructType<T> {
            /// hello from field
            field: usize,
            /// hello from field 2
            field2: T,
        }

        #[derive(Reflect)]
        /// I am a unit test type
        struct UnitType;

        #[derive(Reflect)]
        /// I am a tuple test type
        struct TupleStructType(pub usize, #[doc = "hello"] pub String);

        #[derive(Reflect)]
        enum EnumType {
            /// hello from variant
            Unit,
            /// hello from variant 2
            Struct {
                /// hello from field
                field: usize,
            },
            /// hello from variant 3
            TupleStruct(usize, #[doc = "asd"] String),
        }

        type_registry.register::<StructType<usize>>();
        type_registry.register::<UnitType>();
        type_registry.register::<TupleStructType>();
        type_registry.register::<EnumType>();

        let function = |_: ReflectReference, _: usize| 2usize;
        let function_info = function
            .get_function_info("hello_world".into(), StructType::<usize>::into_namespace())
            .with_docs("hello docs");

        let function_with_complex_args =
            |_: ReflectReference, _: (usize, String), _: Option<Vec<Ref<EnumType>>>| 2usize;
        let function_with_complex_args_info = function_with_complex_args
            .get_function_info("hello_world".into(), StructType::<usize>::into_namespace())
            .with_arg_names(&["ref_", "tuple", "option_vec_ref_wrapper"]);

        let global_function = |_: usize| 2usize;
        let global_function_info = global_function
            .get_function_info("hello_world".into(), GlobalNamespace::into_namespace())
            .with_arg_names(&["arg1"]);

        let lad_file = LadFileBuilder::new(&type_registry)
            .set_sorted(true)
            .add_function_info(function_info)
            .add_function_info(global_function_info)
            .add_function_info(function_with_complex_args_info)
            .add_type::<StructType<usize>>()
            .add_type::<UnitType>()
            .add_type::<TupleStructType>()
            .add_type_info(EnumType::type_info())
            .build();
        let serialized = serialize_lad_file(&lad_file, true).unwrap();

        if BLESS_TEST_FILE {
            let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
            let path_to_test_assets = std::path::Path::new(&manifest_dir).join("test_assets");

            println!("Blessing test file at {:?}", path_to_test_assets);
            std::fs::write(path_to_test_assets.join("test.lad.json"), &serialized).unwrap();
            return;
        }

        let expected = include_str!("../test_assets/test.lad.json");

        assert_eq!(
            serialized.trim(),
            expected.trim(),
            "Expected:---\n {}\n---\nGot: ---\n{}\n---",
            expected,
            serialized
        );
    }

    #[test]
    fn test_asset_deserializes_correctly() {
        let asset = include_str!("../test_assets/test.lad.json");
        let deserialized = parse_lad_file(asset).unwrap();
        assert_eq!(deserialized.version, LAD_VERSION);
    }
}
