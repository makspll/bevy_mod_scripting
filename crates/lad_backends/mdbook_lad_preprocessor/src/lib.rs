//! The library crate for the mdbook LAD preprocessor.
#![allow(missing_docs)]

use mdbook::{errors::Error, preprocess::Preprocessor};
use sections::Section;
mod argument_visitor;
mod markdown;
mod sections;

const LAD_EXTENSION: &str = "lad.json";

pub struct LADPreprocessor;

impl LADPreprocessor {
    /// Checks if a chapter is a LAD file.
    fn is_lad_file(chapter: &mdbook::book::Chapter) -> bool {
        chapter
            .source_path
            .as_ref()
            .and_then(|a| a.file_name())
            .map(|s| s.to_string_lossy().ends_with(LAD_EXTENSION))
            .unwrap_or(false)
    }

    /// Process a chapter that is a LAD file.
    ///
    /// `parent` is the optional parent chapter reference,
    /// and `chapter_index` is the index of the chapter among its siblings.
    fn process_lad_chapter(
        chapter: &mdbook::book::Chapter,
        parent: Option<&mdbook::book::Chapter>,
        chapter_index: usize,
    ) -> Result<mdbook::book::Chapter, Error> {
        let chapter_title = chapter.name.trim_end_matches(".lad.json").to_owned();
        let ladfile = ladfile::parse_lad_file(&chapter.content)
            .map_err(|e| Error::new(e).context("Failed to parse LAD file"))?;
        log::debug!(
            "Parsed LAD file: {}",
            serde_json::to_string_pretty(&ladfile).unwrap_or_default()
        );
        let new_chapter = Section::Summary {
            ladfile: &ladfile,
            title: Some(chapter_title),
        }
        .into_chapter(parent, chapter_index);
        log::debug!(
            "New chapter: {}",
            serde_json::to_string_pretty(&new_chapter).unwrap_or_default()
        );
        Ok(new_chapter)
    }
}

impl Preprocessor for LADPreprocessor {
    fn name(&self) -> &str {
        "lad-preprocessor"
    }

    fn run(
        &self,
        _ctx: &mdbook::preprocess::PreprocessorContext,
        mut book: mdbook::book::Book,
    ) -> mdbook::errors::Result<mdbook::book::Book> {
        let mut errors = Vec::new();

        // first replace children in parents
        book.for_each_mut(|item| {
            if let mdbook::BookItem::Chapter(parent) = item {
                // First, collect the indices and new chapters for LAD file chapters.
                let replacements: Vec<(usize, mdbook::book::Chapter)> = parent
                    .sub_items
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, item)| {
                        if let mdbook::BookItem::Chapter(chapter) = item {
                            if LADPreprocessor::is_lad_file(chapter) {
                                match LADPreprocessor::process_lad_chapter(
                                    chapter,
                                    Some(parent),
                                    idx,
                                ) {
                                    Ok(new_chapter) => return Some((idx, new_chapter)),
                                    Err(e) => {
                                        errors.push(e);
                                        return None;
                                    }
                                }
                            }
                        }
                        None
                    })
                    .collect();

                // Then, apply the replacements.
                for (idx, new_chapter) in replacements {
                    if let mdbook::BookItem::Chapter(chapter) = &mut parent.sub_items[idx] {
                        *chapter = new_chapter;
                    }
                }
            }
        });

        // then try match items themselves
        book.for_each_mut(|item| {
            if let mdbook::BookItem::Chapter(chapter) = item {
                if !LADPreprocessor::is_lad_file(chapter) {
                    return;
                }

                let new_chapter = match LADPreprocessor::process_lad_chapter(chapter, None, 0) {
                    Ok(new_chapter) => new_chapter,
                    Err(e) => {
                        errors.push(e);
                        return;
                    }
                };

                *chapter = new_chapter;
            }
        });

        log::debug!(
            "Book after LAD processing: {}",
            serde_json::to_string_pretty(&book).unwrap_or_default()
        );

        if !errors.is_empty() {
            // return on first error
            for error in errors {
                log::error!("{}", error);
                Err(error)?;
            }
        }

        Ok(book)
    }
}
