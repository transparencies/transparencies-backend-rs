use serde_json::Value;

use crate::domain::{
    data_processing::MatchDataResponses,
    types::api::{
        MatchInfo,
        MatchInfoResult,
        Players,
        Teams,
    },
};
use ron::ser::{
    to_writer_pretty,
    PrettyConfig,
};
use std::{
    fs,
    io::BufWriter,
    result,
    sync::Arc,
    time::Duration,
};

use stable_eyre::eyre::{
    Report,
    WrapErr,
};

use serde::Serialize;

// Error handling
type ProcessingErrorStrings = Vec<String>;
use super::error::ProcessingError;

type Result<T> = result::Result<T, ProcessingError>;

#[derive(Clone, Debug, Serialize)]
pub struct MatchInfoProcessor {
    responses: MatchDataResponses,
    match_info: Option<MatchInfo>,
    players: Option<Players>,
    teams: Option<Teams>,
    result: Option<MatchInfoResult>,
    errors: Option<ProcessingErrorStrings>,
}

impl MatchInfoProcessor {
    pub fn new_with_response(responses: MatchDataResponses) -> Result<Self> {
        Ok(Self {
            responses,
            match_info: None,
            players: None,
            teams: None,
            result: None,
            errors: None,
        })
    }

    pub fn process(// &self
    ) -> Result<Self> {
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

    pub fn assemble(// &self
    ) -> Result<MatchInfoResult> {
        todo!();
    }

    // Return Teams array
    // Return Ratings for `profile_id`
}
