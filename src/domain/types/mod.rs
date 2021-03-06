//! Collection of all the types used in this repository

pub mod aoc_ref;
pub mod aoe2net;
pub mod api;
pub mod error;
pub mod match_data;
pub mod requests;

use log::trace;
pub use match_data::MatchDataResponses;
pub use requests::*;

use std::collections::HashMap;

use self::aoc_ref::RefDataLists;
use crate::STANDARD;
use serde::Serialize;

/// The "Database" we use, which is in-memory for lookup of
/// player names and other "more" static content
#[derive(Debug, Clone, Default, Serialize)]
pub struct InMemoryDb {
    /// Translations for aoe2net
    pub aoe2net_languages: HashMap<&'static str, serde_json::Value>,
    /// Containing the Players (Aliases), Platforms and Teams of
    /// aoc-reference-data
    pub github_file_content: RefDataLists,
}

impl InMemoryDb {
    /// Return the [`InMemoryDb`] with only the language needed
    ///
    /// # Panics
    /// Could panic if the [`HashMap`] in [`static@crate::STANDARD`] is
    /// returning None
    pub fn retain_language(
        &mut self,
        language: &str,
    ) -> Self {
        trace!("Checking HashMap for language: {:?}", language);
        if self.aoe2net_languages.contains_key(language) {
            trace!(
                "Cleaning HashMap of other languages than language: {:?} ...",
                language
            );
            self.aoe2net_languages.retain(|&lang, _| lang == language);
        }
        else {
            // Set standard language value to `English`
            // if wrong language is set in `Query`
            let std_language = *(STANDARD.get(&"language").unwrap());
            trace!(
                "Cleaning HashMap of other languages than language: {:?} ...",
                std_language
            );

            self.aoe2net_languages
                .retain(|&lang, _| lang == std_language);
        }

        Self {
            aoe2net_languages: self.aoe2net_languages.clone(),
            github_file_content: self.github_file_content.clone(),
        }
    }
}
