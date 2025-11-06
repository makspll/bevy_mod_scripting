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
}

impl ArgumentVisitor for MarkdownArgumentVisitor<'_> {
    fn walk_lad_type_id(&mut self, type_id: &LadTypeId) {
        if let Some(prim) = self.ladfile.primitive_kind(type_id) {
            self.visit_lad_bms_primitive_kind(prim);
        } else {
            self.visit_lad_type_id(type_id);
        }
    }

    fn visit_lad_bms_primitive_kind(&mut self, primitive_kind: &ladfile::ReflectionPrimitiveKind) {
        let prim_id = primitive_kind.to_string();
        if let Some(linkifier) = &self.linkifier {
            let link_value = (linkifier)(prim_id.clone());
            let link_value = link_value.to_string_lossy().to_string().replace("\\", "/");
            self.buffer.link(prim_id, link_value);
        } else {
            self.buffer.text(prim_id);
        }
    }

    fn visit_lad_type_id(&mut self, type_id: &ladfile::LadTypeId) {
        // Write identifier<Generic1TypeIdentifier, Generic2TypeIdentifier>
        let generics = self.ladfile.get_type_generics(type_id);

        let type_identifier = self
            .ladfile
            .get_type_identifier(type_id, self.raw_type_id_replacement);
        if let Some(generics) = generics {
            self.buffer.text(type_identifier);
            self.buffer.text('<');
            for (i, generic) in generics.iter().enumerate() {
                if i > 0 {
                    self.buffer.text(", ");
                }
                self.visit_lad_type_id(&generic.type_id);
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

    fn walk_option(&mut self, inner: &ladfile::LadFieldOrVariableKind) {
        // Write Optional<inner>
        self.buffer.text("Optional<");
        self.visit(inner);
        self.buffer.text(">");
    }

    fn walk_vec(&mut self, inner: &ladfile::LadFieldOrVariableKind) {
        // Write Vec<inner>
        self.buffer.text("Vec<");
        self.visit(inner);
        self.buffer.text(">");
    }

    fn walk_hash_map(
        &mut self,
        key: &ladfile::LadFieldOrVariableKind,
        value: &ladfile::LadFieldOrVariableKind,
    ) {
        // Write HashMap<key, value>
        self.buffer.text("HashMap<");
        self.visit(key);
        self.buffer.text(", ");
        self.visit(value);
        self.buffer.text(">");
    }

    fn walk_tuple(&mut self, inner: &[ladfile::LadFieldOrVariableKind]) {
        // Write (inner1, inner2, ...)
        self.buffer.text("(");
        for (idx, arg) in inner.iter().enumerate() {
            self.visit(arg);
            if idx < inner.len() - 1 {
                self.buffer.text(", ");
            }
        }
        self.buffer.text(")");
    }

    fn walk_union(&mut self, inner: &[ladfile::LadFieldOrVariableKind]) {
        // Write `T1 | T2`
        for (idx, arg) in inner.iter().enumerate() {
            self.visit(arg);
            if idx < inner.len() - 1 {
                self.buffer.text(" | ");
            }
        }
    }

    fn walk_array(&mut self, inner: &ladfile::LadFieldOrVariableKind, size: usize) {
        // Write [inner; size]
        self.buffer.text("[");
        self.visit(inner);
        self.buffer.text("; ");
        self.buffer.text(size.to_string());
        self.buffer.text("]");
    }
}

#[cfg(test)]
mod test {
    use ladfile::{LadFieldOrVariableKind, ReflectionPrimitiveKind};

    use super::*;

    fn setup_ladfile() -> ladfile::LadFile {
        // load test file from ../../../ladfile_builder/test_assets/
        let ladfile = ladfile::EXAMPLE_LADFILE;
        ladfile::parse_lad_file(ladfile).unwrap()
    }

    #[test]
    fn test_linkifier_visitor_creates_links() {
        let ladfile = setup_ladfile();

        let mut visitor = MarkdownArgumentVisitor::new_with_linkifier(&ladfile, |str| {
            PathBuf::from("root\\asd").join(str)
        });

        let second_type_id = ladfile.types.iter().nth(1).unwrap().0;
        visitor.visit_lad_type_id(second_type_id);
        assert_eq!(
            visitor.buffer.build(),
            "GenericStructType<[Usize](root/asd/Usize)>"
        );
    }

    #[test]
    fn test_visit_type_id() {
        let ladfile = setup_ladfile();

        let first_type_id = ladfile.types.first().unwrap().0;
        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);

        visitor.visit_lad_type_id(first_type_id);
        assert_eq!(visitor.buffer.build(), "PlainStructType");

        visitor.buffer.clear();

        let second_type_id = ladfile.types.iter().nth(1).unwrap().0;
        visitor.visit_lad_type_id(second_type_id);
        assert_eq!(visitor.buffer.build(), "GenericStructType<Usize>");
    }

    #[test]
    fn test_visit_ref() {
        let ladfile = setup_ladfile();

        let first_type_id = ladfile.types.first().unwrap().0;
        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);

        visitor.visit(&LadFieldOrVariableKind::Ref(first_type_id.clone()));
        assert_eq!(visitor.buffer.build(), "PlainStructType");
    }

    #[test]
    fn test_visit_mut() {
        let ladfile = setup_ladfile();

        let first_type_id = ladfile.types.first().unwrap().0;
        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);

        visitor.visit(&LadFieldOrVariableKind::Mut(first_type_id.clone()));
        assert_eq!(visitor.buffer.build(), "PlainStructType");
    }

    #[test]
    fn test_visit_val() {
        let ladfile = setup_ladfile();

        let first_type_id = ladfile.types.first().unwrap().0;
        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);

        visitor.visit(&LadFieldOrVariableKind::Val(first_type_id.clone()));
        assert_eq!(visitor.buffer.build(), "PlainStructType");
    }

    #[test]
    fn test_visit_option() {
        let ladfile = setup_ladfile();

        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);

        visitor.visit(&LadFieldOrVariableKind::Option(Box::new(
            LadFieldOrVariableKind::Primitive(ReflectionPrimitiveKind::Bool),
        )));
        assert_eq!(visitor.buffer.build(), "Optional<Bool>");
    }

    #[test]
    fn test_visit_vec() {
        let ladfile = setup_ladfile();

        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);

        visitor.visit(&LadFieldOrVariableKind::Vec(Box::new(
            LadFieldOrVariableKind::Primitive(ReflectionPrimitiveKind::Bool),
        )));
        assert_eq!(visitor.buffer.build(), "Vec<Bool>");
    }

    #[test]
    fn test_visit_hash_map() {
        let ladfile = setup_ladfile();

        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);

        visitor.visit(&LadFieldOrVariableKind::HashMap(
            Box::new(LadFieldOrVariableKind::Primitive(
                ReflectionPrimitiveKind::Bool,
            )),
            Box::new(LadFieldOrVariableKind::Primitive(
                ReflectionPrimitiveKind::String,
            )),
        ));

        assert_eq!(visitor.buffer.build(), "HashMap<Bool, String>");
    }

    #[test]
    fn test_visit_nested_hash_map() {
        let ladfile = setup_ladfile();

        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);
        let first_type_id = ladfile.types.first().unwrap().0;

        visitor.visit(&LadFieldOrVariableKind::HashMap(
            Box::new(LadFieldOrVariableKind::Primitive(
                ReflectionPrimitiveKind::Bool,
            )),
            Box::new(LadFieldOrVariableKind::Union(vec![
                LadFieldOrVariableKind::Val(first_type_id.clone()),
                LadFieldOrVariableKind::Union(vec![
                    LadFieldOrVariableKind::Val(first_type_id.clone()),
                    LadFieldOrVariableKind::Val(first_type_id.clone()),
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

        visitor.visit(&LadFieldOrVariableKind::Tuple(vec![
            LadFieldOrVariableKind::Primitive(ReflectionPrimitiveKind::Bool),
            LadFieldOrVariableKind::Primitive(ReflectionPrimitiveKind::String),
        ]));
        assert_eq!(visitor.buffer.build(), "(Bool, String)");
    }

    #[test]
    fn test_visit_array() {
        let ladfile = setup_ladfile();

        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);

        visitor.visit(&LadFieldOrVariableKind::Array(
            Box::new(LadFieldOrVariableKind::Primitive(
                ReflectionPrimitiveKind::Bool,
            )),
            5,
        ));
        assert_eq!(visitor.buffer.build(), "[Bool; 5]");
    }

    #[test]
    fn test_visit_unknown() {
        let ladfile = setup_ladfile();

        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);

        let first_type_id = ladfile.types.first().unwrap().0;

        visitor.visit(&LadFieldOrVariableKind::Unknown(first_type_id.clone()));
        assert_eq!(visitor.buffer.build(), "PlainStructType");
    }
}
