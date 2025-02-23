//! The library crate for the mdbook LAD preprocessor.
#![allow(missing_docs)]

use mdbook::{errors::Error, preprocess::Preprocessor, BookItem};
use sections::Section;
mod argument_visitor;
mod markdown;
mod sections;

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
            if let mdbook::BookItem::Chapter(parent) = item {
                let mut replacements = Vec::default();
                for (ladfile_idx, child) in parent.sub_items.iter().enumerate() {
                    if let BookItem::Chapter(child) = child {
                        let is_lad_file = child
                            .source_path
                            .as_ref()
                            .and_then(|a| a.file_name())
                            .is_some_and(|a| a.to_string_lossy().ends_with(LAD_EXTENSION));

                        if !is_lad_file {
                            continue;
                        }

                        let chapter_title =
                            child.name.clone().trim_end_matches(".lad.json").to_owned();

                        let ladfile = match ladfile::parse_lad_file(&child.content) {
                            Ok(lad) => lad,
                            Err(e) => {
                                log::debug!("Failed to parse LAD file: {:?}", e);
                                errors.push(Error::new(e).context("Failed to parse LAD file"));
                                continue;
                            }
                        };

                        log::debug!(
                            "Parsed LAD file: {}",
                            serde_json::to_string_pretty(&ladfile).unwrap_or_default()
                        );

                        let new_chapter = Section::Summary {
                            ladfile: &ladfile,
                            title: Some(chapter_title),
                        }
                        .into_chapter(Some(parent), ladfile_idx);

                        log::debug!(
                            "New chapter: {}",
                            serde_json::to_string_pretty(&new_chapter).unwrap_or_default()
                        );

                        // replace
                        replacements.push((ladfile_idx, BookItem::Chapter(new_chapter)));
                    }
                }

                for (idx, replacement) in replacements {
                    log::debug!(
                        "Replacing chapter at index {}. With : \n{}",
                        idx,
                        serde_json::to_string_pretty(&replacement).unwrap_or_default()
                    );
                    parent.sub_items[idx] = replacement;
                }
            }
        });

        // book.for_each_mut(|item| {
        //     if let mdbook::BookItem::Chapter(chapter) = item {
        //         let is_lad_chapter = chapter
        //             .source_path
        //             .as_ref()
        //             .and_then(|a| a.file_name())
        //             .is_some_and(|a| a.to_string_lossy().ends_with(LAD_EXTENSION));

        //         if !is_lad_chapter {
        //             log::debug!("Skipping non-LAD chapter: {:?}", chapter.source_path);
        //             log::trace!(
        //                 "Non-LAD chapter: {}",
        //                 serde_json::to_string_pretty(&chapter).unwrap_or_default()
        //             );
        //             return;
        //         }

        //         let chapter_title = chapter
        //             .name
        //             .clone()
        //             .trim_end_matches(".lad.json")
        //             .to_owned();

        //         let lad = match ladfile::parse_lad_file(&chapter.content) {
        //             Ok(lad) => lad,
        //             Err(e) => {
        //                 log::debug!("Failed to parse LAD file: {:?}", e);
        //                 errors.push(Error::new(e).context("Failed to parse LAD file"));
        //                 return;
        //             }
        //         };

        //         log::debug!(
        //             "Parsed LAD file: {}",
        //             serde_json::to_string_pretty(&lad).unwrap_or_default()
        //         );

        //         let mut new_chapter = Section::Summary {
        //             ladfile: &lad,
        //             title: Some(chapter_title),
        //         }
        //         .into_chapter(None, 0);

        //         new_chapter.path = new_chapter
        //             .path
        //             .map(|m| chapter.path.as_ref().cloned().unwrap_or_default().join(m));

        //         new_chapter.source_path = new_chapter.source_path.map(|m| {
        //             chapter
        //                 .source_path
        //                 .as_ref()
        //                 .cloned()
        //                 .unwrap_or_default()
        //                 .join(m)
        //         });

        //         new_chapter.parent_names = chapter.parent_names.clone();

        //         // let sections = sections::lad_file_to_sections(&lad, Some(chapter_title));

        //         // let new_chapter = sections::section_to_chapter(
        //         //     sections,
        //         //     Some(chapter),
        //         //     chapter.parent_names.clone(),
        //         //     chapter.number.clone(),
        //         //     None,
        //         //     None,
        //         // );

        //         // serialize chapter to json
        //         log::debug!(
        //             "New chapter: {}",
        //             serde_json::to_string_pretty(&new_chapter).unwrap_or_default()
        //         );

        //         *chapter = new_chapter;
        //     }
        // });

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
