//! Everything around `MatchInfoProcessing` resembles in here
//! Beware, there is a close connection to the `MatchDataResponses`
//! in many places

use tracing::trace;

use serde_json::Value;

use crate::domain::{
    data_processing::MatchDataResponses,
    types::{
        aoe2net,
        api::{
            MatchInfo,
            MatchInfoResult,
            MatchSize,
            MatchStatus,
            PlayerRaw,
            Players,
            Rating,
            TeamRaw,
            Teams,
        },
    },
    util,
};

use std::{
    convert::TryInto,
    result,
};

use serde::Serialize;

// Error handling
type ProcessingErrorStrings = Vec<String>;
use crate::domain::types::error::ProcessingError;

type Result<T> = result::Result<T, ProcessingError>;

impl Rating {
    /// Calculate the win rate of for that Rating data structure
    #[allow(clippy::cast_precision_loss)]
    pub fn calculate_win_rate(&mut self) {
        if self.losses == 0 {
            self.win_rate = Some(100_f32);
        }
        else {
            self.win_rate = Some(
                (self.wins as f32 / (self.wins as f32 + self.losses as f32))
                    * 100_f32,
            );
        }
    }
}

/// Contains everything needed to assemble a `MatchInfoResult`
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
    /// Create a new `MatchInfoProcessor` from `MatchDataResponses`
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

    /// Process all given information and set up this datastructure to be
    /// finally assembled to a `MatchInfoResult`
    pub fn process(&mut self) -> Result<Self> {
        // TODO Error handling instead of unwrap
        // Collect errors in &self.errors or alike
        trace!("Processing MatchDataResponses ...");

        let players_vec = &self.responses.aoe2net.players_temp.clone();

        let mut players_raw = Vec::with_capacity(players_vec.len() as usize);
        let mut teams_raw: Vec<TeamRaw> = Vec::new();

        let mut diff_team = Vec::with_capacity(8);

        trace!("Creating vector for player information.");
        // Create the vector for the player information
        let amount_of_successfully_processed_players = self
            .process_all_players(
                players_vec,
                &mut players_raw,
                &mut diff_team,
            )?;
        trace!("Successfully created vector for player information.");

        trace!("Creating different teams vectors.");
        // Create the different teams vectors
        let amount_of_successfully_processed_teams =
            assemble_teams_to_vec(diff_team, &players_raw, &mut teams_raw);
        trace!("Successfully created different teams vectors.");

        trace!("Calculating match size ...");
        let match_size = match (
            amount_of_successfully_processed_players,
            amount_of_successfully_processed_teams,
        ) {
            (2, 2) => MatchSize::G1v1,
            (4, 2) => MatchSize::G2v2,
            (6, 2) => MatchSize::G3v3,
            (8, 2) => MatchSize::G4v4,
            (6, 3) => MatchSize::G2v2v2,
            (8, 4) => MatchSize::G2v2v2v2,
            (_, _) => MatchSize::Custom,
        };
        trace!("Successfully calculated match size: {:?}", match_size);

        trace!("Translate rating type ...");
        let translated_last_match_rating_type =
            &self.responses.get_translated_string_from_id(
                "rating_type",
                self.responses.get_rating_type_id()?,
            )?;
        trace!("Successfully translated rating type.");

        trace!("Translate map type ...");
        let translated_last_match_map_type =
            &self.responses.get_translated_string_from_id(
                "map_type",
                self.responses.get_map_type_id()?,
            )?;
        trace!("Successfully translated map type.");

        trace!("Translate into game type from match type...");
        let translated_last_match_match_type =
            &self.responses.get_translated_string_from_id(
                "game_type",
                self.responses.get_game_type_id()?,
            )?;
        trace!("Successfully translated game type.");

        trace!("Getting match status ...");
        let match_status = if let Ok(time) =
            &self.responses.get_finished_time()?.parse::<usize>()
        {
            MatchStatus::Finished(*time)
        }
        else {
            MatchStatus::Running
        };
        trace!("Sucessfully got match status.");

        trace!("Assembling information to MatchInfo and MatchInfoResult ...");
        // Assemble Information to MatchInfo
        let match_info_raw = MatchInfo::builder()
            .match_size(match_size)
            .game_type(translated_last_match_match_type.to_string())
            .rating_type(translated_last_match_rating_type.to_string())
            .map_name(translated_last_match_map_type.to_string())
            .server(self.responses.get_server_location()?)
            .teams(Teams(teams_raw.clone()))
            .match_status(match_status)
            .build();

        // Wrap MatchInfo with converted Errors into MatchInfoResult
        let match_info_result = MatchInfoResult::builder()
            .match_info(match_info_raw.clone())
            .build();
        trace!("Successfully assembled information to MatchInfo and MatchInfoResult.");

        Ok(Self {
            responses: self.responses.clone(),
            match_info: Some(match_info_raw),
            players: Some(Players(players_raw)),
            teams: Some(Teams(teams_raw)),
            result: Some(match_info_result),
            errors: None,
        })
    }

    /// Process all the players given in a `Last_Match` response
    fn process_all_players(
        &mut self,
        players_vec: &[aoe2net::Player],
        players_raw: &mut Vec<PlayerRaw>,
        diff_team: &mut Vec<i64>,
    ) -> Result<usize> {
        trace!("Processing all players ...");
        let player_amount = players_vec.len();
        for (_player_number, req_player) in players_vec.iter().enumerate() {
            self.assemble_player_to_vec(req_player, players_raw)?;
            if !diff_team.contains(&req_player.team) {
                diff_team.push(req_player.team)
            }
        }
        trace!("Successfully processed all players.");

        Ok(player_amount)
    }

    fn assemble_player_to_vec(
        &mut self,
        req_player: &aoe2net::Player,
        players_processing: &mut Vec<PlayerRaw>,
    ) -> Result<()> {
        // Lookups
        trace!("Looking up alias ...");
        let looked_up_alias = self.lookup_alias(req_player);
        trace!("Successfully looked up alias.");

        trace!("Looking up rating ...");
        let looked_up_rating = self.lookup_rating(req_player)?;
        trace!("Successfully looked up rating.");

        trace!("Looking up leaderboard ...");
        let looked_up_leaderboard = self.lookup_leaderboard(req_player)?;
        trace!("Successfully looked up leaderboard.");

        trace!("Getting requested player ...");
        let requested_player_boolean = self.get_requested_player(req_player);
        trace!(
            "Successfully got requested player: {:#?}",
            requested_player_boolean
        );

        trace!("Getting player rating ...");
        let mut player_rating =
            get_rating(&looked_up_rating, &looked_up_leaderboard)?;
        trace!("Successfully got requested player rating.");

        trace!("Getting player civilisation translation ...");
        let translated_civilisation_string =
            &self.responses.get_translated_string_from_id(
                "civ",
                req_player
                    .civ
                    .try_into()
                    .expect("Conversion of civilisation id failed."),
            )?;
        trace!("Successfully translated player civilisation.");

        trace!("Calculating player win rate ...");
        player_rating.calculate_win_rate();
        trace!("Successfully calculated player win rate.");

        trace!("Building player struct ...");
        let player_built = build_player(
            player_rating,
            req_player,
            &looked_up_alias,
            translated_civilisation_string.to_string(),
            requested_player_boolean,
        );
        trace!("Successfully built player struct.");

        players_processing.push(player_built);

        Ok(())
    }

    /// Check if the player we currently iterate over is the player the request
    /// was made for on the `matchinfo` endpoint
    fn get_requested_player(
        &self,
        req_player: &aoe2net::Player,
    ) -> bool {
        self.responses.aoe2net.player_last_match.as_ref().map_or(
            false,
            |player_last_match| {
                player_last_match["profile_id"] == req_player.profile_id
            },
        )
    }

    // Lookup a corresponding player in the `leaderboard` response
    fn lookup_leaderboard(
        &mut self,
        req_player: &aoe2net::Player,
    ) -> Result<Value> {
        let looked_up_leaderboard = if let Some(looked_up_leaderboard) =
            self.responses.lookup_leaderboard_for_profile_id(
                &(req_player.profile_id.to_string()),
            ) {
            looked_up_leaderboard
        }
        else {
            return Err(ProcessingError::LeaderboardNotFound(
                req_player.profile_id,
            ));
        };

        Ok(looked_up_leaderboard["leaderboard"][0].clone())
    }

    // Lookup a corresponding player's `rating`
    fn lookup_rating(
        &mut self,
        req_player: &aoe2net::Player,
    ) -> Result<Value> {
        let looked_up_rating = if let Some(looked_up_rating) =
            self.responses.lookup_player_rating_for_profile_id(
                &(req_player.profile_id.to_string()),
            ) {
            looked_up_rating
        }
        else {
            return Err(ProcessingError::LookupRatingNotFound(
                req_player.profile_id,
            ));
        };

        Ok(looked_up_rating[0].clone())
    }

    // Lookup a corresponding player's `alias` in the `index` of the
    // aoc-reference-data
    fn lookup_alias(
        &mut self,
        req_player: &aoe2net::Player,
    ) -> Option<crate::domain::types::aoc_ref::players::Player> {
        // Lookup profile id in alias list
        self.responses
            .db
            .github_file_content
            .lookup_player_alias_for_profile_id(
                &(req_player.profile_id.to_string()),
            )
    }

    /// Create a `MatchInfoResult`
    pub fn assemble(&self) -> Result<MatchInfoResult> {
        trace!("Assembling MatchInfoResult to frontend ...");
        self.result
            .as_ref()
            .map_or(Err(ProcessingError::AssemblyError), |result| {
                Ok(result.clone())
            })
    }
}

/// Create the `Teams` vectors from the `Players`
fn assemble_teams_to_vec(
    mut diff_team: Vec<i64>,
    players_raw: &[PlayerRaw],
    teams_raw: &mut Vec<TeamRaw>,
) -> usize {
    trace!("Sorting amount of teams vector ...");
    diff_team.sort_unstable();
    trace!("Finished sorting amount of teams vector.");

    let team_amount = diff_team.len();

    let mut player_vec_helper: Vec<PlayerRaw> =
        Vec::with_capacity(diff_team.len());

    trace!("Iterating through different teams ...");
    // Iterate through different teams
    while let Some(team) = diff_team.pop() {
        // Empty vec, as we start a new team
        trace!("Emptying helper vector for players ...");
        player_vec_helper.clear();
        // Iterate through players
        trace!("Iterating through players of team number {:?} ...", team);
        for player in players_raw.to_owned() {
            if *player.team_number() == team {
                player_vec_helper.push(player);
            }
        }

        trace!("Build team number {:?} ...", team);
        let team = TeamRaw::builder()
            .team_number(team)
            .players(Players(player_vec_helper.clone()))
            .build();

        teams_raw.push(team);
    }
    trace!("Finished iterating through teams.");

    team_amount
}

/// Build a player with the builder pattern
fn build_player(
    player_rating: Rating,
    req_player: &aoe2net::Player,
    looked_up_alias: &Option<crate::domain::types::aoc_ref::players::Player>,
    translated_civilisation_string: String,
    requested: bool,
) -> PlayerRaw {
    let player_raw = PlayerRaw::builder()
        .rating(player_rating)
        .player_number(req_player.color)
        .team_number(req_player.team)
        .name(looked_up_alias.as_ref().map_or_else(
            || req_player.name.clone(),
            |lookup_player| lookup_player.name.clone(),
        ))
        .country(looked_up_alias.as_ref().map_or_else(
            || req_player.country.to_string(),
            |lookup_player| lookup_player.country.clone(),
        ))
        .civilisation(translated_civilisation_string)
        .requested(requested)
        .build();

    player_raw
}

/// Get a `Rating` datastructure from a `response` for a given player
fn get_rating(
    looked_up_rating: &Value,
    looked_up_leaderboard: &Value,
) -> Result<Rating> {
    // TODO Get rid of expect and gracefully handle errors
    let player_rating = Rating::builder()
        .mmr(
            serde_json::from_str::<u32>(&serde_json::to_string(
                &looked_up_rating["rating"],
            )?)
            .expect("MMR parsing failed."),
        )
        .rank(
            serde_json::from_str::<u64>(&serde_json::to_string(
                &looked_up_leaderboard["rank"],
            )?)
            .expect("Rank parsing failed."),
        )
        .wins(
            serde_json::from_str::<u64>(&serde_json::to_string(
                &looked_up_rating["num_wins"],
            )?)
            .expect("Wins parsing failed."),
        )
        .losses(
            serde_json::from_str::<u64>(&serde_json::to_string(
                &looked_up_rating["num_losses"],
            )?)
            .expect("Losses parsing failed."),
        )
        .streak(
            serde_json::from_str::<i32>(&serde_json::to_string(
                &looked_up_rating["streak"],
            )?)
            .expect("Streak parsing failed."),
        )
        .highest_mmr(
            serde_json::from_str::<u32>(&serde_json::to_string(
                &looked_up_leaderboard["highest_rating"],
            )?)
            .expect("Highest-MMR parsing failed."),
        )
        .build();

    Ok(player_rating)
}
