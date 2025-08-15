use crate::{
    argument_visitor::MarkdownArgumentVisitor,
    markdown::{markdown_substring, IntoMarkdown, Markdown, MarkdownBuilder, TableBuilder},
    markdown_vec,
};
use ladfile::{
    ArgumentVisitor, LadArgument, LadBMSPrimitiveKind, LadFile, LadFunction, LadInstance, LadType,
    LadTypeId, LadTypeKind, LadTypeLayout,
};
use mdbook::book::{Chapter, SectionNumber};
use std::{borrow::Cow, collections::HashSet, path::PathBuf};

fn print_type(ladfile: &LadFile, type_: &LadTypeId) -> String {
    let mut visitor = MarkdownArgumentVisitor::new(ladfile);
    visitor.visit_lad_type_id(type_);
    visitor.build()
}

fn print_type_with_replacement(
    ladfile: &LadFile,
    type_: &LadTypeId,
    raw_type_id_replacement: &'static str,
) -> String {
    let mut visitor =
        MarkdownArgumentVisitor::new(ladfile).with_raw_type_id_replacement(raw_type_id_replacement);
    visitor.visit_lad_type_id(type_);
    visitor.build()
}

fn build_escaped_visitor(arg_visitor: MarkdownArgumentVisitor<'_>) -> String {
    arg_visitor
        .build()
        .replace("<", "\\<")
        .replace(">", "\\>")
        .replace("|", "\\|")
}

#[derive(Debug)]
pub(crate) enum SectionData<'a> {
    Summary {
        title: Option<String>,
    },
    /// A link directory to all the types within the ladfile
    TypeSummary,
    /// A link directory to all global functions within the ladfile
    FunctionSummary,
    /// A link directory to all global instances within the ladfile
    InstancesSummary,
    TypeDetail {
        lad_type_id: &'a LadTypeId,
        lad_type: &'a LadType,
    },
    FunctionDetail {
        types_directory: PathBuf,
        function: &'a LadFunction,
    },
}

/// Sections which convert to single markdown files
#[derive(Debug)]
pub(crate) struct Section<'a> {
    /// The path to the parent we can use for absolute links
    pub parent_path: PathBuf,
    pub ladfile: &'a LadFile,
    pub data: SectionData<'a>,
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
    pub(crate) fn new(parent_path: PathBuf, ladfile: &'a LadFile, data: SectionData<'a>) -> Self {
        Self {
            ladfile,
            data,
            parent_path,
        }
    }

    /// convert into a chapter, including children
    pub(crate) fn into_chapter(self, parent: Option<&Chapter>, index: usize) -> Chapter {
        let mut builder = MarkdownBuilder::new();
        self.to_markdown(&mut builder);

        let default_chapter = Chapter::default();
        let parent = match parent {
            Some(parent) => parent,
            None => &default_chapter,
        };

        let parent_path = self.parent_path.clone();

        let parent_source_path = parent
            .source_path
            .clone()
            .unwrap_or_default()
            .with_extension("");

        let current_number = if let Some(mut parent_number) = parent.number.clone() {
            parent_number.push(index as u32);
            parent_number
        } else {
            SectionNumber(vec![index as u32])
        };

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
        match &self.data {
            SectionData::Summary { title, .. } => {
                title.as_deref().unwrap_or("Bindings Summary").to_owned()
            }
            SectionData::TypeSummary { .. } => "Types".to_owned(),
            SectionData::FunctionSummary { .. } => "Functions".to_owned(),
            SectionData::InstancesSummary { .. } => "Globals".to_owned(),
            SectionData::TypeDetail { lad_type_id, .. } => print_type(self.ladfile, lad_type_id),
            SectionData::FunctionDetail { function, .. } => function.identifier.to_string(),
        }
    }

    pub(crate) fn is_code_heading(&self) -> bool {
        matches!(
            self.data,
            SectionData::TypeDetail { .. } | SectionData::FunctionDetail { .. }
        )
    }

    pub(crate) fn file_name(&self) -> String {
        linkify_filename(self.title()) + ".md"
    }

    pub(crate) fn children(&self) -> Vec<Section<'a>> {
        let child_parent_path = self
            .parent_path
            .join(linkify_filename(self.title()))
            .with_extension("");

        match self.data {
            SectionData::Summary { .. } => {
                vec![
                    Section::new(
                        child_parent_path.clone(),
                        self.ladfile,
                        SectionData::InstancesSummary,
                    ),
                    Section::new(
                        child_parent_path.clone(),
                        self.ladfile,
                        SectionData::FunctionSummary,
                    ),
                    Section::new(
                        child_parent_path.clone(),
                        self.ladfile,
                        SectionData::TypeSummary,
                    ),
                ]
            }
            SectionData::TypeSummary => self
                .ladfile
                .types
                .iter()
                .map(|(lad_type_id, lad_type)| {
                    Section::new(
                        child_parent_path.clone(),
                        self.ladfile,
                        SectionData::TypeDetail {
                            lad_type,
                            lad_type_id,
                        },
                    )
                })
                .collect(),

            SectionData::FunctionSummary => {
                let associated_functions = self
                    .ladfile
                    .types
                    .iter()
                    .flat_map(|t| &t.1.associated_functions)
                    .collect::<HashSet<_>>();

                let non_associated_functions = self
                    .ladfile
                    .functions
                    .iter()
                    .filter_map(|f| (!associated_functions.contains(f.0)).then_some(f.1));

                non_associated_functions
                    .map(|function| {
                        Section::new(
                            child_parent_path.clone(),
                            self.ladfile,
                            SectionData::FunctionDetail {
                                function,
                                types_directory: PathBuf::from("../types"),
                            },
                        )
                    })
                    .collect()
            }
            SectionData::InstancesSummary { .. } => {
                vec![]
            }
            SectionData::TypeDetail { lad_type, .. } => lad_type
                .associated_functions
                .iter()
                .filter_map(|f| {
                    let function = self.ladfile.functions.get(f)?;
                    Some(Section::new(
                        child_parent_path.clone(),
                        self.ladfile,
                        SectionData::FunctionDetail {
                            function,
                            types_directory: PathBuf::from("../../types"),
                        },
                    ))
                })
                .collect(),
            SectionData::FunctionDetail { .. } => vec![],
        }
    }

    pub(crate) fn section_items(&self) -> Vec<SectionItem<'_>> {
        match self.data {
            SectionData::Summary { .. } => {
                let title = self.title().clone();

                vec![SectionItem::Markdown {
                    markdown: Box::new(move |builder| {
                        builder.heading(2, "Contents");
                        builder.text("This is an automatically generated file, you'll find links to the contents below");
                        builder.table(|builder| {
                            builder.headers(vec!["Section", "Contents"]);
                            builder.row(markdown_vec![
                                Markdown::new_paragraph("Types").code(),
                                Markdown::Link {
                                    text: Box::new("Describes all available binding types"),
                                    url: format!("./{}/types.md", linkify_filename(title.clone())),
                                    anchor: false
                                }
                            ]);
                            builder.row(markdown_vec![
                                Markdown::new_paragraph("Global Functions").code(),
                                Markdown::Link {
                                    text:
                                        Box::new("Documents all the global functions present in the bindings"),
                                    url: format!(
                                        "./{}/functions.md",
                                        linkify_filename(title.clone())
                                    ),
                                    anchor: false
                                }
                            ]);
                            builder.row(markdown_vec![
                                Markdown::new_paragraph("Globals").code(),
                                Markdown::Link {
                                    text: Box::new("Documents all global variables present in the bindings"),
                                    url: format!(
                                        "./{}/globals.md",
                                        linkify_filename(title.clone())
                                    ),
                                    anchor: false
                                }
                            ]);
                        });
                    }),
                }]
            }
            SectionData::InstancesSummary => {
                let instances = self.ladfile.globals.iter().collect::<Vec<_>>();
                let types_directory = PathBuf::from("./types");
                vec![SectionItem::InstancesSummary {
                    instances,
                    ladfile: self.ladfile,
                    types_directory,
                }]
            }
            SectionData::TypeSummary => {
                let types = self.ladfile.types.keys().collect::<Vec<_>>();
                vec![SectionItem::TypesSummary {
                    types,
                    types_directory: PathBuf::from("./types").to_string_lossy().to_string(),
                    ladfile: self.ladfile,
                }]
            }
            SectionData::FunctionSummary => {
                let associated_functions = self
                    .ladfile
                    .types
                    .iter()
                    .flat_map(|t| &t.1.associated_functions)
                    .collect::<HashSet<_>>();

                let non_associated_functions = self
                    .ladfile
                    .functions
                    .iter()
                    .filter_map(|f| (!associated_functions.contains(f.0)).then_some(f.1))
                    .collect();
                vec![
                    SectionItem::Markdown {
                        markdown: Box::new(|builder| {
                            builder.heading(2, "Non-Associated Functions");
                            builder.text("Global functions that are not associated with any type and callable from anywhere in the script.");
                        }),
                    },
                    SectionItem::FunctionsSummary {
                        functions: non_associated_functions,
                        functions_directory: "functions".to_owned(),
                    },
                ]
            }
            SectionData::TypeDetail { lad_type, .. } => {
                let functions = lad_type
                    .associated_functions
                    .iter()
                    .filter_map(|i| self.ladfile.functions.get(i))
                    .collect::<Vec<_>>();
                vec![
                    SectionItem::Layout {
                        layout: &lad_type.layout,
                    },
                    SectionItem::Description { lad_type },
                    SectionItem::Markdown {
                        markdown: Box::new(|builder| {
                            builder.heading(2, "Associated Functions");
                        }),
                    },
                    SectionItem::FunctionsSummary {
                        functions,
                        functions_directory: linkify_filename(self.title()),
                    },
                ]
            }
            SectionData::FunctionDetail {
                function,
                ref types_directory,
            } => {
                vec![SectionItem::FunctionDetails {
                    function,
                    ladfile: self.ladfile,
                    types_directory: types_directory.clone(),
                }]
            }
        }
    }
}

impl IntoMarkdown for Section<'_> {
    fn to_markdown(&self, builder: &mut MarkdownBuilder) {
        if self.is_code_heading() {
            builder.heading(1, Markdown::new_paragraph(self.title()).code());
        } else {
            builder.heading(1, self.title());
        }

        for item in self.section_items() {
            item.to_markdown(builder);
        }
    }
}

const NO_DOCS_STRING: &str = "No Documentation 🚧";

/// Items which combine markdown elements to build a section
pub enum SectionItem<'a> {
    Markdown {
        markdown: Box<dyn Fn(&mut MarkdownBuilder) + 'static>,
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
        types_directory: PathBuf,
    },
    TypesSummary {
        types: Vec<&'a LadTypeId>,
        types_directory: String,
        ladfile: &'a ladfile::LadFile,
    },
    InstancesSummary {
        ladfile: &'a ladfile::LadFile,
        instances: Vec<(&'a Cow<'static, str>, &'a LadInstance)>,
        types_directory: PathBuf,
    },
}

impl std::fmt::Debug for SectionItem<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            SectionItem::Markdown { .. } => "Markdown",
            SectionItem::Layout { .. } => "Layout",
            SectionItem::Description { .. } => "Description",
            SectionItem::FunctionsSummary { .. } => "FunctionsSummary",
            SectionItem::FunctionDetails { .. } => "FunctionDetails",
            SectionItem::TypesSummary { .. } => "TypesSummary",
            SectionItem::InstancesSummary { .. } => "InstancesSummary",
        })
    }
}

impl IntoMarkdown for SectionItem<'_> {
    fn to_markdown(&self, builder: &mut MarkdownBuilder) {
        match self {
            SectionItem::Markdown { markdown } => (markdown)(builder),
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
                builder.heading(2, "Description").quote(Markdown::Raw(
                    description
                        .documentation
                        .as_deref()
                        .unwrap_or(NO_DOCS_STRING)
                        .to_owned(),
                ));
            }
            SectionItem::FunctionsSummary {
                functions,
                functions_directory: functions_path,
            } => {
                builder.text("For function details and documentation, click on the function link.");

                // make a table of functions as a quick reference, make them link to function details sub-sections
                builder.table(|builder| {
                    builder.headers(vec!["Function", "Summary"]);
                    for function in functions.iter() {
                        let first_col = function.identifier.to_string();

                        // first line with content from documentation trimmed to 100 chars
                        let second_col = function
                            .documentation
                            .as_deref()
                            .map(|doc| markdown_substring(doc, 100))
                            .unwrap_or_else(|| NO_DOCS_STRING.to_string());

                        builder.row(markdown_vec![
                            Markdown::Link {
                                text: Box::new(first_col),
                                url: format!("./{}/{}.md", functions_path, function.identifier),
                                anchor: false
                            },
                            Markdown::new_paragraph(second_col.to_string().replace("\n", " ")),
                        ]);
                    }
                });
            }
            SectionItem::TypesSummary {
                types,
                types_directory,
                ladfile,
            } => {
                builder.heading(2, "Available Types");
                builder.text("All registered reflect-able types which can be constructed and directly manipulated by scripts.");

                // make a table of types as a quick reference, make them link to type details sub-sections
                builder.table(|builder| {
                    builder.headers(vec!["Type", "Summary"]);
                    for type_ in types.iter() {
                        let printed_type_for_url = print_type(ladfile, type_);
                        let printed_type_pretty =
                            print_type_with_replacement(ladfile, type_, "Unknown");

                        let documentation = ladfile.get_type_documentation(type_);

                        // first line with content from documentation trimmed to 100 chars
                        let second_col = documentation
                            .map(|doc| markdown_substring(doc, 100))
                            .unwrap_or_else(|| NO_DOCS_STRING.to_string());

                        let mut link_builder = MarkdownBuilder::new();
                        link_builder.tight_inline();
                        link_builder.link(
                            Markdown::new_paragraph(printed_type_pretty).code(),
                            format!(
                                "./{types_directory}/{}.md",
                                linkify_filename(printed_type_for_url)
                            ),
                        );

                        builder.row(markdown_vec![
                            link_builder,
                            Markdown::new_paragraph(second_col.replace("\n", " ")),
                        ]);
                    }
                });
            }
            SectionItem::InstancesSummary {
                instances,
                ladfile,
                types_directory,
            } => {
                builder.heading(2, "Global Values");
                builder.text("Global values that are accessible anywhere inside scripts. You should avoid naming conflicts with these and trying to overwrite or edit them.");
                // make a table of instances as a quick reference, make them link to instance details sub-sections

                // first build a non-static instance table
                let instances = instances
                    .iter()
                    .map(|(k, v)| {
                        let name = k.to_string();
                        let types_directory = types_directory.clone();
                        let mut arg_visitor = MarkdownArgumentVisitor::new_with_linkifier(
                            ladfile,
                            move |lad_type_id, ladfile| {
                                let printed_type =
                                    linkify_filename(print_type(ladfile, &lad_type_id));
                                Some(types_directory.join(printed_type).with_extension("md"))
                            },
                        );
                        arg_visitor.visit(&v.type_kind);
                        let escaped = build_escaped_visitor(arg_visitor);
                        (v.is_static, name, escaped)
                    })
                    .collect::<Vec<_>>();

                builder.heading(3, "Instances");
                builder.text("Instances containing actual accessible values.");
                builder.table(|builder| {
                    builder.headers(vec!["Instance", "Type"]);
                    for (_, name, instance) in instances.iter().filter(|(a, _, _)| !*a) {
                        builder.row(markdown_vec![
                            Markdown::new_paragraph(name).code(),
                            Markdown::Raw(instance.clone())
                        ]);
                    }
                });

                builder.heading(3, "Static Instances");
                builder.text("Static type references, existing for the purpose of typed static function calls.");
                builder.table(|builder| {
                    builder.headers(vec!["Instance", "Type"]);
                    for (_, name, instance) in instances.iter().filter(|(a, _, _)| *a) {
                        builder.row(markdown_vec![
                            Markdown::new_paragraph(name).code(),
                            Markdown::Raw(instance.clone())
                        ]);
                    }
                });
            }
            SectionItem::FunctionDetails {
                function,
                ladfile,
                types_directory,
            } => {
                // if the function takes in a FunctionCallContext argument, we notify that it's an impure function
                // which potentially tries to access anything in the world
                if function.arguments.iter().any(|a| {
                    matches!(
                        a.kind,
                        LadTypeKind::Primitive(LadBMSPrimitiveKind::FunctionCallContext)
                    )
                }) {
                    builder.raw(
                    r#"
                        <div class="warning">
                            This function is impure, it might potentially try to access anything in the world.
                            If you are using it in the context of a script system, it might cause access errors.
                        </div>
                    "#.trim(),
                    );
                    builder.append("\n\n");
                }

                // we don't escape this, this is already markdown
                builder.quote(Markdown::Raw(
                    function
                        .documentation
                        .as_deref()
                        .unwrap_or(NO_DOCS_STRING)
                        .to_owned(),
                ));

                builder.heading(4, "Arguments");
                let headers = vec!["Name", "Type", "Documentation"];
                builder.table(|builder| {
                    builder.headers(headers.clone());
                    for (idx, arg) in function.arguments.iter().enumerate() {
                        build_lad_function_argument_row(
                            idx,
                            arg,
                            ladfile,
                            types_directory.clone(),
                            builder,
                        );
                    }
                });

                builder.heading(4, "Returns");
                builder.table(|builder| {
                    builder.headers(headers.clone());
                    build_lad_function_argument_row(
                        0,
                        &function.return_type,
                        ladfile,
                        types_directory.clone(),
                        builder,
                    )
                });
            }
        }
    }
}

fn build_lad_function_argument_row(
    idx: usize,
    arg: &LadArgument,
    ladfile: &LadFile,
    types_directory: PathBuf,
    builder: &mut TableBuilder,
) {
    // we exclude function call context as it's not something scripts pass down
    if matches!(
        arg.kind,
        LadTypeKind::Primitive(LadBMSPrimitiveKind::FunctionCallContext)
    ) {
        return;
    }

    let types_directory = types_directory.to_owned();
    let mut arg_visitor =
        MarkdownArgumentVisitor::new_with_linkifier(ladfile, move |lad_type_id, ladfile| {
            let printed_type = linkify_filename(print_type(ladfile, &lad_type_id));
            Some(types_directory.join(printed_type).with_extension("md"))
        });
    arg_visitor.visit(&arg.kind);
    let markdown = build_escaped_visitor(arg_visitor);

    let arg_name = arg
        .name
        .as_ref()
        .cloned()
        .unwrap_or_else(|| Cow::Owned(format!("arg{idx}")));

    builder.row(markdown_vec![
        Markdown::new_paragraph(arg_name).bold(),
        Markdown::Raw(markdown),
        Markdown::Raw(
            arg.documentation
                .as_deref()
                .unwrap_or(NO_DOCS_STRING)
                .to_owned()
        )
    ]);
}
