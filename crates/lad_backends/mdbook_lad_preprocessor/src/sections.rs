use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

use ladfile::{
    LadFile, LadFunction, LadFunctionNamespace, LadGenericType, LadInstance, LadInstantiation,
    LadNamedType, LadTypeDefinition, LadTypeId, LadTypeKind, LadTypeLayout, LadVariant,
    LadVisitable,
};
use mdbook_preprocessor::book::{BookItem, Chapter};

use crate::{
    argument_visitor::MarkdownArgumentVisitor,
    docstrings::{DocLinks, render_docs, summary},
    markdown::{IntoMarkdown, Markdown, MarkdownBuilder},
    markdown_vec,
};

/// The longest summary of a function or type in a table, see [`summary`]
const SUMMARY_LENGTH: usize = 120;

/// The title of the page of a type, i.e. `Transform` or `Option<Vec3>`
pub(crate) fn type_page_title(ladfile: &LadFile, lad_type_id: &LadTypeId) -> String {
    print_type(ladfile, lad_type_id, None, None)
}

/// A type as it is printed in the body of a page: linked to its page in `types_directory`, with unknown types named
/// as such, and escaped so generics aren't read as HTML tags.
fn print_linked_type(
    ladfile: &LadFile,
    type_: &impl LadVisitable,
    types_directory: &str,
) -> String {
    print_type(
        ladfile,
        type_,
        Some("Unknown"),
        Some((PathBuf::from(types_directory), true)),
    )
}

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
/// The unit type `()` is spelled out, as nothing of it would be left.
pub fn linkify_filename(name: impl Into<String>) -> String {
    name.into()
        .to_lowercase()
        .replace("()", "unit")
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
    pub(crate) fn into_chapter(self, parent: Option<&Chapter>) -> Chapter {
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

        // generated pages are unnumbered, numbers like `19.1.29.0.37.` only crowd the sidebar
        let mut chapter = Chapter {
            name: self.display_title(),
            content: builder.build(),
            parent_names: vec![parent.name.clone()],
            path: Some(parent_path.join(self.file_name())),
            source_path: Some(parent_source_path.join(self.file_name())),
            number: None,
            sub_items: vec![],
        };

        chapter.sub_items = self
            .children()
            .into_iter()
            .map(|c| c.into_chapter(Some(&chapter)))
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
                type_page_title(self.ladfile, lad_type_id)
            }
            SectionData::GenericTypeDetail { generic_type, .. } => generic_type.name_with_params(),
        }
    }

    /// The title as it is shown, which names types that aren't in the file `Unknown` rather than by their id, see [`Self::title`] for the one pages are named by
    pub(crate) fn display_title(&self) -> String {
        match &self.data {
            SectionData::TypeDetail { lad_type_id, .. } => {
                print_type(self.ladfile, *lad_type_id, Some("Unknown"), None)
            }
            _ => self.title(),
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

    /// The functions of the instantiations of a generic type, titled by the instantiation, i.e. `Time<Virtual>.pause`
    fn generic_type_functions(
        &self,
        generic_type_id: &LadTypeId,
        generic_type: &'a LadGenericType,
    ) -> Vec<TitledFunction<'a>> {
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
            })
            .collect()
    }

    pub(crate) fn is_code_heading(&self) -> bool {
        matches!(
            self.data,
            SectionData::TypeDetail { .. } | SectionData::GenericTypeDetail { .. }
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

    /// Whether the namespace has no types, functions or values, so no page
    fn namespace_is_empty(&self, namespace: &str) -> bool {
        self.types_in_namespace(namespace).is_empty()
            && self
                .ladfile
                .namespaces
                .get(namespace)
                .is_none_or(|lad_namespace| {
                    lad_namespace.functions.is_empty() && lad_namespace.values.is_empty()
                })
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
                .filter(|namespace| !self.namespace_is_empty(namespace))
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
                .collect(),
            // functions are documented on the page they are listed on
            SectionData::FunctionSummary
            | SectionData::TypeDetail { .. }
            | SectionData::GenericTypeDetail { .. } => vec![],
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

    /// The page listing the namespaces, with what each contains
    fn type_summary_items(&self) -> Vec<SectionItem<'a>> {
        // the namespace pages are in the directory of the type pages, which is next to this page
        let namespaces = self
            .namespaces()
            .into_iter()
            .filter_map(|namespace| {
                let page = Section::new(
                    PathBuf::new(),
                    self.ladfile,
                    SectionData::Namespace { namespace },
                );
                let count = |n: usize, what: &str| match n {
                    0 => None,
                    1 => Some(format!("1 {what}")),
                    n => Some(format!("{n} {what}s")),
                };
                let lad_namespace = self.ladfile.namespaces.get(namespace);
                let contents = [
                    count(self.types_in_namespace(namespace).len(), "type"),
                    count(self.namespace_functions(namespace).len(), "function"),
                    count(lad_namespace.map_or(0, |n| n.values.len()), "value"),
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
                // namespaces with nothing in them have no page
                (!self.namespace_is_empty(namespace))
                    .then(|| (page.title(), page.file_name(), contents.join(", ")))
            })
            .collect::<Vec<_>>();
        vec![SectionItem::Markdown {
            markdown: Box::new(move |builder| {
                builder.heading(2, "Namespaces");
                builder.raw("Types are grouped by the namespace scripts import them from, i.e. `import_namespace(\"math\")`.");
                builder.append("\n\n");
                builder.table(|builder| {
                    builder.headers(vec!["Namespace", "Contents"]);
                    for (title, file_name, contents) in namespaces.iter() {
                        builder.row(markdown_vec![
                            Markdown::Link {
                                text: Box::new(Markdown::new_paragraph(title.clone()).code()),
                                url: format!("./types/{file_name}"),
                                anchor: false
                            },
                            Markdown::new_paragraph(contents.clone()),
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
        items.push(SectionItem::Functions {
            functions: self.namespace_functions(namespace),
            ladfile: self.ladfile,
            types_directory: ".".to_owned(),
            by_instantiation: false,
        });
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
            SectionItem::Functions {
                functions: self
                    .global_functions()
                    .into_iter()
                    .map(|function| (None, function))
                    .collect(),
                ladfile: self.ladfile,
                // the type pages are below the page of the types, which is next to this one
                types_directory: "./types".to_owned(),
                by_instantiation: false,
            },
        ]
    }

    /// The page of a type: what it is, its description, fields and functions
    fn type_detail_items(
        &self,
        lad_type_id: &LadTypeId,
        lad_type: &'a LadTypeDefinition,
    ) -> Vec<SectionItem<'a>> {
        let metadata = &lad_type.metadata;
        let badges = [
            (metadata.is_component, "Component"),
            (metadata.is_resource, "Resource"),
            (metadata.is_bundle, "Bundle"),
            (metadata.is_asset, "Asset"),
        ]
        .into_iter()
        .filter_map(|(is, badge)| is.then_some(badge))
        .collect();
        vec![
            SectionItem::Badges { badges },
            SectionItem::Description {
                documentation: lad_type.documentation.as_deref(),
                ladfile: self.ladfile,
                types_directory: ".".to_owned(),
            },
            SectionItem::Layout {
                layout: &lad_type.layout,
                ladfile: self.ladfile,
                types_directory: PathBuf::from("./"),
            },
            SectionItem::Functions {
                functions: self.type_functions(lad_type_id, lad_type),
                ladfile: self.ladfile,
                types_directory: ".".to_owned(),
                by_instantiation: false,
            },
        ]
    }

    /// The page of a generic type, with its layout in terms of its parameters, the arguments it is instantiated with and its functions
    fn generic_type_detail_items(
        &self,
        generic_type_id: &'a LadTypeId,
        generic_type: &'a LadGenericType,
    ) -> Vec<SectionItem<'a>> {
        vec![
            SectionItem::Badges {
                badges: vec!["Generic"],
            },
            SectionItem::Description {
                documentation: generic_type.documentation.as_deref(),
                ladfile: self.ladfile,
                types_directory: ".".to_owned(),
            },
            SectionItem::Layout {
                layout: &generic_type.layout,
                ladfile: self.ladfile,
                types_directory: PathBuf::from("./"),
            },
            SectionItem::Instantiations {
                generic_type_id,
                instantiations: generic_type.instantiations.as_slice(),
                ladfile: self.ladfile,
                types_directory: PathBuf::from("./"),
            },
            SectionItem::Functions {
                functions: self.generic_type_functions(generic_type_id, generic_type),
                ladfile: self.ladfile,
                types_directory: ".".to_owned(),
                by_instantiation: true,
            },
        ]
    }
}

impl IntoMarkdown for Section<'_> {
    fn to_markdown(&self, builder: &mut MarkdownBuilder) {
        if self.is_code_heading() {
            builder.heading(1, Markdown::new_paragraph(self.display_title()).code());
        } else {
            builder.heading(1, self.display_title());
        }
        // marks the page as generated, so the book's `lad.css` styles what follows it
        builder.block(r#"<div class="lad-page"></div>"#);

        for item in self.section_items() {
            item.to_markdown(builder);
        }
    }
}

/// Items which combine markdown elements to build a section
pub enum SectionItem<'a> {
    Markdown {
        markdown: Box<dyn Fn(&mut MarkdownBuilder) + 'static>,
    },
    /// What kind of type the page is about, i.e. a component
    Badges { badges: Vec<&'static str> },
    Layout {
        layout: &'a LadTypeLayout,
        ladfile: &'a LadFile,
        types_directory: PathBuf,
    },
    Description {
        documentation: Option<&'a str>,
        ladfile: &'a LadFile,
        types_directory: String,
    },
    /// The instantiations of a generic type
    Instantiations {
        generic_type_id: &'a LadTypeId,
        instantiations: &'a [LadInstantiation],
        ladfile: &'a LadFile,
        types_directory: PathBuf,
    },
    /// The functions of a page, grouped by kind, each with a summary and a section of its own
    Functions {
        /// The functions, with the variant of an enum or the instantiation of a generic type they are on if any
        functions: Vec<TitledFunction<'a>>,
        ladfile: &'a LadFile,
        types_directory: String,
        /// Whether the prefixes are instantiations, whose functions of the same name are documented together
        by_instantiation: bool,
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
            SectionItem::Badges { .. } => "Badges",
            SectionItem::Layout { .. } => "Layout",
            SectionItem::Description { .. } => "Description",
            SectionItem::Instantiations { .. } => "Instantiations",
            SectionItem::Functions { .. } => "Functions",
            SectionItem::TypesSummary { .. } => "TypesSummary",
            SectionItem::InstancesSummary { .. } => "InstancesSummary",
        })
    }
}

impl IntoMarkdown for SectionItem<'_> {
    fn to_markdown(&self, builder: &mut MarkdownBuilder) {
        match self {
            SectionItem::Markdown { markdown } => (markdown)(builder),
            SectionItem::Badges { badges } => {
                if !badges.is_empty() {
                    let badges = badges
                        .iter()
                        .map(|badge| format!(r#"<span class="lad-badge">{badge}</span>"#))
                        .collect::<String>();
                    builder.block(format!(r#"<p class="lad-badges">{badges}</p>"#));
                }
            }
            SectionItem::Layout {
                layout,
                ladfile,
                types_directory,
            } => layout_markdown(layout, ladfile, types_directory, builder),
            SectionItem::Description {
                documentation,
                ladfile,
                types_directory,
            } => {
                if let Some(documentation) = documentation {
                    let links = DocLinks::new(ladfile);
                    // the page's title is the only heading above the description
                    builder.block(render_docs(documentation, &links, types_directory, 2));
                }
            }
            SectionItem::Instantiations {
                generic_type_id,
                instantiations,
                ladfile,
                types_directory,
            } => instantiations_markdown(
                generic_type_id,
                instantiations,
                ladfile,
                types_directory,
                builder,
            ),
            SectionItem::Functions {
                functions,
                ladfile,
                types_directory,
                by_instantiation,
            } => functions_markdown(
                functions,
                ladfile,
                types_directory,
                *by_instantiation,
                builder,
            ),
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
        }
    }
}

/// The fields of a struct, the variants of an enum with their fields, or a note that the type is opaque
fn layout_markdown(
    layout: &LadTypeLayout,
    ladfile: &LadFile,
    types_directory: &Path,
    builder: &mut MarkdownBuilder,
) {
    let types_directory = types_directory.to_string_lossy();
    let types_directory = types_directory.trim_end_matches('/');
    let print = |kind: &LadTypeKind| print_linked_type(ladfile, kind, types_directory);
    match layout {
        LadTypeLayout::Opaque => {
            builder.block(
                r#"<p class="lad-note">Opaque: scripts can't access its fields, only its functions.</p>"#,
            );
        }
        LadTypeLayout::MonoVariant(LadVariant::Struct { fields, .. }) if !fields.is_empty() => {
            builder.heading(2, "Fields");
            builder.table(|builder| {
                builder.headers(vec!["Field", "Type"]);
                for field in fields {
                    builder.row(markdown_vec![
                        Markdown::new_paragraph(field.name.clone()).code(),
                        Markdown::Raw(print(&field.type_)),
                    ]);
                }
            });
        }
        LadTypeLayout::MonoVariant(LadVariant::TupleStruct { fields, .. })
            if !fields.is_empty() =>
        {
            builder.heading(2, "Fields");
            builder.table(|builder| {
                builder.headers(vec!["Index", "Type"]);
                for (index, field) in fields.iter().enumerate() {
                    builder.row(markdown_vec![
                        Markdown::new_paragraph(index.to_string()).code(),
                        Markdown::Raw(print(&field.type_)),
                    ]);
                }
            });
        }
        // unit structs
        LadTypeLayout::MonoVariant(_) => {}
        // enums of unit variants only, i.e. `TimerMode`, have no fields to show
        LadTypeLayout::Enum(variants)
            if variants
                .iter()
                .all(|variant| matches!(variant, LadVariant::Unit { .. })) =>
        {
            builder.heading(2, "Variants");
            builder.table(|builder| {
                builder.headers(vec!["Variant"]);
                for variant in variants {
                    if let LadVariant::Unit { name } = variant {
                        builder.row(markdown_vec![
                            Markdown::new_paragraph(name.to_string()).code()
                        ]);
                    }
                }
            });
        }
        LadTypeLayout::Enum(variants) => {
            builder.heading(2, "Variants");
            builder.table(|builder| {
                builder.headers(vec!["Variant", "Fields"]);
                for variant in variants {
                    let (name, fields) = match variant {
                        LadVariant::Unit { name } => (name, String::new()),
                        LadVariant::TupleStruct { name, fields } => (
                            name,
                            format!(
                                "({})",
                                fields
                                    .iter()
                                    .map(|field| print(&field.type_))
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            ),
                        ),
                        LadVariant::Struct { name, fields } => (
                            name,
                            format!(
                                "{{ {} }}",
                                fields
                                    .iter()
                                    .map(|field| format!(
                                        "{}: {}",
                                        escape_name(&field.name),
                                        print(&field.type_)
                                    ))
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            ),
                        ),
                    };
                    builder.row(markdown_vec![
                        Markdown::new_paragraph(name.to_string()).code(),
                        Markdown::Raw(fields),
                    ]);
                }
            });
        }
    }
}

/// The instantiations a generic type is used as, i.e. `Handle<Image>`
fn instantiations_markdown(
    generic_type_id: &LadTypeId,
    instantiations: &[LadInstantiation],
    ladfile: &LadFile,
    types_directory: &Path,
    builder: &mut MarkdownBuilder,
) {
    if instantiations.is_empty() {
        return;
    }
    let types_directory = types_directory.to_string_lossy();
    let used_as = instantiations
        .iter()
        .map(|instantiation| {
            print_linked_type(
                ladfile,
                &LadTypeKind::Val(LadNamedType::Generic {
                    generic_type: generic_type_id.clone(),
                    args: instantiation.args.clone(),
                }),
                types_directory.trim_end_matches('/'),
            )
        })
        .collect::<Vec<_>>();
    builder.heading(2, "Instantiations");
    builder.block(used_as.join(", "));
}

/// Escapes a name for markdown, so i.e. `_self` isn't read as emphasis
fn escape_name(name: &str) -> String {
    name.replace('_', "\\_").replace('*', "\\*")
}

/// What a function is, which the functions of a page are grouped by, in order
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum FunctionKind {
    Static,
    Method,
    Operator,
    Variant,
}

impl FunctionKind {
    fn of(function: &LadFunction, on_variant: bool) -> Self {
        if on_variant {
            FunctionKind::Variant
        } else if function.metadata.operator.is_some() {
            FunctionKind::Operator
        } else if function.receiver {
            FunctionKind::Method
        } else {
            FunctionKind::Static
        }
    }

    fn heading(self) -> &'static str {
        match self {
            FunctionKind::Static => "Functions",
            FunctionKind::Method => "Methods",
            FunctionKind::Operator => "Operators",
            FunctionKind::Variant => "Variant Functions",
        }
    }
}

/// Functions documented together: one function, or the functions of the same name on each instantiation of a generic type
struct FunctionEntry<'a> {
    kind: FunctionKind,
    title: String,
    functions: Vec<&'a LadFunction>,
    /// The instantiations the functions are on, for generic types
    on: Vec<String>,
}

impl FunctionEntry<'_> {
    /// The id of the section of the entry, see [`linkify_filename`]
    fn anchor(&self) -> String {
        linkify_filename(&self.title)
    }
}

/// The functions of a page, grouped by kind and sorted by name, each group with a summary table linking to a section per function
fn functions_markdown(
    functions: &[TitledFunction<'_>],
    ladfile: &LadFile,
    types_directory: &str,
    by_instantiation: bool,
    builder: &mut MarkdownBuilder,
) {
    let mut entries: Vec<FunctionEntry<'_>> = Vec::new();
    for (prefix, function) in functions {
        let on_variant = prefix.is_some() && !by_instantiation;
        let kind = FunctionKind::of(function, on_variant);
        let title = match on_variant {
            true => function_title(prefix.as_deref(), function),
            false => function_title(None, function),
        };
        let index = match entries
            .iter()
            .position(|entry| entry.kind == kind && entry.title == title)
        {
            Some(index) => index,
            None => {
                entries.push(FunctionEntry {
                    kind,
                    title,
                    functions: vec![],
                    on: vec![],
                });
                entries.len() - 1
            }
        };
        let entry = &mut entries[index];
        entry.functions.push(function);
        if by_instantiation && let Some(prefix) = prefix {
            entry.on.push(prefix.to_string());
        }
    }
    entries.sort_by(|a, b| (a.kind, &a.title).cmp(&(b.kind, &b.title)));

    let links = DocLinks::new(ladfile);
    let mut rest = entries.as_slice();
    while let Some(first) = rest.first() {
        let kind = first.kind;
        let len = rest.iter().take_while(|entry| entry.kind == kind).count();
        let (group, next) = rest.split_at(len);
        rest = next;
        builder.heading(2, kind.heading());
        builder.table(|builder| {
            builder.headers(vec!["Name", "Summary"]);
            for entry in group {
                let documentation = entry.functions[0].documentation.as_deref();
                builder.row(markdown_vec![
                    Markdown::Link {
                        text: Box::new(Markdown::new_paragraph(entry.title.clone()).code()),
                        url: format!("#{}", entry.anchor()),
                        anchor: false
                    },
                    Markdown::Raw(
                        documentation.map_or_else(String::new, |doc| summary(doc, SUMMARY_LENGTH))
                    ),
                ]);
            }
        });
        for entry in group {
            function_markdown(entry, ladfile, &links, types_directory, builder);
        }
    }
}

/// The section of a function: its signature, documentation, and the documentation of its arguments
fn function_markdown(
    entry: &FunctionEntry<'_>,
    ladfile: &LadFile,
    links: &DocLinks,
    types_directory: &str,
    builder: &mut MarkdownBuilder,
) {
    let function = entry.functions[0];
    builder.block(format!(
        "### `{}` {{#{} .lad-function}}",
        entry.title,
        entry.anchor()
    ));
    if !entry.on.is_empty() {
        let on = entry
            .on
            .iter()
            .map(|on| {
                format!(
                    "<code>{}</code>",
                    on.replace('<', "&lt;").replace('>', "&gt;")
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        builder.block(format!(r#"<p class="lad-note">On {on}</p>"#));
    }
    builder.block(format!(
        "<div class=\"lad-signature\">\n\n{}\n\n</div>",
        signature(function, ladfile, types_directory)
    ));
    // what each parameter of a generic function stands for, i.e. the component type it is given
    for param in &function.generics {
        let given_as = function
            .arguments
            .iter()
            .filter(|argument| {
                matches!(&argument.kind, LadTypeKind::Type { of: LadNamedType::Param { param: of }, .. } if of == param)
            })
            .filter_map(|argument| argument.name.as_deref())
            .map(|name| format!("`{name}`"))
            .collect::<Vec<_>>();
        builder.block(match given_as.is_empty() {
            true => format!(r#"<p class="lad-note">Generic over <code>{param}</code>.</p>"#),
            false => format!(
                "<p class=\"lad-note\">Generic over <code>{param}</code>, the type given as {}.</p>",
                given_as
                    .iter()
                    .map(|name| format!("<code>{}</code>", name.trim_matches('`')))
                    .collect::<Vec<_>>()
                    .join(" or ")
            ),
        });
    }
    if let Some(documentation) = &function.documentation {
        // below the page's title, the group's heading and the function's
        builder.block(render_docs(documentation, links, types_directory, 3));
    }

    // only arguments which say something more than their type
    let arguments = function
        .arguments
        .iter()
        .filter_map(|argument| {
            let documentation = argument.documentation.as_deref()?;
            let name = argument.name.as_deref().unwrap_or_default();
            Some(format!(
                "- `{}`: {}",
                display_argument_name(name),
                one_line(&render_docs(documentation, links, types_directory, 3))
            ))
        })
        .collect::<Vec<_>>();
    if !arguments.is_empty() {
        builder.block(format!("**Arguments**\n\n{}", arguments.join("\n")));
    }
    if let Some(documentation) = &function.return_type.documentation {
        builder.block(format!(
            "**Returns**: {}",
            one_line(&render_docs(documentation, links, types_directory, 3))
        ));
    }
}

/// Joins the lines of a short piece of markdown, so it stays in a list item
fn one_line(markdown: &str) -> String {
    markdown.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The name of an argument as written in signatures, where the receiver is `self`
fn display_argument_name(name: &str) -> &str {
    match name {
        "_self" | "self_" => "self",
        name => name,
    }
}

/// The signature of a function, language neutral: `name<A>(arg: Type, ...) -> Return`, with the types linked to their pages
fn signature(function: &LadFunction, ladfile: &LadFile, types_directory: &str) -> String {
    let generics = match function.generics.is_empty() {
        true => String::new(),
        false => format!("\\<{}\\>", function.generics.join(", ")),
    };
    let arguments = function
        .arguments
        .iter()
        .enumerate()
        .map(|(index, argument)| {
            let name = argument
                .name
                .as_deref()
                .map(|name| display_argument_name(name).to_owned())
                .unwrap_or_else(|| format!("arg{}", index + 1));
            format!(
                "{}: {}",
                escape_name(&name),
                print_linked_type(ladfile, &argument.kind, types_directory)
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    let returns = print_linked_type(ladfile, &function.return_type.kind, types_directory);
    let returns = match returns.as_str() {
        "()" => String::new(),
        returns => format!(" -> {returns}"),
    };
    format!(
        "**{}**{generics}({arguments}){returns}",
        escape_name(&function.identifier)
    )
}

/// A table of types as a quick reference, sorted by name, linking to the type pages
fn types_summary_markdown(
    types: &[&LadTypeId],
    types_directory: &str,
    ladfile: &LadFile,
    builder: &mut MarkdownBuilder,
) {
    if types.is_empty() {
        return;
    }
    builder.heading(2, "Types");

    let mut rows = types
        .iter()
        .map(|type_| {
            let printed_type_for_url = print_type(ladfile, *type_, None, None);
            let printed_type_pretty = print_type(ladfile, *type_, Some("Unknown"), None);
            let summary = ladfile
                .get_type_documentation(type_)
                .map(|doc| summary(doc, SUMMARY_LENGTH))
                .unwrap_or_default();
            (printed_type_pretty, printed_type_for_url, summary)
        })
        .collect::<Vec<_>>();
    rows.sort_by_key(|row| row.0.to_lowercase());

    builder.table(|builder| {
        builder.headers(vec!["Type", "Summary"]);
        for (pretty, for_url, summary) in rows {
            let mut link_builder = MarkdownBuilder::new();
            link_builder.tight_inline();
            link_builder.link(
                Markdown::new_paragraph(pretty).code(),
                format!("./{types_directory}/{}.md", linkify_filename(for_url)),
            );
            builder.row(markdown_vec![link_builder, Markdown::Raw(summary)]);
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
            .into_chapter(None);
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
        let builtins = Section::new(PathBuf::new(), &ladfile, SectionData::FunctionSummary);

        // the builtins are documented on their page, which has no pages below it
        assert!(builtins.children().is_empty());
        let items = builtins.section_items();
        let Some(SectionItem::Functions { functions, .. }) = items.last() else {
            panic!("expected the functions, got {items:?}");
        };
        let names = functions
            .iter()
            .map(|(_, function)| function.identifier.as_ref())
            .collect::<Vec<_>>();
        assert!(!names.contains(&"new"), "{names:?}");
        assert!(names.contains(&"hello_world"), "{names:?}");
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
        assert!(enum_page.children().is_empty());

        // each has a section on the page of the enum, with an id not shared with the same function on other variants,
        // linked from the summary of the variant functions
        let mut builder = MarkdownBuilder::new();
        enum_page.to_markdown(&mut builder);
        let markdown = builder.build();
        assert!(markdown.contains("## Variant Functions"), "{markdown}");
        for (title, anchor) in [("Unit.new", "unitnew"), ("Struct.new", "structnew")] {
            assert!(
                markdown.contains(&format!("### `{title}` {{#{anchor} .lad-function}}")),
                "{markdown}"
            );
            assert!(
                markdown.contains(&format!("[`{title}`](#{anchor})")),
                "{markdown}"
            );
        }
    }

    #[test]
    fn generic_type_pages_describe_the_type_and_what_it_is_used_with() {
        let mut ladfile = setup_ladfile();
        // a function of the instantiation with `usize`
        let mut function = ladfile.functions.values().next().unwrap().clone();
        function.identifier = "only_on_usize".into();
        let (generic_type_id, generic_type) = ladfile.generic_types.iter_mut().next().unwrap();
        let id = ladfile::LadFunctionId::new_string_id("only_on_usize".into());
        generic_type.instantiations[0]
            .associated_functions
            .push(id.clone());
        ladfile.functions.insert(id, function);
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
        assert!(markdown.contains("| `value` | T |"), "{markdown}");
        assert!(!markdown.contains("Opaque:"), "{markdown}");
        assert!(
            markdown.contains("I am the documentation of a generic type."),
            "{markdown}"
        );
        // its instantiations, linked to the pages of their arguments
        assert!(markdown.contains("## Instantiations"), "{markdown}");
        assert!(markdown.contains("(./usize.md)"), "{markdown}");

        // the functions of its instantiations are documented on its page, with the instantiations they are on
        assert!(page.children().is_empty());
        assert!(
            markdown.contains("### `only_on_usize` {#only_on_usize .lad-function}"),
            "{markdown}"
        );
        assert!(
            markdown.contains("On <code>OpaqueGenericType&lt;"),
            "{markdown}"
        );
    }
}
