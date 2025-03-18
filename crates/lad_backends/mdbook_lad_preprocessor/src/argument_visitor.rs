//! Defines a visitor for function arguments of the `LAD` format.

use ladfile::{ArgumentVisitor, LadTypeId};

use crate::markdown::MarkdownBuilder;

pub(crate) struct MarkdownArgumentVisitor<'a> {
    ladfile: &'a ladfile::LadFile,
    buffer: MarkdownBuilder,
    linkifier: Box<dyn Fn(LadTypeId, &'a ladfile::LadFile) -> Option<String> + 'static>,
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
            linkifier: Box::new(|_, _| None),
            raw_type_id_replacement: None,
        }
    }

    /// Create a new instance of the visitor with a custom linkifier function
    pub fn new_with_linkifier<
        F: Fn(LadTypeId, &'a ladfile::LadFile) -> Option<String> + 'static,
    >(
        ladfile: &'a ladfile::LadFile,
        linkifier: F,
    ) -> Self {
        let mut without = Self::new(ladfile);
        without.linkifier = Box::new(linkifier);
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
        } else {
            // link the type
            let link_value = (self.linkifier)(type_id.clone(), self.ladfile);
            let link_display = type_identifier;
            if let Some(link_value) = link_value {
                self.buffer.link(link_display, link_value);
            } else {
                self.buffer.text(link_display);
            }
        }
    }

    fn walk_option(&mut self, inner: &ladfile::LadTypeKind) {
        // Write Optional<inner>
        self.buffer.text("Optional<");
        self.visit(inner);
        self.buffer.text(">");
    }

    fn walk_vec(&mut self, inner: &ladfile::LadTypeKind) {
        // Write Vec<inner>
        self.buffer.text("Vec<");
        self.visit(inner);
        self.buffer.text(">");
    }

    fn walk_hash_map(&mut self, key: &ladfile::LadTypeKind, value: &ladfile::LadTypeKind) {
        // Write HashMap<key, value>
        self.buffer.text("HashMap<");
        self.visit(key);
        self.buffer.text(", ");
        self.visit(value);
        self.buffer.text(">");
    }

    fn walk_tuple(&mut self, inner: &[ladfile::LadTypeKind]) {
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

    fn walk_union(&mut self, inner: &[ladfile::LadTypeKind]) {
        // Write `T1 | T2`
        for (idx, arg) in inner.iter().enumerate() {
            self.visit(arg);
            if idx < inner.len() - 1 {
                self.buffer.text(" | ");
            }
        }
    }

    fn walk_array(&mut self, inner: &ladfile::LadTypeKind, size: usize) {
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
    use ladfile::LadTypeKind;

    use super::*;

    fn setup_ladfile() -> ladfile::LadFile {
        // load test file from ../../../ladfile_builder/test_assets/
        let ladfile = ladfile::EXAMPLE_LADFILE;
        ladfile::parse_lad_file(ladfile).unwrap()
    }

    #[test]
    fn test_linkifier_visitor_creates_links() {
        let ladfile = setup_ladfile();

        let mut visitor =
            MarkdownArgumentVisitor::new_with_linkifier(&ladfile, |type_id, ladfile| {
                Some(format!(
                    "root/{}",
                    ladfile.get_type_identifier(&type_id, None)
                ))
            });

        let first_type_id = ladfile.types.first().unwrap().0;
        visitor.visit_lad_type_id(first_type_id);
        assert_eq!(visitor.buffer.build(), "StructType<[usize](root/usize)>");
    }

    #[test]
    fn test_visit_type_id() {
        let ladfile = setup_ladfile();

        let first_type_id = ladfile.types.first().unwrap().0;
        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);

        visitor.visit_lad_type_id(first_type_id);
        assert_eq!(visitor.buffer.build(), "StructType<usize>");

        visitor.buffer.clear();

        let second_type_id = ladfile.types.iter().nth(1).unwrap().0;
        visitor.visit_lad_type_id(second_type_id);
        assert_eq!(visitor.buffer.build(), "EnumType");
    }

    #[test]
    fn test_visit_ref() {
        let ladfile = setup_ladfile();

        let first_type_id = ladfile.types.first().unwrap().0;
        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);

        visitor.visit(&LadTypeKind::Ref(first_type_id.clone()));
        assert_eq!(visitor.buffer.build(), "StructType<usize>");
    }

    #[test]
    fn test_visit_mut() {
        let ladfile = setup_ladfile();

        let first_type_id = ladfile.types.first().unwrap().0;
        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);

        visitor.visit(&LadTypeKind::Mut(first_type_id.clone()));
        assert_eq!(visitor.buffer.build(), "StructType<usize>");
    }

    #[test]
    fn test_visit_val() {
        let ladfile = setup_ladfile();

        let first_type_id = ladfile.types.first().unwrap().0;
        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);

        visitor.visit(&LadTypeKind::Val(first_type_id.clone()));
        assert_eq!(visitor.buffer.build(), "StructType<usize>");
    }

    #[test]
    fn test_visit_option() {
        let ladfile = setup_ladfile();

        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);

        visitor.visit(&LadTypeKind::Option(Box::new(LadTypeKind::Primitive(
            ladfile::LadBMSPrimitiveKind::Bool,
        ))));
        assert_eq!(visitor.buffer.build(), "Optional<bool>");
    }

    #[test]
    fn test_visit_vec() {
        let ladfile = setup_ladfile();

        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);

        visitor.visit(&LadTypeKind::Vec(Box::new(LadTypeKind::Primitive(
            ladfile::LadBMSPrimitiveKind::Bool,
        ))));
        assert_eq!(visitor.buffer.build(), "Vec<bool>");
    }

    #[test]
    fn test_visit_hash_map() {
        let ladfile = setup_ladfile();

        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);

        visitor.visit(&LadTypeKind::HashMap(
            Box::new(LadTypeKind::Primitive(ladfile::LadBMSPrimitiveKind::Bool)),
            Box::new(LadTypeKind::Primitive(ladfile::LadBMSPrimitiveKind::String)),
        ));

        assert_eq!(visitor.buffer.build(), "HashMap<bool, String>");
    }

    #[test]
    fn test_visit_nested_hash_map() {
        let ladfile = setup_ladfile();

        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);
        let first_type_id = ladfile.types.first().unwrap().0;

        visitor.visit(&LadTypeKind::HashMap(
            Box::new(LadTypeKind::Primitive(ladfile::LadBMSPrimitiveKind::Bool)),
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
            "HashMap<bool, StructType<usize> | StructType<usize> | StructType<usize>>"
        );
    }

    #[test]
    fn test_visit_tuple() {
        let ladfile = setup_ladfile();

        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);

        visitor.visit(&LadTypeKind::Tuple(vec![
            LadTypeKind::Primitive(ladfile::LadBMSPrimitiveKind::Bool),
            LadTypeKind::Primitive(ladfile::LadBMSPrimitiveKind::String),
        ]));
        assert_eq!(visitor.buffer.build(), "(bool, String)");
    }

    #[test]
    fn test_visit_array() {
        let ladfile = setup_ladfile();

        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);

        visitor.visit(&LadTypeKind::Array(
            Box::new(LadTypeKind::Primitive(ladfile::LadBMSPrimitiveKind::Bool)),
            5,
        ));
        assert_eq!(visitor.buffer.build(), "[bool; 5]");
    }

    #[test]
    fn test_visit_unknown() {
        let ladfile = setup_ladfile();

        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);

        let first_type_id = ladfile.types.first().unwrap().0;

        visitor.visit(&LadTypeKind::Unknown(first_type_id.clone()));
        assert_eq!(visitor.buffer.build(), "StructType<usize>");
    }
}
