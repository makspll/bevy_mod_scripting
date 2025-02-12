//! The library crate for the mdbook LAD preprocessor.
#![allow(missing_docs)]

use mdbook::{errors::Error, preprocess::Preprocessor};

const LAD_EXTENSION: &str = "lad.json";

pub struct LADPreprocessor;

impl Preprocessor for LADPreprocessor {
    fn name(&self) -> &str {
        "lad-preprocessor"
    }

    fn run(
        &self,
        _ctx: &mdbook::preprocess::PreprocessorContext,
        mut book: mdbook::book::Book,
    ) -> mdbook::errors::Result<mdbook::book::Book> {
        let mut errors = Vec::default();
        book.for_each_mut(|item| {
            if let mdbook::BookItem::Chapter(chapter) = item {
                let is_lad_chapter = chapter
                    .source_path
                    .as_ref()
                    .and_then(|a| a.file_name())
                    .is_some_and(|a| a.to_string_lossy().ends_with(LAD_EXTENSION));

                if !is_lad_chapter {
                    log::debug!("Skipping non-LAD chapter: {:?}", chapter.source_path);
                    return;
                }

                let lad = match ladfile::parse_lad_file(&chapter.content) {
                    Ok(lad) => lad,
                    Err(e) => {
                        log::debug!("Failed to parse LAD file: {:?}", e);
                        errors.push(Error::new(e).context("Failed to parse LAD file"));
                        return;
                    }
                };

                log::debug!("Parsed LAD file: {:?}", lad);

                // for now just replace the content with a list of types in the LAD file
                let content = lad
                    .types
                    .iter()
                    .map(|(_, lad)| lad.identifier.clone())
                    .collect::<Vec<_>>()
                    .join("\n");

                chapter.content = content;
            }
        });

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
