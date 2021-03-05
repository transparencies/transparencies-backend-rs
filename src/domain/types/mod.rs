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
use crate::STANDARD_LANGUAGE;
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
pub struct InMemoryDb {
    pub aoe2net_languages: HashMap<&'static str, serde_json::Value>,
    pub github_file_content: RefDataLists,
}

impl InMemoryDb {
    // Return the `InMemoryDb` only with the language needed
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
            let std_language = STANDARD_LANGUAGE;
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
