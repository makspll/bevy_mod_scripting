/// Escapes Markdown reserved characters in the given text.
fn escape_markdown(text: &str) -> String {
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
pub trait IntoMarkdown {
    fn to_markdown(&self, builder: &mut MarkdownBuilder);
}

/// Comprehensive enum representing various Markdown constructs.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Markdown {
    Heading {
        level: u8,
        text: String,
    },
    Paragraph(String),
    InlineCode(String),
    CodeBlock {
        language: Option<String>,
        code: String,
    },
    List {
        ordered: bool,
        items: Vec<String>,
    },
    Quote(String),
    Image {
        alt: String,
        src: String,
    },
    Link {
        text: String,
        url: String,
        anchor: bool,
    },
    HorizontalRule,
    Table {
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
    },
}

impl IntoMarkdown for Markdown {
    fn to_markdown(&self, builder: &mut MarkdownBuilder) {
        match self {
            Markdown::Heading { level, text } => {
                // Clamp the header level to Markdown's 1-6.
                let clamped_level = level.clamp(&1, &6);
                let hashes = "#".repeat(*clamped_level as usize);
                // Escape the text for Markdown
                builder.append(&format!("{} {}", hashes, escape_markdown(text)));
            }
            Markdown::Paragraph(text) => {
                builder.append(&escape_markdown(text));
            }
            Markdown::CodeBlock { language, code } => {
                // Do not escape code blocks
                let lang = language.as_deref().unwrap_or("");
                builder.append(&format!("```{}\n{}\n```", lang, code));
            }
            Markdown::InlineCode(code) => {
                // Do not escape inline code
                builder.append(&format!("`{}`", code));
            }
            Markdown::List { ordered, items } => {
                let list_output = items
                    .iter()
                    .enumerate()
                    .map(|(i, item)| {
                        let escaped_item = escape_markdown(item);
                        if *ordered {
                            format!("{}. {}", i + 1, escaped_item)
                        } else {
                            format!("- {}", escaped_item)
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("\n");
                builder.append(&list_output);
            }
            Markdown::Quote(text) => {
                let quote_output = text
                    .lines()
                    .map(|line| format!("> {}", escape_markdown(line)))
                    .collect::<Vec<String>>()
                    .join("\n");
                builder.append(&quote_output);
            }
            Markdown::Image { alt, src } => {
                // Escape alt text while leaving src untouched.
                builder.append(&format!("![{}]({})", escape_markdown(alt), src));
            }
            Markdown::Link { text, url, anchor } => {
                // anchors must be lowercase, only contain letters or dashes
                let url = if *anchor {
                    // prefix with #
                    format!(
                        "#{}",
                        url.to_lowercase()
                            .replace(" ", "-")
                            .replace(|c: char| !c.is_alphabetic(), "")
                    )
                } else {
                    url.clone()
                };
                // Escape link text while leaving url untouched.
                builder.append(&format!("[{}]({})", escape_markdown(text), url));
            }
            Markdown::HorizontalRule => {
                builder.append("---");
            }
            Markdown::Table { headers, rows } => {
                // Generate a Markdown table:
                // Header row:
                let header_line = format!("| {} |", headers.join(" | "));
                // Separator row:
                let separator_line = format!(
                    "|{}|",
                    headers
                        .iter()
                        .map(|_| " --- ")
                        .collect::<Vec<&str>>()
                        .join("|")
                );
                // Rows:
                let rows_lines = rows
                    .iter()
                    .map(|row| format!("| {} |", row.join(" | ")))
                    .collect::<Vec<String>>()
                    .join("\n");
                builder.append(&format!(
                    "{}\n{}\n{}",
                    header_line, separator_line, rows_lines
                ));
            }
        }
    }
}

/// Builder pattern for generating comprehensive Markdown documentation.
/// Now also doubles as the accumulator for the generated markdown.
pub struct MarkdownBuilder {
    elements: Vec<Markdown>,
    output: String,
    inline: bool,
}

#[allow(dead_code)]
impl MarkdownBuilder {
    /// Creates a new MarkdownBuilder.
    pub fn new() -> Self {
        MarkdownBuilder {
            elements: Vec::new(),
            output: String::new(),
            inline: false,
        }
    }

    pub fn inline(&mut self) -> &mut Self {
        self.inline = true;
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
    pub fn heading(&mut self, level: u8, text: impl Into<String>) -> &mut Self {
        self.elements.push(Markdown::Heading {
            level: level.min(6),
            text: text.into(),
        });
        self
    }

    /// Adds a paragraph element.
    pub fn paragraph(&mut self, text: impl Into<String>) -> &mut Self {
        self.elements.push(Markdown::Paragraph(text.into()));
        self
    }

    /// Adds a code block element.
    pub fn codeblock(
        &mut self,
        language: Option<impl Into<String>>,
        code: impl Into<String>,
    ) -> &mut Self {
        self.elements.push(Markdown::CodeBlock {
            language: language.map(|l| l.into()),
            code: code.into(),
        });
        self
    }

    /// Adds an inline code element.
    pub fn inline_code(&mut self, code: impl Into<String>) -> &mut Self {
        self.elements.push(Markdown::InlineCode(code.into()));
        self
    }

    /// Adds a list element.
    pub fn list(&mut self, ordered: bool, items: Vec<impl Into<String>>) -> &mut Self {
        let converted_items: Vec<String> = items.into_iter().map(|s| s.into()).collect();
        self.elements.push(Markdown::List {
            ordered,
            items: converted_items,
        });
        self
    }

    /// Adds a quote element.
    pub fn quote(&mut self, text: impl Into<String>) -> &mut Self {
        self.elements.push(Markdown::Quote(text.into()));
        self
    }

    /// Adds an image element.
    pub fn image(&mut self, alt: impl Into<String>, src: impl Into<String>) -> &mut Self {
        self.elements.push(Markdown::Image {
            alt: alt.into(),
            src: src.into(),
        });
        self
    }

    /// Adds a link element.
    pub fn link(&mut self, text: impl Into<String>, url: impl Into<String>) -> &mut Self {
        self.elements.push(Markdown::Link {
            text: text.into(),
            url: url.into(),
            anchor: false,
        });
        self
    }

    pub fn section_link(&mut self, text: impl Into<String>, url: impl Into<String>) -> &mut Self {
        self.elements.push(Markdown::Link {
            text: text.into(),
            url: url.into(),
            anchor: true,
        });
        self
    }

    /// Adds a horizontal rule element.
    pub fn horizontal_rule(&mut self) -> &mut Self {
        self.elements.push(Markdown::HorizontalRule);
        self
    }

    /// Adds a table element via a mini builder.
    pub fn table(&mut self, f: impl FnOnce(&mut TableBuilder)) -> &mut Self {
        let mut builder = TableBuilder::new();
        f(&mut builder);
        self.elements.push(builder.build());
        self
    }

    /// Builds the markdown document as a single String by delegating the conversion
    /// of each element to its `into_markdown` implementation.
    pub fn build(&mut self) -> String {
        let len = self.elements.len();
        for (i, element) in self.elements.clone().into_iter().enumerate() {
            element.to_markdown(self);
            if i < len - 1 {
                if self.inline {
                    self.append(" ");
                } else {
                    self.append("\n\n");
                }
            }
        }
        self.output.clone()
    }
}

/// Mini builder for constructing Markdown tables.
pub struct TableBuilder {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
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
    pub fn row(&mut self, row: Vec<impl Into<String>>) -> &mut Self {
        let converted: Vec<String> = row.into_iter().map(|cell| cell.into()).collect();
        self.rows.push(converted);
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
            .paragraph("This is the introduction with some _underscores_ and `backticks`.")
            .codeblock(Some("rust"), "fn main() { println!(\"Hello, world!\"); }")
            .list(
                false,
                vec![
                    "First bullet with #hash",
                    "Second bullet with [brackets]",
                    "Third bullet with (parentheses)",
                ],
            )
            .quote("This is a quote!\nIt spans multiple lines.")
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
                    .row(vec!["Row 2 Col 1", "Row 2 Col 2"]);
            })
            .build();
        let expected = r#"
            # Documentation Title \*with special chars\*

            This is the introduction with some \_underscores\_ and \`backticks\`\.

            ```rust
            fn main() { println!("Hello, world!"); }
            ```

            - First bullet with \#hash
            - Second bullet with \[brackets\]
            - Third bullet with \(parentheses\)

            > This is a quote\!
            > It spans multiple lines\.

            ![Rust Logo](https://www.rust-lang.org/logos/rust-logo-512x512.png)

            [Rust Homepage](https://www.rust-lang.org)

            ---

            | Header 1 | Header 2 |
            | --- | --- |
            | Row 1 Col 1 | Row 1 Col 2 |
            | Row 2 Col 1 | Row 2 Col 2 |
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
}
