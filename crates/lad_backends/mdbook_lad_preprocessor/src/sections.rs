use std::{borrow::Cow, path::PathBuf};

use ladfile::{ArgumentVisitor, LadFunction, LadInstance, LadType, LadTypeLayout};
use mdbook::book::{Chapter, SectionNumber};

use crate::{
    argument_visitor::MarkdownArgumentVisitor,
    markdown::{self, IntoMarkdown, Markdown, MarkdownBuilder},
    markdown_vec,
};

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

    let new_path = root_path
        .unwrap_or_default()
        .join(section.section.file_name());

    let new_source_path = root_source_path
        .unwrap_or_default()
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

pub(crate) fn lad_file_to_sections(
    ladfile: &ladfile::LadFile,
    title: Option<String>,
) -> SectionAndChildren<'_> {
    let summary = Section::Summary { ladfile, title };

    let children = ladfile
        .types
        .iter()
        .map(|(_, lad_type)| Section::TypeDetail { lad_type, ladfile })
        .map(|section| SectionAndChildren {
            section,
            children: Vec::new(),
        })
        .collect();

    SectionAndChildren {
        section: summary,
        children,
    }
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
    TypeDetail {
        lad_type: &'a LadType,
        ladfile: &'a ladfile::LadFile,
    },
}

impl Section<'_> {
    pub(crate) fn title(&self) -> String {
        match self {
            Section::Summary { title, .. } => {
                title.as_deref().unwrap_or("Bindings Summary").to_owned()
            }
            Section::TypeDetail {
                lad_type: type_id, ..
            } => type_id.identifier.clone(),
        }
    }

    pub(crate) fn file_name(&self) -> String {
        self.title().to_lowercase().replace(" ", "_") + ".md"
    }

    pub(crate) fn section_items(&self) -> Vec<SectionItem> {
        match self {
            Section::Summary { ladfile, .. } => {
                let types = ladfile.types.values().collect::<Vec<_>>();
                let instances = ladfile.globals.iter().collect::<Vec<_>>();
                vec![
                    SectionItem::InstancesSummary { instances },
                    SectionItem::TypesSummary { types },
                ]
            }
            Section::TypeDetail {
                lad_type: type_id,
                ladfile,
            } => {
                let functions = type_id
                    .associated_functions
                    .iter()
                    .filter_map(|i| ladfile.functions.get(i))
                    .collect::<Vec<_>>();

                vec![
                    SectionItem::Layout {
                        layout: &type_id.layout,
                    },
                    SectionItem::Description { lad_type: type_id },
                    SectionItem::FunctionsSummary { functions },
                ]
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

/// Items which combine markdown elements to build a section
pub enum SectionItem<'a> {
    Layout {
        layout: &'a LadTypeLayout,
    },
    Description {
        lad_type: &'a LadType,
    },
    FunctionsSummary {
        functions: Vec<&'a LadFunction>,
    },
    FunctionDetails {
        function: &'a LadFunction,
        ladfile: &'a ladfile::LadFile,
    },
    TypesSummary {
        types: Vec<&'a LadType>,
    },
    InstancesSummary {
        instances: Vec<(&'a Cow<'static, str>, &'a LadInstance)>,
    },
}

impl IntoMarkdown for SectionItem<'_> {
    fn to_markdown(&self, builder: &mut MarkdownBuilder) {
        match self {
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
                builder.heading(2, "Description").quote(
                    description
                        .documentation
                        .as_deref()
                        .unwrap_or("None available. 🚧"),
                );
            }
            SectionItem::FunctionsSummary { functions } => {
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
                            .map(|doc| {
                                let doc = doc.trim();
                                if doc.len() > 100 {
                                    format!("{}...", &doc[..100])
                                } else {
                                    doc.to_owned()
                                }
                            })
                            .unwrap_or_else(|| "No documentation available. 🚧".to_owned());

                        builder.row(markdown_vec![
                            Markdown::new_paragraph(first_col).code(),
                            Markdown::Link {
                                text: second_col,
                                url: function.identifier.to_string(),
                                anchor: true
                            }
                        ]);
                    }
                });
            }
            SectionItem::TypesSummary { types } => {
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
                            .unwrap_or_else(|| "No documentation available. 🚧".to_owned());

                        builder.row(markdown_vec![
                            Markdown::new_paragraph(first_col).code(),
                            Markdown::Link {
                                text: second_col,
                                url: type_.identifier.to_string(),
                                anchor: true
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
                builder.heading(
                    3,
                    Markdown::new_paragraph(function.identifier.to_string()).code(),
                );

                builder.heading(4, "Arguments");
                builder.list(
                    false,
                    function
                        .arguments
                        .iter()
                        .enumerate()
                        .map(|(idx, arg)| {
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
                                Markdown::new_paragraph(markdown).code()
                            ]
                        })
                        .collect(),
                );

                builder.heading(4, "Return Type");

                builder.text(function.return_type.to_string());
            }
        }
    }
}
