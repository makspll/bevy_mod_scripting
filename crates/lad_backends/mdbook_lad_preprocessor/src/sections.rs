use crate::{
    argument_visitor::MarkdownArgumentVisitor,
    markdown::{markdown_substring, IntoMarkdown, Markdown, MarkdownBuilder},
    markdown_vec,
};
use ladfile::{
    ArgumentVisitor, LadArgument, LadFile, LadFunction, LadInstance, LadType, LadTypeId,
    LadTypeLayout,
};
use mdbook::book::Chapter;
use std::{borrow::Cow, collections::HashSet};

fn print_type(ladfile: &LadFile, type_: &LadTypeId) -> String {
    let mut visitor = MarkdownArgumentVisitor::new(ladfile);
    visitor.visit_lad_type_id(type_);
    visitor.build()
}

/// Sections which convert to single markdown files
pub(crate) enum Section<'a> {
    Summary {
        ladfile: &'a ladfile::LadFile,
        title: Option<String>,
    },
    /// A link directory to all the types within the ladfile
    TypeSummary { ladfile: &'a ladfile::LadFile },
    /// A link directory to all global functions within the ladfile
    FunctionSummary { ladfile: &'a ladfile::LadFile },
    /// A link directory to all global instances within the ladfile
    InstancesSummary { ladfile: &'a ladfile::LadFile },
    TypeDetail {
        lad_type_id: &'a LadTypeId,
        lad_type: &'a LadType,
        ladfile: &'a ladfile::LadFile,
    },
    FunctionDetail {
        function: &'a LadFunction,
        ladfile: &'a ladfile::LadFile,
    },
}

/// Makes a filename safe to put in links
pub fn linkify_filename(name: impl Into<String>) -> String {
    name.into()
        .to_lowercase()
        .replace(" ", "_")
        .replace("<", "")
        .replace(">", "")
}

impl<'a> Section<'a> {
    /// convert into a chapter, including children
    pub(crate) fn into_chapter(self, parent: Option<&Chapter>, index: usize) -> Chapter {
        let mut builder = MarkdownBuilder::new();
        self.to_markdown(&mut builder);

        let default_chapter = Chapter::default();
        let parent = match parent {
            Some(parent) => parent,
            None => &default_chapter,
        };

        let parent_path = parent.path.clone().unwrap_or_default().with_extension("");
        let parent_source_path = parent
            .source_path
            .clone()
            .unwrap_or_default()
            .with_extension("");

        let mut current_number = parent.number.clone().unwrap_or_default();
        current_number.push(index as u32);

        let mut chapter = Chapter {
            name: self.title(),
            content: builder.build(),
            parent_names: vec![parent.name.clone()],
            path: Some(parent_path.join(self.file_name())),
            source_path: Some(parent_source_path.join(self.file_name())),
            number: Some(current_number),
            sub_items: vec![],
        };

        chapter.sub_items = self
            .children()
            .into_iter()
            .enumerate()
            .map(|(i, c)| c.into_chapter(Some(&chapter), i))
            .map(mdbook::BookItem::Chapter)
            .collect();

        chapter
    }

    pub(crate) fn title(&self) -> String {
        match self {
            Section::Summary { title, .. } => {
                title.as_deref().unwrap_or("Bindings Summary").to_owned()
            }
            Section::TypeSummary { .. } => "Types".to_owned(),
            Section::FunctionSummary { .. } => "Functions".to_owned(),
            Section::InstancesSummary { .. } => "Globals".to_owned(),
            Section::TypeDetail {
                ladfile,
                lad_type_id,
                ..
            } => print_type(ladfile, lad_type_id),
            Section::FunctionDetail { function, .. } => function.identifier.to_string(),
        }
    }

    pub(crate) fn file_name(&self) -> String {
        linkify_filename(self.title()) + ".md"
    }

    pub(crate) fn children(&self) -> Vec<Section<'a>> {
        match self {
            Section::Summary { ladfile, .. } => {
                vec![
                    Section::InstancesSummary { ladfile },
                    Section::FunctionSummary { ladfile },
                    Section::TypeSummary { ladfile },
                ]
            }
            Section::TypeSummary { ladfile } => ladfile
                .types
                .iter()
                .map(|(lad_type_id, lad_type)| Section::TypeDetail {
                    lad_type,
                    ladfile,
                    lad_type_id,
                })
                .collect(),

            Section::FunctionSummary { ladfile } => {
                let associated_functions = ladfile
                    .types
                    .iter()
                    .flat_map(|t| &t.1.associated_functions)
                    .collect::<HashSet<_>>();

                let non_associated_functions = ladfile
                    .functions
                    .iter()
                    .filter_map(|f| (!associated_functions.contains(f.0)).then_some(f.1));

                non_associated_functions
                    .map(|function| Section::FunctionDetail { function, ladfile })
                    .collect()
            }
            Section::InstancesSummary { .. } => {
                vec![]
            }
            Section::TypeDetail {
                lad_type, ladfile, ..
            } => lad_type
                .associated_functions
                .iter()
                .filter_map(|f| {
                    let function = ladfile.functions.get(f)?;
                    Some(Section::FunctionDetail { function, ladfile })
                })
                .collect(),
            Section::FunctionDetail { .. } => vec![],
        }
    }

    pub(crate) fn section_items(&self) -> Vec<SectionItem> {
        match self {
            Section::Summary { .. } => {
                let mut builder = MarkdownBuilder::new();
                builder.heading(1, self.title());
                builder.heading(2, "Contents");
                builder.text("This is an automatically generated file, you'll find links to the contents below");
                builder.table(|builder| {
                    builder.headers(vec!["Section", "Contents"]);
                    builder.row(markdown_vec![
                        Markdown::new_paragraph("Types").code(),
                        Markdown::Link {
                            text: "Describes all available binding types".into(),
                            url: format!("./{}/types.md", linkify_filename(self.title())),
                            anchor: false
                        }
                    ]);
                    builder.row(markdown_vec![
                        Markdown::new_paragraph("Global Functions").code(),
                        Markdown::Link {
                            text: "Documents all the global functions present in the bindings"
                                .into(),
                            url: format!("./{}/functions.md", linkify_filename(self.title())),
                            anchor: false
                        }
                    ]);
                    builder.row(markdown_vec![
                        Markdown::new_paragraph("Globals").code(),
                        Markdown::Link {
                            text: "Documents all global variables present in the bindings".into(),
                            url: format!("./{}/globals.md", linkify_filename(self.title())),
                            anchor: false
                        }
                    ]);
                });
                vec![SectionItem::Markdown {
                    markdown: Box::new(builder),
                }]
            }
            Section::InstancesSummary { ladfile } => {
                let instances = ladfile.globals.iter().collect::<Vec<_>>();
                vec![SectionItem::InstancesSummary { instances, ladfile}]
            }
            Section::TypeSummary { ladfile } => {
                let types = ladfile.types.keys().collect::<Vec<_>>();
                vec![SectionItem::TypesSummary {
                    types,
                    types_directory: linkify_filename(self.title()),
                    ladfile,
                }]
            }
            Section::FunctionSummary { ladfile } => {
                let associated_functions = ladfile
                    .types
                    .iter()
                    .flat_map(|t| &t.1.associated_functions)
                    .collect::<HashSet<_>>();

                let non_associated_functions = ladfile
                    .functions
                    .iter()
                    .filter_map(|f| (!associated_functions.contains(f.0)).then_some(f.1))
                    .collect();

                vec![SectionItem::FunctionsSummary {
                    functions: non_associated_functions,
                    functions_directory: "functions".to_owned(),
                }]
            }
            Section::TypeDetail {
                lad_type, ladfile, ..
            } => {
                let functions = lad_type
                    .associated_functions
                    .iter()
                    .filter_map(|i| ladfile.functions.get(i))
                    .collect::<Vec<_>>();

                vec![
                    SectionItem::Layout {
                        layout: &lad_type.layout,
                    },
                    SectionItem::Description { lad_type },
                    SectionItem::FunctionsSummary {
                        functions,
                        functions_directory: linkify_filename(self.title()),
                    },
                ]
            }
            Section::FunctionDetail { function, ladfile } => {
                vec![SectionItem::FunctionDetails { function, ladfile }]
            }
        }
    }
}

impl IntoMarkdown for Section<'_> {
    fn to_markdown(&self, builder: &mut MarkdownBuilder) {
        builder.heading(1, self.title());

        for item in self.section_items() {
            item.to_markdown(builder);
        }
    }
}

const NO_DOCS_STRING: &str = "No Documentation 🚧";

/// Items which combine markdown elements to build a section
pub enum SectionItem<'a> {
    Markdown {
        markdown: Box<dyn IntoMarkdown>,
    },
    Layout {
        layout: &'a LadTypeLayout,
    },
    Description {
        lad_type: &'a LadType,
    },
    FunctionsSummary {
        functions: Vec<&'a LadFunction>,
        functions_directory: String,
    },
    FunctionDetails {
        function: &'a LadFunction,
        ladfile: &'a ladfile::LadFile,
    },
    TypesSummary {
        types: Vec<&'a LadTypeId>,
        types_directory: String,
        ladfile: &'a ladfile::LadFile,
    },
    InstancesSummary {
        ladfile: &'a ladfile::LadFile,
        instances: Vec<(&'a Cow<'static, str>, &'a LadInstance)>,
    },
}

impl IntoMarkdown for SectionItem<'_> {
    fn to_markdown(&self, builder: &mut MarkdownBuilder) {
        match self {
            SectionItem::Markdown { markdown } => markdown.to_markdown(builder),
            SectionItem::Layout { layout } => {
                // process the variants here
                let opaque = layout.for_each_variant(
                    |v, _i| match v {
                        ladfile::LadVariant::TupleStruct { name, fields } => {
                            builder.heading(3, name.to_string()).list(
                                true,
                                fields
                                    .iter()
                                    .map(|f| Markdown::new_paragraph(f.type_.to_string()))
                                    .collect(),
                            );
                        }
                        ladfile::LadVariant::Struct { name, fields } => {
                            builder.heading(3, name.to_string()).list(
                                false,
                                fields
                                    .iter()
                                    .map(|f| {
                                        markdown_vec![
                                            Markdown::new_paragraph(f.name.clone()).bold(),
                                            Markdown::new_paragraph(":"),
                                            f.type_.to_string()
                                        ]
                                    })
                                    .collect(),
                            );
                        }
                        ladfile::LadVariant::Unit { name } => {
                            builder.heading(3, name.to_string());
                        }
                    },
                    "Opaque Type. 🔒",
                );

                if let Some(opaque) = opaque {
                    builder.text(opaque);
                }
            }
            SectionItem::Description {
                lad_type: description,
            } => {
                builder.heading(2, "Description").quote(Markdown::Raw {
                    text: description
                        .documentation
                        .as_deref()
                        .unwrap_or(NO_DOCS_STRING)
                        .to_owned(),
                });
            }
            SectionItem::FunctionsSummary {
                functions,
                functions_directory: functions_path,
            } => {
                builder.heading(2, "Functions");

                // make a table of functions as a quick reference, make them link to function details sub-sections
                builder.table(|builder| {
                    builder.headers(vec!["Function", "Summary"]);
                    for function in functions.iter() {
                        let mut first_col = function.identifier.to_string();
                        first_col.push('(');
                        for (idx, arg) in function.arguments.iter().enumerate() {
                            first_col.push_str(
                                &arg.name
                                    .as_ref()
                                    .cloned()
                                    .unwrap_or_else(|| Cow::Owned(format!("arg{}", idx))),
                            );
                            if idx != function.arguments.len() - 1 {
                                first_col.push_str(", ");
                            }
                        }
                        first_col.push(')');

                        // first line with content from documentation trimmed to 100 chars
                        let second_col = function
                            .documentation
                            .as_deref()
                            .map(|doc| markdown_substring(doc, 100))
                            .unwrap_or_else(|| NO_DOCS_STRING);

                        builder.row(markdown_vec![
                            Markdown::new_paragraph(first_col).code(),
                            Markdown::Link {
                                text: second_col.to_owned().replace("\n", " "),
                                url: format!("./{}/{}.md", functions_path, function.identifier),
                                anchor: false
                            }
                        ]);
                    }
                });
            }
            SectionItem::TypesSummary {
                types,
                types_directory,
                ladfile,
            } => {
                builder.heading(2, "Types");

                // make a table of types as a quick reference, make them link to type details sub-sections
                builder.table(|builder| {
                    builder.headers(vec!["Type", "Summary"]);
                    for type_ in types.iter() {
                        let printed_type = print_type(ladfile, type_);

                        let documentation = ladfile.get_type_documentation(type_);

                        // first line with content from documentation trimmed to 100 chars
                        let second_col = documentation
                            .map(|doc| markdown_substring(doc, 100))
                            .unwrap_or_else(|| NO_DOCS_STRING);

                        builder.row(markdown_vec![
                            Markdown::new_paragraph(printed_type.clone()).code(),
                            Markdown::Link {
                                text: second_col.to_owned().replace("\n", " "),
                                url: format!(
                                    "./{types_directory}/{}.md",
                                    linkify_filename(printed_type)
                                ),
                                anchor: false
                            }
                        ]);
                    }
                });
            }
            SectionItem::InstancesSummary { instances, ladfile } => {
                builder.heading(2, "Global Values");
                builder.text("Global values that are accessible anywhere inside scripts. You should avoid naming conflicts with these and trying to overwrite or edit them.");
                // make a table of instances as a quick reference, make them link to instance details sub-sections

                // first build a non-static instance table
                let instances = instances.iter().map(|(k,v)| {
                    let name = k.to_string();
                    let mut arg_visitor = MarkdownArgumentVisitor::new(ladfile);
                    arg_visitor.visit(&v.type_kind);

                    (v.is_static, name, arg_visitor.build())
                }).collect::<Vec<_>>();

                builder.heading(3, "Instances");
                builder.text("Instances containing actual accessible values.");
                builder.table(|builder| {
                    builder.headers(vec!["Instance", "Type"]);
                    for (_, name, instance) in instances.iter().filter(|(a,_,_)| !*a) {
                        builder.row(markdown_vec![
                            Markdown::new_paragraph(name).code(),
                            Markdown::new_paragraph(instance).code()
                        ]);
                    }
                });

                builder.heading(3, "Static Instances");
                builder.text("Static type references, existing for the purpose of typed static function calls.");
                builder.table(|builder| {
                    builder.headers(vec!["Instance", "Type"]);
                    for (_, name, instance) in instances.iter().filter(|(a,_,_)| *a) {
                        builder.row(markdown_vec![
                            Markdown::new_paragraph(name).code(),
                            Markdown::new_paragraph(instance).code()
                        ]);
                    }
                });
            }
            SectionItem::FunctionDetails { function, ladfile } => {
                // we don't escape this, this is already markdown
                builder.quote(Markdown::Raw {
                    text: function
                        .documentation
                        .as_deref()
                        .unwrap_or(NO_DOCS_STRING)
                        .to_owned(),
                });

                builder.heading(4, "Arguments");
                builder.list(
                    false,
                    function
                        .arguments
                        .iter()
                        .enumerate()
                        .map(|(idx, arg)| lad_argument_to_list_elem(idx, arg, ladfile))
                        .collect(),
                );

                builder.heading(4, "Returns");
                builder.list(
                    false,
                    vec![lad_argument_to_list_elem(0, &function.return_type, ladfile)],
                );
            }
        }
    }
}

fn lad_argument_to_list_elem(
    idx: usize,
    arg: &LadArgument,
    ladfile: &LadFile,
) -> impl IntoMarkdown {
    let mut arg_visitor = MarkdownArgumentVisitor::new(ladfile);
    arg_visitor.visit(&arg.kind);
    let markdown = arg_visitor.build();

    let arg_name = arg
        .name
        .as_ref()
        .cloned()
        .unwrap_or_else(|| Cow::Owned(format!("arg{}", idx)));
    markdown_vec![
        Markdown::new_paragraph(arg_name).bold(),
        Markdown::new_paragraph(":"),
        Markdown::new_paragraph(markdown).code(),
        Markdown::new_paragraph("-"),
        Markdown::Raw {
            text: arg
                .documentation
                .as_deref()
                .unwrap_or(NO_DOCS_STRING)
                .to_owned()
        }
    ]
}
