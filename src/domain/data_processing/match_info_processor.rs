use log::debug;
use serde_json::Value;

use crate::domain::{
    data_processing::MatchDataResponses,
    types::{
        aoe2net,
        api::{
            MatchInfo,
            MatchInfoResult,
            Players,
            PlayersRaw,
            Teams,
        },
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
    #[must_use]
    pub fn new_with_response(responses: MatchDataResponses) -> Self {
        Self {
            responses,
            match_info: None,
            players: None,
            teams: None,
            result: None,
            errors: None,
        }
    }

    pub fn process(&mut self) -> Self {
        // TODO Error handling instead of unwrap
        // Collect errors in &self.errors or alike
        let players_array = self
            .responses
            .parse_all_players::<Vec<aoe2net::Players>>()
            .unwrap();

        let mut player_raw = Vec::with_capacity(players_array.len() as usize);

        let mut translation: Option<serde_json::Value> = None;

        if self.responses.db.aoe2net_languages.len() == 1 {
            for (language, translation_value) in
                self.responses.db.aoe2net_languages.drain().take(1)
            {
                debug!("Translation that was used: {:?}", language);
                translation = Some(translation_value);
            }
        }
        else {
                translation = None;
        };

        for (_player_number, req_player) in players_array.iter().enumerate() {
            let lookuped_player = self
                .responses
                .db
                .github_file_content
                .lookup_player_alias_for_profile_id(
                    req_player.profile_id.to_string(),
                );

            player_raw.push(
                PlayersRaw::builder()
                // TODO Rating struct
                //.rating(player.rating)
                .player_number(req_player.slot)
                .name(if let Some(lookup_player) = &lookuped_player {
                    lookup_player.name.clone()
                } else {
                    req_player.name.clone()
                } )
                .country(if let Some(lookup_player) = &lookuped_player {
                    lookup_player.country.clone()
                } else {
                    req_player.country.to_string()
                } ), /* TODO Language lookup
                      * .civilisation(civilisation)
                      * .build() */
            )
        }

        // let _player_result = Players(player_raw);

        println!(
            "Players array (Length: {:?}): {:#?}",
            players_array.len(),
            players_array
        );

        // Read in Teams
        // Read Players into Teams
        // Read Ratings into Players
        // Assemble Information to MatchInfo
        // Wrap MatchInfo with Erros into MatchInfoResult

        Self {
            responses: self.responses.clone(),
            match_info: None,
            players: None,
            teams: None,
            result: None,
            errors: None,
        }
    }

    pub fn assemble(&self) -> Result<MatchInfoResult> {
        self.result
            .as_ref()
            .map_or(Err(ProcessingError::AssemblyError), |result| {
                Ok(result.clone())
            })
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
}
