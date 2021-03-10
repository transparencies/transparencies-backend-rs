use std::{
    fs,
    io::BufReader,
    path::PathBuf,
};

use serde::{
    Deserialize,
    Serialize,
};

use crate::domain::types::{
    api::{
        MatchInfoRequest,
        MatchInfoResult,
    },
    error::TestCaseError,
};

use derive_getters::Getters;

#[derive(Default, Serialize, Deserialize)]
pub struct TestCases(pub Vec<TestCase>);

impl TestCases {
    pub fn add(
        mut self,
        root_dir: &str,
    ) -> Result<Self, TestCaseError> {
        self.0.push(TestCase::new_with_root(root_dir).parse_from()?);
        Ok(self)
    }
}

#[derive(Default, Getters, Serialize, Deserialize, Clone)]
pub struct TestCase {
    resource_root_dir: PathBuf,
    pub parsed_request: MatchInfoRequest,
    pub parsed_result: MatchInfoResult,
    pub profile_ids: Vec<String>,
    last_match: serde_json::Value,
}

impl TestCase {
    pub fn new_with_root(root_dir: &str) -> Self {
        Self {
            resource_root_dir: PathBuf::from(root_dir),
            parsed_request: MatchInfoRequest::default(),
            parsed_result: MatchInfoResult::default(),
            profile_ids: Vec::with_capacity(8),
            last_match: serde_json::Value::default(),
        }
    }

    /// Create a [`MatchInfoResult`]from a parsed `RON` file
    ///
    /// # Panics
    /// Panics when the file can not be created or data cannot be written to the
    /// file
    pub fn new_from_file(path: PathBuf) -> Result<Self, TestCaseError> {
        Ok(ron::de::from_reader::<_, Self>(BufReader::new(
            fs::File::open(path).expect("file should open read only"),
        ))?)
    }

    pub fn parse_from(self) -> Result<Self, TestCaseError> {
        #[allow(unused_assignments)]
        let mut helper = PathBuf::default();

        Ok(Self {
            parsed_request: ron::de::from_reader::<_, MatchInfoRequest>(
                BufReader::new(fs::File::open({
                    helper = self.resource_root_dir.to_owned();
                    helper.set_file_name("match_info_request.ron");
                    helper.as_path()
                })?),
            )?,
            parsed_result: ron::de::from_reader::<_, MatchInfoResult>(
                BufReader::new(fs::File::open({
                    helper = self.resource_root_dir.to_owned();
                    helper.set_file_name("match_info_result.ron");
                    helper.as_path()
                })?),
            )?,
            last_match: serde_json::from_reader::<_, serde_json::Value>(
                BufReader::new(fs::File::open({
                    helper = self.resource_root_dir.to_owned();
                    helper.set_file_name("aoe2net/last_match.json");
                    helper.as_path()
                })?),
            )?,
            ..self
        })
    }
}
