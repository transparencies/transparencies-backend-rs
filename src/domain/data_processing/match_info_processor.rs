use serde_json::Value;

use crate::domain::{
    data_processing::MatchDataResponses,
    types::api::MatchInfoResult,
};
use ron::ser::{
    to_writer_pretty,
    PrettyConfig,
};
use std::{
    fs,
    io::BufWriter,
    sync::Arc,
    time::Duration,
};

use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct MatchInfoProcessor {
    responses: MatchDataResponses,
    result: Option<MatchInfoResult>,
}

impl MatchInfoProcessor {
    pub fn new_with_response(responses: MatchDataResponses) -> Self {
        Self {
            responses,
            result: None,
        }
    }

    pub fn process(&self) -> Self {
        todo!();

        // Read in Teams
        // Read Players into Teams
        // Read Ratings into Players
        // Assemble Information to MatchInfo
        // Wrap MatchInfo with Erros into MatchInfoResult
    }

    pub fn export_data_to_file(&self) {
        let ron_config = PrettyConfig::new()
            .with_depth_limit(8)
            .with_separate_tuple_members(true)
            .with_enumerate_arrays(true)
            .with_indentor("\t".to_owned());

        // Open the file in writable mode with buffer.
        let file = fs::File::create("logs/match_info_result.ron").unwrap();
        let writer = BufWriter::new(file);

        // Write data to file
        to_writer_pretty(writer, &self.result, ron_config)
            .expect("Unable to write data");
    }

    pub fn assemble(&self) -> MatchInfoResult {
        todo!();
    }

    // Return Teams array
    // Return Ratings for `profile_id`
}
