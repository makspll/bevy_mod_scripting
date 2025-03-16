//! Defines a visitor for function arguments of the `LAD` format.

use ladfile::ArgumentVisitor;

use crate::markdown::MarkdownBuilder;

pub(crate) struct MarkdownArgumentVisitor<'a> {
    ladfile: &'a ladfile::LadFile,
    buffer: MarkdownBuilder,
}
impl<'a> MarkdownArgumentVisitor<'a> {
    pub fn new(ladfile: &'a ladfile::LadFile) -> Self {
        let mut builder = MarkdownBuilder::new();
        builder.tight_inline().set_escape_mode(false);
        Self {
            ladfile,
            buffer: builder,
        }
    }

    pub fn build(mut self) -> String {
        self.buffer.build()
    }
}

impl ArgumentVisitor for MarkdownArgumentVisitor<'_> {
    fn visit_lad_type_id(&mut self, type_id: &ladfile::LadTypeId) {
        // Write identifier<Generic1TypeIdentifier, Generic2TypeIdentifier>
        self.buffer.text(self.ladfile.get_type_identifier(type_id));
        if let Some(generics) = self.ladfile.get_type_generics(type_id) {
            self.buffer.text('<');
            for (i, generic) in generics.iter().enumerate() {
                self.visit_lad_type_id(&generic.type_id);
                if i > 0 {
                    self.buffer.text(", ");
                }
            }
            self.buffer.text('>');
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
    fn test_visit_type_id() {
        let ladfile = setup_ladfile();

        let first_type_id = ladfile.types.first().unwrap().0;
        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);

        visitor.visit_lad_type_id(first_type_id);
        assert_eq!(visitor.buffer.build(), "EnumType");

        visitor.buffer.clear();

        let second_type_id = ladfile.types.iter().nth(1).unwrap().0;
        visitor.visit_lad_type_id(second_type_id);
        assert_eq!(visitor.buffer.build(), "StructType<usize>");
    }

    #[test]
    fn test_visit_ref() {
        let ladfile = setup_ladfile();

        let first_type_id = ladfile.types.first().unwrap().0;
        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);

        visitor.visit(&LadTypeKind::Ref(first_type_id.clone()));
        assert_eq!(visitor.buffer.build(), "EnumType");
    }

    #[test]
    fn test_visit_mut() {
        let ladfile = setup_ladfile();

        let first_type_id = ladfile.types.first().unwrap().0;
        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);

        visitor.visit(&LadTypeKind::Mut(first_type_id.clone()));
        assert_eq!(visitor.buffer.build(), "EnumType");
    }

    #[test]
    fn test_visit_val() {
        let ladfile = setup_ladfile();

        let first_type_id = ladfile.types.first().unwrap().0;
        let mut visitor = MarkdownArgumentVisitor::new(&ladfile);

        visitor.visit(&LadTypeKind::Val(first_type_id.clone()));
        assert_eq!(visitor.buffer.build(), "EnumType");
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
            Box::new(LadTypeKind::Union(
                vec![
                    LadTypeKind::Val(first_type_id.clone()),
                    LadTypeKind::Union(vec![
                        LadTypeKind::Val(first_type_id.clone()),
                        LadTypeKind::Val(first_type_id.clone()),
                    ]),
                ]
            )),
        ));
        assert_eq!(visitor.buffer.build(), "HashMap<bool, EnumType | EnumType | EnumType>");
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
        assert_eq!(visitor.buffer.build(), "EnumType");
    }
}
