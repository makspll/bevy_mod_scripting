//! Parsing definitions for the LAD (Language Agnostic Decleration) file format.
pub mod plugin;

use bevy_ecs::{
    reflect::{ReflectComponent, ReflectResource},
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
        bound::asset_handle_type_info, namespace::FunctionScope,
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

/// A builder for constructing LAD files.
/// This should be your preferred way of constructing LAD files.
pub struct LadFileBuilder<'t> {
    file: LadFile,
    type_id_mapping: HashMap<TypeId, LadTypeId>,
    type_registry: &'t TypeRegistry,
    sorted: bool,
    exclude_types_involving_unregistered_types: bool,
    generic_types: Vec<GenericTypeSpec>,
}

/// A generic type declared with [`LadFileBuilder::add_generic_type`]
struct GenericTypeSpec {
    /// The path of the type, without generics
    path: String,
    /// The names of the parameters
    params: Vec<String>,
    documentation: String,
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

    /// Add a global instance to the LAD file.
    ///
    /// Requires the type to be registered via [`Self::add_type`] or [`Self::add_type_info`] first to provide rich type information.
    ///
    /// If `is_static` is true, the instance will be treated as a static instance
    /// and hence not support method call syntax or method calls (i.e. only functions without a self parameter can be called on them).
    pub fn add_instance<T: 'static + TypedThrough>(
        &mut self,
        key: impl Into<Cow<'static, str>>,
        is_static: bool,
    ) -> &mut Self {
        let type_info = T::through_type_info();
        let type_kind = self.lad_type_kind_from_through_type(&type_info);
        self.file.builtins.insert(
            key.into(),
            LadInstance {
                type_kind,
                is_static,
            },
        );
        self
    }

    /// An untyped version of [`Self::add_instance`].
    ///
    /// Adds a global instance to the LAD file.
    pub fn add_instance_dynamic(
        &mut self,
        key: impl Into<Cow<'static, str>>,
        is_static: bool,
        through_type: ThroughTypeInfo,
    ) -> &mut Self {
        let type_kind = self.lad_type_kind_from_through_type(&through_type);
        self.file.builtins.insert(
            key.into(),
            LadInstance {
                type_kind,
                is_static,
            },
        );
        self
    }

    /// Adds a global instance to the LAD file with a custom lad type kind.
    pub fn add_instance_manually(
        &mut self,
        key: impl Into<Cow<'static, str>>,
        is_static: bool,
        type_kind: LadTypeKind,
    ) -> &mut Self {
        self.file.builtins.insert(
            key.into(),
            LadInstance {
                type_kind,
                is_static,
            },
        );
        self
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
        }

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
            layout: self.lad_layout_from_type_info(type_info),
            generated,
            insignificance,
            metadata: LadTypeMetadata {
                is_component,
                is_resource,
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
    /// The pattern is the path of the type, followed by the names of its parameters prefixed with `$`,
    /// i.e. `bevy_asset::handle::Handle<$A>`. It is matched against the instantiations in the type registry, which
    /// only expose the arguments of a generic type and not its definition, so the layout of the type is not described.
    ///
    /// # Errors
    /// If the pattern is malformed.
    pub fn add_generic_type(
        &mut self,
        pattern: &str,
        documentation: &str,
    ) -> Result<&mut Self, String> {
        let (path, params) = pattern
            .trim()
            .strip_suffix('>')
            .and_then(|pattern| pattern.split_once('<'))
            .ok_or_else(|| format!("`{pattern}` is not of the form `path::Type<$A, $B>`"))?;
        let params = params
            .split(',')
            .map(|param| {
                param
                    .trim()
                    .strip_prefix('$')
                    .filter(|name| !name.is_empty())
                    .map(str::to_owned)
                    .ok_or_else(|| format!("`{param}` in `{pattern}` is not a `$Name`"))
            })
            .collect::<Result<Vec<_>, _>>()?;
        self.generic_types.push(GenericTypeSpec {
            path: path.to_owned(),
            params,
            documentation: documentation.to_owned(),
        });
        Ok(self)
    }

    /// If the type is an instantiation of a declared generic type, adds the generic type to the file and returns its id.
    fn add_generic_type_of(
        &mut self,
        type_info: &TypeInfo,
        crate_: &Option<String>,
        namespace: &str,
    ) -> Option<LadTypeId> {
        let path = type_info.type_path_table().path();
        let generic_path = path.split('<').next().unwrap_or(path);
        let spec = self.generic_types.iter().find(|spec| {
            spec.path == generic_path && spec.params.len() == type_info.generics().len()
        })?;

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
            });
        Some(id)
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
    pub fn add_function_info(&mut self, function_info: &FunctionInfo) -> &mut Self {
        let function_id = self.lad_function_id_from_info(function_info);
        let namespace = match function_info.namespace {
            FunctionScope::Global => LadFunctionNamespace::Global,
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
        let receiver = match &namespace {
            LadFunctionNamespace::Type(type_id) => arguments.first().is_some_and(|arg| {
                matches!(&arg.kind, LadTypeKind::Ref(id) | LadTypeKind::Mut(id) | LadTypeKind::Val(id) if id == type_id)
            }),
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

        // associate functions on type namespaces with their types
        for (function_id, function) in file.functions.iter() {
            match &function.namespace {
                LadFunctionNamespace::Type(type_id) => {
                    if let Some(t) = file.types.get_mut(type_id) {
                        if t.generic_type.is_some() {
                            warn!(
                                "Function {} is on {}, an instantiation of a generic type which is documented as opaque, so the function is not documented by backends.",
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
                    type_: self.lad_type_kind_from_type_id(field.type_id()),
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
                    type_: self.lad_type_kind_from_type_id(field.type_id()),
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
            bevy_mod_scripting_bindings::function::namespace::FunctionScope::Global => {
                "".to_string()
            }
            bevy_mod_scripting_bindings::function::namespace::FunctionScope::OnType(type_id) => {
                self.lad_id_from_type_id(type_id).to_string()
            }
        };

        LadFunctionId::new_string_id(format!("{}::{}", namespace_string, function_info.name))
    }

    fn lad_type_kind_from_through_type(&mut self, through_type: &ThroughTypeInfo) -> LadTypeKind {
        self.lad_kind(through_type, false)
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
        match through_type {
            ThroughTypeInfo::Ref(type_info) => LadTypeKind::Ref(leaf(self, type_info)),
            ThroughTypeInfo::Mut(type_info) => LadTypeKind::Mut(leaf(self, type_info)),
            ThroughTypeInfo::Val(type_info) => LadTypeKind::Val(leaf(self, type_info)),
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
                    Some(handle_type_info) => LadTypeKind::Val(leaf(self, handle_type_info)),
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

    use bevy_mod_scripting_bindings::{
        Union, V,
        docgen::info::GetFunctionInfo,
        function::{
            from::R,
            namespace::{GlobalNamespace, IntoFunctionScope},
        },
    };
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

        /// I am a generic type, which the builder is told about so only my arguments are known
        #[derive(Reflect)]
        #[reflect(no_field_bounds, from_reflect = false)]
        struct OpaqueGenericType<T> {
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
            PlainStructType::into_function_scope(),
        );

        let function = |_: ReflectReference, _: usize| 2usize;
        let function_info = function
            .get_function_info(
                "hello_world".into(),
                GenericStructType::<usize>::into_function_scope(),
            )
            .with_docs("hello docs");

        let function_with_complex_args =
            |_: ReflectReference, _: (usize, String), _: Option<Vec<R<EnumType>>>| 2usize;
        let function_with_complex_args_info = function_with_complex_args
            .get_function_info("hello_world".into(), GenericStructType::<usize>::into_function_scope())
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
            .get_function_info("hello_world".into(), GlobalNamespace::into_function_scope())
            .with_arg_names(&["arg1"]);

        impl TypedThrough for OpaqueGenericType<usize> {
            fn through_type_info() -> ThroughTypeInfo {
                ThroughTypeInfo::Opaque(Self::type_info())
            }
        }
        let generic_function = |_: R<HoldsOpaqueType>, _: V<OpaqueGenericType<usize>>| {
            V::new(OpaqueGenericType::<usize> {
                phantom: std::marker::PhantomData,
            })
        };
        let generic_function_info = generic_function.get_function_info(
            "swap_generic".into(),
            HoldsOpaqueType::into_function_scope(),
        );

        let mut lad_file = LadFileBuilder::new(&type_registry)
            .set_description("## Hello gentlemen\n I am  markdown file.\n - hello\n - world")
            .set_sorted(true)
            .add_generic_type(
                "ladfile_builder::test::OpaqueGenericType<$T>",
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
            .add_instance::<V<GenericStructType<usize>>>("my_static_instance", true)
            .add_instance::<Vec<V<UnitType>>>("my_non_static_instance", false)
            .add_instance::<HashMap<String, Union<String, String>>>("map", false)
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
            .get_function_info(Cow::Borrowed("load"), FunctionScope::Global)
            .bind_placeholders(TestAsset::type_info(), None);

        let lad_file = LadFileBuilder::new_empty(&type_registry)
            .add_function_info(&info)
            .build();

        let function = lad_file
            .functions
            .values()
            .next()
            .expect("the function is added");
        let LadTypeKind::Val(handle) = &function.return_type.kind else {
            panic!("expected a value, got {:?}", function.return_type.kind);
        };
        assert!(
            handle.to_string().contains("Handle<"),
            "expected the handle type, got {handle}"
        );
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
        Ref, Mut, Val, Option, Vec, HashMap, HashSet, InteropResult, Tuple, UntypedTuple, Array,
        Primitive, Union, Unknown, Fields
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
            LadTypeKind::Ref(_)
            | LadTypeKind::Mut(_)
            | LadTypeKind::Val(_)
            | LadTypeKind::UntypedTuple
            | LadTypeKind::Primitive(_)
            | LadTypeKind::Unknown(_)
            | LadTypeKind::Fields { .. } => {}
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
            ("value", Kind::Val(id(TypeId::of::<FieldValue>()))),
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
                Kind::Val(id(TypeId::of::<bevy_asset::Handle<FieldAsset>>())),
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
                Kind::Ref(value.clone()),
            ),
            (
                "mut",
                M::<FieldValue>::through_type_info(),
                Kind::Mut(value.clone()),
            ),
            (
                "val",
                V::<FieldValue>::through_type_info(),
                Kind::Val(value.clone()),
            ),
            (
                "union",
                Union::<V<FieldValue>, f32>::through_type_info(),
                Kind::Union(vec![
                    Kind::Val(value.clone()),
                    Kind::Primitive(Primitive::F32),
                ]),
            ),
            (
                "variadic tuple",
                VariadicTuple::through_type_info(),
                Kind::UntypedTuple,
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
                Kind::Val(handle),
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
    struct TwoParams<A, B> {
        #[reflect(ignore)]
        phantom: std::marker::PhantomData<(A, B)>,
    }

    #[test]
    fn malformed_generic_patterns_are_rejected() {
        let registry = TypeRegistry::default();
        let mut builder = LadFileBuilder::new_empty(&registry);

        // parameters are needed
        let error = builder.add_generic_type("a::b::C", "").err().unwrap();
        assert!(error.contains("is not of the form"), "{error}");
        let error = builder.add_generic_type("a::b::C<", "").err().unwrap();
        assert!(error.contains("is not of the form"), "{error}");

        // and they are prefixed with a dollar
        let error = builder.add_generic_type("a::b::C<A>", "").err().unwrap();
        assert!(error.contains("is not a `$Name`"), "{error}");
        let error = builder
            .add_generic_type("a::b::C<$A, $>", "")
            .err()
            .unwrap();
        assert!(error.contains("is not a `$Name`"), "{error}");

        // nothing was declared by the failed attempts
        assert!(builder.add_generic_type("a::b::C<$A, $B>", "").is_ok());
        assert_eq!(builder.generic_types.len(), 1);
    }

    #[test]
    fn generic_types_match_by_path_and_number_of_parameters() {
        let mut registry = TypeRegistry::default();
        registry.register::<TwoParams<usize, f32>>();

        for (pattern, matches) in [
            ("ladfile_builder::test::TwoParams<$First, $Second>", true),
            // the number of parameters has to match
            ("ladfile_builder::test::TwoParams<$A>", false),
            ("ladfile_builder::test::TwoParams<$A, $B, $C>", false),
            // and the path, which includes the module
            ("ladfile_builder::test::Other<$A, $B>", false),
            ("TwoParams<$A, $B>", false),
        ] {
            let mut builder = LadFileBuilder::new_empty(&registry);
            builder
                .add_generic_type(pattern, "docs")
                .unwrap()
                .add_type::<TwoParams<usize, f32>>();
            let file = builder.build();

            let instantiation = file.types.values().next().unwrap();
            assert_eq!(instantiation.generic_type.is_some(), matches, "{pattern}");
            assert_eq!(file.generic_types.len(), matches as usize, "{pattern}");

            if matches {
                // the parameters are named by the pattern
                let generic_type = file.generic_types.values().next().unwrap();
                assert_eq!(generic_type.params, ["First", "Second"]);
                assert_eq!(generic_type.identifier, "TwoParams");
                assert_eq!(generic_type.documentation.as_deref(), Some("docs"));
            }
        }
    }
}
