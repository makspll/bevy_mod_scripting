//! Rust doc comments as they read in the book: rustdoc-only syntax resolved, headings nested under the page's,
//! and one-line summaries for tables.

use std::collections::HashMap;

use ladfile::LadFile;

use crate::sections::{linkify_filename, type_page_title};

/// The pages of the types of a file by their identifier, to link rustdoc paths like ``[`Transform`]`` to.
pub(crate) struct DocLinks {
    /// `None` if two types share the identifier, so a path can't tell them apart
    pages: HashMap<String, Option<String>>,
}

impl DocLinks {
    pub(crate) fn new(ladfile: &LadFile) -> Self {
        let mut pages: HashMap<String, Option<String>> = HashMap::new();
        let mut insert = |identifier: &str, page: String| {
            pages
                .entry(identifier.to_owned())
                .and_modify(|existing| {
                    if existing.as_deref() != Some(page.as_str()) {
                        *existing = None
                    }
                })
                .or_insert(Some(page));
        };
        for (id, lad_type) in ladfile.types.iter() {
            insert(
                &lad_type.identifier,
                linkify_filename(type_page_title(ladfile, id)) + ".md",
            );
        }
        for generic_type in ladfile.generic_types.values() {
            insert(
                &generic_type.identifier,
                linkify_filename(generic_type.name_with_params()) + ".md",
            );
        }
        Self { pages }
    }

    /// The page of the type a rustdoc path names, i.e. `bevy_math::Vec3`, `Self::forward` or `Vec<T>`.
    fn page(&self, path: &str) -> Option<&str> {
        let path = path.trim().trim_matches('`').trim_end_matches("()");
        let path = path.split('<').next().unwrap_or(path);
        let mut segments = path.rsplit("::");
        let last = segments.next()?;
        // `Type::method` links to the type
        let lookup = |name: &str| self.pages.get(name).and_then(|page| page.as_deref());
        lookup(last).or_else(|| segments.next().and_then(lookup))
    }
}

/// Whether a link target is a rustdoc path rather than a url, i.e. `bevy_math::Vec3` or `Self::forward`.
fn is_rust_path(target: &str) -> bool {
    !target.contains("://")
        && !target.starts_with(['#', '.', '/'])
        && !target.ends_with(".md")
        && !target.ends_with(".html")
}

/// How rustdoc links are rewritten.
#[derive(Clone, Copy)]
enum Links<'a> {
    /// Linked to the pages of their types in the given directory, or left as plain text
    Resolve(&'a DocLinks, &'a str),
    /// Replaced by their text, for summaries which are cut
    Strip,
}

impl Links<'_> {
    fn link(&self, text: &str, target: &str) -> String {
        match self {
            Links::Resolve(links, directory) => match links.page(target) {
                Some(page) => format!("[{text}]({directory}/{page})"),
                None => text.to_owned(),
            },
            Links::Strip => text.to_owned(),
        }
    }
}

/// Rewrites the rustdoc links of a line which isn't code, see [`render_docs`].
fn rewrite_links(line: &str, links: Links<'_>, references: &HashMap<String, String>) -> String {
    let chars: Vec<char> = line.chars().collect();
    let mut out = String::with_capacity(line.len());
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            // code spans are copied as they are
            '`' => {
                let ticks = chars[i..].iter().take_while(|c| **c == '`').count();
                let fence: String = "`".repeat(ticks);
                let rest: String = chars[i + ticks..].iter().collect();
                match rest.find(&fence) {
                    Some(end) => {
                        let len = ticks + rest[..end].chars().count() + ticks;
                        out.extend(&chars[i..i + len]);
                        i += len;
                    }
                    None => {
                        out.extend(&chars[i..]);
                        i = chars.len();
                    }
                }
            }
            '[' => {
                let Some(close) = matching_bracket(&chars, i) else {
                    out.push('[');
                    i += 1;
                    continue;
                };
                let text: String = chars[i + 1..close].iter().collect();
                let after = close + 1;
                match chars.get(after) {
                    // `[text](target)`
                    Some('(') => {
                        let Some(end) = chars[after..].iter().position(|c| *c == ')') else {
                            out.extend(&chars[i..]);
                            break;
                        };
                        let target: String = chars[after + 1..after + end].iter().collect();
                        out.push_str(&match (is_rust_path(&target), links) {
                            (true, _) => links.link(&text, &target),
                            (false, Links::Strip) => text.clone(),
                            (false, Links::Resolve(..)) => format!("[{text}]({target})"),
                        });
                        i = after + end + 1;
                    }
                    // `[text][reference]`
                    Some('[') => {
                        let Some(end) = chars[after..].iter().position(|c| *c == ']') else {
                            out.extend(&chars[i..]);
                            break;
                        };
                        let reference: String = chars[after + 1..after + end].iter().collect();
                        let key = if reference.is_empty() {
                            &text
                        } else {
                            &reference
                        };
                        let target = references.get(key).cloned().unwrap_or_else(|| key.clone());
                        out.push_str(&links.link(&text, &target));
                        i = after + end + 1;
                    }
                    // `[Transform]`, a shortcut to the item of that name
                    _ => {
                        let target = references
                            .get(&text)
                            .cloned()
                            .unwrap_or_else(|| text.clone());
                        out.push_str(&links.link(&text, &target));
                        i = after;
                    }
                }
            }
            c => {
                out.push(c);
                i += 1;
            }
        }
    }
    out
}

/// The index of the `]` closing the `[` at `open`, skipping code spans.
fn matching_bracket(chars: &[char], open: usize) -> Option<usize> {
    let mut depth = 0;
    let mut in_code = false;
    for (offset, c) in chars[open..].iter().enumerate() {
        match c {
            '`' => in_code = !in_code,
            '[' if !in_code => depth += 1,
            ']' if !in_code => {
                depth -= 1;
                if depth == 0 {
                    return Some(open + offset);
                }
            }
            _ => {}
        }
    }
    None
}

/// The reference definitions of a doc, i.e. `[Vec3]: bevy_math::Vec3`, which rustdoc resolves to items.
fn reference_definitions(doc: &str) -> HashMap<String, String> {
    doc.lines()
        .filter_map(|line| {
            let line = line.trim();
            let rest = line.strip_prefix('[')?;
            let (name, target) = rest.split_once("]:")?;
            Some((name.to_owned(), target.trim().to_owned()))
        })
        .collect()
}

/// Whether the line is a reference definition, see [`reference_definitions`]
fn is_reference_definition(line: &str) -> bool {
    let line = line.trim();
    line.starts_with('[') && line.contains("]:")
}

/// A code fence opening or closing a code block, with its info string.
fn code_fence(line: &str) -> Option<&str> {
    let trimmed = line.trim_start();
    trimmed
        .strip_prefix("```")
        .or_else(|| trimmed.strip_prefix("~~~"))
}

/// Renders a doc comment as markdown of the book.
///
/// - rustdoc links (`[`Transform`]`, `[text](bevy_math::Vec3)`, reference definitions) link to the pages of their
///   types in `types_directory`, or become plain text when the type has no page.
/// - headings are nested `heading_offset` levels deeper, so `# Examples` doesn't outrank the page's own headings.
/// - lines rustdoc hides in rust examples (`# use ...`) are removed.
pub(crate) fn render_docs(
    doc: &str,
    links: &DocLinks,
    types_directory: &str,
    heading_offset: usize,
) -> String {
    let references = reference_definitions(doc);
    let mut out = Vec::new();
    // the info string of the code block the line is in, if any
    let mut code: Option<String> = None;
    // doc comments keep the space after `///`, which rustdoc removes
    let indented = doc
        .lines()
        .filter(|line| !line.trim().is_empty())
        .all(|line| line.starts_with(' '));
    let lines = doc.lines().map(|line| match indented {
        true => line.strip_prefix(' ').unwrap_or(line),
        false => line,
    });
    for line in lines {
        if let Some(info) = code_fence(line) {
            code = match code {
                Some(_) => None,
                None => Some(info.trim().to_owned()),
            };
            out.push(line.to_owned());
            continue;
        }
        match &code {
            Some(info) => {
                let is_rust = info.is_empty() || info.split(',').any(|i| i.trim() == "rust");
                let trimmed = line.trim_start();
                if is_rust && (trimmed == "#" || trimmed.starts_with("# ")) {
                    continue;
                }
                out.push(line.to_owned());
            }
            None if is_reference_definition(line) => {}
            None => {
                let line = rewrite_links(line, Links::Resolve(links, types_directory), &references);
                let trimmed = line.trim_start();
                let hashes = trimmed.chars().take_while(|c| *c == '#').count();
                if hashes > 0 && trimmed[hashes..].starts_with(' ') {
                    let level = (hashes + heading_offset).min(6);
                    out.push(format!("{}{}", "#".repeat(level), &trimmed[hashes..]));
                } else {
                    out.push(line);
                }
            }
        }
    }
    out.join("\n")
}

/// The first paragraph of a doc comment on one line, for tables: links become their text, and long paragraphs are cut
/// after their first sentence, or at a word, so that no formatting is left open.
pub(crate) fn summary(doc: &str, max_chars: usize) -> String {
    let references = reference_definitions(doc);
    let paragraph = doc
        .lines()
        .map(str::trim)
        .skip_while(|line| line.is_empty())
        .take_while(|line| {
            !line.is_empty()
                && !line.starts_with('#')
                && code_fence(line).is_none()
                && !is_reference_definition(line)
        })
        .map(|line| rewrite_links(line, Links::Strip, &references))
        .collect::<Vec<_>>()
        .join(" ");

    let mut text = if paragraph.chars().count() <= max_chars {
        paragraph
    } else {
        let cut: String = paragraph.chars().take(max_chars).collect();
        match cut.find(". ") {
            Some(end) => cut[..=end].to_owned(),
            None => {
                let at_word = cut.rfind(' ').map_or(cut.as_str(), |end| &cut[..end]);
                format!("{}…", at_word.trim_end_matches([',', ';', ':']))
            }
        }
    };
    // a code span cut in half is closed
    if text.matches('`').count() % 2 == 1 {
        let ellipsis = text.ends_with('…');
        if ellipsis {
            text.pop();
        }
        text.push('`');
        if ellipsis {
            text.push('…');
        }
    }
    // a table cell ends at a pipe
    text.replace('|', "\\|")
}

#[cfg(test)]
mod test {
    use super::*;

    fn links() -> DocLinks {
        DocLinks {
            pages: HashMap::from([
                ("Transform".to_owned(), Some("transform.md".to_owned())),
                ("Vec3".to_owned(), Some("vec3.md".to_owned())),
                ("Ambiguous".to_owned(), None),
            ]),
        }
    }

    #[test]
    fn rustdoc_links_link_to_type_pages_or_become_text() {
        let doc = "Set its [`Transform`], or [`Vec3`](bevy_math::Vec3), see [`forward`](Self::forward), \
            [`ChildOf`](bevy_ecs::ChildOf), [`Ambiguous`] and [the book](https://bevy.org). `[not a link]`";
        assert_eq!(
            render_docs(doc, &links(), ".", 2),
            "Set its [`Transform`](./transform.md), or [`Vec3`](./vec3.md), see `forward`, \
            `ChildOf`, `Ambiguous` and [the book](https://bevy.org). `[not a link]`"
        );
    }

    #[test]
    fn reference_links_are_resolved_and_their_definitions_removed() {
        let doc = "Uses a [vector][v].\n\n[v]: bevy_math::Vec3";
        assert_eq!(
            render_docs(doc, &links(), ".", 2),
            "Uses a [vector](./vec3.md).\n"
        );
    }

    #[test]
    fn headings_are_nested_and_hidden_example_lines_removed() {
        let doc =
            "# Examples\n```\n# use bevy::prelude::*;\nlet a = 1; // [not a link]\n```\n## More";
        assert_eq!(
            render_docs(doc, &links(), ".", 2),
            "### Examples\n```\nlet a = 1; // [not a link]\n```\n#### More"
        );
    }

    #[test]
    fn summaries_are_the_first_paragraph_on_one_line() {
        let doc = "A [`Transform`]\nat a position.\n\n# Example\n```\ncode\n```";
        assert_eq!(summary(doc, 100), "A `Transform` at a position.");
    }

    #[test]
    fn long_summaries_are_cut_after_a_sentence_or_at_a_word() {
        assert_eq!(
            summary(
                "Rotates the transform. Errors if the axis is zero, which it may be.",
                40
            ),
            "Rotates the transform."
        );
        assert_eq!(
            summary(
                "Rotates the transform around `some_axis_with_a_long_name` by angle",
                40
            ),
            "Rotates the transform around…"
        );
        // a code span cut in half is closed
        assert_eq!(
            summary("Uses `a b` and `c d e f`", 18),
            "Uses `a b` and `c`…"
        );
    }
}
