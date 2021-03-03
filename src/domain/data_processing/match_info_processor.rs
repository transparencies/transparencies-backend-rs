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
            Rating,
            Teams,
            TeamsRaw,
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

impl Rating {
    #[allow(clippy::cast_precision_loss)]
    pub fn calculate_win_rate(&mut self) {
        if self.wins == 0 {
            self.win_rate = None;
        }
        else {
            self.win_rate =
                Some((self.losses as f32 / self.wins as f32) * 100_f32);
        }
    }
}
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

    pub fn process(&mut self) -> Result<Self> {
        // TODO Error handling instead of unwrap
        // Collect errors in &self.errors or alike

        let players_vec = &self.responses.aoe2net.players_temp;

        let mut players_raw = Vec::with_capacity(players_vec.len() as usize);
        let mut _teams_raw: Vec<TeamsRaw> = Vec::new();

        let mut translation: Option<serde_json::Value> = None;

        if self.responses.db.aoe2net_languages.len() == 1 {
            for (language, translation_value) in
                self.responses.db.aoe2net_languages.drain().take(1)
            {
                debug!("Translation that was used: {:?}", language);
                translation = Some(translation_value);
            }
        }

        for (_player_number, req_player) in players_vec.iter().enumerate() {
            // Lookup profile id in alias list
            let looked_up_alias = self
                .responses
                .db
                .github_file_content
                .lookup_player_alias_for_profile_id(
                    &(req_player.profile_id.to_string()),
                );

            if let Some(looked_up_rating) =
                self.responses.lookup_player_rating_for_profile_id(
                    &(req_player.profile_id.to_string()),
                )
            {
                if let Some(looked_up_leaderboard) =
                    self.responses.lookup_leaderboard_for_profile_id(
                        &(req_player.profile_id.to_string()),
                    )
                {
                    let mut player_rating = Rating::builder()
                        .mmr(
                            looked_up_rating["rating"]
                                .to_string()
                                .parse::<u32>()?,
                        )
                        .rank(
                            looked_up_leaderboard["leaderboard"]["rank"]
                                .to_string()
                                .parse::<u64>()?,
                        )
                        .wins(
                            looked_up_rating["num_wins"]
                                .to_string()
                                .parse::<u64>()?,
                        )
                        .losses(
                            looked_up_rating["num_losses"]
                                .to_string()
                                .parse::<u64>()?,
                        )
                        .streak(
                            looked_up_rating["streak"]
                                .to_string()
                                .parse::<i32>()?,
                        )
                        .highest_mmr(
                            looked_up_leaderboard["leaderboard"]
                                ["highest_rating"]
                                .to_string()
                                .parse::<u32>()?,
                        )
                        .build();

                    // TODO: check if winrate calculation is right
                    player_rating.calculate_win_rate();

                    players_raw.push(
                        PlayersRaw::builder()
                            .rating(player_rating)
                            .player_number(req_player.color)
                            .name(looked_up_alias.as_ref().map_or_else(
                                || req_player.name.clone(),
                                |lookup_player| lookup_player.name.clone(),
                            ))
                            .country(looked_up_alias.as_ref().map_or_else(
                                || req_player.country.to_string(),
                                |lookup_player| lookup_player.country.clone(),
                            ))
                            .civilisation(
                                if let Some(translation) = &translation {
                                    translation["civ"]
                                        [req_player.civ.to_string()]
                                    .to_string()
                                }
                                else {
                                    return Err(
                                        ProcessingError::CivilisationError,
                                    );
                                },
                            )
                            .build(),
                    )
                }
            }
        }

        // let _player_result = Players(player_raw);

        // println!(
        //     "Players array (Length: {:?}): {:#?}",
        //     players_array.len(),
        //     players_array
        // );

        // Read in Teams
        // Read Players into Teams
        // Read Ratings into Players
        // Assemble Information to MatchInfo
        // Wrap MatchInfo with Erros into MatchInfoResult

        Ok(Self {
            responses: self.responses.clone(),
            match_info: None,
            players: Some(Players(players_raw)),
            teams: None,
            result: None,
            errors: None,
        })
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
