use std::{
    fs,
    io::BufReader,
    path::PathBuf,
};

use crate::domain::types::{
    api::{
        MatchInfoRequest,
        MatchInfoResult,
    },
    error::TestCaseError,
};
use serde::{
    Deserialize,
    Serialize,
};
use serde_json::Value as JsonValue;

use derive_getters::Getters;

#[derive(Default, Serialize, Deserialize)]
pub struct TestCases(pub Vec<TestCase>);

impl TestCases {
    /// Add another case to the vector of [`TestCases`]
    ///
    /// # Errors
    // TODO
    pub fn add_case(
        mut self,
        root_dir: PathBuf,
    ) -> Result<Self, TestCaseError> {
        self.0
            .push(TestCase::with_root_dir(root_dir).parse_from_root_dir()?);
        Ok(self)
    }
}

#[derive(Default, Getters, Serialize, Deserialize, Clone)]
pub struct TestCase {
    resource_dir: PathBuf,
    pub parsed_request: MatchInfoRequest,
    pub parsed_result: MatchInfoResult,
    pub profile_ids: Vec<String>,
    last_match: JsonValue,
}

impl TestCase {
    #[must_use]
    pub fn with_root_dir(dir: PathBuf) -> Self {
        Self {
            // resource_dir: PathBuf::from(dir),
            resource_dir: dir,
            parsed_request: MatchInfoRequest::default(),
            parsed_result: MatchInfoResult::default(),
            profile_ids: Vec::with_capacity(8),
            last_match: JsonValue::default(),
        }
    }

    /// Create a [`MatchInfoResult`]from a parsed `RON` file
    ///
    /// # Errors
    // TODO
    /// # Panics
    /// Panics when the file can not be created or data cannot be written to the
    /// file
    pub fn with_file(path: PathBuf) -> Result<Self, TestCaseError> {
        Ok(ron::de::from_reader::<_, Self>(BufReader::new(
            fs::File::open(path)?,
        ))?)
    }

    /// Parses the basic data (e.g. matchinfo request, result, and last match)
    /// directly into the [`TestCase`] struct
    ///
    /// # Errors
    // TODO
    /// # Panics
    // TODO
    pub fn parse_from_root_dir(self) -> Result<Self, TestCaseError> {
        let mut req = self.resource_dir.clone();
        req.push("match_info_request.ron");

        let mut resp = self.resource_dir.clone();
        resp.push("match_info_result.ron");

        let mut last_match = self.resource_dir.clone();
        last_match.push("aoe2net");
        last_match.push("last_match.json");

        Ok(Self {
            parsed_request: ron::de::from_reader::<_, MatchInfoRequest>(
                BufReader::new(fs::File::open(req)?),
            )?,
            parsed_result: ron::de::from_reader::<_, MatchInfoResult>(
                BufReader::new(fs::File::open(resp)?),
            )?,
            last_match: serde_json::from_reader::<_, JsonValue>(
                BufReader::new(fs::File::open(last_match)?),
            )?,
            ..self
        })
    }
}
