//! Generic types: their declarations, layouts in terms of their parameters, and how their instantiations are named.
//!
//! Reflection only sees instantiations (`Time<Virtual>`), so generic types are declared with a marker in place of each
//! parameter (`Time<A>`), and described from that, see [`LadFileBuilder::add_generic_type`].

use std::any::TypeId;

use bevy_mod_scripting_bindings::function::generics::A;
use bevy_platform::collections::HashSet;
use bevy_reflect::{GenericInfo, TypeInfo, Typed};
use ladfile::*;

use crate::LadFileBuilder;

/// Whether the type stands for a generic parameter when declaring a generic type, see [`LadFileBuilder::add_generic_type`].
pub(crate) fn is_param_marker(type_id: TypeId) -> bool {
    type_id == TypeId::of::<A>()
}

/// The parameter a marker stands for, named by the marker's path (which no declared name is) until
/// [`LadFileBuilder::make_or_get_generic_layout`] names it as declared.
pub(crate) fn marker_param(type_path: &str) -> LadTypeKind {
    LadTypeKind::Param(type_path.to_owned())
}

/// Collects the generic types the kind applies to arguments, i.e. `Handle` for `Vec<Handle<T>>` or `Fields<Handle<T>>`.
fn named_generic_types(kind: &LadTypeKind, named: &mut Vec<LadTypeId>) {
    kind.for_each(&mut |kind| {
        let named_type = match kind {
            LadTypeKind::Fields { of, .. } => Some(of),
            kind => kind.named(),
        };
        if let Some(LadNamedType::Generic { generic_type, .. }) = named_type {
            named.push(generic_type.clone());
        }
    })
}

/// Collects the generic types the fields of the layout apply to arguments, see [`named_generic_types`].
fn named_generic_types_in_layout(layout: &LadTypeLayout, named: &mut Vec<LadTypeId>) {
    for (_, kinds) in field_kinds(layout) {
        kinds
            .into_iter()
            .for_each(|kind| named_generic_types(kind, named));
    }
}

/// The kinds of the fields of each variant, in order.
pub(crate) fn field_kinds(layout: &LadTypeLayout) -> Vec<(&str, Vec<&LadTypeKind>)> {
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

/// A generic type declared with [`LadFileBuilder::add_generic_type`]
pub(crate) struct GenericTypeSpec {
    /// The path of the type, without generics
    path: String,
    /// The identifier of the type, without generics
    identifier: String,
    /// The crate defining the type
    crate_: Option<String>,
    /// The namespace of the type, the one of its instantiations
    namespace: String,
    /// The names of the parameters
    params: Vec<String>,
    documentation: String,
    /// The type instantiated with markers, which the layout is described from
    type_info: &'static TypeInfo,
    /// The layout, in terms of the parameters, described once all generic types are declared, see [`LadFileBuilder::make_or_get_generic_layout`]
    layout: Option<LadTypeLayout>,
}

impl GenericTypeSpec {
    /// The id of the generic type in the file, its path
    fn id(&self) -> LadTypeId {
        LadTypeId::new_string_id(self.path.clone().into())
    }
}

impl LadFileBuilder<'_> {
    /// Declares a generic type, so its instantiations are documented as such instead of as unrelated types.
    ///
    /// `G` is the generic type instantiated with a marker in place of each parameter, i.e. `Handle<A>`, `namespace` is the
    /// one of its instantiations, i.e. `asset`, and `params` names the parameters in order. Reflection can't tell which fields of an instantiation like `Time<Virtual>` are of a parameter's
    /// type, but in `Time<A>` those are exactly the fields of type `A`, so the layout is described from `G`.
    /// Instantiations in the type registry are matched by path and number of parameters.
    ///
    /// The only marker so far is [`A`], so only types with one parameter can be declared.
    ///
    /// # Errors
    /// If a parameter of `G` is not instantiated with a marker, two share one, or `params` doesn't name each of them.
    pub fn add_generic_type<G: Typed>(
        &mut self,
        namespace: &str,
        params: &[&str],
        documentation: &str,
    ) -> Result<&mut Self, String> {
        self.add_generic_type_info(G::type_info(), namespace, params, documentation)
    }

    /// The same as [`Self::add_generic_type`], for a generic type known by its type information, i.e. from [`bevy_mod_scripting_bindings::function::generics::AppGenericTypes`].
    ///
    /// # Errors
    /// See [`Self::add_generic_type`].
    pub fn add_generic_type_info(
        &mut self,
        type_info: &'static TypeInfo,
        namespace: &str,
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
        let mut markers = HashSet::new();
        for (generic, name) in generics.iter().zip(params) {
            if !is_param_marker(generic.type_id()) {
                return Err(format!(
                    "`{path}` does not instantiate `{name}` with a marker, i.e. `A`"
                ));
            }
            if !markers.insert(generic.type_id()) {
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
            namespace: namespace.to_owned(),
            params: params.iter().map(|&name| name.to_owned()).collect(),
            documentation: documentation.to_owned(),
            type_info,
            layout: None,
        });
        // layouts name the declared generic types they use, i.e. `Handle<T>`, so they are described again
        // with the new one known, whatever order they are declared in
        for spec in &mut self.generic_types {
            spec.layout = None;
        }
        Ok(self)
    }

    /// The layout of the declared generic type, described from its instantiation with markers and cached the first time it's needed.
    pub(crate) fn make_or_get_generic_layout(&mut self, index: usize) -> LadTypeLayout {
        if let Some(layout) = &self.generic_types[index].layout {
            return layout.clone();
        }
        let type_info = self.generic_types[index].type_info;
        let mut layout = self.lad_layout_from_type_info(type_info);
        for (generic, name) in type_info
            .generics()
            .iter()
            .zip(&self.generic_types[index].params)
        {
            layout.replace_param(generic.type_path(), &LadTypeKind::Param(name.clone()));
        }
        self.generic_types[index].layout = Some(layout.clone());
        layout
    }

    /// The declared generic type as documented in the file, see [`Self::make_or_get_generic_layout`].
    fn make_lad_generic_type(&mut self, index: usize) -> LadGenericType {
        let layout = self.make_or_get_generic_layout(index);
        let spec = &self.generic_types[index];
        LadGenericType {
            identifier: spec.identifier.clone(),
            crate_: spec.crate_.clone(),
            path: spec.path.clone(),
            namespace: spec.namespace.clone(),
            params: spec.params.clone(),
            documentation: Some(spec.documentation.clone()),
            layout,
            instantiations: Vec::new(),
        }
    }

    /// If the type is an instantiation of a declared generic type, adds the generic type to the file and returns its id.
    pub(crate) fn add_generic_type_of(&mut self, type_info: &TypeInfo) -> Option<LadTypeId> {
        let index = self.matches_against_declared_generic_shape(type_info)?;
        let id = self.generic_types[index].id();
        if !self.file.generic_types.contains_key(&id) {
            let generic_type = self.make_lad_generic_type(index);
            self.file.generic_types.insert(id.clone(), generic_type);
        }
        Some(id)
    }

    /// Records the instantiation on its generic type, with its arguments and metadata, as it isn't a type of the file.
    pub(crate) fn record_instantiation(
        &mut self,
        generic_type: &LadTypeId,
        type_info: &TypeInfo,
        metadata: LadTypeMetadata,
    ) {
        let args: Vec<_> = type_info
            .generics()
            .iter()
            .map(|arg| self.lad_generic_arg_kind(arg, false))
            .collect();
        if let Some(generic) = self.file.generic_types.get_mut(generic_type)
            && !generic.instantiations.iter().any(|i| i.args == args)
        {
            generic.instantiations.push(LadInstantiation {
                args,
                associated_functions: Vec::new(),
                metadata,
            });
        }
    }

    /// The index of the declared generic type the type is an instantiation of, matched by path and number of parameters
    pub(crate) fn matches_against_declared_generic_shape(
        &self,
        type_info: &TypeInfo,
    ) -> Option<usize> {
        let path = type_info.type_path_table().path();
        let generic_path = path.split('<').next().unwrap_or(path);
        self.generic_types.iter().position(|spec| {
            spec.path == generic_path && spec.params.len() == type_info.generics().len()
        })
    }

    /// The id of the declared generic type the type is an instantiation of, see [`Self::matches_against_declared_generic_shape`]
    pub(crate) fn matches_against_declared_generic_type(
        &self,
        type_info: &TypeInfo,
    ) -> Option<LadTypeId> {
        self.matches_against_declared_generic_shape(type_info)
            .map(|index| self.generic_types[index].id())
    }

    /// The kind of an argument of a generic type, as a field of its type would be described, or the parameter a marker stands for.
    /// Also adds the type to the file if `register` is set.
    pub(crate) fn lad_generic_arg_kind(
        &mut self,
        arg: &GenericInfo,
        register: bool,
    ) -> LadTypeKind {
        if is_param_marker(arg.type_id()) {
            return marker_param(arg.type_path());
        }
        // reflection only gives the arguments as type ids
        let type_info = self.type_registry.get_type_info(arg.type_id());
        if register && let Some(type_info) = type_info {
            self.add_type_info(type_info);
        }
        self.lad_field_kind(arg.type_id(), type_info)
    }

    /// Adds the declared generic types the file names in layouts and signatures but has no instantiations of,
    /// which would otherwise have added them, and those their layouts name in turn.
    pub(crate) fn add_named_generic_types(&mut self, file: &mut LadFile) {
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
            named_generic_types_in_layout(layout, &mut named);
        }
        while let Some(id) = named.pop() {
            if file.generic_types.contains_key(&id) {
                continue;
            }
            let Some(index) = self.generic_types.iter().position(|spec| spec.id() == id) else {
                continue;
            };
            let generic_type = self.make_lad_generic_type(index);
            named_generic_types_in_layout(&generic_type.layout, &mut named);
            file.generic_types.insert(id, generic_type);
        }
    }
}
