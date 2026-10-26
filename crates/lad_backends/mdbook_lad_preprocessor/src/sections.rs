use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

use ladfile::{
    LadArgument, LadFile, LadFunction, LadFunctionNamespace, LadGenericType, LadInstance,
    LadInstantiation, LadNamedType, LadTypeDefinition, LadTypeId, LadTypeKind, LadTypeLayout,
    LadVisitable, ReflectionPrimitiveKind,
};
use mdbook_preprocessor::book::{BookItem, Chapter, SectionNumber};

use crate::{
    argument_visitor::MarkdownArgumentVisitor,
    markdown::{IntoMarkdown, Markdown, MarkdownBuilder, TableBuilder, markdown_substring},
    markdown_vec,
};

fn print_type(
    ladfile: &LadFile,
    type_: &impl LadVisitable,
    raw_type_id_replacement: Option<&'static str>,
    linkifier_base_path_and_escape_opt: Option<(PathBuf, bool)>,
) -> String {
    let mut visitor =
        if let Some((linkifier_base_path, _)) = linkifier_base_path_and_escape_opt.clone() {
            MarkdownArgumentVisitor::new_with_linkifier(ladfile, move |str| {
                let printed_type = linkify_filename(str);
                linkifier_base_path.join(printed_type).with_extension("md")
            })
        } else {
            MarkdownArgumentVisitor::new(ladfile)
        };
    if let Some(replacement) = raw_type_id_replacement {
        visitor = visitor.with_raw_type_id_replacement(replacement);
    }

    type_.accept(&mut visitor);
    let mut printed = visitor.build();
    if let Some((_, escape_opt)) = linkifier_base_path_and_escape_opt
        && escape_opt
    {
        printed = escape_table_cell(printed)
    }
    printed
}

fn escape_table_cell(markdown: String) -> String {
    markdown
        .replace("<", "\\<")
        .replace(">", "\\>")
        .replace("|", "\\|")
}

#[derive(Debug)]
pub(crate) enum SectionData<'a> {
    Summary {
        title: Option<String>,
    },
    /// A link directory to the namespaces of the types within the ladfile
    TypeSummary,
    /// The types of a namespace, as returned by `import_namespace`. The empty namespace holds the types without a crate, i.e. primitives
    Namespace {
        namespace: &'a str,
    },
    /// The builtins: the values and functions every script has without importing anything
    FunctionSummary,
    TypeDetail {
        lad_type_id: &'a LadTypeId,
        lad_type: &'a LadTypeDefinition,
    },
    /// A generic type, which is documented once for all its instantiations
    GenericTypeDetail {
        generic_type_id: &'a LadTypeId,
        generic_type: &'a LadGenericType,
    },
    FunctionDetail {
        types_directory: PathBuf,
        function: &'a LadFunction,
        /// What the title is prefixed with: the variant of an enum, or the instantiation of a generic type the function is on, if any
        variant: Option<Cow<'a, str>>,
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

/// The title of the page of a namespace, the empty namespace is the one of the types without a crate
fn namespace_title(namespace: &str) -> &str {
    if namespace.is_empty() {
        "root"
    } else {
        namespace
    }
}

/// A function with what its title is prefixed with, see [`function_title`]
type TitledFunction<'a> = (Option<Cow<'a, str>>, &'a LadFunction);

/// The title of a function's page, which includes the variant for functions on variants of enums,
/// as these all have the same names, i.e. `Variant.new`, and the instantiation for functions of a single
/// instantiation of a generic type, i.e. `Time<Virtual>.pause`.
fn function_title(variant: Option<&str>, function: &LadFunction) -> String {
    match variant {
        Some(variant) => format!("{variant}.{}", function.identifier_with_overload()),
        None => function.identifier_with_overload().to_string(),
    }
}

/// Makes a filename safe to put in links, and to create on any platform.
///
/// Type names can contain anything, i.e. the paths of unregistered types, `::`, `<`, `,`... but windows,
/// for example, does not allow `:` in filenames. Extensions are added by callers, so `.` is not kept either.
pub fn linkify_filename(name: impl Into<String>) -> String {
    name.into()
        .to_lowercase()
        .replace(' ', "_")
        .chars()
        .filter(|c| c.is_alphanumeric() || matches!(c, '_' | '-'))
        .collect()
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
            SectionNumber::new(vec![index as u32])
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
            .map(BookItem::Chapter)
            .collect();

        chapter
    }

    pub(crate) fn title(&self) -> String {
        match &self.data {
            SectionData::Summary { title, .. } => {
                title.as_deref().unwrap_or("Bindings Summary").to_owned()
            }
            SectionData::TypeSummary { .. } => "Types".to_owned(),
            SectionData::Namespace { namespace } => namespace_title(namespace).to_owned(),
            SectionData::FunctionSummary { .. } => "Builtins".to_owned(),
            SectionData::TypeDetail { lad_type_id, .. } => {
                print_type(self.ladfile, *lad_type_id, None, None)
            }
            SectionData::GenericTypeDetail { generic_type, .. } => generic_type.name_with_params(),
            SectionData::FunctionDetail {
                function, variant, ..
            } => function_title(variant.as_deref(), function),
        }
    }

    /// The builtin functions, as opposed to the ones on types, variants of enums and in namespaces
    fn global_functions(&self) -> Vec<&'a LadFunction> {
        self.ladfile
            .builtin_functions()
            .map(|(_, function)| function)
            .collect()
    }

    /// The functions which are members of the namespace, i.e. `import_namespace("bms").log_info`
    fn namespace_functions(&self, namespace: &str) -> Vec<TitledFunction<'a>> {
        self.ladfile
            .namespaces
            .get(namespace)
            .into_iter()
            .flat_map(|lad_namespace| lad_namespace.functions.values())
            .filter_map(|id| self.ladfile.functions.get(id))
            .map(|function| (None, function))
            .collect()
    }

    /// The functions on the variants of the given enum, with the variant they are on
    fn variant_functions(&self, enum_id: &LadTypeId) -> Vec<TitledFunction<'a>> {
        self.ladfile
            .functions
            .values()
            .filter_map(|function| match &function.namespace {
                LadFunctionNamespace::Variant { type_id, variant }
                    if type_id.type_id() == Some(enum_id) =>
                {
                    Some((Some(Cow::Borrowed(variant.as_ref())), function))
                }
                _ => None,
            })
            .collect()
    }

    /// The functions with a page below the type's page, the ones on the type and the ones on its variants
    fn type_functions(
        &self,
        lad_type_id: &LadTypeId,
        lad_type: &'a LadTypeDefinition,
    ) -> Vec<TitledFunction<'a>> {
        lad_type
            .associated_functions
            .iter()
            .filter_map(|f| Some((None, self.ladfile.functions.get(f)?)))
            .chain(self.variant_functions(lad_type_id))
            .collect()
    }

    /// The functions with a page below the generic type's page: the ones of every instantiation and of their variants,
    /// then the ones of a single instantiation, titled by it, i.e. `Time<Virtual>.pause`
    fn generic_type_functions(
        &self,
        generic_type_id: &LadTypeId,
        generic_type: &'a LadGenericType,
    ) -> Vec<TitledFunction<'a>> {
        let variant_functions =
            self.ladfile
                .functions
                .values()
                .filter_map(|function| match &function.namespace {
                    LadFunctionNamespace::Variant {
                        type_id: LadNamedType::Generic { generic_type, args },
                        variant,
                    } if generic_type == generic_type_id
                        && args
                            .iter()
                            .all(|arg| matches!(arg.named(), Some(LadNamedType::Param { .. }))) =>
                    {
                        Some((Some(Cow::Borrowed(variant.as_ref())), function))
                    }
                    _ => None,
                });
        let instantiation_functions =
            generic_type
                .instantiations
                .iter()
                .flat_map(|instantiation| {
                    let title = print_type(
                        self.ladfile,
                        &LadTypeKind::Val(LadNamedType::Generic {
                            generic_type: generic_type_id.clone(),
                            args: instantiation.args.clone(),
                        }),
                        None,
                        None,
                    );
                    instantiation
                        .associated_functions
                        .iter()
                        .filter_map(|f| self.ladfile.functions.get(f))
                        .map(move |function| (Some(Cow::Owned(title.clone())), function))
                });
        generic_type
            .associated_functions
            .iter()
            .filter_map(|f| Some((None, self.ladfile.functions.get(f)?)))
            .chain(variant_functions)
            .chain(instantiation_functions)
            .collect()
    }

    pub(crate) fn is_code_heading(&self) -> bool {
        matches!(
            self.data,
            SectionData::TypeDetail { .. }
                | SectionData::GenericTypeDetail { .. }
                | SectionData::FunctionDetail { .. }
        )
    }

    /// The namespaces types or namespace members are in, sorted, with the root namespace last
    fn namespaces(&self) -> Vec<&'a str> {
        let mut namespaces = self
            .ladfile
            .types
            .iter()
            .map(|(_, lad_type)| lad_type.namespace.as_str())
            .chain(
                self.ladfile
                    .generic_types
                    .values()
                    .map(|generic_type| generic_type.namespace.as_str()),
            )
            .chain(
                self.ladfile
                    .namespaces
                    .keys()
                    .map(|namespace| namespace.as_ref()),
            )
            .collect::<Vec<_>>();
        namespaces.sort_by_key(|namespace| (namespace.is_empty(), *namespace));
        namespaces.dedup();
        namespaces
    }

    /// The ids of the types in the namespace which have a page of their own, generic types last
    fn types_in_namespace(&self, namespace: &str) -> Vec<&'a LadTypeId> {
        self.ladfile
            .types
            .iter()
            .filter(|(_, lad_type)| lad_type.namespace == namespace)
            .map(|(id, _)| id)
            .chain(
                self.ladfile
                    .generic_types
                    .iter()
                    .filter(|(_, generic_type)| generic_type.namespace == namespace)
                    .map(|(id, _)| id),
            )
            .collect()
    }

    pub(crate) fn file_name(&self) -> String {
        match self.data {
            // namespaces share a directory with the types, which can have the name of a namespace, i.e. `Time` and `time`
            SectionData::Namespace { .. } => {
                format!("namespace_{}.md", linkify_filename(self.title()))
            }
            _ => linkify_filename(self.title()) + ".md",
        }
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
                .namespaces()
                .into_iter()
                .map(|namespace| {
                    Section::new(
                        child_parent_path.clone(),
                        self.ladfile,
                        SectionData::Namespace { namespace },
                    )
                })
                .collect(),
            // the pages of the types stay next to the page of the namespace, so they link to each other
            // in the same way no matter the namespace
            SectionData::Namespace { namespace } => self
                .ladfile
                .types
                .iter()
                .filter(|(_, lad_type)| lad_type.namespace == namespace)
                .map(|(lad_type_id, lad_type)| {
                    Section::new(
                        self.parent_path.clone(),
                        self.ladfile,
                        SectionData::TypeDetail {
                            lad_type,
                            lad_type_id,
                        },
                    )
                })
                .chain(
                    self.ladfile
                        .generic_types
                        .iter()
                        .filter(|(_, generic_type)| generic_type.namespace == namespace)
                        .map(|(generic_type_id, generic_type)| {
                            Section::new(
                                self.parent_path.clone(),
                                self.ladfile,
                                SectionData::GenericTypeDetail {
                                    generic_type_id,
                                    generic_type,
                                },
                            )
                        }),
                )
                .chain(self.namespace_functions(namespace).into_iter().map(
                    |(variant, function)| {
                        Section::new(
                            child_parent_path.clone(),
                            self.ladfile,
                            SectionData::FunctionDetail {
                                function,
                                types_directory: PathBuf::from("../../types"),
                                variant,
                            },
                        )
                    },
                ))
                .collect(),

            SectionData::FunctionSummary => self
                .global_functions()
                .into_iter()
                .map(|function| {
                    Section::new(
                        child_parent_path.clone(),
                        self.ladfile,
                        SectionData::FunctionDetail {
                            function,
                            types_directory: PathBuf::from("../types"),
                            variant: None,
                        },
                    )
                })
                .collect(),
            SectionData::TypeDetail {
                lad_type_id,
                lad_type,
            } => self
                .type_functions(lad_type_id, lad_type)
                .into_iter()
                .map(|(variant, function)| {
                    Section::new(
                        child_parent_path.clone(),
                        self.ladfile,
                        SectionData::FunctionDetail {
                            function,
                            types_directory: PathBuf::from("../../types"),
                            variant,
                        },
                    )
                })
                .collect(),
            SectionData::GenericTypeDetail {
                generic_type_id,
                generic_type,
            } => self
                .generic_type_functions(generic_type_id, generic_type)
                .into_iter()
                .map(|(variant, function)| {
                    Section::new(
                        child_parent_path.clone(),
                        self.ladfile,
                        SectionData::FunctionDetail {
                            function,
                            types_directory: PathBuf::from("../../types"),
                            variant,
                        },
                    )
                })
                .collect(),
            SectionData::FunctionDetail { .. } => vec![],
        }
    }

    pub(crate) fn section_items(&self) -> Vec<SectionItem<'_>> {
        match self.data {
            SectionData::Summary { .. } => self.summary_items(),
            SectionData::TypeSummary => self.type_summary_items(),
            SectionData::Namespace { namespace } => self.namespace_items(namespace),
            SectionData::FunctionSummary => self.function_summary_items(),
            SectionData::TypeDetail {
                lad_type_id,
                lad_type,
            } => self.type_detail_items(lad_type_id, lad_type),
            SectionData::GenericTypeDetail {
                generic_type_id,
                generic_type,
            } => self.generic_type_detail_items(generic_type_id, generic_type),
            SectionData::FunctionDetail {
                function,
                ref types_directory,
                ..
            } => {
                vec![SectionItem::FunctionDetails {
                    function,
                    ladfile: self.ladfile,
                    types_directory: types_directory.clone(),
                }]
            }
        }
    }

    /// The summary page, linking to the types and builtins pages
    fn summary_items(&self) -> Vec<SectionItem<'a>> {
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
                        Markdown::new_paragraph("Builtins").code(),
                        Markdown::Link {
                            text: Box::new(
                                "The values and functions every script has without importing anything"
                            ),
                            url: format!("./{}/builtins.md", linkify_filename(title.clone())),
                            anchor: false
                        }
                    ]);
                });
            }),
        }]
    }

    /// The page listing the namespaces, with the number of types in each
    fn type_summary_items(&self) -> Vec<SectionItem<'a>> {
        // the namespace pages are in the directory of the type pages, which is next to this page
        let namespaces = self
            .namespaces()
            .into_iter()
            .map(|namespace| {
                let page = Section::new(
                    PathBuf::new(),
                    self.ladfile,
                    SectionData::Namespace { namespace },
                );
                (
                    page.title(),
                    page.file_name(),
                    self.types_in_namespace(namespace).len(),
                )
            })
            .collect::<Vec<_>>();
        vec![SectionItem::Markdown {
            markdown: Box::new(move |builder| {
                builder.heading(2, "Namespaces");
                builder.raw("Types are grouped by the namespace scripts import them from, i.e. `import_namespace(\"math\")`.");
                builder.append("\n\n");
                builder.table(|builder| {
                    builder.headers(vec!["Namespace", "Types"]);
                    for (title, file_name, count) in namespaces.iter() {
                        builder.row(markdown_vec![
                            Markdown::Link {
                                text: Box::new(Markdown::new_paragraph(title.clone()).code()),
                                url: format!("./types/{file_name}"),
                                anchor: false
                            },
                            Markdown::new_paragraph(count.to_string()),
                        ]);
                    }
                });
            }),
        }]
    }

    /// The page of a namespace, listing its types and functions
    fn namespace_items(&self, namespace: &str) -> Vec<SectionItem<'a>> {
        let intro = if namespace.is_empty() {
            "The types without a crate, such as primitives.".to_owned()
        } else {
            format!(
                "The types and functions in the `{namespace}` namespace, as returned by `import_namespace(\"{namespace}\")`."
            )
        };
        let mut items = vec![
            SectionItem::Markdown {
                markdown: Box::new(move |builder| {
                    builder.raw(intro.clone());
                    builder.append("\n\n");
                }),
            },
            SectionItem::TypesSummary {
                types: self.types_in_namespace(namespace),
                // the pages of the types are next to this one
                types_directory: ".".to_owned(),
                ladfile: self.ladfile,
            },
        ];
        let values = self
            .ladfile
            .namespaces
            .get(namespace)
            .map(|lad_namespace| lad_namespace.values.iter().collect::<Vec<_>>())
            .unwrap_or_default();
        if !values.is_empty() {
            items.push(SectionItem::InstancesSummary {
                instances: values,
                ladfile: self.ladfile,
                types_directory: PathBuf::from("."),
            });
        }
        let functions = self.namespace_functions(namespace);
        if !functions.is_empty() {
            items.push(SectionItem::Markdown {
                markdown: Box::new(|builder| {
                    builder.heading(2, "Functions");
                }),
            });
            items.push(SectionItem::FunctionsSummary {
                functions,
                functions_directory: linkify_filename(self.title()),
            });
        }
        items
    }

    /// The page of the builtins: the functions every script has without importing anything
    fn function_summary_items(&self) -> Vec<SectionItem<'a>> {
        vec![
            SectionItem::Markdown {
                markdown: Box::new(|builder| {
                    builder.raw("The only globals of scripts. Everything else is imported from a namespace, see the namespace pages under Types, i.e. `import_namespace(\"bms\").world`.");
                    builder.append("\n\n");
                }),
            },
            SectionItem::FunctionsSummary {
                functions: self
                    .global_functions()
                    .into_iter()
                    .map(|function| (None, function))
                    .collect(),
                functions_directory: linkify_filename(self.title()),
            },
        ]
    }

    /// The page of a type, with its layout, description and associated functions
    fn type_detail_items(
        &self,
        lad_type_id: &LadTypeId,
        lad_type: &'a LadTypeDefinition,
    ) -> Vec<SectionItem<'a>> {
        let functions = self.type_functions(lad_type_id, lad_type);
        vec![
            SectionItem::Layout {
                layout: &lad_type.layout,
                ladfile: self.ladfile,
                types_directory: PathBuf::from("./"),
            },
            SectionItem::Description {
                documentation: lad_type.documentation.as_deref(),
            },
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

    /// The page of a generic type, with its layout in terms of its parameters, the arguments it is instantiated with and its functions
    fn generic_type_detail_items(
        &self,
        generic_type_id: &LadTypeId,
        generic_type: &'a LadGenericType,
    ) -> Vec<SectionItem<'a>> {
        let instantiations = generic_type.instantiations.as_slice();
        let functions = self.generic_type_functions(generic_type_id, generic_type);
        vec![
            SectionItem::Layout {
                layout: &generic_type.layout,
                ladfile: self.ladfile,
                types_directory: PathBuf::from("./"),
            },
            SectionItem::Description {
                documentation: generic_type.documentation.as_deref(),
            },
            SectionItem::Instantiations {
                instantiations,
                ladfile: self.ladfile,
                types_directory: PathBuf::from("./"),
            },
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
        ladfile: &'a LadFile,
        types_directory: PathBuf,
    },
    Description {
        documentation: Option<&'a str>,
    },
    /// The instantiations of a generic type
    Instantiations {
        instantiations: &'a [LadInstantiation],
        ladfile: &'a LadFile,
        types_directory: PathBuf,
    },
    FunctionsSummary {
        /// The functions, with the variant of an enum they are on if any
        functions: Vec<TitledFunction<'a>>,
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
            SectionItem::Instantiations { .. } => "Instantiations",
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
            SectionItem::Layout {
                layout,
                ladfile,
                types_directory,
            } => layout_markdown(layout, ladfile, types_directory, builder),
            SectionItem::Description { documentation } => {
                builder.heading(2, "Description").quote(Markdown::Raw(
                    documentation.unwrap_or(NO_DOCS_STRING).to_owned(),
                ));
            }
            SectionItem::Instantiations {
                instantiations,
                ladfile,
                types_directory,
            } => instantiations_markdown(instantiations, ladfile, types_directory, builder),
            SectionItem::FunctionsSummary {
                functions,
                functions_directory,
            } => functions_summary_markdown(functions, functions_directory, builder),
            SectionItem::TypesSummary {
                types,
                types_directory,
                ladfile,
            } => types_summary_markdown(types, types_directory, ladfile, builder),
            SectionItem::InstancesSummary {
                instances,
                ladfile,
                types_directory,
            } => instances_summary_markdown(instances, ladfile, types_directory, builder),
            SectionItem::FunctionDetails {
                function,
                ladfile,
                types_directory,
            } => function_details_markdown(function, ladfile, types_directory, builder),
        }
    }
}

fn build_lad_function_argument_row(
    arg: &LadArgument,
    ladfile: &LadFile,
    types_directory: PathBuf,
    builder: &mut TableBuilder,
) {
    let types_directory = types_directory.to_owned();
    let escaped = print_type(ladfile, &arg.kind, None, Some((types_directory, false)));

    // arguments are always named, returns only if their docs name them
    let arg_name = arg.name.as_deref().unwrap_or("return").to_owned();

    builder.row(markdown_vec![
        Markdown::new_paragraph(arg_name).bold(),
        Markdown::Raw(escaped),
        Markdown::Raw(
            arg.documentation
                .as_deref()
                .unwrap_or(NO_DOCS_STRING)
                .to_owned()
        )
    ]);
}

/// Lists the fields of each variant of a type, or marks the type as opaque if it has none
fn layout_markdown(
    layout: &LadTypeLayout,
    ladfile: &LadFile,
    types_directory: &Path,
    builder: &mut MarkdownBuilder,
) {
    // process the variants here
    let opaque = layout.for_each_variant(
        |v, _i| match v {
            ladfile::LadVariant::TupleStruct { name, fields } => {
                builder.heading(3, name.to_string()).list(
                    true,
                    fields
                        .iter()
                        .map(|f| {
                            Markdown::Raw(print_type(
                                ladfile,
                                &f.type_,
                                None,
                                Some((types_directory.to_path_buf(), true)),
                            ))
                        })
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
                                Markdown::Raw(print_type(
                                    ladfile,
                                    &f.type_,
                                    None,
                                    Some((types_directory.to_path_buf(), true))
                                ))
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

/// Lists the arguments each instantiation of a generic type is used with
fn instantiations_markdown(
    instantiations: &[LadInstantiation],
    ladfile: &LadFile,
    types_directory: &Path,
    builder: &mut MarkdownBuilder,
) {
    builder.heading(2, "Instantiations");
    builder.text("The arguments this generic type is used with.");
    builder.list(
        false,
        instantiations
            .iter()
            .map(|instantiation| {
                let arguments = instantiation
                    .args
                    .iter()
                    .map(|argument| {
                        print_type(
                            ladfile,
                            argument,
                            None,
                            Some((types_directory.to_path_buf(), true)),
                        )
                    })
                    .collect::<Vec<_>>();
                Markdown::Raw(arguments.join(", "))
            })
            .collect(),
    );
}

/// A table of functions as a quick reference, linking to the function details sub-pages
fn functions_summary_markdown(
    functions: &[TitledFunction<'_>],
    functions_path: &str,
    builder: &mut MarkdownBuilder,
) {
    builder.text("For function details and documentation, click on the function link.");

    // make a table of functions as a quick reference, make them link to function details sub-sections
    builder.table(|builder| {
        builder.headers(vec!["Function", "Summary"]);
        for (variant, function) in functions.iter() {
            let first_col = function_title(variant.as_deref(), function);

            // first line with content from documentation trimmed to 100 chars
            let second_col = function
                .documentation
                .as_deref()
                .map(|doc| markdown_substring(doc, 100))
                .unwrap_or_else(|| NO_DOCS_STRING.to_string());

            builder.row(markdown_vec![
                Markdown::Link {
                    text: Box::new(first_col.clone()),
                    url: format!("./{}/{}.md", functions_path, linkify_filename(first_col)),
                    anchor: false
                },
                Markdown::new_paragraph(second_col.to_string().replace("\n", " ")),
            ]);
        }
    });
}

/// A table of types as a quick reference, linking to the type details sub-pages
fn types_summary_markdown(
    types: &[&LadTypeId],
    types_directory: &str,
    ladfile: &LadFile,
    builder: &mut MarkdownBuilder,
) {
    builder.heading(2, "Available Types");
    builder.text("All registered reflect-able types which can be constructed and directly manipulated by scripts.");

    // make a table of types as a quick reference, make them link to type details sub-sections
    builder.table(|builder| {
        builder.headers(vec!["Type", "Summary"]);
        for type_ in types.iter() {
            let printed_type_for_url = print_type(ladfile, *type_, None, None);
            let printed_type_pretty = print_type(ladfile, *type_, Some("Unknown"), None);

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

/// A table of the values in a namespace, i.e. `world` in `bms`
fn instances_summary_markdown(
    instances: &[(&Cow<'static, str>, &LadInstance)],
    ladfile: &LadFile,
    types_directory: &Path,
    builder: &mut MarkdownBuilder,
) {
    builder.heading(2, "Values");
    builder.table(|builder| {
        builder.headers(vec!["Name", "Type"]);
        for (name, instance) in instances {
            let printed = print_type(
                ladfile,
                &instance.type_kind,
                None,
                Some((types_directory.to_path_buf(), true)),
            );
            builder.row(markdown_vec![
                Markdown::new_paragraph(name.to_string()).code(),
                Markdown::Raw(printed)
            ]);
        }
    });
}

/// The details of a function: its impurity, documentation, arguments and return type
fn function_details_markdown(
    function: &LadFunction,
    ladfile: &LadFile,
    types_directory: &Path,
    builder: &mut MarkdownBuilder,
) {
    // if the function takes in a FunctionCallContext argument, we notify that it's an impure function
    // which potentially tries to access anything in the world
    if function.arguments.iter().any(|a| {
        matches!(
            a.kind,
            LadTypeKind::Primitive(ReflectionPrimitiveKind::FunctionCallContext)
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
        for arg in &function.arguments {
            build_lad_function_argument_row(arg, ladfile, types_directory.to_path_buf(), builder);
        }
    });

    builder.heading(4, "Returns");
    builder.table(|builder| {
        builder.headers(headers.clone());
        build_lad_function_argument_row(
            &function.return_type,
            ladfile,
            types_directory.to_path_buf(),
            builder,
        )
    });
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup_ladfile() -> LadFile {
        ladfile::parse_lad_file(ladfile::EXAMPLE_LADFILE).unwrap()
    }

    /// The pages of the namespaces, which are the children of the types page
    fn namespace_pages(ladfile: &LadFile) -> Vec<Section<'_>> {
        Section::new(PathBuf::new(), ladfile, SectionData::TypeSummary).children()
    }

    /// The pages of the types, which are below the pages of their namespaces
    fn type_pages(ladfile: &LadFile) -> Vec<Section<'_>> {
        namespace_pages(ladfile)
            .into_iter()
            .flat_map(|namespace| namespace.children())
            .collect()
    }

    #[test]
    fn generic_types_have_a_page_and_their_instantiations_do_not() {
        let ladfile = setup_ladfile();
        let titles = type_pages(&ladfile)
            .iter()
            .map(|child| child.title())
            .collect::<Vec<_>>();

        assert!(
            titles.contains(&"OpaqueGenericType<T>".to_owned()),
            "{titles:?}"
        );
        assert!(
            !titles
                .iter()
                .any(|title| title.starts_with("OpaqueGenericType<U")),
            "{titles:?}"
        );
        // generic types which were not declared are only known by their instantiations
        assert!(
            titles.contains(&"GenericStructType<Usize>".to_owned()),
            "{titles:?}"
        );

        // and the pages of the namespaces list the same
        let listed = namespace_pages(&ladfile)
            .iter()
            .map(|namespace| match namespace.section_items().as_slice() {
                [_, SectionItem::TypesSummary { types, .. }, ..] => types.len(),
                items => panic!("expected a types summary, got {items:?}"),
            })
            .sum::<usize>();
        assert_eq!(listed, titles.len());
    }

    /// What windows can't have in a filename, and what would be taken for an extension or a path
    const INVALID_IN_FILENAMES: &[char] = &['<', '>', ':', '"', '/', '\\', '|', '?', '*', '.', ','];

    #[test]
    fn filenames_can_be_created_on_every_platform() {
        let name = linkify_filename(
            r#"MeshExtractableData<alloc::vec::Vec<alloc::string::String>, "a|b?*"> v1.5\x/y"#,
        );
        assert!(!name.contains(INVALID_IN_FILENAMES), "{name}");
        assert_eq!(
            linkify_filename("MeshExtractableData<alloc::vec::Vec<alloc::string::String>>"),
            "meshextractabledataallocvecvecallocstringstring"
        );
        // names which were fine before are unchanged
        assert_eq!(
            linkify_filename("GenericStructType<Usize>"),
            "genericstructtypeusize"
        );
        assert_eq!(linkify_filename("Bindings Summary"), "bindings_summary");
        assert_eq!(linkify_filename("hello_world-1"), "hello_world-1");
    }

    #[test]
    fn pages_of_types_with_unknown_arguments_can_be_created_on_every_platform() {
        let mut ladfile = setup_ladfile();

        // a generic type which was not declared, with an argument the file doesn't know the type of,
        // as is the case for containers, which are not types of their own
        let generic_struct =
            LadTypeId::new_string_id("ladfile_builder::test::GenericStructType<usize>".into());
        let mut definition = ladfile.types[&generic_struct].clone();
        definition.generics[0].type_id =
            LadTypeId::new_string_id("alloc::vec::Vec<alloc::string::String>".into());
        ladfile.types.insert(
            LadTypeId::new_string_id("with_unknown_argument".into()),
            definition,
        );

        let pages = type_pages(&ladfile);
        let unknown = pages
            .iter()
            .find(|page| page.title().contains("alloc::vec::Vec"))
            .expect("the page is there");
        assert_eq!(
            unknown.file_name(),
            "genericstructtypeallocvecvecallocstringstring.md"
        );

        for page in pages {
            let file_name = page.file_name();
            let stem = file_name.strip_suffix(".md").unwrap();
            assert!(!stem.contains(INVALID_IN_FILENAMES), "{file_name}");
        }
    }

    #[test]
    fn types_are_sectioned_by_namespace() {
        let ladfile = setup_ladfile();
        let namespaces = namespace_pages(&ladfile);

        // the root namespace, which holds the primitives, comes last
        let titles = namespaces
            .iter()
            .map(|page| page.title())
            .collect::<Vec<_>>();
        assert_eq!(titles, ["ladfile_builder", "root"]);
        let file_names = namespaces
            .iter()
            .map(|page| page.file_name())
            .collect::<Vec<_>>();
        assert_eq!(
            file_names,
            ["namespace_ladfile_builder.md", "namespace_root.md"]
        );

        let types = |namespace: &Section| {
            namespace
                .children()
                .iter()
                .map(|page| page.title())
                .collect::<Vec<_>>()
        };
        let in_crate = types(&namespaces[0]);
        for expected in [
            "PlainStructType",
            "EnumType",
            "GenericStructType<Usize>",
            "OpaqueGenericType<T>",
        ] {
            assert!(in_crate.contains(&expected.to_owned()), "{in_crate:?}");
        }
        assert!(!in_crate.contains(&"Bool".to_owned()), "{in_crate:?}");

        let root = types(&namespaces[1]);
        assert!(root.contains(&"Bool".to_owned()), "{root:?}");
        assert!(!root.contains(&"PlainStructType".to_owned()), "{root:?}");
    }

    #[test]
    fn the_pages_of_types_are_next_to_the_pages_of_their_namespace() {
        fn paths(chapter: &Chapter, out: &mut Vec<PathBuf>) {
            out.extend(chapter.path.clone());
            for item in &chapter.sub_items {
                if let BookItem::Chapter(chapter) = item {
                    paths(chapter, out);
                }
            }
        }

        let mut ladfile = setup_ladfile();
        // types can have the name of a namespace, i.e. `Time` and `time`
        let plain = LadTypeId::new_string_id("ladfile_builder::test::PlainStructType".into());
        let mut definition = ladfile.types[&plain].clone();
        definition.identifier = "ladfile_builder".into();
        ladfile
            .types
            .insert(LadTypeId::new_string_id("same_name".into()), definition);

        let chapter = Section::new(PathBuf::from("lad"), &ladfile, SectionData::TypeSummary)
            .into_chapter(None, 0);
        let mut paths_found = Vec::new();
        paths(&chapter, &mut paths_found);

        let path = |path: &str| PathBuf::from(path);
        assert_eq!(chapter.path, Some(path("lad/types.md")));
        for expected in [
            // namespaces below the types page
            "lad/types/namespace_ladfile_builder.md",
            "lad/types/namespace_root.md",
            // types below the namespaces, in the same directory so they link to each other in the same way
            "lad/types/plainstructtype.md",
            "lad/types/opaquegenerictypet.md",
            "lad/types/ladfile_builder.md",
            "lad/types/bool.md",
            // and the functions below the types
            "lad/types/plainstructtype/plain_struct_function.md",
        ] {
            assert!(
                paths_found.contains(&path(expected)),
                "{expected} is not in {paths_found:?}"
            );
        }

        // no two pages are in the same place
        let unique = paths_found.iter().collect::<std::collections::HashSet<_>>();
        assert_eq!(unique.len(), paths_found.len(), "{paths_found:?}");
    }

    #[test]
    fn the_pages_list_the_namespaces_and_their_types() {
        let ladfile = setup_ladfile();

        let mut builder = MarkdownBuilder::new();
        Section::new(PathBuf::new(), &ladfile, SectionData::TypeSummary).to_markdown(&mut builder);
        let types = builder.build();
        assert!(
            types.contains("(./types/namespace_ladfile_builder.md)"),
            "{types}"
        );
        assert!(types.contains("(./types/namespace_root.md)"), "{types}");

        let namespaces = namespace_pages(&ladfile);
        let mut builder = MarkdownBuilder::new();
        namespaces[0].to_markdown(&mut builder);
        let namespace = builder.build();
        assert!(
            namespace.contains("`import_namespace(\"ladfile_builder\")`"),
            "{namespace}"
        );
        // the pages of the types are in the same directory
        assert!(
            namespace.contains("(././plainstructtype.md)"),
            "{namespace}"
        );
        assert!(
            namespace.contains("(././opaquegenerictypet.md)"),
            "{namespace}"
        );
    }

    /// The example file, with a function with the same name on two variants of an enum, i.e. constructors
    fn ladfile_with_variant_functions() -> LadFile {
        let mut ladfile = setup_ladfile();
        let enum_id = LadTypeId::new_string_id("ladfile_builder::test::EnumType".into());
        let template = ladfile
            .functions
            .values()
            .find(|function| matches!(function.namespace, LadFunctionNamespace::Global))
            .unwrap()
            .clone();

        for variant in ["Unit", "Struct"] {
            let mut function = template.clone();
            function.identifier = "new".into();
            function.namespace = LadFunctionNamespace::Variant {
                type_id: enum_id.clone().into(),
                variant: variant.into(),
            };
            ladfile.functions.insert(
                ladfile::LadFunctionId::new_string_id(format!("EnumType.{variant}.new")),
                function,
            );
        }
        ladfile
    }

    #[test]
    fn functions_on_variants_are_not_global_functions() {
        let ladfile = ladfile_with_variant_functions();
        let functions = Section::new(PathBuf::new(), &ladfile, SectionData::FunctionSummary);

        let titles = functions
            .children()
            .iter()
            .map(|page| page.title())
            .collect::<Vec<_>>();
        assert!(!titles.contains(&"new".to_owned()), "{titles:?}");
        assert!(titles.contains(&"hello_world".to_owned()), "{titles:?}");

        let items = functions.section_items();
        let Some(SectionItem::FunctionsSummary { functions, .. }) = items.last() else {
            panic!("expected a functions summary, got {items:?}");
        };
        assert_eq!(functions.len(), titles.len());
        assert!(functions.iter().all(|(variant, _)| variant.is_none()));
    }

    #[test]
    fn functions_on_variants_are_documented_with_their_enum() {
        let ladfile = ladfile_with_variant_functions();
        let types = type_pages(&ladfile);
        let enum_page = types
            .iter()
            .find(|page| page.title() == "EnumType")
            .expect("the enum has a page");

        // each gets a page of its own, with a name that is not shared with the same function on other variants
        let pages = enum_page.children();
        let titles = pages.iter().map(|page| page.title()).collect::<Vec<_>>();
        assert_eq!(titles, ["Unit.new", "Struct.new"]);
        let file_names = pages
            .iter()
            .map(|page| page.file_name())
            .collect::<Vec<_>>();
        assert_eq!(file_names, ["unitnew.md", "structnew.md"]);

        // and are linked from the page of the enum
        let mut builder = MarkdownBuilder::new();
        enum_page.to_markdown(&mut builder);
        let markdown = builder.build();
        assert!(
            markdown.contains(r"[Unit\.new](./enumtype/unitnew.md)"),
            "{markdown}"
        );
        assert!(
            markdown.contains(r"[Struct\.new](./enumtype/structnew.md)"),
            "{markdown}"
        );
    }

    #[test]
    fn generic_type_pages_describe_the_type_and_what_it_is_used_with() {
        let mut ladfile = setup_ladfile();
        // a function of every instantiation, and one of the instantiation with `usize`
        let template = ladfile.functions.values().next().unwrap().clone();
        let (generic_type_id, generic_type) = ladfile.generic_types.iter_mut().next().unwrap();
        for (name, instantiation) in [("value", None), ("only_on_usize", Some(0))] {
            let id = ladfile::LadFunctionId::new_string_id(name.into());
            let mut function = template.clone();
            function.identifier = name.into();
            match instantiation {
                None => generic_type.associated_functions.push(id.clone()),
                Some(i) => generic_type.instantiations[i]
                    .associated_functions
                    .push(id.clone()),
            }
            ladfile.functions.insert(id, function);
        }
        let generic_type_id = generic_type_id.clone();
        let (generic_type_id, generic_type) = ladfile
            .generic_types
            .get_key_value(&generic_type_id)
            .unwrap();
        let page = Section::new(
            PathBuf::new(),
            &ladfile,
            SectionData::GenericTypeDetail {
                generic_type_id,
                generic_type,
            },
        );
        assert_eq!(page.file_name(), "opaquegenerictypet.md");

        let mut builder = MarkdownBuilder::new();
        page.to_markdown(&mut builder);
        let markdown = builder.build();

        assert!(markdown.contains("# `OpaqueGenericType<T>`"), "{markdown}");
        // its layout, in terms of its parameter
        assert!(markdown.contains("- **value**:T"), "{markdown}");
        assert!(!markdown.contains("Opaque Type"), "{markdown}");
        assert!(
            markdown.contains("> I am the documentation of a generic type."),
            "{markdown}"
        );
        // the arguments of the instantiations, linked to their pages
        assert!(markdown.contains("- [Usize](./usize.md)"), "{markdown}");

        // its functions have pages below it, the ones of an instantiation titled by it
        let titles: Vec<_> = page.children().iter().map(|child| child.title()).collect();
        assert_eq!(
            titles,
            ["value", "OpaqueGenericType<Usize>.only_on_usize"],
            "{markdown}"
        );
        assert!(
            markdown.contains("](./opaquegenerictypet/opaquegenerictypeusizeonly_on_usize.md)"),
            "{markdown}"
        );
    }
}
