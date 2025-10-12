use std::borrow::Cow;

/// Takes the first n characters from the markdown, without splitting any formatting.
pub(crate) fn markdown_substring(markdown: &str, length: usize) -> String {
    if markdown.len() <= length {
        return markdown.to_string();
    }
    let mut end = length;
    for &(open, close) in &[("`", "`"), ("**", "**"), ("*", "*"), ("_", "_"), ("[", "]")] {
        // Count markers in the already cut substring.
        let count = markdown[..end].matches(open).count();
        // Check if an opening marker starts right at the cutoff.
        let extra = if markdown[end..].starts_with(open) {
            1
        } else {
            0
        };
        if (count + extra) % 2 == 1 {
            let search_start = if extra == 1 { end + open.len() } else { end };
            if let Some(pos) = markdown[search_start..].find(close) {
                end = search_start + pos + close.len();
                // Special handling for links: if the marker is "[" then check if a '(' follows.
                if open == "[" && markdown.len() > end && markdown[end..].starts_with('(') {
                    let paren_search_start = end + 1;
                    if let Some(paren_pos) = markdown[paren_search_start..].find(')') {
                        end = paren_search_start + paren_pos + 1;
                    }
                }
            } else {
                return markdown.to_string();
            }
        }
    }

    let trimmed = markdown[..end].to_string();
    // append ...
    format!("{trimmed}...")
}

/// Escapes Markdown reserved characters in the given text.
fn escape_markdown(text: &str, escape: bool) -> String {
    if !escape {
        return text.to_string();
    }

    // Characters that should be escaped in markdown
    let escape_chars = r"\`*_{}[]()#+-.!";
    let mut escaped = String::with_capacity(text.len());
    for c in text.chars() {
        if escape_chars.contains(c) {
            escaped.push('\\');
        }
        escaped.push(c);
    }
    escaped
}

/// Trait for converting elements into markdown strings.
pub trait IntoMarkdown: std::fmt::Debug {
    fn to_markdown(&self, builder: &mut MarkdownBuilder);
}

/// Comprehensive enum representing various Markdown constructs.
#[allow(dead_code)]
#[derive(Debug)]
pub enum Markdown {
    Heading {
        level: u8,
        content: Box<dyn IntoMarkdown>,
    },
    Paragraph {
        text: String,
        bold: bool,
        italic: bool,
        code: bool,
    },
    CodeBlock {
        language: Option<String>,
        code: String,
    },
    List {
        ordered: bool,
        items: Vec<Box<dyn IntoMarkdown>>,
    },
    Quote(String),
    Image {
        alt: String,
        src: String,
    },
    Link {
        text: Box<dyn IntoMarkdown>,
        url: String,
        anchor: bool,
    },
    HorizontalRule,
    Table {
        headers: Vec<String>,
        rows: Vec<Vec<Box<dyn IntoMarkdown>>>,
    },
    Raw(String),
}

#[allow(dead_code)]
impl Markdown {
    pub fn new_paragraph(text: impl Into<String>) -> Self {
        Markdown::Paragraph {
            text: text.into(),
            bold: false,
            italic: false,
            code: false,
        }
    }

    pub fn space() -> Self {
        Markdown::Paragraph {
            text: " ".to_owned(),
            bold: false,
            italic: false,
            code: false,
        }
    }

    pub fn bold(self) -> Self {
        match self {
            Markdown::Paragraph { text, .. } => Markdown::Paragraph {
                text,
                bold: true,
                italic: false,
                code: false,
            },
            _ => self,
        }
    }

    pub fn italic(self) -> Self {
        match self {
            Markdown::Paragraph { text, .. } => Markdown::Paragraph {
                text,
                bold: false,
                italic: true,
                code: false,
            },
            _ => self,
        }
    }

    pub fn code(self) -> Self {
        match self {
            Markdown::Paragraph { text, .. } => Markdown::Paragraph {
                text,
                bold: false,
                italic: false,
                code: true,
            },
            _ => self,
        }
    }
}

impl IntoMarkdown for Markdown {
    fn to_markdown(&self, builder: &mut MarkdownBuilder) {
        match self {
            Markdown::Heading { level, content } => {
                // Clamp the header level to Markdown's 1-6.
                let clamped_level = level.clamp(&1, &6);
                let hashes = "#".repeat(*clamped_level as usize);
                builder.append(&hashes);
                builder.append(" ");
                builder.with_tight_inline(|builder| {
                    content.to_markdown(builder);
                });
            }
            Markdown::Paragraph {
                text,
                bold,
                italic,
                code,
            } => {
                if *bold {
                    builder.append("**");
                }
                if *italic {
                    builder.append("_");
                }
                if *code {
                    builder.append("`");
                }

                let escaped = if *code {
                    // this might be a bug in the markdown renderer but we need to escape those for tables
                    text.clone()
                } else {
                    escape_markdown(text, builder.escape)
                };

                builder.append(&escaped);

                if *code {
                    builder.append("`");
                }
                if *italic {
                    builder.append("_");
                }
                if *bold {
                    builder.append("**");
                }
            }
            Markdown::CodeBlock { language, code } => {
                // Do not escape code blocks
                let lang = language.as_deref().unwrap_or("");
                builder.append(&format!("```{lang}\n{code}\n```"));
            }
            Markdown::List { ordered, items } => {
                items.iter().enumerate().for_each(|(i, item)| {
                    if *ordered {
                        builder.append(&(i + 1).to_string());
                        builder.append(". ");
                        builder.with_tight_inline(|builder| {
                            item.to_markdown(builder);
                        });
                    } else {
                        builder.append("- ");
                        builder.with_tight_inline(|builder| {
                            item.to_markdown(builder);
                        });
                    }

                    if i < items.len() - 1 {
                        builder.append("\n");
                    }
                });
            }
            Markdown::Quote(text) => {
                let quote_output = text
                    .lines()
                    .map(|line| format!("> {line}"))
                    .collect::<Vec<String>>()
                    .join("\n");
                builder.append(&quote_output);
            }
            Markdown::Image { alt, src } => {
                // Escape alt text while leaving src untouched.
                builder.append(&format!(
                    "![{}]({})",
                    escape_markdown(alt, builder.escape),
                    src
                ));
            }
            Markdown::Link { text, url, anchor } => {
                // anchors must be lowercase, only contain letters or dashes
                builder.append("[");
                builder.with_tight_inline(|builder| text.to_markdown(builder));
                builder.append("](");

                let url = if *anchor {
                    // prefix with #
                    format!(
                        "#{}",
                        url.to_lowercase()
                            .replace(" ", "-")
                            .replace(|c: char| !c.is_alphabetic(), "")
                    )
                } else {
                    url.clone().replace("\\", "/")
                };

                builder.append(&url);
                builder.append(")");
            }
            Markdown::HorizontalRule => {
                builder.append("---");
            }
            Markdown::Table { headers, rows } => {
                if rows.is_empty() {
                    return;
                }

                let header_line = format!("| {} |", headers.join(" | "));
                builder.append(&header_line);
                builder.append("\n");

                // Separator row:
                let separator_line = format!(
                    "|{}|\n",
                    headers
                        .iter()
                        .map(|_| " --- ")
                        .collect::<Vec<&str>>()
                        .join("|")
                );
                builder.append(&separator_line);

                for (row_idx, row) in rows.iter().enumerate() {
                    builder.append("| ");
                    for (i, cell) in row.iter().enumerate() {
                        builder.with_tight_inline(|builder| {
                            cell.to_markdown(builder);
                        });
                        if i < row.len() - 1 {
                            builder.append(" | ");
                        }
                    }
                    builder.append(" |");
                    if row_idx < rows.len() - 1 {
                        builder.append("\n");
                    }
                }
            }
            Markdown::Raw(text) => {
                builder.append(text);
            }
        }
        builder.separate();
    }
}

impl IntoMarkdown for &str {
    fn to_markdown(&self, builder: &mut MarkdownBuilder) {
        builder.append(&escape_markdown(self, builder.escape));
        builder.separate();
    }
}

impl IntoMarkdown for String {
    fn to_markdown(&self, builder: &mut MarkdownBuilder) {
        builder.append(&escape_markdown(self.as_ref(), builder.escape));
        builder.separate();
    }
}

impl IntoMarkdown for Cow<'_, str> {
    fn to_markdown(&self, builder: &mut MarkdownBuilder) {
        builder.append(&escape_markdown(self.as_ref(), builder.escape));
        builder.separate();
    }
}

impl IntoMarkdown for Box<dyn IntoMarkdown> {
    fn to_markdown(&self, builder: &mut MarkdownBuilder) {
        self.as_ref().to_markdown(builder)
    }
}

/// Usage: `markdown_vec![item1, item2, item3]`
/// Creates `Vec<dyn IntoMarkdown>`` from a list of items.
#[macro_export]
macro_rules! markdown_vec {
    ($($x:expr),*$(,)?) => {
        vec![$(
            Box::new($x) as Box<dyn IntoMarkdown>
        ),*]
    };
}

impl<T: IntoMarkdown> IntoMarkdown for Vec<T> {
    fn to_markdown(&self, builder: &mut MarkdownBuilder) {
        for item in self.iter() {
            item.to_markdown(builder);
        }
    }
}

/// Builder pattern for generating comprehensive Markdown documentation.
/// Now also doubles as the accumulator for the generated markdown.
#[derive(Clone, Debug)]
pub struct MarkdownBuilder {
    output: String,
    pub inline: bool,
    pub tight_inline: bool,
    pub escape: bool,
}

#[allow(dead_code)]
impl MarkdownBuilder {
    /// Clears the builder's buffer
    pub fn clear(&mut self) {
        self.output.clear();
    }

    pub fn with_tight_inline<F: FnOnce(&mut MarkdownBuilder)>(&mut self, f: F) {
        let prev_inline = self.inline;
        let prev_tight_inline = self.tight_inline;
        self.tight_inline();
        f(self);
        self.inline = prev_inline;
        self.tight_inline = prev_tight_inline;
    }

    /// Creates a new MarkdownBuilder.
    pub fn new() -> Self {
        MarkdownBuilder {
            output: String::new(),
            inline: false,
            tight_inline: false,
            escape: true,
        }
    }

    // inserts the correct separator
    // this should be used after each element is added
    pub fn separate(&mut self) {
        self.output.push_str(self.separator());
    }

    fn separator(&self) -> &'static str {
        if self.inline {
            if self.tight_inline { "" } else { " " }
        } else {
            "\n\n"
        }
    }

    /// Disables or enables the automatic escaping of Markdown reserved characters.
    /// by default it is enabled.
    ///
    /// Will only affect elements which are escaped by default such as text.
    pub fn set_escape_mode(&mut self, escape: bool) -> &mut Self {
        self.escape = escape;
        self
    }

    /// Enables inline mode, which prevents newlines from being inserted for elements that support it
    pub fn inline(&mut self) -> &mut Self {
        self.inline = true;
        self.tight_inline = false;
        self
    }

    /// Disables inline mode.
    pub fn non_inline(&mut self) -> &mut Self {
        self.inline = false;
        self.tight_inline = false;
        self
    }

    /// Enables inline mode on top of disabling the automatic space separator.
    /// Each element will simply be concatenated without any separator.
    pub fn tight_inline(&mut self) -> &mut Self {
        self.inline = true;
        self.tight_inline = true;
        self
    }

    /// Adds an in-place slot for more complex markdown generation while preserving the builder flow.
    pub fn complex(&mut self, f: impl FnOnce(&mut MarkdownBuilder)) -> &mut Self {
        f(self);
        self
    }

    /// Appends raw text to the output without processing it.
    pub fn append(&mut self, text: &str) {
        self.output.push_str(text);
    }

    /// Adds a heading element (Levels from 1-6).
    pub fn heading(&mut self, level: u8, text: impl IntoMarkdown + 'static) -> &mut Self {
        Markdown::Heading {
            level: level.min(6),
            content: Box::new(text),
        }
        .to_markdown(self);
        self
    }

    /// Adds a raw markdown element
    pub fn raw(&mut self, text: impl Into<String>) -> &mut Self {
        self.append(&text.into());
        self
    }

    /// Adds a paragraph element.
    pub fn text(&mut self, text: impl Into<String>) -> &mut Self {
        Markdown::Paragraph {
            text: text.into(),
            bold: false,
            italic: false,
            code: false,
        }
        .to_markdown(self);
        self
    }

    /// Adds a bold element.
    pub fn bold(&mut self, text: impl Into<String>) -> &mut Self {
        Markdown::Paragraph {
            text: text.into(),
            bold: true,
            italic: false,
            code: false,
        }
        .to_markdown(self);
        self
    }

    /// Adds an italic element.
    pub fn italic(&mut self, text: impl Into<String>) -> &mut Self {
        Markdown::Paragraph {
            text: text.into(),
            bold: false,
            italic: true,
            code: false,
        }
        .to_markdown(self);
        self
    }

    /// Adds a code block element.
    pub fn codeblock(
        &mut self,
        language: Option<impl Into<String>>,
        code: impl Into<String>,
    ) -> &mut Self {
        Markdown::CodeBlock {
            language: language.map(|l| l.into()),
            code: code.into(),
        }
        .to_markdown(self);
        self
    }

    /// Adds an inline code element.
    pub fn inline_code(&mut self, code: impl Into<String>) -> &mut Self {
        Markdown::Paragraph {
            text: code.into(),
            bold: false,
            italic: false,
            code: true,
        }
        .to_markdown(self);
        self
    }

    /// Adds a list element.
    pub fn list(&mut self, ordered: bool, items: Vec<impl IntoMarkdown + 'static>) -> &mut Self {
        Markdown::List {
            ordered,
            items: items
                .into_iter()
                .map(|i| Box::new(i) as Box<dyn IntoMarkdown>)
                .collect(),
        }
        .to_markdown(self);
        self
    }

    /// Adds a quote element.
    pub fn quote(&mut self, text: impl IntoMarkdown) -> &mut Self {
        let mut builder = MarkdownBuilder::new();
        builder.tight_inline();
        text.to_markdown(&mut builder);
        Markdown::Quote(builder.build()).to_markdown(self);
        self
    }

    /// Adds an image element.
    pub fn image(&mut self, alt: impl Into<String>, src: impl Into<String>) -> &mut Self {
        Markdown::Image {
            alt: alt.into(),
            src: src.into(),
        }
        .to_markdown(self);
        self
    }

    /// Adds a link element.
    pub fn link(&mut self, text: impl IntoMarkdown + 'static, url: impl Into<String>) -> &mut Self {
        Markdown::Link {
            text: Box::new(text),
            url: url.into(),
            anchor: false,
        }
        .to_markdown(self);
        self
    }

    pub fn section_link(
        &mut self,
        text: impl IntoMarkdown + 'static,
        url: impl Into<String>,
    ) -> &mut Self {
        Markdown::Link {
            text: Box::new(text),
            url: url.into(),
            anchor: true,
        }
        .to_markdown(self);
        self
    }

    /// Adds a horizontal rule element.
    pub fn horizontal_rule(&mut self) -> &mut Self {
        Markdown::HorizontalRule.to_markdown(self);
        self
    }

    /// Adds a table element via a mini builder.
    pub fn table(&mut self, f: impl FnOnce(&mut TableBuilder)) -> &mut Self {
        let mut builder = TableBuilder::new();
        f(&mut builder);
        log::info!("Table Builder: {builder:#?}");
        builder.build().to_markdown(self);
        self
    }

    /// Builds the markdown document as a single String by delegating the conversion
    /// of each element to its `into_markdown` implementation.
    pub fn build(&mut self) -> String {
        // replace inline placeholders with the characters they represent,
        // at the same time remove multiple consecutive placeholders
        self.output.clone()
    }
}

impl IntoMarkdown for MarkdownBuilder {
    fn to_markdown(&self, builder: &mut MarkdownBuilder) {
        builder.append(&self.output);
    }
}

/// Mini builder for constructing Markdown tables.
#[derive(Debug)]
pub struct TableBuilder {
    headers: Vec<String>,
    rows: Vec<Vec<Box<dyn IntoMarkdown>>>,
}

impl TableBuilder {
    /// Creates a new TableBuilder.
    pub fn new() -> Self {
        TableBuilder {
            headers: vec![],
            rows: vec![],
        }
    }

    /// Sets the headers for the table.
    pub fn headers(&mut self, headers: Vec<impl Into<String>>) -> &mut Self {
        self.headers = headers.into_iter().map(|h| h.into()).collect();
        self
    }

    /// Adds a row to the table.
    pub fn row(&mut self, row: Vec<impl IntoMarkdown + 'static>) -> &mut Self {
        self.rows.push(
            row.into_iter()
                .map(|r| Box::new(r) as Box<dyn IntoMarkdown>)
                .collect(),
        );
        self
    }

    /// Finalizes and builds the table as a Markdown variant.
    pub fn build(self) -> Markdown {
        Markdown::Table {
            headers: self.headers,
            rows: self.rows,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markdown_builder() {
        let mut builder = MarkdownBuilder::new();
        let markdown = builder
            .heading(1, "Documentation Title *with special chars*")
            .text("This is the introduction with some _underscores_ and `backticks`.")
            .codeblock(Some("rust"), "fn main() { println!(\"Hello, world!\"); }")
            .list(
                false,
                markdown_vec![
                    "First bullet with #hash",
                    Markdown::new_paragraph("Second bullet with [brackets]")
                        .bold()
                        .code(),
                ],
            )
            .quote("This is a quote!\nIt spans multiple lines.")
            .list(
                true,
                Vec::from_iter(vec![markdown_vec![
                    Markdown::new_paragraph("italic").italic(),
                    Markdown::space(),
                    Markdown::new_paragraph("bold").bold(),
                    Markdown::space(),
                    Markdown::new_paragraph("code").code(),
                ]]),
            )
            .image(
                "Rust Logo",
                "https://www.rust-lang.org/logos/rust-logo-512x512.png",
            )
            .link("Rust Homepage", "https://www.rust-lang.org")
            .horizontal_rule()
            .table(|table| {
                table
                    .headers(vec!["Header 1", "Header 2"])
                    .row(vec!["Row 1 Col 1", "Row 1 Col 2"])
                    .row(markdown_vec![
                        "Row 2 Col 1",
                        Markdown::new_paragraph("HashMap<String, A | B | C>").code()
                    ])
                    .row(markdown_vec![
                        "Hello",
                        Markdown::Link {
                            text: Box::new("iam a link"),
                            url: "to a thing".to_owned(),
                            anchor: false
                        }
                    ]);
            })
            .build();
        let expected = r#"
            # Documentation Title \*with special chars\*

            This is the introduction with some \_underscores\_ and \`backticks\`\.

            ```rust
            fn main() { println!("Hello, world!"); }
            ```

            - First bullet with \#hash
            - `Second bullet with [brackets]`

            > This is a quote\!
            > It spans multiple lines\.

            1. _italic_ **bold** `code`

            ![Rust Logo](https://www.rust-lang.org/logos/rust-logo-512x512.png)

            [Rust Homepage](https://www.rust-lang.org)

            ---

            | Header 1 | Header 2 |
            | --- | --- |
            | Row 1 Col 1 | Row 1 Col 2 |
            | Row 2 Col 1 | `HashMap<String, A | B | C>` |
            | Hello | [iam a link](to a thing) |
        "#;

        let trimmed_indentation_expected = expected
            .lines()
            .map(|line| line.trim())
            .collect::<Vec<&str>>()
            .join("\n");
        let trimmed_indentation_expected = trimmed_indentation_expected.trim();

        let trimmed_indentation_markdown = markdown
            .lines()
            .map(|line| line.trim())
            .collect::<Vec<&str>>()
            .join("\n");
        let trimmed_indentation_markdown = trimmed_indentation_markdown.trim();

        pretty_assertions::assert_eq!(trimmed_indentation_expected, trimmed_indentation_markdown);
    }

    #[test]
    fn test_markdown_substring_works() {
        // Test markdown_substring with simple 5–7 character inputs.
        let cases = vec![
            // Inline code: "a`bcd`" → with len 3, substring "a`b" is extended to the full inline segment.
            ("a`bcd`", 3, "a`bcd`..."),
            // Bold: "a**b**" → with len 3, substring "a**" is extended to "a**b**".
            ("a**b**", 3, "a**b**..."),
            // Italic: "a*b*" → with len 1, substring "["a*", extended to "a*b*".
            ("a*b*", 1, "a*b*..."),
            // Underscore: "a_b_" → with len 1, extended to "a_b_".
            ("a_b_", 1, "a_b_..."),
            // Link-like: "[x](y)" → with len 1, extended to the next closing bracket.
            ("[x](y)", 1, "[x](y)..."),
        ];
        for (input, len, expected) in cases {
            assert_eq!(
                expected,
                markdown_substring(input, len),
                "Failed for input: {input}"
            );
        }
    }
}
