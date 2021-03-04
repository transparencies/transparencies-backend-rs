use log::debug;
// use predicates::str::diff;
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
            Players,
            PlayersRaw,
            Rating,
            TeamRaw,
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

        let players_vec = &self.responses.aoe2net.players_temp.clone();

        let mut players_raw = Vec::with_capacity(players_vec.len() as usize);
        let mut teams_raw: Vec<TeamRaw> = Vec::new();

        let language = &self.responses.get_translation_for_language();

        let mut diff_team = Vec::with_capacity(8);

        // Create the vector for the player information
        let amount_of_successfully_processed_players = self
            .process_all_players(
                players_vec,
                language,
                &mut players_raw,
                &mut diff_team,
            )?;

        // Create the different teams vectors
        let amount_of_successfully_processed_teams =
            assemble_teams_to_vec(diff_team, &players_raw, &mut teams_raw)?;

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

        let translated_last_match_rating_type =
            &self.responses.get_translated_string_from_id(
                "rating_type",
                self.responses.get_rating_type_id()?,
            )?;

        let translated_last_match_map_type =
            &self.responses.get_translated_string_from_id(
                "map_type",
                self.responses.get_map_type_id()?,
            )?;

        let translated_last_match_match_type =
            &self.responses.get_translated_string_from_id(
                "match_type",
                self.responses.get_match_type_id()?,
            )?;

        let match_status = if let Ok(time) =
            &self.responses.get_finished_time()?.parse::<usize>()
        {
            MatchStatus::Finished(*time)
        }
        else {
            MatchStatus::Running
        };

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

        Ok(Self {
            responses: self.responses.clone(),
            match_info: Some(match_info_raw),
            players: Some(Players(players_raw)),
            teams: Some(Teams(teams_raw)),
            result: Some(match_info_result),
            errors: None,
        })
    }

    fn process_all_players(
        &mut self,
        players_vec: &Vec<aoe2net::Player>,
        translation: &Option<Value>,
        players_raw: &mut Vec<PlayersRaw>,
        diff_team: &mut Vec<i64>,
    ) -> Result<usize> {
        let player_amount = players_vec.len();
        for (_player_number, req_player) in players_vec.iter().enumerate() {
            self.assemble_player_to_vec(req_player, translation, players_raw)?;
            if !diff_team.contains(&req_player.team) {
                diff_team.push(req_player.team)
            }
        }
        Ok(player_amount)
    }

    fn assemble_player_to_vec(
        &mut self,
        req_player: &aoe2net::Player,
        translation: &Option<Value>,
        players_raw: &mut Vec<PlayersRaw>,
    ) -> Result<()> {
        // Lookups
        let looked_up_alias = self.lookup_alias(req_player);
        let looked_up_rating = self.lookup_rating(req_player)?;
        let looked_up_leaderboard = self.lookup_leaderboard(req_player)?;
        let requested_player = self.get_requested_player(req_player);

        let mut player_rating =
            get_rating(looked_up_rating, looked_up_leaderboard)?;

        // TODO: check if winrate calculation is right
        player_rating.calculate_win_rate();

        let player_raw = build_player(
            player_rating,
            req_player,
            looked_up_alias,
            translation,
            requested_player,
        )?;

        players_raw.push(player_raw);

        Ok(())
    }

    fn get_requested_player(
        &self,
        req_player: &aoe2net::Player,
    ) -> bool {
        let requested_player = if let Some(player_last_match) =
            &self.responses.aoe2net.player_last_match
        {
            player_last_match["last_match"]["profile_id"]
                == req_player.profile_id
        }
        else {
            false
        };

        requested_player
    }

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

        Ok(looked_up_leaderboard)
    }

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

        Ok(looked_up_rating)
    }

    fn lookup_alias(
        &mut self,
        req_player: &aoe2net::Player,
    ) -> Option<crate::domain::types::aoc_ref::players::Player> {
        // Lookup profile id in alias list
        let looked_up_alias = self
            .responses
            .db
            .github_file_content
            .lookup_player_alias_for_profile_id(
                &(req_player.profile_id.to_string()),
            );

        looked_up_alias
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

fn assemble_teams_to_vec(
    mut diff_team: Vec<i64>,
    players_raw: &Vec<PlayersRaw>,
    teams_raw: &mut Vec<TeamRaw>,
) -> Result<usize> {
    diff_team.sort();

    let team_amount = diff_team.len();

    let mut player_vec_helper: Vec<PlayersRaw> =
        Vec::with_capacity(diff_team.len());

    // Iterate through different teams
    while let Some(team) = diff_team.pop() {
        // Empty vec, as we start a new team
        player_vec_helper.clear();
        // Iterate through players
        while let Some(player) = players_raw.clone().pop() {
            player_vec_helper.push(player)
        }

        let team = TeamRaw::builder()
            .team_number(team)
            .players(Players(player_vec_helper.clone()))
            .build();

        teams_raw.push(team);
    }

    Ok(team_amount)
}

fn build_player(
    player_rating: Rating,
    req_player: &aoe2net::Player,
    looked_up_alias: Option<crate::domain::types::aoc_ref::players::Player>,
    translation: &Option<Value>,
    requested: bool,
) -> Result<PlayersRaw> {
    let player_raw = PlayersRaw::builder()
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
                translation["civ"][req_player.civ.to_string()].to_string()
            }
            else {
                return Err(ProcessingError::CivilisationError);
            },
        )
        .requested(requested)
        .build();

    Ok(player_raw)
}

fn get_rating(
    looked_up_rating: Value,
    looked_up_leaderboard: Value,
) -> Result<Rating> {
    let player_rating = Rating::builder()
        .mmr(looked_up_rating["rating"].to_string().parse::<u32>()?)
        .rank(
            looked_up_leaderboard["leaderboard"]["rank"]
                .to_string()
                .parse::<u64>()?,
        )
        .wins(looked_up_rating["num_wins"].to_string().parse::<u64>()?)
        .losses(looked_up_rating["num_losses"].to_string().parse::<u64>()?)
        .streak(looked_up_rating["streak"].to_string().parse::<i32>()?)
        .highest_mmr(
            looked_up_leaderboard["leaderboard"]["highest_rating"]
                .to_string()
                .parse::<u32>()?,
        )
        .build();

    Ok(player_rating)
}
