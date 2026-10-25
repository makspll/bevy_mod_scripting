//! Defines a visitor for function arguments of the `LAD` format.

use std::path::PathBuf;

use ladfile::{ArgumentVisitor, LadTypeId, LadVisitable};

use crate::markdown::MarkdownBuilder;

pub(crate) struct MarkdownArgumentVisitor<'a> {
    ladfile: &'a ladfile::LadFile,
    buffer: MarkdownBuilder,
    linkifier: Option<Box<dyn Fn(String) -> PathBuf + 'static>>,
    pub raw_type_id_replacement: Option<&'static str>,
}
impl<'a> MarkdownArgumentVisitor<'a> {
    /// Create a new instance of the visitor
    pub fn new(ladfile: &'a ladfile::LadFile) -> Self {
        let mut builder = MarkdownBuilder::new();
        builder.tight_inline().set_escape_mode(false);
        Self {
            ladfile,
            buffer: builder,
            linkifier: None,
            raw_type_id_replacement: None,
        }
    }

    /// Create a new instance of the visitor with a custom linkifier function
    pub fn new_with_linkifier<F: Fn(String) -> PathBuf + 'static>(
        ladfile: &'a ladfile::LadFile,
        linkifier: F,
    ) -> Self {
        let mut without = Self::new(ladfile);
        without.linkifier = Some(Box::new(linkifier));
        without
    }

    /// Set the raw type id replacement
    pub fn with_raw_type_id_replacement(mut self, replacement: &'static str) -> Self {
        self.raw_type_id_replacement = Some(replacement);
        self
    }

    pub fn build(mut self) -> String {
        self.buffer.build()
    }

    /// Writes the text, linked to the page of `target` if there is a linkifier
    fn link_or_text(&mut self, text: String, target: String) {
        if let Some(linkifier) = &self.linkifier {
            let link_value = (linkifier)(target);
            let link_value = link_value.to_string_lossy().to_string().replace("\\", "/");
            self.buffer.link(text, link_value);
        } else {
            self.buffer.text(text);
        }
    }
}

impl ArgumentVisitor for MarkdownArgumentVisitor<'_> {
    type Output = ();

    fn visit_primitive(&mut self, primitive_kind: &ladfile::ReflectionPrimitiveKind) {
        let prim_id = primitive_kind.to_string();
        self.link_or_text(prim_id.clone(), prim_id);
    }

    fn visit_type_id(&mut self, type_id: &ladfile::LadTypeId) {
        // the generic type itself, i.e. Handle<A>
        if let Some(generic_type) = self.ladfile.generic_types.get(type_id) {
            let name = generic_type.name_with_params();
            self.link_or_text(name.clone(), name);
            return;
        }

        // Write identifier<Generic1TypeIdentifier, Generic2TypeIdentifier>
        let generics = self.ladfile.get_type_generics(type_id);

        let type_identifier = self
            .ladfile
            .get_type_identifier(type_id, self.raw_type_id_replacement);
        if let Some(generics) = generics {
            // instantiations of generic types link to the page of the generic type
            if let Some((generic_type, _)) = self.ladfile.get_generic_instance(type_id) {
                self.link_or_text(type_identifier.to_string(), generic_type.name_with_params());
            } else {
                self.buffer.text(type_identifier);
            }
            self.buffer.text('<');
            for (i, generic) in generics.iter().enumerate() {
                if i > 0 {
                    self.buffer.text(", ");
                }
                self.visit_type_id(&generic.type_id);
            }
            self.buffer.text('>');
        } else if let Some(linkifier) = &self.linkifier {
            // link the type, by building a string of the type linked to first
            let mut sub_visitor = MarkdownArgumentVisitor::new(self.ladfile);
            sub_visitor.raw_type_id_replacement = self.raw_type_id_replacement;
            type_id.accept(&mut sub_visitor);
            let linked_string = sub_visitor.build();
            let link_value = (linkifier)(linked_string);
            let link_value = link_value.to_string_lossy().to_string().replace("\\", "/");
            self.buffer.link(type_identifier, link_value);
        } else {
            self.buffer.text(type_identifier);
        }
    }

    fn visit_option(&mut self, inner: &ladfile::LadTypeKind) {
        self.buffer.text("Optional<");
        self.visit(inner);
        self.buffer.text(">");
    }

    fn visit_vec(&mut self, inner: &ladfile::LadTypeKind) {
        self.buffer.text("Vec<");
        self.visit(inner);
        self.buffer.text(">");
    }

    fn visit_hash_map(&mut self, key: &ladfile::LadTypeKind, value: &ladfile::LadTypeKind) {
        self.buffer.text("HashMap<");
        self.visit(key);
        self.buffer.text(", ");
        self.visit(value);
        self.buffer.text(">");
    }

    fn visit_hash_set(&mut self, inner: &ladfile::LadTypeKind) {
        self.visit(inner);
    }

    fn visit_tuple(&mut self, inner: &[ladfile::LadTypeKind]) {
        self.buffer.text("(");
        for (idx, arg) in inner.iter().enumerate() {
            self.visit(arg);
            if idx < inner.len() - 1 {
                self.buffer.text(", ");
            }
        }
        self.buffer.text(")");
    }

    fn visit_untyped_tuple(&mut self) {}

    fn visit_tuple_with_rest(
        &mut self,
        head: &[ladfile::LadTypeKind],
        rest: &ladfile::LadTypeKind,
    ) {
        self.buffer.text("(");
        for arg in head {
            self.visit(arg);
            self.buffer.text(", ");
        }
        self.buffer.text("...");
        self.visit(rest);
        self.buffer.text(")");
    }

    fn visit_union(&mut self, inner: &[ladfile::LadTypeKind]) {
        for (idx, arg) in inner.iter().enumerate() {
            self.visit(arg);
            if idx < inner.len() - 1 {
                self.buffer.text(" | ");
            }
        }
    }

    fn visit_array(&mut self, inner: &ladfile::LadTypeKind, size: usize) {
        self.buffer.text("[");
        self.visit(inner);
        self.buffer.text("; ");
        self.buffer.text(size.to_string());
        self.buffer.text("]");
    }

    fn visit_fields(&mut self, type_id: &LadTypeId, variant: Option<&str>) {
        self.buffer.text("Fields<");
        self.visit_type_id(type_id);
        if let Some(variant) = variant {
            self.buffer.text("::");
            self.buffer.text(variant);
        }
        self.buffer.text(">");
    }

    fn visit_param(&mut self, param: &str) {
        self.buffer.text(param);
    }

    fn visit_fields_of(&mut self, param: &str) {
        self.buffer.text("Fields<");
        self.buffer.text(param);
        self.buffer.text(">");
    }

    fn visit_generic(&mut self, generic_type: &LadTypeId, args: &[ladfile::LadTypeKind]) {
        let name = self
            .ladfile
            .generic_types
            .get(generic_type)
            .map(|generic| (generic.identifier.clone(), generic.name_with_params()));
        match name {
            Some((identifier, target)) => self.link_or_text(identifier, target),
            None => {
                self.buffer.text(generic_type.to_string());
            }
        }
        self.buffer.text("<");
        for (idx, arg) in args.iter().enumerate() {
            if idx > 0 {
                self.buffer.text(", ");
            }
            self.visit(arg);
        }
        self.buffer.text(">");
    }

    fn visit_type_of(
        &mut self,
        bound: ladfile::LadTypeBound,
        param: &str,
        _carrier: &ladfile::LadTypeKind,
    ) {
        self.buffer.text(match bound {
            ladfile::LadTypeBound::Type => "Type<",
            ladfile::LadTypeBound::Component => "ComponentType<",
            ladfile::LadTypeBound::Resource => "ResourceType<",
            ladfile::LadTypeBound::Bundle => "BundleType<",
            ladfile::LadTypeBound::Asset => "AssetType<",
        });
        self.buffer.text(param);
        self.buffer.text(">");
    }
}

#[cfg(test)]
mod test {
    use ladfile::{LadTypeKind, ReflectionPrimitiveKind};

    use super::*;

    fn setup_ladfile() -> ladfile::LadFile {
        // load test file from ../../../ladfile_builder/test_assets/
        let ladfile = ladfile::EXAMPLE_LADFILE;
        ladfile::parse_lad_file(ladfile).unwrap()
    }

    fn plain_struct_id() -> LadTypeId {
        LadTypeId::new_string_id("ladfile_builder::test::PlainStructType".into())
    }

    fn generic_struct_id() -> LadTypeId {
        LadTypeId::new_string_id("ladfile_builder::test::GenericStructType<usize>".into())
    }

    #[test]
    fn test_linkifier_visitor_creates_links() {
        let ladfile = setup_ladfile();

        let mut visitor = MarkdownArgumentVisitor::new_with_linkifier(&ladfile, |str| {
            PathBuf::from("root\\asd").join(str)
        });

        let second_type_id = &generic_struct_id();
        visitor.visit_type_id(second_type_id);
        assert_eq!(
            visitor.buffer.build(),
            "GenericStructType<[Usize](root/asd/Usize)>"
        );
    }

    #[test]
    fn test_visit_type_id() {
        let ladfile = setup_ladfile();

        let first_type_id = &plain_struct_id();
        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);

        visitor.visit_type_id(first_type_id);
        assert_eq!(visitor.buffer.build(), "PlainStructType");

        visitor.buffer.clear();

        let second_type_id = &generic_struct_id();
        visitor.visit_type_id(second_type_id);
        assert_eq!(visitor.buffer.build(), "GenericStructType<Usize>");
    }

    #[test]
    fn test_visit_generic_instantiation_links_the_generic_type() {
        let ladfile = setup_ladfile();
        let instantiation =
            LadTypeId::new_string_id("ladfile_builder::test::OpaqueGenericType<usize>".into());
        let generic_type =
            LadTypeId::new_string_id("ladfile_builder::test::OpaqueGenericType".into());

        // the instantiation links to the generic type, and its arguments to their own pages
        let mut visitor = MarkdownArgumentVisitor::new_with_linkifier(&ladfile, |str| {
            PathBuf::from("root\\asd").join(str)
        });
        visitor.visit_type_id(&instantiation);
        assert_eq!(
            visitor.buffer.build(),
            "[OpaqueGenericType](root/asd/OpaqueGenericType<T>)<[Usize](root/asd/Usize)>"
        );

        // and the generic type is printed with its parameters
        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);
        visitor.visit_type_id(&generic_type);
        assert_eq!(visitor.buffer.build(), "OpaqueGenericType<T>");

        // types which were not declared as generic types are not linked
        let mut visitor = MarkdownArgumentVisitor::new_with_linkifier(&ladfile, |str| {
            PathBuf::from("root\\asd").join(str)
        });
        visitor.visit_type_id(&generic_struct_id());
        assert_eq!(
            visitor.buffer.build(),
            "GenericStructType<[Usize](root/asd/Usize)>"
        );
    }

    #[test]
    fn test_visit_ref() {
        let ladfile = setup_ladfile();

        let first_type_id = &plain_struct_id();
        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);

        visitor.visit(&LadTypeKind::Ref(first_type_id.clone()));
        assert_eq!(visitor.buffer.build(), "PlainStructType");
    }

    #[test]
    fn test_visit_mut() {
        let ladfile = setup_ladfile();

        let first_type_id = &plain_struct_id();
        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);

        visitor.visit(&LadTypeKind::Mut(first_type_id.clone()));
        assert_eq!(visitor.buffer.build(), "PlainStructType");
    }

    #[test]
    fn test_visit_val() {
        let ladfile = setup_ladfile();

        let first_type_id = &plain_struct_id();
        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);

        visitor.visit(&LadTypeKind::Val(first_type_id.clone()));
        assert_eq!(visitor.buffer.build(), "PlainStructType");
    }

    #[test]
    fn test_visit_option() {
        let ladfile = setup_ladfile();

        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);

        visitor.visit(&LadTypeKind::Option(Box::new(LadTypeKind::Primitive(
            ReflectionPrimitiveKind::Bool,
        ))));
        assert_eq!(visitor.buffer.build(), "Optional<Bool>");
    }

    #[test]
    fn test_visit_vec() {
        let ladfile = setup_ladfile();

        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);

        visitor.visit(&LadTypeKind::Vec(Box::new(LadTypeKind::Primitive(
            ReflectionPrimitiveKind::Bool,
        ))));
        assert_eq!(visitor.buffer.build(), "Vec<Bool>");
    }

    #[test]
    fn test_visit_hash_map() {
        let ladfile = setup_ladfile();

        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);

        visitor.visit(&LadTypeKind::HashMap(
            Box::new(LadTypeKind::Primitive(ReflectionPrimitiveKind::Bool)),
            Box::new(LadTypeKind::Primitive(ReflectionPrimitiveKind::String)),
        ));

        assert_eq!(visitor.buffer.build(), "HashMap<Bool, String>");
    }

    #[test]
    fn test_visit_nested_hash_map() {
        let ladfile = setup_ladfile();

        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);
        let first_type_id = &plain_struct_id();

        visitor.visit(&LadTypeKind::HashMap(
            Box::new(LadTypeKind::Primitive(ReflectionPrimitiveKind::Bool)),
            Box::new(LadTypeKind::Union(vec![
                LadTypeKind::Val(first_type_id.clone()),
                LadTypeKind::Union(vec![
                    LadTypeKind::Val(first_type_id.clone()),
                    LadTypeKind::Val(first_type_id.clone()),
                ]),
            ])),
        ));
        assert_eq!(
            visitor.buffer.build(),
            "HashMap<Bool, PlainStructType | PlainStructType | PlainStructType>"
        );
    }

    #[test]
    fn test_visit_tuple() {
        let ladfile = setup_ladfile();

        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);

        visitor.visit(&LadTypeKind::Tuple(vec![
            LadTypeKind::Primitive(ReflectionPrimitiveKind::Bool),
            LadTypeKind::Primitive(ReflectionPrimitiveKind::String),
        ]));
        assert_eq!(visitor.buffer.build(), "(Bool, String)");

        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);
        visitor.visit(&LadTypeKind::TupleWithRest {
            head: vec![LadTypeKind::Primitive(ReflectionPrimitiveKind::Bool)],
            rest: Box::new(LadTypeKind::Primitive(ReflectionPrimitiveKind::String)),
        });
        assert_eq!(visitor.buffer.build(), "(Bool, ...String)");
    }

    #[test]
    fn test_visit_array() {
        let ladfile = setup_ladfile();

        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);

        visitor.visit(&LadTypeKind::Array(
            Box::new(LadTypeKind::Primitive(ReflectionPrimitiveKind::Bool)),
            5,
        ));
        assert_eq!(visitor.buffer.build(), "[Bool; 5]");
    }

    #[test]
    fn test_visit_unknown() {
        let ladfile = setup_ladfile();

        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);

        let first_type_id = &plain_struct_id();

        visitor.visit(&LadTypeKind::Unknown(first_type_id.clone()));
        assert_eq!(visitor.buffer.build(), "PlainStructType");
    }
}
