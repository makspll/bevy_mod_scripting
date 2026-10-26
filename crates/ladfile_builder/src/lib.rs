//! Parsing definitions for the LAD (Language Agnostic Decleration) file format.
pub mod plugin;

use bevy_ecs::{
    reflect::{ReflectBundle, ReflectComponent, ReflectResource},
    world::World,
};
use bevy_log::warn;
use bevy_mod_scripting_bindings::{
    MarkAsCore, MarkAsGenerated, MarkAsSignificant, ScriptNamespace, ScriptValue,
    docgen::{
        TypedThrough,
        info::FunctionInfo,
        typed_through::{BoundPlaceholder, ThroughTypeInfo, primitive_kind_of, primitive_type_id},
    },
    function::{
        bound::{asset_handle_type_info, is_asset_type},
        generics::A,
        namespace::FunctionScope,
        script_function::FunctionCallContext,
    },
    into_through_type_info_with_registry,
};
pub use bevy_mod_scripting_bindings_domain::*; // re-export the thing we use
use bevy_platform::collections::{HashMap, HashSet};
use bevy_reflect::{
    NamedField, ReflectRef, TypeInfo, TypeRegistry, Typed, UnnamedField, enums::VariantInfo,
    std_traits::ReflectDefault,
};
use ladfile::*;
use std::{
    any::TypeId,
    borrow::Cow,
    cmp::{max, min},
};

/// Whether the type stands for a generic parameter when declaring a generic type, see [`LadFileBuilder::add_generic_type`].
fn is_param_marker(type_id: TypeId) -> bool {
    type_id == TypeId::of::<A>()
}

/// Whether the type path mentions a marker standing for a generic parameter, i.e. `Arc<A>`.
fn mentions_param_marker(type_path: &str) -> bool {
    type_path.contains(<A as bevy_reflect::TypePath>::type_path())
}

/// Collects the generic types the kind applies to arguments, i.e. `Handle` for `Vec<Handle<T>>`.
fn named_generic_types(kind: &LadTypeKind, named: &mut Vec<LadTypeId>) {
    kind.for_each(&mut |kind| {
        if let LadTypeKind::Ref(LadNamedType::Generic { generic_type, .. })
        | LadTypeKind::Mut(LadNamedType::Generic { generic_type, .. })
        | LadTypeKind::Val(LadNamedType::Generic { generic_type, .. }) = kind
        {
            named.push(generic_type.clone());
        }
    })
}

/// The kinds of the fields of each variant, in order.
fn field_kinds(layout: &LadTypeLayout) -> Vec<(&str, Vec<&LadTypeKind>)> {
    let variants = match layout {
        LadTypeLayout::Opaque => return vec![],
        LadTypeLayout::MonoVariant(variant) => std::slice::from_ref(variant),
        LadTypeLayout::Enum(variants) => variants.as_slice(),
    };
    variants
        .iter()
        .map(|variant| match variant {
            LadVariant::TupleStruct { name, fields } => {
                (name.as_ref(), fields.iter().map(|f| &f.type_).collect())
            }
            LadVariant::Struct { name, fields } => {
                (name.as_ref(), fields.iter().map(|f| &f.type_).collect())
            }
            LadVariant::Unit { name } => (name.as_ref(), vec![]),
        })
        .collect()
}

/// A builder for constructing LAD files.
/// This should be your preferred way of constructing LAD files.
pub struct LadFileBuilder<'t> {
    file: LadFile,
    type_id_mapping: HashMap<TypeId, LadTypeId>,
    type_registry: &'t TypeRegistry,
    sorted: bool,
    exclude_types_involving_unregistered_types: bool,
    generic_types: Vec<GenericTypeSpec>,
    /// The names of the parameters of the generic type whose layout is being described, by the marker standing for each,
    /// see [`LadFileBuilder::add_generic_type`]
    describing_params: HashMap<TypeId, String>,
}

/// A generic type declared with [`LadFileBuilder::add_generic_type`]
struct GenericTypeSpec {
    /// The path of the type, without generics
    path: String,
    /// The identifier of the type, without generics
    identifier: String,
    /// The crate defining the type
    crate_: Option<String>,
    /// The names of the parameters
    params: Vec<String>,
    documentation: String,
    /// The type instantiated with markers, which the layout is described from
    type_info: &'static TypeInfo,
    /// The names of the parameters, by the marker standing for each
    markers: HashMap<TypeId, String>,
    /// The layout, in terms of the parameters, described once all generic types are declared, see [`LadFileBuilder::generic_layout`]
    layout: Option<LadTypeLayout>,
}

impl<'t> LadFileBuilder<'t> {
    /// Create a new LAD file builder without default primitives.
    pub fn new_empty(type_registry: &'t TypeRegistry) -> Self {
        Self {
            file: LadFile::new(),
            type_id_mapping: HashMap::new(),
            type_registry,
            sorted: false,
            exclude_types_involving_unregistered_types: false,
            generic_types: Vec::new(),
            describing_params: HashMap::new(),
        }
    }

    /// Create a new LAD file builder loaded with primitives.
    pub fn new(type_registry: &'t TypeRegistry) -> Self {
        use ReflectionPrimitiveKind::*;
        let mut builder = Self::new_empty(type_registry);

        builder
            .add_bms_primitive(Bool,"A boolean value")
            .add_bms_primitive(Isize, "A signed pointer-sized integer")
            .add_bms_primitive(I8, "A signed 8-bit integer")
            .add_bms_primitive(I16, "A signed 16-bit integer")
            .add_bms_primitive(I32, "A signed 32-bit integer")
            .add_bms_primitive(I64, "A signed 64-bit integer")
            .add_bms_primitive(I128, "A signed 128-bit integer")
            .add_bms_primitive(Usize, "An unsigned pointer-sized integer")
            .add_bms_primitive(U8, "An unsigned 8-bit integer")
            .add_bms_primitive(U16, "An unsigned 16-bit integer")
            .add_bms_primitive(U32, "An unsigned 32-bit integer")
            .add_bms_primitive(U64, "An unsigned 64-bit integer")
            .add_bms_primitive(U128, "An unsigned 128-bit integer")
            .add_bms_primitive(F32, "A 32-bit floating point number")
            .add_bms_primitive(F64, "A 64-bit floating point number")
            .add_bms_primitive(Char, "An 8-bit character")
            .add_bms_primitive(Str, "A string slice")
            .add_bms_primitive(String, "A heap allocated string")
            .add_bms_primitive(OsString, "A heap allocated OS string")
            .add_bms_primitive(PathBuf, "A heap allocated file path")
            .add_bms_primitive(FunctionCallContext, "Function call context, if accepted by a function, means the function can access the world in arbitrary ways.")
            .add_bms_primitive(DynamicFunction, "A callable dynamic function")
            .add_bms_primitive(DynamicFunctionMut, "A stateful and callable dynamic function")
            .add_bms_primitive(ScriptValue, "A value representing the union of all representable values")
            .add_bms_primitive(ReflectReference, "A reference to a reflectable type")
            .add_bms_primitive(StaticTypeRef, "A type itself, used to reach its static members, i.e. `Vec3` in `Vec3.new{}`. Functions on it are shared by all types");

        builder
    }

    /// Set whether types involving unregistered types should be excluded.
    /// I.e. `HashMap<T, V>` with T or V not being registered will be excluded.
    pub fn set_exclude_including_unregistered(&mut self, exclude: bool) -> &mut Self {
        self.exclude_types_involving_unregistered_types = exclude;
        self
    }

    /// Set whether the LAD file should be sorted at build time.
    pub fn set_sorted(&mut self, sorted: bool) -> &mut Self {
        self.sorted = sorted;
        self
    }

    /// Add a BMS primitive to the LAD file.
    /// Will do nothing if the type is not a BMS primitive.
    pub fn add_bms_primitive(
        &mut self,
        primitive: ReflectionPrimitiveKind,
        docs: impl Into<Cow<'static, str>>,
    ) -> &mut Self {
        let type_ident = primitive.to_string();
        let lad_type_id = LadTypeId::new_string_id(type_ident.clone().into());
        if let Some(type_id) = primitive_type_id(&primitive) {
            self.type_id_mapping.insert(type_id, lad_type_id.clone());
        }
        self.file.types.insert(
            lad_type_id,
            LadTypeDefinition {
                identifier: type_ident.clone(),
                crate_: None,
                namespace: String::new(),
                path: type_ident,
                generics: vec![],
                generic_type: None,
                documentation: Some(docs.into().to_string()),
                associated_functions: vec![],
                layout: LadTypeLayout::Opaque,
                generated: false,
                insignificance: default_importance(),
                metadata: LadTypeMetadata {
                    is_component: false,
                    is_resource: false,
                    is_bundle: false,
                    is_asset: false,
                    is_reflect: false,
                    mapped_to_primitive_kind: Some(primitive),
                    default: None,
                    misc: Default::default(),
                },
            },
        );
        self
    }

    /// Mark a type as generated.
    pub fn mark_generated(&mut self, type_id: TypeId) -> &mut Self {
        let type_id = self.lad_id_from_type_id(type_id);
        if let Some(t) = self.file.types.get_mut(&type_id) {
            t.generated = true;
        }
        self
    }

    /// Set the insignificance value of a type.
    pub fn set_insignificance(&mut self, type_id: TypeId, importance: usize) -> &mut Self {
        let type_id = self.lad_id_from_type_id(type_id);
        if let Some(t) = self.file.types.get_mut(&type_id) {
            t.insignificance = importance;
        }
        self
    }

    /// Adds a value of the given type to a script namespace, see [`Self::add_namespace_value`].
    pub fn add_namespace_value_of<T: 'static + TypedThrough>(
        &mut self,
        namespace: impl Into<Cow<'static, str>>,
        name: impl Into<Cow<'static, str>>,
    ) -> &mut Self {
        self.add_namespace_value(namespace, name, &T::through_type_info())
    }

    /// Adds a type which does not implement reflect to the list of types.
    pub fn add_nonreflect_type<T: 'static>(
        &mut self,
        crate_: Option<&str>,
        documentation: &str,
    ) -> &mut Self {
        let path = std::any::type_name::<T>().to_string();
        let identifier = path
            .split("::")
            .last()
            .map(|o| o.to_owned())
            .unwrap_or_else(|| path.clone());

        let lad_type_id = self.lad_id_from_type_id(std::any::TypeId::of::<T>());
        self.file.types.insert(
            lad_type_id,
            LadTypeDefinition {
                identifier,
                crate_: crate_.map(|s| s.to_owned()),
                // types which are not reflected cannot be imported by scripts
                namespace: String::new(),
                path,
                generics: vec![],
                generic_type: None,
                documentation: Some(documentation.trim().to_owned()),
                associated_functions: vec![],
                layout: LadTypeLayout::Opaque,
                generated: false,
                insignificance: default_importance(),
                metadata: LadTypeMetadata {
                    is_component: false,
                    is_resource: false,
                    is_bundle: false,
                    is_asset: false,
                    is_reflect: false,
                    mapped_to_primitive_kind: primitive_kind_of(std::any::TypeId::of::<T>()),
                    default: None,
                    misc: Default::default(),
                },
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
        let registration = self.type_registry.get(type_info.type_id());

        let mut insignificance = default_importance();
        let mut generated = false;
        let mut is_component = false;
        let mut is_resource = false;
        let mut is_bundle = false;
        let is_reflect = true;
        if let Some(registration) = registration {
            if registration.contains::<MarkAsGenerated>() {
                generated = true;
            }
            if registration.contains::<MarkAsCore>() {
                insignificance = default_importance() / 2;
            }
            if registration.contains::<MarkAsSignificant>() {
                insignificance = default_importance() / 4;
            }
            if registration.contains::<ReflectResource>() {
                is_resource = true
            }
            if registration.contains::<ReflectComponent>() {
                is_component = true
            }
            if registration.contains::<ReflectBundle>() {
                is_bundle = true
            }
        }
        let is_asset = is_asset_type(self.type_registry, type_info.type_id());

        let crate_ = type_info
            .type_path_table()
            .crate_name()
            .map(|s| s.to_owned());
        let namespace = registration
            .and_then(|r| r.data::<ScriptNamespace>())
            .map(|n| n.0.to_string())
            .or_else(|| crate_.clone())
            .unwrap_or_default();

        let type_id = self.lad_id_from_type_id(type_info.type_id());
        let generic_type = self.add_generic_type_of(type_info, &crate_, &namespace);
        let layout = self.lad_layout_from_type_info(type_info);
        // whether fields have defaults depends on the argument, so only their kinds are declared
        if let Some(declared) = self.instantiated_layout(type_info)
            && field_kinds(&declared) != field_kinds(&layout)
        {
            warn!(
                "The layout of {type_id} is not the one its generic type declares, the declaration of the generic type may be wrong.\nreflected: {layout:?}\ndeclared: {declared:?}"
            );
        }
        let lad_type = LadTypeDefinition {
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
            generic_type,
            documentation: type_info.docs().map(|s| s.to_string()),
            associated_functions: Vec::new(),
            crate_,
            namespace,
            path: type_info.type_path_table().path().to_owned(),
            layout,
            generated,
            insignificance,
            metadata: LadTypeMetadata {
                is_component,
                is_resource,
                is_bundle,
                is_asset,
                is_reflect,
                mapped_to_primitive_kind: primitive_kind_of(type_info.type_id()),
                default: self.lad_default(type_info.type_id()),
                misc: Default::default(),
            },
        };
        self.file.types.insert(type_id, lad_type);
        self
    }

    /// Declares a generic type, so its instantiations are documented as such instead of as unrelated types.
    ///
    /// `G` is the generic type instantiated with a marker in place of each parameter, i.e. `Handle<A>`, and `params` names
    /// the parameters in order. Reflection can't tell which fields of an instantiation like `Time<Virtual>` are of a parameter's
    /// type, but in `Time<A>` those are exactly the fields of type `A`, so the layout is described from `G`.
    /// Instantiations in the type registry are matched by path and number of parameters.
    ///
    /// The only marker so far is [`A`], so only types with one parameter can be declared.
    ///
    /// # Errors
    /// If a parameter of `G` is not instantiated with a marker, two share one, or `params` doesn't name each of them.
    pub fn add_generic_type<G: Typed>(
        &mut self,
        params: &[&str],
        documentation: &str,
    ) -> Result<&mut Self, String> {
        self.add_generic_type_info(G::type_info(), params, documentation)
    }

    /// The same as [`Self::add_generic_type`], for a generic type known by its type information, i.e. from [`bevy_mod_scripting_bindings::function::generics::AppGenericTypes`].
    ///
    /// # Errors
    /// See [`Self::add_generic_type`].
    pub fn add_generic_type_info(
        &mut self,
        type_info: &'static TypeInfo,
        params: &[&str],
        documentation: &str,
    ) -> Result<&mut Self, String> {
        let path = type_info.type_path_table().path();
        let generics = type_info.generics();
        if generics.is_empty() || generics.len() != params.len() {
            return Err(format!(
                "`{path}` has {} generic parameters, but {} names were given",
                generics.len(),
                params.len()
            ));
        }
        // each marker stands for one parameter, so the fields of each can be told apart
        let mut markers = HashMap::new();
        for (generic, name) in generics.iter().zip(params) {
            if !is_param_marker(generic.type_id()) {
                return Err(format!(
                    "`{path}` does not instantiate `{name}` with a marker, i.e. `A`"
                ));
            }
            if markers
                .insert(generic.type_id(), (*name).to_owned())
                .is_some()
            {
                return Err(format!(
                    "`{path}` instantiates more than one parameter with the same marker"
                ));
            }
        }

        self.generic_types.push(GenericTypeSpec {
            path: path.split('<').next().unwrap_or(path).to_owned(),
            identifier: type_info
                .type_path_table()
                .ident()
                .unwrap_or_default()
                .to_owned(),
            crate_: type_info.type_path_table().crate_name().map(str::to_owned),
            params: params.iter().map(|&name| name.to_owned()).collect(),
            documentation: documentation.to_owned(),
            type_info,
            markers,
            layout: None,
        });
        // layouts name the declared generic types they use, i.e. `Handle<T>`, so they are described again
        // with the new one known, whatever order they are declared in
        for spec in &mut self.generic_types {
            spec.layout = None;
        }
        Ok(self)
    }

    /// The layout of the declared generic type, described from its instantiation with markers the first time it's needed.
    fn generic_layout(&mut self, index: usize) -> LadTypeLayout {
        if let Some(layout) = &self.generic_types[index].layout {
            return layout.clone();
        }
        let spec = &self.generic_types[index];
        let type_info = spec.type_info;
        self.describing_params = spec.markers.clone();
        let layout = self.lad_layout_from_type_info(type_info);
        self.describing_params.clear();
        self.generic_types[index].layout = Some(layout.clone());
        layout
    }

    /// If the type is an instantiation of a declared generic type, adds the generic type to the file and returns its id.
    fn add_generic_type_of(
        &mut self,
        type_info: &TypeInfo,
        crate_: &Option<String>,
        namespace: &str,
    ) -> Option<LadTypeId> {
        let index = self.declared_generic_index(type_info)?;
        let layout = self.generic_layout(index);
        let spec = &self.generic_types[index];
        let id = LadTypeId::new_string_id(spec.path.clone().into());
        self.file
            .generic_types
            .entry(id.clone())
            .or_insert_with(|| LadGenericType {
                identifier: type_info
                    .type_path_table()
                    .ident()
                    .unwrap_or_default()
                    .to_string(),
                crate_: crate_.clone(),
                path: spec.path.clone(),
                namespace: namespace.to_owned(),
                params: spec.params.clone(),
                documentation: Some(spec.documentation.clone()),
                layout,
            });
        Some(id)
    }

    /// The index of the declared generic type the type is an instantiation of, matched by path and number of parameters
    fn declared_generic_index(&self, type_info: &TypeInfo) -> Option<usize> {
        let path = type_info.type_path_table().path();
        let generic_path = path.split('<').next().unwrap_or(path);
        self.generic_types.iter().position(|spec| {
            spec.path == generic_path && spec.params.len() == type_info.generics().len()
        })
    }

    /// The layout of an instantiation of a declared generic type as its declaration describes it, i.e. `{ context: Virtual }`
    /// for `Time<Virtual>`. It should be the layout reflection gives the instantiation.
    fn instantiated_layout(&mut self, type_info: &TypeInfo) -> Option<LadTypeLayout> {
        let index = self.declared_generic_index(type_info)?;
        let args = type_info.generics();
        let mut layout = self.generic_layout(index);
        let params = self.generic_types[index].params.clone();
        // the parameters are in the order of the arguments
        for (param, arg) in params.iter().zip(args.iter()) {
            let arg = self.lad_generic_arg_kind(arg.type_id(), false);
            layout.replace_param(param, &arg);
        }
        Some(layout)
    }

    /// Adds all nested types within the given `ThroughTypeInfo`.
    pub fn add_through_type_info(&mut self, type_info: &ThroughTypeInfo) -> &mut Self {
        self.lad_kind(type_info, true);
        self
    }

    /// Add a function definition to the LAD file.
    /// Will overwrite any existing function definitions with the same function id.
    ///
    /// Parses argument and return specific docstrings as per: <https://github.com/rust-lang/rust/issues/57525>
    ///
    /// i.e. looks for blocks like:
    /// ```rust,ignore
    /// /// Arguments:
    /// ///  * `arg_name`: docstring1
    /// ///  * `arg_name2`: docstring2
    /// ///
    /// /// Returns:
    /// ///  * `return_name`: return docstring
    /// ```
    ///
    /// And then removes them from the original block, instead putting it in each argument / return docstring
    ///
    /// Functions which are members of a namespace are added with [`Self::add_namespace_function`], as their info doesn't say which namespace.
    pub fn add_function_info(&mut self, function_info: &FunctionInfo) -> &mut Self {
        let function_id = self.lad_function_id_from_info(function_info);
        let namespace = match function_info.namespace {
            FunctionScope::Builtin | FunctionScope::Member => LadFunctionNamespace::Global,
            FunctionScope::OnType(type_id) => {
                LadFunctionNamespace::Type(self.lad_id_from_type_id(type_id))
            }
        };
        self.insert_function(function_id, namespace, function_info)
    }

    /// Adds a function which is a member of a script namespace, i.e. `import_namespace("bms").log_info`.
    pub fn add_namespace_function(
        &mut self,
        namespace: impl Into<Cow<'static, str>>,
        function_info: &FunctionInfo,
    ) -> &mut Self {
        let namespace = namespace.into();
        let function_id =
            LadFunctionId::new_string_id(format!("{namespace}::{}", function_info.name));
        self.file
            .namespaces
            .entry(namespace.clone())
            .or_default()
            .functions
            .insert(function_info.name.clone(), function_id.clone());
        self.insert_function(
            function_id,
            LadFunctionNamespace::Namespace { namespace },
            function_info,
        )
    }

    /// Adds a value which is a member of a script namespace.
    pub fn add_namespace_value(
        &mut self,
        namespace: impl Into<Cow<'static, str>>,
        name: impl Into<Cow<'static, str>>,
        type_info: &ThroughTypeInfo,
    ) -> &mut Self {
        self.add_through_type_info(type_info);
        let type_kind = self.lad_type_kind_from_through_type(type_info);
        self.add_namespace_instance(namespace, name, type_kind)
    }

    /// Adds a value which is a member of a script namespace, with a type the builder can't describe itself, replacing any value with that name.
    pub fn add_namespace_instance(
        &mut self,
        namespace: impl Into<Cow<'static, str>>,
        name: impl Into<Cow<'static, str>>,
        type_kind: LadTypeKind,
    ) -> &mut Self {
        self.file
            .namespaces
            .entry(namespace.into())
            .or_default()
            .values
            .insert(
                name.into(),
                LadInstance {
                    type_kind,
                    is_static: false,
                },
            );
        self
    }

    /// Adds a function on a variant of an enum, i.e. a function on `EnumVariantRef` whose placeholders
    /// were bound to the variant with [`FunctionInfo::bind_placeholders`].
    pub fn add_variant_function_info(
        &mut self,
        function_info: &FunctionInfo,
        enum_type: TypeId,
        variant: &str,
    ) -> &mut Self {
        let type_id = self.lad_id_from_type_id(enum_type);
        let function_id =
            LadFunctionId::new_string_id(format!("{type_id}::{variant}::{}", function_info.name));
        let namespace = LadFunctionNamespace::Variant {
            type_id,
            variant: variant.to_owned().into(),
        };
        self.insert_function(function_id, namespace, function_info)
    }

    fn insert_function(
        &mut self,
        function_id: LadFunctionId,
        namespace: LadFunctionNamespace,
        function_info: &FunctionInfo,
    ) -> &mut Self {
        let default_docstring = Cow::Owned("".into());
        let (main_docstring, arg_docstrings, return_docstring) =
            Self::split_docstring(function_info.docs.as_ref().unwrap_or(&default_docstring));

        let mut identifier = function_info.name.as_ref();
        let mut overload_index = None;
        if identifier.contains("-") {
            let mut parts = identifier.split("-");
            if let Some(less_overload) = parts.next() {
                identifier = less_overload;
            }
            if let Some(number) = parts.next() {
                overload_index = number.parse::<usize>().ok()
            }
        }

        // the call context is filled in by BMS, scripts never pass it
        let arguments: Vec<_> = function_info
            .arg_info
            .iter()
            .filter(|arg| arg.type_id != TypeId::of::<FunctionCallContext>())
            .enumerate()
            .map(|(idx, arg)| {
                let kind = match &arg.type_info {
                    Some(through_type) => self.lad_type_kind_from_through_type(through_type),
                    None => LadTypeKind::Unknown(self.lad_id_from_type_id(arg.type_id)),
                };
                LadArgument {
                    kind,
                    documentation: arg_docstrings.iter().find_map(|(name, doc)| {
                        (Some(name.as_str()) == arg.name.as_deref())
                            .then_some(Cow::Owned(doc.clone()))
                    }),
                    name: Some(
                        arg.name
                            .clone()
                            .unwrap_or_else(|| Cow::Owned(format!("arg{}", idx + 1))),
                    ),
                }
            })
            .collect();
        // from the signature rather than the documented kinds, which spell instantiations of generic types by their generic type
        let first_arg = function_info
            .arg_info
            .iter()
            .find(|arg| arg.type_id != TypeId::of::<FunctionCallContext>())
            .and_then(|arg| arg.type_info.as_ref());
        let receiver = match &namespace {
            LadFunctionNamespace::Type(_) => match (function_info.namespace, first_arg) {
                (
                    FunctionScope::OnType(type_id),
                    Some(
                        ThroughTypeInfo::Ref(arg)
                        | ThroughTypeInfo::Mut(arg)
                        | ThroughTypeInfo::Val(arg),
                    ),
                ) => arg.type_id() == type_id,
                _ => false,
            },
            // functions on variants are called on the variant, i.e. `MyEnum.Variant:new{}`
            LadFunctionNamespace::Variant { .. } => !arguments.is_empty(),
            LadFunctionNamespace::Global | LadFunctionNamespace::Namespace { .. } => false,
        };

        let lad_function = LadFunction {
            identifier: identifier.to_owned().into(),
            overload_index,
            arguments,
            receiver,
            return_type: LadArgument {
                name: return_docstring.as_ref().cloned().map(|(n, _)| n.into()),
                documentation: return_docstring.map(|(_, v)| v.into()),
                kind: function_info
                    .return_info
                    .type_info
                    .clone()
                    .map(|info| self.lad_type_kind_from_through_type(&info))
                    .unwrap_or_else(|| {
                        LadTypeKind::Unknown(
                            self.lad_id_from_type_id(function_info.return_info.type_id),
                        )
                    }),
            },
            documentation: (!main_docstring.is_empty()).then_some(main_docstring.into()),
            namespace,
            metadata: LadFunctionMetadata {
                operator: ScriptOperatorNames::parse(identifier),
                misc: Default::default(),
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

    fn has_unknowns(&self, type_id: TypeId) -> bool {
        if primitive_kind_of(type_id).is_some() {
            return false;
        }

        let type_info = match self.type_registry.get_type_info(type_id) {
            Some(info) => info,
            None => return true,
        };
        let inner_type_ids: Vec<_> = match type_info {
            TypeInfo::Struct(struct_info) => {
                struct_info.generics().iter().map(|g| g.type_id()).collect()
            }
            TypeInfo::TupleStruct(tuple_struct_info) => tuple_struct_info
                .generics()
                .iter()
                .map(|g| g.type_id())
                .collect(),
            TypeInfo::Tuple(tuple_info) => {
                tuple_info.generics().iter().map(|g| g.type_id()).collect()
            }
            TypeInfo::List(list_info) => vec![list_info.item_ty().id()],
            TypeInfo::Array(array_info) => vec![array_info.item_ty().id()],
            TypeInfo::Map(map_info) => vec![map_info.key_ty().id(), map_info.value_ty().id()],
            TypeInfo::Set(set_info) => vec![set_info.value_ty().id()],
            TypeInfo::Enum(enum_info) => enum_info.generics().iter().map(|g| g.type_id()).collect(),
            TypeInfo::Opaque(_) => vec![],
        };

        inner_type_ids.iter().any(|id| self.has_unknowns(*id))
    }

    /// Build the finalized and optimized LAD file.
    pub fn build(&mut self) -> LadFile {
        let mut file = std::mem::replace(&mut self.file, LadFile::new());

        if self.exclude_types_involving_unregistered_types {
            let mut to_remove = HashSet::new();
            for reg in self.type_registry.iter() {
                let type_id = reg.type_id();
                if self.has_unknowns(type_id) {
                    to_remove.insert(self.lad_id_from_type_id(type_id));
                }
            }

            // remove those type ids
            file.types.retain(|id, _| !to_remove.contains(id));
        }

        // declared generic types named by layouts and signatures, but without instantiations in the file,
        // which would otherwise have added them, with the namespace of their instantiations
        let mut named = Vec::new();
        for function in file.functions.values() {
            for arg in function.arguments.iter().chain([&function.return_type]) {
                named_generic_types(&arg.kind, &mut named);
            }
        }
        for layout in file
            .types
            .values()
            .map(|t| &t.layout)
            .chain(file.generic_types.values().map(|t| &t.layout))
        {
            for (_, kinds) in field_kinds(layout) {
                kinds
                    .into_iter()
                    .for_each(|kind| named_generic_types(kind, &mut named));
            }
        }
        // the layouts of the generic types added name generic types in turn
        while let Some(id) = named.pop() {
            if file.generic_types.contains_key(&id) {
                continue;
            }
            let Some(index) = self
                .generic_types
                .iter()
                .position(|spec| spec.path == id.to_string())
            else {
                continue;
            };
            let layout = self.generic_layout(index);
            for (_, kinds) in field_kinds(&layout) {
                kinds
                    .into_iter()
                    .for_each(|kind| named_generic_types(kind, &mut named));
            }
            let spec = &self.generic_types[index];
            file.generic_types.insert(
                id,
                LadGenericType {
                    identifier: spec.identifier.clone(),
                    crate_: spec.crate_.clone(),
                    path: spec.path.clone(),
                    namespace: spec.crate_.clone().unwrap_or_default(),
                    params: spec.params.clone(),
                    documentation: Some(spec.documentation.clone()),
                    layout,
                },
            );
        }

        // associate functions on type namespaces with their types
        for (function_id, function) in file.functions.iter() {
            match &function.namespace {
                LadFunctionNamespace::Type(type_id) => {
                    if let Some(t) = file.types.get_mut(type_id) {
                        if t.generic_type.is_some() {
                            warn!(
                                "Function {} is on {}, an instantiation of a generic type, functions of which backends don't document yet.",
                                function_id, type_id
                            );
                        }
                        t.associated_functions.push(function_id.clone());
                    } else {
                        warn!(
                            "Function {} is on type {}, but the type is not registered in the LAD file.",
                            function_id, type_id
                        );
                    }
                }
                // variants are not types, backends find their functions by namespace
                LadFunctionNamespace::Global
                | LadFunctionNamespace::Variant { .. }
                | LadFunctionNamespace::Namespace { .. } => {}
            }
        }

        // the types scripts import from each namespace, by the name `import_namespace` gives them
        let mut types_by_id: Vec<_> = file.documented_types().collect();
        types_by_id.sort_by_key(|(id, _)| *id);
        let mut namespace_types = Vec::new();
        for (type_id, lad_type) in types_by_id {
            // primitives are values, and types which aren't reflected aren't in the type registry, so neither can be imported
            if lad_type.metadata.mapped_to_primitive_kind.is_some() || !lad_type.metadata.is_reflect
            {
                continue;
            }
            namespace_types.push((
                Cow::<'static, str>::Owned(lad_type.namespace.clone()),
                Cow::<'static, str>::Owned(lad_type.identifier.clone()),
                type_id.clone(),
            ));
        }
        for (namespace, name, type_id) in namespace_types {
            let types = &mut file.namespaces.entry(namespace).or_default().types;
            match types.get(&name) {
                // as at runtime, the first type with a name wins
                Some(existing) => warn!(
                    "{type_id} is not reachable from its namespace, {existing} has the same name"
                ),
                None => {
                    types.insert(name, type_id);
                }
            }
        }

        if self.sorted {
            file.types.sort_by(|ak, av, bk, bv| {
                let complexity_a: usize = av
                    .path
                    .char_indices()
                    .filter_map(|(_, c)| (c == '<' || c == ',').then_some(1))
                    .sum();
                let complexity_b = bv
                    .path
                    .char_indices()
                    .filter_map(|(_, c)| (c == '<' || c == ',').then_some(1))
                    .sum();

                let has_functions_a = !av.associated_functions.is_empty();
                let has_functions_b = !bv.associated_functions.is_empty();

                let ordered_by_name = ak.cmp(bk);
                let ordered_by_generics_complexity = complexity_a.cmp(&complexity_b);
                let ordered_by_generated = av.generated.cmp(&bv.generated);
                let ordered_by_having_functions = has_functions_b.cmp(&has_functions_a);
                let ordered_by_significance = av.insignificance.cmp(&bv.insignificance);

                ordered_by_significance
                    .then(ordered_by_having_functions)
                    .then(ordered_by_generics_complexity)
                    .then(ordered_by_name)
                    .then(ordered_by_generated)
            });

            file.functions.sort_keys();
            file.generic_types.sort_keys();
            file.namespaces.sort_keys();
            for namespace in file.namespaces.values_mut() {
                namespace.types.sort_keys();
                namespace.functions.sort_keys();
                namespace.values.sort_keys();
            }
        }

        file
    }

    /// Checks if a line is one of:
    /// - `# key:`
    /// - `key:`
    /// - `key`
    /// - `## key`
    ///
    /// Or similar patterns
    fn is_docstring_delimeter(key: &str, line: &str) -> bool {
        line.trim()
            .trim_start_matches("#")
            .trim_end_matches(":")
            .trim()
            .eq_ignore_ascii_case(key)
    }

    /// Parses lines of the pattern:
    /// * `arg` : val
    ///
    /// returning (arg,val) without markup
    fn parse_arg_docstring(line: &str) -> Option<(&str, &str)> {
        static ARG_DOCSTRING: std::sync::LazyLock<Option<regex::Regex>> =
            std::sync::LazyLock::new(|| {
                regex::Regex::new(r#"\s*\*\s*`(?<arg>[^`]+)`\s*[:-]\s*(?<val>.+[^\s]).*$"#).ok()
            });
        let captures = ARG_DOCSTRING.as_ref()?.captures(line)?;
        let arg = captures.name("arg")?;
        let val = captures.name("val")?;

        Some((arg.as_str(), val.as_str()))
    }

    /// Splits the docstring, into the following:
    /// - The main docstring
    /// - The argument docstrings
    /// - The return docstring
    ///
    /// While removing any prefixes
    fn split_docstring(
        docstring: &str,
    ) -> (String, Vec<(String, String)>, Option<(String, String)>) {
        // find a line containing only `Arguments:` ignoring spaces and markdown headings
        let lines = docstring.lines().collect::<Vec<_>>();

        // this must exist for us to parse any of the areas
        let argument_line_idx = match lines
            .iter()
            .enumerate()
            .find_map(|(idx, l)| Self::is_docstring_delimeter("arguments", l).then_some(idx))
        {
            Some(a) => a,
            None => return (docstring.to_owned(), vec![], None),
        };

        // this can, not exist, if arguments area does
        let return_line_idx = lines.iter().enumerate().find_map(|(idx, l)| {
            (Self::is_docstring_delimeter("returns", l)
                || Self::is_docstring_delimeter("return", l))
            .then_some(idx)
        });

        let return_area_idx = return_line_idx.unwrap_or(usize::MAX);
        let return_area_first = argument_line_idx > return_area_idx;
        let argument_range = match return_area_first {
            true => argument_line_idx..lines.len(),
            false => argument_line_idx..return_area_idx,
        };
        let return_range = match return_area_first {
            true => return_area_idx..argument_line_idx,
            false => return_area_idx..lines.len(),
        };
        let non_main_area =
            min(return_area_idx, argument_line_idx)..max(return_area_idx, argument_line_idx);

        let parsed_lines = lines
            .iter()
            .enumerate()
            .map(|(i, l)| {
                match Self::parse_arg_docstring(l) {
                    Some(parsed) => {
                        // figure out if it's in the argument, return or neither of the areas
                        // if return area doesn't exist assign everything to arguments
                        let in_argument_range = argument_range.contains(&i);
                        let in_return_range = return_range.contains(&i);
                        (l, Some((in_argument_range, in_return_range, parsed)))
                    }
                    None => (l, None),
                }
            })
            .collect::<Vec<_>>();

        // collect all argument docstrings, and the first return docstring, removing those lines from the docstring (and the argument/return headers)
        // any other ones leave alone
        let main_docstring = parsed_lines
            .iter()
            .enumerate()
            .filter_map(|(i, (l, parsed))| {
                ((!non_main_area.contains(&i) || !l.trim().is_empty())
                    && (i != return_area_idx && i != argument_line_idx)
                    && (parsed.is_none() || parsed.is_some_and(|(a, b, _)| !a && !b)))
                .then_some((**l).to_owned())
            })
            .collect::<Vec<_>>();

        let arg_docstrings = parsed_lines
            .iter()
            .filter_map(|(_l, parsed)| {
                parsed.and_then(|(is_arg, is_return, (a, b))| {
                    (is_arg && !is_return).then_some((a.to_owned(), b.to_owned()))
                })
            })
            .collect();

        let return_docstring = parsed_lines.iter().find_map(|(_l, parsed)| {
            parsed.and_then(|(is_arg, is_return, (a, b))| {
                (!is_arg && is_return).then_some((a.to_owned(), b.to_owned()))
            })
        });

        (main_docstring.join("\n"), arg_docstrings, return_docstring)
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
                    type_: self.lad_field_kind(
                        field.type_id(),
                        field.type_path(),
                        field.type_info(),
                    ),
                    has_default: self.has_default(field.type_id()),
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
                    type_: self.lad_field_kind(
                        field.type_id(),
                        field.type_path(),
                        field.type_info(),
                    ),
                    has_default: self.has_default(field.type_id()),
                })
                .collect(),
        }
    }

    /// Whether the type has a registered `Default`, which is what constructors use to fill in missing fields
    fn has_default(&self, type_id: TypeId) -> bool {
        self.type_registry
            .get_type_data::<ReflectDefault>(type_id)
            .is_some()
    }

    fn lad_default(&self, type_id: TypeId) -> Option<LadDefault> {
        let default = self
            .type_registry
            .get_type_data::<ReflectDefault>(type_id)?
            .default();
        let variant = match default.reflect_ref() {
            ReflectRef::Enum(value) => Some(Cow::Owned(value.variant_name().to_owned())),
            _ => None,
        };
        Some(LadDefault { variant })
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
                            VariantInfo::Struct(struct_variant_info) => {
                                let fields = (0..struct_variant_info.field_len())
                                    .filter_map(|i| struct_variant_info.field_at(i));

                                self.struct_variant_from_named_fields(variant_name.into(), fields)
                            }
                            VariantInfo::Tuple(tuple_variant_info) => {
                                let fields = (0..tuple_variant_info.field_len())
                                    .filter_map(|i| tuple_variant_info.field_at(i));

                                self.tuple_struct_variant_from_fields(variant_name.into(), fields)
                            }
                            VariantInfo::Unit(_) => LadVariant::Unit {
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

    /// Should only be used on fields, as those are never going to contain
    /// untyped structures, i.e. are going to be fully reflectable
    fn lad_type_kind_from_type_id(&mut self, type_id: TypeId) -> LadTypeKind {
        if let Some(type_info) = self.type_registry.get_type_info(type_id) {
            let through_type_info =
                into_through_type_info_with_registry(type_info, self.type_registry);
            self.lad_type_kind_from_through_type(&through_type_info)
        } else {
            LadTypeKind::Unknown(self.lad_id_from_type_id(type_id))
        }
    }

    /// The kind of a field of the given type.
    ///
    /// In the layout of a generic type, the types mentioning its parameters (`A`, `Vec<A>`) aren't registered,
    /// so they are described from the field's own type information.
    fn lad_field_kind(
        &mut self,
        type_id: TypeId,
        type_path: &str,
        type_info: Option<&'static TypeInfo>,
    ) -> LadTypeKind {
        if !self.describing_params.is_empty() {
            if let Some(type_info) = type_info {
                let kind = self.lad_type_kind_from_through_type(
                    &into_through_type_info_with_registry(type_info, self.type_registry),
                );
                let mut mentions_param = false;
                kind.for_each(&mut |kind| mentions_param |= matches!(kind, LadTypeKind::Param(_)));
                if mentions_param {
                    return kind;
                }
            }
            // i.e. in an opaque type, or in a generic type the layout has no kind for
            if mentions_param_marker(type_path) {
                warn!(
                    "The type of a field, `{type_path}`, uses a parameter in a way the layout of the generic type can't describe, it is documented as unknown"
                );
                return LadTypeKind::Unknown(LadTypeId::new_string_id(type_path.to_owned().into()));
            }
        }
        self.lad_type_kind_from_type_id(type_id)
    }

    /// Figures out whether the type is a primitive or not and creates the right type id
    fn lad_id_from_type_id(&mut self, type_id: TypeId) -> LadTypeId {
        // a special exception
        if type_id == std::any::TypeId::of::<World>() {
            return LadTypeId::new_string_id("World".into());
        }

        if let Some(lad_id) = self.type_id_mapping.get(&type_id) {
            return lad_id.clone();
        }

        let new_id = match primitive_kind_of(type_id) {
            Some(primitive) => LadTypeId::new_string_id(primitive.to_string().into()),
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
            FunctionScope::Builtin | FunctionScope::Member => "".to_string(),
            FunctionScope::OnType(type_id) => self.lad_id_from_type_id(type_id).to_string(),
        };

        LadFunctionId::new_string_id(format!("{}::{}", namespace_string, function_info.name))
    }

    fn lad_type_kind_from_through_type(&mut self, through_type: &ThroughTypeInfo) -> LadTypeKind {
        self.lad_kind(through_type, false)
    }

    /// The named type of a value or reference: an instantiation of a declared generic type is the generic type applied
    /// to its arguments, i.e. `Handle<Image>`, any other type is its id. Also adds the types to the file if `register` is set.
    fn lad_named_type(&mut self, type_info: &'static TypeInfo, register: bool) -> LadNamedType {
        if register {
            self.add_type_info(type_info);
        }
        let Some(generic_type) = self.declared_generic_type(type_info) else {
            return self.lad_id_from_type_id(type_info.type_id()).into();
        };
        let args = type_info
            .generics()
            .iter()
            .map(|arg| self.lad_generic_arg_kind(arg.type_id(), register))
            .collect();
        LadNamedType::Generic { generic_type, args }
    }

    /// The id of the declared generic type the type is an instantiation of, see [`Self::declared_generic_index`]
    fn declared_generic_type(&self, type_info: &TypeInfo) -> Option<LadTypeId> {
        self.declared_generic_index(type_info)
            .map(|index| LadTypeId::new_string_id(self.generic_types[index].path.clone().into()))
    }

    /// The kind of an argument of a generic type, as a field of its type would be described, or a parameter of the
    /// generic type whose layout is being described.
    fn lad_generic_arg_kind(&mut self, type_id: TypeId, register: bool) -> LadTypeKind {
        if let Some(param) = self.describing_params.get(&type_id) {
            return LadTypeKind::Param(param.clone());
        }
        if register && let Some(type_info) = self.type_registry.get_type_info(type_id) {
            self.add_type_info(type_info);
        }
        self.lad_type_kind_from_type_id(type_id)
    }

    /// Converts a description of a type to its LAD kind, also adding the types it involves to the file if `register` is set.
    fn lad_kind(&mut self, through_type: &ThroughTypeInfo, register: bool) -> LadTypeKind {
        let leaf = |builder: &mut Self, type_info: &'static TypeInfo| {
            if register {
                builder.add_type_info(type_info);
            }
            builder.lad_id_from_type_id(type_info.type_id())
        };
        let nested = |builder: &mut Self, inner: &ThroughTypeInfo| {
            Box::new(builder.lad_kind(inner, register))
        };
        // in the layout of a generic type, the markers are its parameters
        if let ThroughTypeInfo::Ref(type_info)
        | ThroughTypeInfo::Mut(type_info)
        | ThroughTypeInfo::Val(type_info)
        | ThroughTypeInfo::Opaque(type_info) = through_type
            && let Some(param) = self.describing_params.get(&type_info.type_id())
        {
            return LadTypeKind::Param(param.clone());
        }
        match through_type {
            ThroughTypeInfo::Ref(type_info) => {
                LadTypeKind::Ref(self.lad_named_type(type_info, register))
            }
            ThroughTypeInfo::Mut(type_info) => {
                LadTypeKind::Mut(self.lad_named_type(type_info, register))
            }
            ThroughTypeInfo::Val(type_info) => {
                LadTypeKind::Val(self.lad_named_type(type_info, register))
            }
            // the type's own conversions decide how scripts see it
            ThroughTypeInfo::Opaque(type_info) => match primitive_kind_of(type_info.type_id()) {
                Some(primitive) => LadTypeKind::Primitive(primitive),
                None => LadTypeKind::Unknown(leaf(self, type_info)),
            },
            ThroughTypeInfo::Primitive(primitive) => LadTypeKind::Primitive(primitive.clone()),
            ThroughTypeInfo::Option(inner) => LadTypeKind::Option(nested(self, inner)),
            ThroughTypeInfo::Vec(inner) => LadTypeKind::Vec(nested(self, inner)),
            ThroughTypeInfo::HashMap(key, value) => {
                LadTypeKind::HashMap(nested(self, key), nested(self, value))
            }
            ThroughTypeInfo::HashSet(inner) => LadTypeKind::HashSet(nested(self, inner)),
            ThroughTypeInfo::Array(inner, size) => LadTypeKind::Array(nested(self, inner), *size),
            ThroughTypeInfo::InteropResult(inner) => {
                LadTypeKind::InteropResult(nested(self, inner))
            }
            ThroughTypeInfo::Tuple(inner) => {
                LadTypeKind::Tuple(inner.iter().map(|i| *nested(self, i)).collect())
            }
            ThroughTypeInfo::Union(inner) => {
                LadTypeKind::Union(inner.iter().map(|i| *nested(self, i)).collect())
            }
            ThroughTypeInfo::UntypedTuple => LadTypeKind::UntypedTuple,
            ThroughTypeInfo::TupleWithRest { head, rest } => LadTypeKind::TupleWithRest {
                head: head.iter().map(|i| *nested(self, i)).collect(),
                rest: nested(self, rest),
            },
            ThroughTypeInfo::Fields { type_info, variant } => LadTypeKind::Fields {
                type_id: leaf(self, type_info),
                variant: variant.map(Cow::Borrowed),
            },
            // functions are bound before they are added, any placeholders left are documented as what they are at runtime
            ThroughTypeInfo::Bound(BoundPlaceholder::Value | BoundPlaceholder::Handle) => {
                LadTypeKind::Primitive(ReflectionPrimitiveKind::ReflectReference)
            }
            ThroughTypeInfo::Bound(BoundPlaceholder::Fields) => self.lad_kind(
                &HashMap::<String, ScriptValue>::through_type_info(),
                register,
            ),
            ThroughTypeInfo::AssetHandle(asset_type_info) => {
                if register {
                    self.add_type_info(asset_type_info);
                }
                match asset_handle_type_info(self.type_registry, asset_type_info.type_id()) {
                    Some(handle_type_info) => {
                        LadTypeKind::Val(self.lad_named_type(handle_type_info, register))
                    }
                    None => LadTypeKind::Primitive(ReflectionPrimitiveKind::ReflectReference),
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use bevy_mod_scripting_bindings::ReflectReference;
    use std::collections::HashMap;

    use bevy_mod_scripting_bindings::{Union, V, docgen::info::GetFunctionInfo, function::from::R};
    use bevy_reflect::Reflect;

    use super::*;

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
    fn parse_docstrings_is_resistant_to_whitespace() {
        pretty_assertions::assert_eq!(
            LadFileBuilder::parse_arg_docstring("* `arg` : doc"),
            Some(("arg", "doc"))
        );
        pretty_assertions::assert_eq!(
            LadFileBuilder::parse_arg_docstring("  * `arg` - doc"),
            Some(("arg", "doc"))
        );
        pretty_assertions::assert_eq!(
            LadFileBuilder::parse_arg_docstring("   *   `arg`   :    doc     "),
            Some(("arg", "doc"))
        );
    }

    #[test]
    fn docstring_delimeter_detection_is_flexible() {
        assert!(LadFileBuilder::is_docstring_delimeter(
            "arguments",
            "arguments"
        ));
        assert!(LadFileBuilder::is_docstring_delimeter(
            "arguments",
            "Arguments:"
        ));
        assert!(LadFileBuilder::is_docstring_delimeter(
            "arguments",
            "## Arguments"
        ));
        assert!(LadFileBuilder::is_docstring_delimeter(
            "arguments",
            "## Arguments:"
        ));
        assert!(LadFileBuilder::is_docstring_delimeter(
            "arguments",
            "Arguments"
        ));
    }

    /// Helper function to assert that splitting the docstring produces the expected output.
    fn assert_docstring_split(
        input: &str,
        expected_main: &str,
        expected_args: &[(&str, &str)],
        expected_return: Option<(&str, &str)>,
        test_name: &str,
    ) {
        let (main, args, ret) = LadFileBuilder::split_docstring(input);

        pretty_assertions::assert_eq!(
            main,
            expected_main,
            "main docstring was incorrect - {}",
            test_name
        );

        let expected_args: Vec<(String, String)> = expected_args
            .iter()
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .collect();
        pretty_assertions::assert_eq!(
            args,
            expected_args,
            "argument docstring was incorrect - {}",
            test_name
        );

        let expected_ret = expected_return.map(|(a, b)| (a.to_string(), b.to_string()));
        pretty_assertions::assert_eq!(
            ret,
            expected_ret,
            "return docstring was incorrect - {}",
            test_name
        );
    }

    #[test]
    fn docstrings_parse_correctly_from_various_formats() {
        assert_docstring_split(
            r#"
                ## Hello
                Arguments: 
                    * `arg1` - some docs
                    * `arg2` : some more docs
                # Returns
                    * `return` : return docs
            "#
            .trim(),
            "## Hello",
            &[("arg1", "some docs"), ("arg2", "some more docs")],
            Some(("return", "return docs")),
            "normal docstring",
        );
        assert_docstring_split(
            r#"
                Arguments: 
                    * `arg1` - some docs
                    * `arg2` : some more docs
                Returns
                    * `return` : return docs
            "#
            .trim(),
            "",
            &[("arg1", "some docs"), ("arg2", "some more docs")],
            Some(("return", "return docs")),
            "empty main docstring",
        );
        assert_docstring_split(
            r#"
                Arguments: 
                    * `arg1` - some docs
                    * `arg2` : some more docs
            "#
            .trim(),
            "",
            &[("arg1", "some docs"), ("arg2", "some more docs")],
            None,
            "no return docstring",
        );
        assert_docstring_split(
            r#"
                Returns
                    * `return` : return docs
            "#
            .trim(),
            r#"
                Returns
                    * `return` : return docs
            "#
            .trim(),
            &[],
            None,
            "no argument docstring",
        );
        assert_docstring_split(
            r#"
                ## Hello
            "#
            .trim(),
            "## Hello",
            &[],
            None,
            "no argument or return docstring",
        );
        // return first
        assert_docstring_split(
            r#"
                Returns
                    * `return` : return docs
                Arguments: 
                    * `arg1` - some docs
                    * `arg2` : some more docs
            "#
            .trim(),
            "",
            &[("arg1", "some docs"), ("arg2", "some more docs")],
            Some(("return", "return docs")),
            "return first",
        );
        // whitespace in between
        assert_docstring_split(
            r#"
                ## Hello


                Arguments: 
                    * `arg1` - some docs
                    * `arg2` : some more docs

                Returns
                    * `return` : return docs
            "#
            .trim(),
            "## Hello\n\n",
            &[("arg1", "some docs"), ("arg2", "some more docs")],
            Some(("return", "return docs")),
            "whitespace in between",
        );
    }

    #[test]
    fn test_serializes_as_expected() {
        let mut type_registry = TypeRegistry::default();

        #[derive(Reflect)]
        /// I am a struct
        struct GenericStructType<T> {
            /// hello from field
            field: usize,
            /// hello from field 2
            field2: T,
        }

        impl TypedThrough for GenericStructType<usize> {
            fn through_type_info() -> ThroughTypeInfo {
                ThroughTypeInfo::Opaque(Self::type_info())
            }
        }

        #[derive(Reflect)]
        /// I am a simple plain struct type
        struct PlainStructType {
            int_field: usize,
        }

        impl TypedThrough for PlainStructType {
            fn through_type_info() -> ThroughTypeInfo {
                ThroughTypeInfo::Opaque(Self::type_info())
            }
        }

        #[derive(Reflect)]
        /// I am a unit test type
        struct UnitType;

        impl TypedThrough for UnitType {
            fn through_type_info() -> ThroughTypeInfo {
                ThroughTypeInfo::Opaque(Self::type_info())
            }
        }

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

        /// I am a generic type, which the builder is told about so my layout is described in terms of my parameter
        #[derive(Reflect)]
        #[reflect(from_reflect = false)]
        struct OpaqueGenericType<T> {
            value: T,
            #[reflect(ignore)]
            phantom: std::marker::PhantomData<T>,
        }

        /// I hold an instantiation of a generic type
        #[derive(Reflect)]
        #[reflect(from_reflect = false)]
        struct HoldsOpaqueType {
            generic: OpaqueGenericType<usize>,
        }

        type_registry.register::<GenericStructType<usize>>();
        type_registry.register::<UnitType>();
        type_registry.register::<TupleStructType>();
        type_registry.register::<EnumType>();
        type_registry.register::<PlainStructType>();
        type_registry.register::<OpaqueGenericType<usize>>();
        type_registry.register::<HoldsOpaqueType>();

        let plain_struct_function =
            |_: R<PlainStructType>, _: usize| PlainStructType { int_field: 2 };
        let plain_struct_function_info = plain_struct_function.get_function_info(
            "plain_struct_function".into(),
            FunctionScope::OnType(TypeId::of::<PlainStructType>()),
        );

        let function = |_: ReflectReference, _: usize| 2usize;
        let function_info = function
            .get_function_info(
                "hello_world".into(),
                FunctionScope::OnType(TypeId::of::<GenericStructType<usize>>()),
            )
            .with_docs("hello docs");

        let function_with_complex_args =
            |_: ReflectReference, _: (usize, String), _: Option<Vec<R<EnumType>>>| 2usize;
        let function_with_complex_args_info = function_with_complex_args
            .get_function_info("hello_world".into(), FunctionScope::OnType(TypeId::of::<GenericStructType::<usize>>()))
            .with_arg_names(&["ref_", "tuple", "option_vec_ref_wrapper"])
            .with_docs(
                "Arguments: ".to_owned()
                    + "\n"
                    + " * `ref_`: I am some docs for argument 1"
                    + "\n"
                    + " * `tuple`: I am some docs for argument 2"
                    + "\n"
                    + " * `option_vec_ref_wrapper`: I am some docs for argument 3"
                    + "\n"
                    + "Returns: "
                    + "\n"
                    + " * `return`: I am some docs for the return type, I provide a name for the return value too",
            );

        let global_function = |_: usize| 2usize;
        let global_function_info = global_function
            .get_function_info("hello_world".into(), FunctionScope::Builtin)
            .with_arg_names(&["arg1"]);

        impl TypedThrough for OpaqueGenericType<usize> {
            fn through_type_info() -> ThroughTypeInfo {
                ThroughTypeInfo::Opaque(Self::type_info())
            }
        }
        let generic_function = |_: R<HoldsOpaqueType>, _: V<OpaqueGenericType<usize>>| {
            V::new(OpaqueGenericType::<usize> {
                value: 0,
                phantom: std::marker::PhantomData,
            })
        };
        let generic_function_info = generic_function.get_function_info(
            "swap_generic".into(),
            FunctionScope::OnType(TypeId::of::<HoldsOpaqueType>()),
        );

        let mut lad_file = LadFileBuilder::new(&type_registry)
            .set_description("## Hello gentlemen\n I am  markdown file.\n - hello\n - world")
            .set_sorted(true)
            .add_generic_type::<OpaqueGenericType<A>>(
                &["T"],
                "I am the documentation of a generic type.",
            )
            .unwrap()
            .add_function_info(&generic_function_info)
            .add_type::<OpaqueGenericType<usize>>()
            .add_type::<HoldsOpaqueType>()
            .add_function_info(&plain_struct_function_info)
            .add_function_info(&function_info)
            .add_function_info(&global_function_info)
            .add_function_info(&function_with_complex_args_info)
            .add_type::<GenericStructType<usize>>()
            .add_type::<UnitType>()
            .add_type::<TupleStructType>()
            .add_type::<PlainStructType>()
            .add_type_info(EnumType::type_info())
            .add_namespace_value_of::<V<GenericStructType<usize>>>(
                "ladfile_builder",
                "my_static_instance",
            )
            .add_namespace_value_of::<Vec<V<UnitType>>>("ladfile_builder", "my_non_static_instance")
            .add_namespace_value_of::<HashMap<String, Union<String, String>>>(
                "ladfile_builder",
                "map",
            )
            .build();

        // normalize the version so we don't have to update it every time
        lad_file.version = "{{version}}".into();
        let mut serialized = serialize_lad_file(&lad_file, true).unwrap();

        normalize_file(&mut serialized);

        if std::env::var("BLESS_MODE").is_ok() {
            let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
            let path_to_test_assets = std::path::Path::new(&manifest_dir)
                .join("..")
                .join("ladfile")
                .join("test_assets");

            println!("Blessing test file at {path_to_test_assets:?}");
            std::fs::write(path_to_test_assets.join("test.lad.json"), &serialized).unwrap();
            panic!("Blessed test file, please rerun the test");
        }

        let mut expected = ladfile::EXAMPLE_LADFILE.to_string();
        normalize_file(&mut expected);

        pretty_assertions::assert_eq!(serialized.trim(), expected.trim(),);
    }

    #[test]
    fn test_asset_deserializes_correctly() {
        let asset = ladfile::EXAMPLE_LADFILE.to_string();
        let deserialized = parse_lad_file(&asset).unwrap();
        assert_eq!(deserialized.version, "{{version}}");
    }

    #[test]
    fn test_nested_unregistered_generic_is_removed() {
        let mut type_registry = TypeRegistry::default();

        #[derive(Reflect)]
        #[reflect(no_field_bounds, from_reflect = false)]
        struct StructType<T> {
            #[reflect(ignore)]
            phantom: std::marker::PhantomData<T>,
        }

        #[derive(Reflect)]
        struct Blah;

        type_registry.register::<StructType<Blah>>();

        let lad_file = LadFileBuilder::new_empty(&type_registry)
            .set_sorted(true)
            .set_exclude_including_unregistered(true)
            .add_type::<StructType<Blah>>()
            .build();

        assert_eq!(lad_file.types.len(), 0);
    }

    #[test]
    fn bound_handles_are_documented_as_the_handle_of_the_asset_type() {
        use bevy_asset::{Asset, Handle, ReflectAsset};
        use bevy_mod_scripting_bindings::function::bound::BoundHandle;

        #[derive(Asset, Reflect)]
        struct TestAsset;

        let mut type_registry = TypeRegistry::default();
        type_registry.register::<TestAsset>();
        type_registry.register::<Handle<TestAsset>>();
        type_registry.register_type_data::<TestAsset, ReflectAsset>();

        let load = |_path: String| -> BoundHandle { unreachable!() };
        let info = load
            .get_function_info(Cow::Borrowed("load"), FunctionScope::Builtin)
            .bind_placeholders(TestAsset::type_info(), None);

        let return_kind = |declare: bool| {
            let mut builder = LadFileBuilder::new_empty(&type_registry);
            if declare {
                builder.add_generic_type::<Handle<A>>(&["A"], "").unwrap();
            }
            let asset = builder.lad_id_from_type_id(TypeId::of::<TestAsset>());
            let lad_file = builder.add_function_info(&info).build();
            let function = lad_file
                .functions
                .values()
                .next()
                .expect("the function is added");
            (function.return_type.kind.clone(), asset)
        };

        // the handle type is the instantiation, or the declared generic type applied to the asset type
        let (kind, _) = return_kind(false);
        assert!(
            matches!(&kind, LadTypeKind::Val(LadNamedType::Id(handle)) if handle.to_string().contains("Handle<")),
            "expected the handle type, got {kind:?}"
        );
        let (kind, asset) = return_kind(true);
        assert_eq!(
            kind,
            LadTypeKind::Val(LadNamedType::Generic {
                generic_type: LadTypeId::new_string_id("bevy_asset::handle::Handle".into()),
                args: vec![LadTypeKind::Val(asset.into())],
            })
        );
    }

    #[test]
    fn type_kinds_come_from_the_type_data() {
        use bevy_asset::{Asset, ReflectAsset};
        use bevy_ecs::{bundle::Bundle, component::Component};

        #[derive(Component, Reflect, Default)]
        #[reflect(Component)]
        struct KindComponent;
        #[derive(Bundle, Reflect, Default)]
        #[reflect(Bundle)]
        struct KindBundle {
            component: KindComponent,
        }
        #[derive(Asset, Reflect)]
        struct KindAsset;

        let mut type_registry = TypeRegistry::default();
        type_registry.register::<KindComponent>();
        type_registry.register::<KindBundle>();
        type_registry.register::<KindAsset>();
        type_registry.register_type_data::<KindAsset, ReflectAsset>();

        let lad_file = LadFileBuilder::new_empty(&type_registry)
            .add_type::<KindComponent>()
            .add_type::<KindBundle>()
            .add_type::<KindAsset>()
            .build();
        let kinds = |identifier: &str| {
            let metadata = &lad_file
                .types
                .values()
                .find(|t| t.identifier == identifier)
                .expect("the type is added")
                .metadata;
            (
                metadata.is_component,
                metadata.is_resource,
                metadata.is_bundle,
                metadata.is_asset,
            )
        };

        assert_eq!(kinds("KindComponent"), (true, false, false, false));
        assert_eq!(kinds("KindBundle"), (false, false, true, false));
        assert_eq!(kinds("KindAsset"), (false, false, false, true));
    }

    // Every variant of the kind enum is listed exactly once here. The name lookup is an exhaustive match, so adding a variant
    // does not compile until it is listed, and `every_kind_is_documented_by_fields_or_arguments` fails until it is exercised.
    macro_rules! variant_names {
        ($ty:ident, $all:ident, $name:ident: $($variant:ident),+) => {
            const $all: &[&str] = &[$(stringify!($variant)),+];

            fn $name(value: &$ty) -> &'static str {
                match value {
                    $($ty::$variant { .. } => stringify!($variant)),+
                }
            }
        };
    }

    variant_names!(
        LadTypeKind, ALL_LAD_KINDS, lad_kind_name:
        Ref, Mut, Val, Option, Vec, HashMap, HashSet, InteropResult, Tuple, UntypedTuple,
        TupleWithRest, Array, Primitive, Union, Unknown, Fields, Param
    );

    /// Records every kind found anywhere in the given kind
    fn collect_kinds(kind: &LadTypeKind, seen: &mut std::collections::BTreeSet<&'static str>) {
        seen.insert(lad_kind_name(kind));
        match kind {
            LadTypeKind::Option(inner)
            | LadTypeKind::Vec(inner)
            | LadTypeKind::HashSet(inner)
            | LadTypeKind::InteropResult(inner)
            | LadTypeKind::Array(inner, _) => collect_kinds(inner, seen),
            LadTypeKind::HashMap(key, value) => {
                collect_kinds(key, seen);
                collect_kinds(value, seen);
            }
            LadTypeKind::Tuple(inner) | LadTypeKind::Union(inner) => {
                inner.iter().for_each(|kind| collect_kinds(kind, seen))
            }
            LadTypeKind::TupleWithRest { head, rest } => {
                head.iter().for_each(|kind| collect_kinds(kind, seen));
                collect_kinds(rest, seen);
            }
            LadTypeKind::Ref(_)
            | LadTypeKind::Mut(_)
            | LadTypeKind::Val(_)
            | LadTypeKind::UntypedTuple
            | LadTypeKind::Primitive(_)
            | LadTypeKind::Unknown(_)
            | LadTypeKind::Fields { .. }
            | LadTypeKind::Param(_) => {}
        }
    }

    #[derive(bevy_asset::Asset, Reflect)]
    struct FieldAsset;

    #[derive(Reflect)]
    struct FieldValue;

    #[derive(Reflect)]
    struct FieldUnregistered;

    /// One field for each way a field can be typed, the registry is expected to know all of them but `FieldUnregistered`.
    #[derive(Reflect)]
    struct AllFieldKinds {
        primitive: f32,
        value: FieldValue,
        unknown: FieldUnregistered,
        vec: Vec<i32>,
        map: bevy_platform::collections::HashMap<String, f32>,
        set: bevy_platform::collections::HashSet<i32>,
        array: [u8; 4],
        option: Option<String>,
        result: Result<i32, String>,
        tuple: (i32, String),
        handle: bevy_asset::Handle<FieldAsset>,
        nested: Vec<bevy_platform::collections::HashSet<Option<i32>>>,
    }

    fn field_test_registry() -> TypeRegistry {
        use bevy_platform::collections::{HashMap, HashSet};

        let mut registry = TypeRegistry::default();
        registry.register::<FieldValue>();
        registry.register::<Vec<i32>>();
        registry.register::<HashMap<String, f32>>();
        registry.register::<HashSet<i32>>();
        registry.register::<[u8; 4]>();
        registry.register::<Option<String>>();
        registry.register::<Result<i32, String>>();
        registry.register::<(i32, String)>();
        registry.register::<Vec<HashSet<Option<i32>>>>();
        registry.register::<FieldAsset>();
        registry.register::<bevy_asset::Handle<FieldAsset>>();
        registry.register_type_data::<FieldAsset, bevy_asset::ReflectAsset>();
        registry
    }

    /// The kinds of all the fields of `AllFieldKinds`, by field name, as documented in the LAD file.
    fn documented_field_kinds(
        registry: &TypeRegistry,
    ) -> (
        HashMap<String, LadTypeKind>,
        impl FnMut(TypeId) -> LadTypeId + use<'_>,
    ) {
        let mut builder = LadFileBuilder::new_empty(registry);
        builder.add_type::<AllFieldKinds>();
        let lad_file = builder.build();

        let layout = lad_file
            .types
            .values()
            .find(|ty| ty.identifier == "AllFieldKinds")
            .map(|ty| ty.layout.clone())
            .expect("the type is added");
        let LadTypeLayout::MonoVariant(LadVariant::Struct { fields, .. }) = layout else {
            panic!("expected a struct, got {layout:?}");
        };

        (
            fields
                .into_iter()
                .map(|field| (field.name, field.type_))
                .collect(),
            move |type_id| builder.lad_id_from_type_id(type_id),
        )
    }

    #[test]
    fn fields_are_documented_with_the_kind_of_their_reflected_type() {
        use LadTypeKind as Kind;
        use bevy_mod_scripting_bindings_domain::ReflectionPrimitiveKind as Primitive;

        let registry = field_test_registry();
        let (kinds, mut id) = documented_field_kinds(&registry);

        let expected = [
            ("primitive", Kind::Primitive(Primitive::F32)),
            ("value", Kind::Val(id(TypeId::of::<FieldValue>()).into())),
            // types the registry does not know about can not be described any further
            (
                "unknown",
                Kind::Unknown(id(TypeId::of::<FieldUnregistered>())),
            ),
            ("vec", Kind::Vec(Box::new(Kind::Primitive(Primitive::I32)))),
            (
                "map",
                Kind::HashMap(
                    Box::new(Kind::Primitive(Primitive::String)),
                    Box::new(Kind::Primitive(Primitive::F32)),
                ),
            ),
            (
                "set",
                Kind::HashSet(Box::new(Kind::Primitive(Primitive::I32))),
            ),
            (
                "array",
                Kind::Array(Box::new(Kind::Primitive(Primitive::U8)), 4),
            ),
            (
                "option",
                Kind::Option(Box::new(Kind::Primitive(Primitive::String))),
            ),
            (
                "result",
                Kind::InteropResult(Box::new(Kind::Primitive(Primitive::I32))),
            ),
            (
                "tuple",
                Kind::Tuple(vec![
                    Kind::Primitive(Primitive::I32),
                    Kind::Primitive(Primitive::String),
                ]),
            ),
            // the same as the documentation of a bound handle
            (
                "handle",
                Kind::Val(id(TypeId::of::<bevy_asset::Handle<FieldAsset>>()).into()),
            ),
            (
                "nested",
                Kind::Vec(Box::new(Kind::HashSet(Box::new(Kind::Option(Box::new(
                    Kind::Primitive(Primitive::I32),
                )))))),
            ),
        ];

        assert_eq!(
            kinds.len(),
            expected.len(),
            "a field of `AllFieldKinds` has no expectation"
        );
        for (field, expected) in expected {
            pretty_assertions::assert_eq!(
                kinds.get(field),
                Some(&expected),
                "the field `{field}` is documented incorrectly"
            );
        }
    }

    /// The kinds which can only come from the signature of a function, as fields are always plain values
    fn documented_argument_kinds(
        registry: &TypeRegistry,
    ) -> Vec<(&'static str, LadTypeKind, LadTypeKind)> {
        use LadTypeKind as Kind;
        use bevy_mod_scripting_bindings::{
            VariadicTuple,
            docgen::TypedThrough,
            function::{
                bound::{BoundFields, BoundHandle},
                from::{M, R, Union, V},
            },
        };
        use bevy_mod_scripting_bindings_domain::ReflectionPrimitiveKind as Primitive;

        let mut builder = LadFileBuilder::new_empty(registry);
        let value = builder.lad_id_from_type_id(TypeId::of::<FieldValue>());
        let handle = builder.lad_id_from_type_id(TypeId::of::<bevy_asset::Handle<FieldAsset>>());

        let cases = vec![
            (
                "ref",
                R::<FieldValue>::through_type_info(),
                Kind::Ref(value.clone().into()),
            ),
            (
                "mut",
                M::<FieldValue>::through_type_info(),
                Kind::Mut(value.clone().into()),
            ),
            (
                "val",
                V::<FieldValue>::through_type_info(),
                Kind::Val(value.clone().into()),
            ),
            (
                "union",
                Union::<V<FieldValue>, f32>::through_type_info(),
                Kind::Union(vec![
                    Kind::Val(value.clone().into()),
                    Kind::Primitive(Primitive::F32),
                ]),
            ),
            (
                "variadic tuple",
                VariadicTuple::through_type_info(),
                Kind::UntypedTuple,
            ),
            (
                "tuple with rest",
                ThroughTypeInfo::TupleWithRest {
                    head: vec![V::<FieldValue>::through_type_info()],
                    rest: Box::new(f32::through_type_info()),
                },
                Kind::TupleWithRest {
                    head: vec![Kind::Val(value.clone().into())],
                    rest: Box::new(Kind::Primitive(Primitive::F32)),
                },
            ),
            (
                "fields",
                BoundFields::through_type_info().bind_placeholders(FieldValue::type_info(), None),
                Kind::Fields {
                    type_id: value.clone(),
                    variant: None,
                },
            ),
            (
                "fields of a variant",
                BoundFields::through_type_info()
                    .bind_placeholders(FieldValue::type_info(), Some("Variant")),
                Kind::Fields {
                    type_id: value.clone(),
                    variant: Some(Cow::Borrowed("Variant")),
                },
            ),
            (
                "bound handle",
                BoundHandle::through_type_info().bind_placeholders(FieldAsset::type_info(), None),
                Kind::Val(handle.into()),
            ),
            (
                "type info",
                ThroughTypeInfo::Opaque(FieldValue::type_info()),
                Kind::Unknown(value),
            ),
        ];

        cases
            .into_iter()
            .map(|(name, info, expected)| {
                (
                    name,
                    builder.lad_type_kind_from_through_type(&info),
                    expected,
                )
            })
            .collect()
    }

    #[test]
    fn arguments_are_documented_with_the_kind_of_their_signature() {
        let registry = field_test_registry();
        for (case, actual, expected) in documented_argument_kinds(&registry) {
            pretty_assertions::assert_eq!(actual, expected, "{case} is documented incorrectly");
        }
    }

    #[test]
    fn every_kind_is_documented_by_fields_or_arguments() {
        let registry = field_test_registry();
        let mut seen = std::collections::BTreeSet::new();

        let (field_kinds, _) = documented_field_kinds(&registry);
        for kind in field_kinds.values() {
            collect_kinds(kind, &mut seen);
        }
        for (_, kind, _) in documented_argument_kinds(&registry) {
            collect_kinds(&kind, &mut seen);
        }
        // parameters only appear in the layouts of generic types
        let (file, _) = one_param_file();
        for generic_type in file.generic_types.values() {
            generic_type.layout.for_each_variant(
                |variant, _| {
                    if let LadVariant::Struct { fields, .. } = variant {
                        fields
                            .iter()
                            .for_each(|f| collect_kinds(&f.type_, &mut seen));
                    }
                },
                (),
            );
        }

        let missing: Vec<_> = ALL_LAD_KINDS
            .iter()
            .filter(|kind| !seen.contains(*kind))
            .collect();
        assert!(
            missing.is_empty(),
            "kinds which neither the fields nor the arguments tests document: {missing:?}"
        );
    }

    #[derive(Reflect)]
    #[reflect(no_field_bounds, from_reflect = false)]
    struct TwoParams<P, Q> {
        #[reflect(ignore)]
        phantom: std::marker::PhantomData<(P, Q)>,
    }

    /// A generic type with fields of its parameter, nested in other types, and of other types
    #[derive(Reflect)]
    #[reflect(from_reflect = false)]
    struct OneParam<T> {
        value: T,
        list: Vec<T>,
        count: usize,
    }

    /// A generic type using its parameter where reflection only exposes type ids, and in other generic types
    #[derive(Reflect)]
    #[reflect(from_reflect = false)]
    struct NestedParam<
        T: bevy_asset::Asset
            + Eq
            + std::hash::Hash
            + bevy_reflect::FromReflect
            + bevy_reflect::GetTypeRegistration
            + Typed,
    > {
        set: bevy_platform::collections::HashSet<T>,
        nested: OneParam<T>,
        handle: bevy_asset::Handle<T>,
        handles: Vec<bevy_asset::Handle<T>>,
    }

    #[test]
    fn generic_types_are_declared_with_markers_in_place_of_their_parameters() {
        let registry = TypeRegistry::default();
        let mut builder = LadFileBuilder::new_empty(&registry);

        // each parameter has to be instantiated with a marker of its own
        let error = builder
            .add_generic_type::<OneParam<usize>>(&["T"], "")
            .err()
            .unwrap();
        assert!(
            error.contains("does not instantiate `T` with a marker"),
            "{error}"
        );
        let error = builder
            .add_generic_type::<TwoParams<A, A>>(&["P", "Q"], "")
            .err()
            .unwrap();
        assert!(
            error.contains("more than one parameter with the same marker"),
            "{error}"
        );
        // and named by the registration
        let error = builder
            .add_generic_type::<OneParam<A>>(&[], "")
            .err()
            .unwrap();
        assert!(
            error.contains("has 1 generic parameters, but 0 names"),
            "{error}"
        );

        // nothing was declared by the failed attempts
        assert!(builder.add_generic_type::<OneParam<A>>(&["T"], "").is_ok());
        assert_eq!(builder.generic_types.len(), 1);
    }

    /// The generic type `OneParam<T>`, declared and instantiated as `OneParam<f32>`
    fn one_param_file() -> (LadFile, LadTypeId) {
        let mut registry = TypeRegistry::default();
        registry.register::<OneParam<f32>>();
        registry.register::<Vec<f32>>();
        registry.register::<TwoParams<usize, f32>>();
        let mut builder = LadFileBuilder::new_empty(&registry);
        builder
            .add_generic_type::<OneParam<A>>(&["T"], "docs")
            .unwrap()
            .add_type::<OneParam<f32>>()
            .add_type::<TwoParams<usize, f32>>();
        let instantiation = builder.lad_id_from_type_id(TypeId::of::<OneParam<f32>>());
        (builder.build(), instantiation)
    }

    #[test]
    fn generic_types_describe_their_layout_in_terms_of_their_parameter() {
        let (file, instantiation) = one_param_file();
        let instantiation = &file.types[&instantiation];

        // instantiations match by path and number of parameters
        let generic_id = instantiation
            .generic_type
            .clone()
            .expect("an instantiation");
        assert_eq!(file.generic_types.len(), 1);
        assert!(
            file.types
                .values()
                .filter(|t| t.identifier == "TwoParams")
                .all(|t| t.generic_type.is_none())
        );

        // the parameter is named by the registration, and the fields of its type are the parameter
        let generic_type = &file.generic_types[&generic_id];
        assert_eq!(generic_type.params, ["T"]);
        assert_eq!(generic_type.identifier, "OneParam");
        assert_eq!(generic_type.documentation.as_deref(), Some("docs"));
        let LadTypeLayout::MonoVariant(LadVariant::Struct { fields, .. }) = &generic_type.layout
        else {
            panic!("expected a struct, got {:?}", generic_type.layout);
        };
        let param = || LadTypeKind::Param("T".into());
        let kinds: Vec<_> = fields.iter().map(|f| (f.name.as_str(), &f.type_)).collect();
        assert_eq!(
            kinds,
            [
                ("value", &param()),
                ("list", &LadTypeKind::Vec(Box::new(param()))),
                (
                    "count",
                    &LadTypeKind::Primitive(ReflectionPrimitiveKind::Usize)
                ),
            ]
        );

        // and the instantiation is the generic type with the parameter replaced by its argument, by name
        let mut declared = generic_type.layout.clone();
        let f32_kind = LadTypeKind::Primitive(ReflectionPrimitiveKind::F32);
        declared.replace_param("U", &f32_kind);
        assert_eq!(declared, generic_type.layout);
        declared.replace_param("T", &f32_kind);
        assert_eq!(field_kinds(&declared), field_kinds(&instantiation.layout));

        // sets only expose the type id of their values, declared generic types are applied to the parameter,
        // and undeclared ones can't be, so their uses of the parameter are unknown, by type path
        let registry = TypeRegistry::default();
        let mut builder = LadFileBuilder::new_empty(&registry);
        // declared before `Handle`, which its layout names all the same
        builder
            .add_generic_type::<NestedParam<A>>(&["T"], "")
            .unwrap()
            .add_generic_type::<bevy_asset::Handle<A>>(&["A"], "")
            .unwrap();
        let LadTypeLayout::MonoVariant(LadVariant::Struct { fields, .. }) =
            builder.generic_layout(0)
        else {
            panic!("expected a struct");
        };
        assert_eq!(fields[0].type_, LadTypeKind::HashSet(Box::new(param())));
        assert_eq!(
            fields[1].type_,
            LadTypeKind::Unknown(LadTypeId::new_string_id(
                <OneParam<A> as bevy_reflect::TypePath>::type_path().into()
            ))
        );
        let handle_of_param = LadNamedType::Generic {
            generic_type: LadTypeId::new_string_id("bevy_asset::handle::Handle".into()),
            args: vec![param()],
        };
        assert_eq!(fields[2].type_, LadTypeKind::Val(handle_of_param.clone()));
        assert_eq!(
            fields[3].type_,
            LadTypeKind::Vec(Box::new(LadTypeKind::Val(handle_of_param)))
        );

        // as for the generic types declared by the plugin
        let registry = field_test_registry();
        let mut builder = LadFileBuilder::new_empty(&registry);
        builder
            .add_generic_type::<bevy_asset::Handle<A>>(&["A"], "")
            .unwrap();
        let handle = <bevy_asset::Handle<FieldAsset> as Typed>::type_info();
        let declared = builder.instantiated_layout(handle).unwrap();
        let reflected = builder.lad_layout_from_type_info(handle);
        assert_eq!(field_kinds(&declared), field_kinds(&reflected));
    }
}
