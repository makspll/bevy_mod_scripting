//! Parsing definitions for the LAD (Language Agnostic Decleration) file format.

use bevy_mod_scripting_core::{
    bindings::{
        function::{namespace::Namespace, script_function::FunctionCallContext},
        ReflectReference,
    },
    docgen::{
        info::FunctionInfo,
        typed_through::{ThroughTypeInfo, TypedWrapperKind, UntypedWrapperKind},
    },
    match_by_type,
};
use bevy_reflect::{
    func::{DynamicFunction, DynamicFunctionMut},
    NamedField, TypeInfo, TypeRegistry, Typed, UnnamedField,
};
use ladfile::*;
use std::{any::TypeId, borrow::Cow, collections::HashMap, ffi::OsString, path::PathBuf};

/// We can assume that the types here will be either primitives
/// or reflect types, as the rest will be covered by typed wrappers
/// so just check
fn primitive_from_type_id(type_id: TypeId) -> Option<LadBMSPrimitiveKind> {
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
        i: str => return Some(LadBMSPrimitiveKind::Str),
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
            .add_bms_primitive::<&'static str>("A string slice")
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
        let kind = match primitive_from_type_id(TypeId::of::<T>()) {
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

    /// Add a global instance to the LAD file.
    ///
    /// Requires the type to be registered via [`Self::add_type`] or [`Self::add_type_info`] first to provide rich type information.
    ///
    /// If `is_static` is true, the instance will be treated as a static instance
    /// and hence not support method call syntax or method calls (i.e. only functions without a self parameter can be called on them).
    pub fn add_instance<T: 'static>(
        &mut self,
        key: impl Into<Cow<'static, str>>,
        is_static: bool,
    ) -> &mut Self {
        let type_id = self.lad_id_from_type_id(TypeId::of::<T>());
        self.file
            .globals
            .insert(key.into(), LadInstance { type_id, is_static });
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
            namespace: match function_info.namespace {
                Namespace::Global => LadFunctionNamespace::Global,
                Namespace::OnType(type_id) => {
                    LadFunctionNamespace::Type(self.lad_id_from_type_id(type_id))
                }
            },
        };
        self.file.functions.insert(function_id, lad_function);
        self
    }

    /// Set the markdown description of the LAD file.
    pub fn set_description(&mut self, description: impl Into<String>) -> &mut Self {
        self.file.description = Some(description.into());
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

        // associate functions on type namespaces with their types
        for (function_id, function) in file.functions.iter() {
            match &function.namespace {
                LadFunctionNamespace::Type(type_id) => {
                    if let Some(t) = file.types.get_mut(type_id) {
                        t.associated_functions.push(function_id.clone());
                    }
                }
                LadFunctionNamespace::Global => {}
            }
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

        let new_id = match primitive_from_type_id(type_id) {
            Some(primitive) => primitive.lad_type_id(),
            None => {
                if let Some(info) = self.type_registry.get_type_info(type_id) {
                    LadTypeId::new_string_id(info.type_path_table().path().into())
                } else {
                    LadTypeId::new_string_id(format!("{type_id:?}").into())
                }
            }
        };

        self.type_id_mapping.insert(type_id, new_id.clone());
        new_id
    }

    fn lad_function_id_from_info(&mut self, function_info: &FunctionInfo) -> LadFunctionId {
        let namespace_string = match function_info.namespace {
            bevy_mod_scripting_core::bindings::function::namespace::Namespace::Global => {
                "".to_string()
            }
            bevy_mod_scripting_core::bindings::function::namespace::Namespace::OnType(type_id) => {
                self.lad_id_from_type_id(type_id).to_string()
            }
        };

        LadFunctionId::new_string_id(format!("{}::{}", namespace_string, function_info.name))
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
                match primitive_from_type_id(type_info.type_id()) {
                    Some(primitive) => LadArgumentKind::Primitive(primitive),
                    None => LadArgumentKind::Unknown(self.lad_id_from_type_id(type_info.type_id())),
                }
            }
        }
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
    use bevy_reflect::Reflect;

    use super::*;

    /// Set to true to put output into test_assets.
    const BLESS_TEST_FILE: bool = false;

    /// normalize line endings etc..
    fn normalize_file(file: &mut String) {
        *file = file.replace("\r\n", "\n");
    }

    #[test]
    fn test_empty_lad_file_serializes_correctly() {
        let lad_file = LadFile::new();
        let serialized = serialize_lad_file(&lad_file, false).unwrap();
        let deserialized = parse_lad_file(&serialized).unwrap();
        assert_eq!(lad_file, deserialized);
        assert_eq!(deserialized.version, ladfile::LAD_VERSION);
    }

    #[test]
    fn test_serializes_as_expected() {
        let mut type_registry = TypeRegistry::default();

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

        let mut lad_file = LadFileBuilder::new(&type_registry)
            .set_description("## Hello gentlemen\n I am  markdown file.\n - hello\n - world")
            .set_sorted(true)
            .add_function_info(function_info)
            .add_function_info(global_function_info)
            .add_function_info(function_with_complex_args_info)
            .add_type::<StructType<usize>>()
            .add_type::<UnitType>()
            .add_type::<TupleStructType>()
            .add_type_info(EnumType::type_info())
            .add_instance::<StructType<usize>>("my_static_instance", true)
            .add_instance::<UnitType>("my_non_static_instance", false)
            .build();

        // normalize the version so we don't have to update it every time
        lad_file.version = "{{version}}".into();
        let mut serialized = serialize_lad_file(&lad_file, true).unwrap();

        normalize_file(&mut serialized);

        if BLESS_TEST_FILE {
            let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
            let path_to_test_assets = std::path::Path::new(&manifest_dir).join("test_assets");

            println!("Blessing test file at {:?}", path_to_test_assets);
            std::fs::write(path_to_test_assets.join("test.lad.json"), &serialized).unwrap();
            return;
        }

        let mut expected = include_str!("../test_assets/test.lad.json").to_owned();
        normalize_file(&mut expected);

        pretty_assertions::assert_eq!(serialized.trim(), expected.trim(),);
    }

    #[test]
    fn test_asset_deserializes_correctly() {
        let asset = include_str!("../test_assets/test.lad.json");
        let deserialized = parse_lad_file(asset).unwrap();
        assert_eq!(deserialized.version, "{{version}}");
    }
}
