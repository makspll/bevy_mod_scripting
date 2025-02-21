use crate::{
    argument_visitor::MarkdownArgumentVisitor,
    markdown::{markdown_substring, IntoMarkdown, Markdown, MarkdownBuilder},
    markdown_vec,
};
use ladfile::{
    ArgumentVisitor, LadArgument, LadFile, LadFunction, LadInstance, LadType, LadTypeLayout,
};
use mdbook::book::{Chapter, SectionNumber};
use std::{borrow::Cow, collections::HashSet, path::PathBuf};

pub(crate) fn section_to_chapter(
    section: SectionAndChildren,
    original_chapter: Option<&Chapter>,
    parent_names: Vec<String>,
    number: Option<SectionNumber>,
    root_path: Option<PathBuf>,
    root_source_path: Option<PathBuf>,
) -> Chapter {
    let mut parent_builder = MarkdownBuilder::new();
    section.section.to_markdown(&mut parent_builder);

    // important to reset the extension of the parent, since when we're nesting
    // we add the filename with .md, but if the parent is being emitted as markdown, then when
    // we create the child, we will create the `parent.md` file as a folder, then when we emit
    // the parent itself, the file (directory) will already exist
    let new_path = root_path
        .unwrap_or_default()
        .with_extension("")
        .join(section.section.file_name());

    let new_source_path = root_source_path
        .unwrap_or_default()
        .with_extension("")
        .join(section.section.file_name());

    let current_number = number.clone().unwrap_or_default();

    let children_chapters = section
        .children
        .into_iter()
        .enumerate()
        .map(|(index, child)| {
            let mut new_number = current_number.clone();
            new_number.push(index as u32);
            section_to_chapter(
                child,
                None,
                vec![section.section.title()],
                Some(new_number),
                Some(new_path.clone()),
                Some(new_source_path.clone()),
            )
        })
        .map(mdbook::BookItem::Chapter)
        .collect();

    if let Some(original) = original_chapter {
        // override content only
        log::debug!(
            "Setting .md extension for chapter paths: {:?}, {:?}.",
            original.path,
            original.source_path
        );

        Chapter {
            content: parent_builder.build(),
            sub_items: children_chapters,
            path: original.path.as_ref().map(|p| p.with_extension("md")),
            source_path: original
                .source_path
                .as_ref()
                .map(|p| p.with_extension("md")),
            ..original.clone()
        }
    } else {
        Chapter {
            name: section.section.title(),
            content: parent_builder.build(),
            number,
            sub_items: children_chapters,
            path: Some(new_path),
            source_path: Some(new_source_path),
            parent_names,
        }
    }
}

fn section_to_section_and_children(section: Section<'_>) -> SectionAndChildren<'_> {
    let children = section
        .children()
        .into_iter()
        .map(section_to_section_and_children)
        .collect();

    SectionAndChildren { children, section }
}

pub(crate) fn lad_file_to_sections(
    ladfile: &ladfile::LadFile,
    title: Option<String>,
) -> SectionAndChildren<'_> {
    section_to_section_and_children(Section::Summary { ladfile, title })
    // build a hierarchy as follows:
    // - Summary
    //   - Instances
    //   - Functions
    //      - Global Function Detail 1
    //   - Types
    //     - Type1
    //       - Type detail 1
    //         - Function detail 1
    //         - Function detail 2
    // let mut types_children = ladfile
    //     .types
    //     .iter()
    //     .map(|(_, lad_type)| (lad_type, Section::TypeDetail { lad_type, ladfile }))
    //     .map(|(lad_type, section)| SectionAndChildren {
    //         section,
    //         children: lad_type
    //             .associated_functions
    //             .iter()
    //             .filter_map(|f| {
    //                 let function = ladfile.functions.get(f)?;
    //                 Some(SectionAndChildren {
    //                     section: Section::FunctionDetail { function, ladfile },
    //                     children: vec![],
    //                 })
    //             })
    //             .collect(),
    //     })
    //     .collect();

    // // now add a `functions` subsection before all types, for global functions

    // SectionAndChildren {
    //     section: summary,
    //     children: vec![
    //         SectionAndChildren {
    //             section: Section::TypeSummary { ladfile },
    //             children: types_children,
    //         },
    //         SectionAndChildren {
    //             section: Section::FunctionSummary { ladfile },
    //             children: vec![],
    //         },
    //     ],
    // }
}
pub(crate) struct SectionAndChildren<'a> {
    section: Section<'a>,
    children: Vec<SectionAndChildren<'a>>,
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
    name.into().to_lowercase().replace(" ", "_")
}

impl<'a> Section<'a> {
    pub(crate) fn title(&self) -> String {
        match self {
            Section::Summary { title, .. } => {
                title.as_deref().unwrap_or("Bindings Summary").to_owned()
            }
            Section::TypeSummary { .. } => "Types".to_owned(),
            Section::FunctionSummary { .. } => "Functions".to_owned(),
            Section::InstancesSummary { .. } => "Globals".to_owned(),
            Section::TypeDetail {
                lad_type: type_id, ..
            } => type_id.identifier.clone(),
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
                    Section::TypeSummary { ladfile },
                    Section::FunctionSummary { ladfile },
                    Section::InstancesSummary { ladfile },
                ]
            }
            Section::TypeSummary { ladfile } => ladfile
                .types
                .iter()
                .map(|(_, lad_type)| Section::TypeDetail { lad_type, ladfile })
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
            Section::TypeDetail { lad_type, ladfile } => lad_type
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
                vec![SectionItem::InstancesSummary { instances }]
            }
            Section::TypeSummary { ladfile } => {
                let types = ladfile.types.values().collect::<Vec<_>>();
                vec![SectionItem::TypesSummary {
                    types,
                    types_directory: linkify_filename(self.title()),
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
            Section::TypeDetail { lad_type, ladfile } => {
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
                        functions_directory: linkify_filename(&lad_type.identifier),
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
        types: Vec<&'a LadType>,
        types_directory: String,
    },
    InstancesSummary {
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
                                text: second_col.to_owned(),
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
            } => {
                builder.heading(2, "Types");

                // make a table of types as a quick reference, make them link to type details sub-sections
                builder.table(|builder| {
                    builder.headers(vec!["Type", "Summary"]);
                    for type_ in types.iter() {
                        let first_col = type_.identifier.to_string();

                        // first line with content from documentation trimmed to 100 chars
                        let second_col = type_
                            .documentation
                            .as_deref()
                            .map(|doc| {
                                let doc = doc.trim();
                                if doc.len() > 100 {
                                    format!("{}...", &doc[..100])
                                } else {
                                    doc.to_owned()
                                }
                            })
                            .unwrap_or_else(|| NO_DOCS_STRING.to_owned());

                        builder.row(markdown_vec![
                            Markdown::new_paragraph(first_col).code(),
                            Markdown::Link {
                                text: second_col,
                                url: format!(
                                    "./{types_directory}/{}.md",
                                    linkify_filename(&type_.identifier)
                                ),
                                anchor: false
                            }
                        ]);
                    }
                });
            }
            SectionItem::InstancesSummary { instances } => {
                builder.heading(2, "Globals");

                // make a table of instances as a quick reference, make them link to instance details sub-sections
                builder.table(|builder| {
                    builder.headers(vec!["Instance", "Type"]);
                    for (key, instance) in instances.iter() {
                        let first_col = key.to_string();

                        builder.row(markdown_vec![
                            Markdown::new_paragraph(first_col).code(),
                            Markdown::new_paragraph(instance.type_id.to_string())
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
