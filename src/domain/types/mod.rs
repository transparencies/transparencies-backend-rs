pub mod aoc_ref;
pub mod aoe2net;
pub mod api;
pub mod match_data;
pub mod requests;

pub use match_data::MatchDataResponses;
pub use requests::*;

use std::{
    collections::HashMap,
    sync::Arc,
};

use tokio::sync::Mutex;

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
    pub fn with_language(
        &mut self,
        language: &str,
    ) -> Self {
        let mut used_language = self.aoe2net_languages.clone();

        if used_language.contains_key(language) {
            used_language.retain(|&lang, _| lang == language);
        }
        else {
            // Set standard language value to `English`
            // if wrong language is set in `Query`
            let std_language = STANDARD_LANGUAGE;
            used_language.retain(|&lang, _| lang == std_language);
        }

        Self {
            aoe2net_languages: used_language,
            github_file_content: self.github_file_content.clone(),
        }
    }
}
