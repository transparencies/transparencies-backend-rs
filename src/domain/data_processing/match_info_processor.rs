//! Everything around [`MatchInfoProcessor`] resembles in here
//! Beware, there is a close connection to the [`MatchDataResponses`]
//! in many places

use tracing::trace;

use serde_json::Value as JsonValue;

use crate::domain::{
    data_processing::MatchDataResponses,
    types::{
        aoc_ref,
        aoe2net::{
            self,
            Aoe2netRequestType,
            RecoveredRating,
        },
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
        error::ProcessingError,
    },
    util,
};

use std::result;

use serde::Serialize;

// Error handling
type ProcessingErrorStrings = Vec<String>;

type Result<T> = result::Result<T, ProcessingError>;

impl Rating {
    /// Calculate the win rate for the Rating data structure and modifies
    /// `win_rate` in-place
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

/// Contains everything needed to assemble a [`MatchInfoResult`]
#[derive(Clone, Debug, Serialize)]
pub struct MatchInfoProcessor {
    /// Responses from all around the place for easier handling of each request
    responses: MatchDataResponses,
    match_info: Option<MatchInfo>,
    players: Option<Players>,
    teams: Option<Teams>,
    result: Option<MatchInfoResult>,
    errors: Option<ProcessingErrorStrings>,
}

impl MatchInfoProcessor {
    /// Create a new `MatchInfoProcessor` from `MatchDataResponses`
    ///
    /// # Arguments
    /// * `responses` - takes a [`MatchDataResponses`] struct to initialize
    ///   `self.responses`
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
    /// finally assembled to a [`MatchInfoResult`]. Modifies its data in-place
    ///
    /// # Errors
    // TODO: Many errors can come up here, that need to be collected and named
    #[tracing::instrument(name = "Processing MatchDataResponses", skip(self))]
    pub fn process(&mut self) -> Result<Self> {
        // TODO: Collect errors in &self.errors or alike
        trace!("Processing MatchDataResponses ...");

        let req_type = if self.responses.aoe2net.match_id.is_some() {
            Aoe2netRequestType::MatchId
        }
        else {
            Aoe2netRequestType::LastMatch
        };

        let players_vec = &self.responses.aoe2net.players_temp.clone();

        let mut players_raw = Vec::with_capacity(players_vec.len() as usize);
        let mut teams_raw: Vec<TeamRaw> = Vec::new();

        let mut diff_team: Vec<i64> = Vec::with_capacity(8);

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
                self.responses.get_rating_type_id(req_type)?,
            )?;
        trace!("Successfully translated rating type.");

        trace!("Translate map type ...");
        let translated_last_match_map_type =
            &self.responses.get_translated_string_from_id(
                "map_type",
                self.responses.get_map_type_id(req_type)?,
            )?;
        trace!("Successfully translated map type.");

        trace!("Translate into game type from match type...");
        let translated_last_match_match_type =
            &self.responses.get_translated_string_from_id(
                "game_type",
                self.responses.get_game_type_id(req_type)?,
            )?;
        trace!("Successfully translated game type.");

        trace!("Getting match status ...");
        let match_status = if let Ok(time) =
            &self.responses.get_finished_time(req_type)?.parse::<usize>()
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
            .server(self.responses.get_server_location(req_type)?)
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
    ///
    /// # Arguments
    /// * `players_vec` - a slice of a vector of [`aoe2net::Player`]s that holds
    ///   all the players that are in that corresponding game
    /// * `players_raw` - a mutable reference to a vector of raw Players to push
    ///   each processed [`PlayerRaw`] to
    /// * `diff_team` - a mutable reference to a vector if integers with the
    ///   unique team numbers of the match to iterate over
    ///
    /// # Errors
    /// Errors are bubbled up into the processing stage of
    /// [`MatchInfoProcessor`]
    fn process_all_players(
        &mut self,
        players_vec: &[aoe2net::Player],
        players_raw: &mut Vec<PlayerRaw>,
        diff_team: &mut Vec<i64>,
    ) -> Result<usize> {
        trace!("Processing all players ...");
        let player_amount = players_vec.len();
        for req_player in players_vec.iter() {
            self.assemble_player_to_vec(req_player, players_raw)?;
            if !diff_team.contains(&req_player.team) {
                diff_team.push(req_player.team)
            }
        }
        trace!("Successfully processed all players.");

        Ok(player_amount)
    }

    /// Uses different funtions to lookup player information and builds a
    /// player. Afterwards pushes it into a vector of [`PlayerRaw`].
    ///
    /// # Arguments
    /// * `req_player` - holding a reference to [`aoe2net::Player`] that
    ///   contains all information we got from the `last_match` response
    /// * `players_processing` - a mutable reference to a vector of
    ///   [`PlayerRaw`] containing the succesfully built players with all
    ///   information belonging to them
    ///
    /// # Errors
    /// Errors are bubbled up into the processing stage of
    /// [`MatchInfoProcessor`]
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
        trace!("Successfully looked up rating: {:#?}", looked_up_rating);

        trace!("Looking up leaderboard ...");
        // TODO: Recover from leaderboard array being empty
        let looked_up_leaderboard = self.lookup_leaderboard(req_player)?;

        trace!("Getting requested player ...");
        let requested_player_boolean = self.get_requested_player(req_player);
        trace!(
            "Successfully got requested player: {:#?}",
            requested_player_boolean
        );

        trace!("Getting player's rating ...");
        let mut player_rating = MatchDataResponses::get_rating(
            &looked_up_rating,
            &looked_up_leaderboard,
        )?;
        trace!(
            "Successfully got requested player's rating: {:?}",
            player_rating
        );

        trace!("Getting player country ...");
        let player_country =
            MatchDataResponses::get_country(&looked_up_leaderboard);
        trace!(
            "Successfully got requested player's country: {:?}",
            player_country
        );

        trace!("Getting player civilisation translation ...");
        let translated_civilisation_string =
            &self.responses.get_translated_string_from_id(
                "civ",
                req_player.civ.to_string().parse::<usize>()?,
            )?;
        trace!("Successfully translated player civilisation.");

        trace!("Calculating player win rate ...");
        player_rating.calculate_win_rate();
        trace!("Successfully calculated player win rate.");

        trace!("Building player struct ...");
        let player_built = build_player(
            player_rating,
            player_country,
            req_player,
            &looked_up_alias,
            translated_civilisation_string.to_string(),
            requested_player_boolean,
        )?;
        trace!("Successfully built player struct.");

        players_processing.push(player_built);

        Ok(())
    }

    /// Check if the player we currently iterate over is the player the request
    /// was made for on our `matchinfo` endpoint and returns this as Boolean
    ///
    /// # Arguments
    /// * `req_player` - holding a reference to [`aoe2net::Player`] that
    ///   contains all information we got from the `last_match` response
    fn get_requested_player(
        &self,
        req_player: &aoe2net::Player,
    ) -> bool {
        self.responses.aoe2net.player_last_match.as_ref().map_or(
            false,
            |player_last_match| {
                util::remove_escaping(
                    player_last_match["profile_id"].to_string(),
                ) == util::remove_escaping(req_player.profile_id.to_string())
            },
        )
    }

    /// Lookup a corresponding player in the `leaderboard` response
    ///
    /// # Arguments
    /// * `req_player` - holding a reference to [`aoe2net::Player`] that
    ///   contains all information we got from the `last_match` response
    ///
    /// # Errors
    /// This function will error out if the Leaderboard [`serde_json::Value`]
    /// could not be found
    fn lookup_leaderboard(
        &mut self,
        req_player: &aoe2net::Player,
    ) -> Result<(RecoveredRating, JsonValue)> {
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

        if looked_up_leaderboard["count"].to_string() >= 1.to_string() {
            return Ok((
                RecoveredRating::Original,
                looked_up_leaderboard["leaderboard"][0].clone(),
            ));
        }
        else if looked_up_leaderboard["count"].to_string() == 0.to_string() {
            // Try to recover
            if let Some(looked_up_leaderboard) =
                self.responses.lookup_leaderboard_for_profile_id(
                    &(format!(
                        "{}_recovery",
                        &req_player.profile_id.to_string()
                    )),
                )
            {
                return Ok((
                    RecoveredRating::Recovered,
                    looked_up_leaderboard.clone(),
                ));
            }
        }

        return Err(ProcessingError::NotRankedLeaderboard(
            req_player.profile_id,
        ));
    }

    /// Lookup a corresponding player's `rating`
    ///
    /// # Arguments
    /// * `req_player` - holding a reference to [`aoe2net::Player`] that
    ///   contains all information we got from the `last_match` response
    ///
    /// # Errors
    /// This function will error out if the rating [`serde_json::Value`]
    /// could not be found
    fn lookup_rating(
        &mut self,
        req_player: &aoe2net::Player,
    ) -> Result<JsonValue> {
        trace!("Looking up rating for player: {:?}", req_player.profile_id);
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

    /// Lookup a corresponding player's `alias` in the `index` of the
    /// `aoc-reference-data` and return an Option of
    /// [`aoc_ref::players::Player`] with all the information from the
    /// `players.yaml` for that player
    ///
    /// # Arguments
    /// * `req_player` - holding a reference to [`aoe2net::Player`] that
    ///   contains all information we got from the `last_match` response
    fn lookup_alias(
        &mut self,
        req_player: &aoe2net::Player,
    ) -> Option<aoc_ref::players::Player> {
        // Lookup profile id in alias list
        self.responses
            .db
            .github_file_content
            .lookup_player_alias_for_profile_id(
                &(req_player.profile_id.to_string()),
            )
    }

    /// Creates a [`MatchInfoResult`]
    ///
    /// # Errors
    /// This function will error if the result of the processing stage could not
    /// be found.
    // TODO: A validation of that [`MatchInfoResult`] still has to be made
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
///
/// # Arguments
/// * `diff_team` - a mutable vector if integers with the unique team numbers of
///   the match to iterate over
/// * `players_raw` - a vector slice of [`PlayerRaw`]
/// * `teams_raw` - a reference to a mutable vector of [`TeamRaw`] to push the
///   `Teams` to
///
/// # Errors
/// Errors are bubbled up into the processing stage of [`MatchInfoProcessor`]
fn assemble_teams_to_vec(
    mut diff_team: Vec<i64>,
    players_raw: &[PlayerRaw],
    teams_raw: &mut Vec<TeamRaw>,
) -> usize {
    trace!("Sorting amount of teams vector ...");
    diff_team.sort_unstable();
    trace!("Finished sorting amount of teams vector.");

    // Keep only the non-used teams available
    let mut available_empty_teams: Vec<i64> = vec![1, 2, 3, 4, 5, 6, 7, 8];
    available_empty_teams.retain(|t| !diff_team.contains(t));

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

        trace!("Sorting members for team {:?} ...", team);
        // Sort for requested player
        player_vec_helper.sort_by(|a, b| a.requested().cmp(&b.requested()));
        trace!("Sorting of team {:?} complete ...", team);

        trace!("Build team number {:?} ...", team);
        // Case: team == `-1` then push each player to a different
        // team
        if team == -1 {
            for ffa_player in player_vec_helper.to_owned() {
                let helper: Vec<PlayerRaw> = vec![ffa_player];
                let own_team = TeamRaw::builder()
                    .team_number(
                        available_empty_teams.first().map_or(-1, |val| *val),
                    )
                    .players(Players(helper.clone()))
                    .build();
                teams_raw.push(own_team);
            }
        }
        else {
            let single_team = TeamRaw::builder()
                .team_number(team)
                .players(Players(player_vec_helper.clone()))
                .build();
            teams_raw.push(single_team);
        }
    }
    trace!("Finished iterating through teams.");

    teams_raw.len()
}

/// Build a player with the builder pattern
///
/// # Arguments
/// * `player_rating` - the [`Rating`] information for that corresponding player
/// * `player_country` - a String wrapped in an Option that is `None` if the
///   country was `not set` resulting in a `Standard Value` for our API of
///   `null`
/// * `req_player` - a reference to [`aoe2net::Player`] information for that
///   corresponding player
/// * `looked_up_alias` - a reference to an Option of
///   [`aoc_ref::players::Player`] with all player information coming from
///   `aoc-reference-data`
/// * `translated_civilisation_string` - a language-dependent String for the
///   players civilisation
/// * `requested` - a Boolean that show if the player we are currently building
///   is the player the request on our API was made for
fn build_player(
    player_rating: Rating,
    player_country: Option<String>,
    req_player: &aoe2net::Player,
    looked_up_alias: &Option<aoc_ref::players::Player>,
    translated_civilisation_string: String,
    requested: bool,
) -> Result<PlayerRaw> {
    let player_raw = PlayerRaw::builder()
        .rating(player_rating)
        .player_number(req_player.color.to_string().parse::<i64>()?)
        .team_number(req_player.team)
        .name(looked_up_alias.as_ref().map_or_else(
            || {
                let name = util::remove_escaping(req_player.name.to_string());
                name
            },
            |lookup_player| lookup_player.name.clone(),
        ))
        .country(looked_up_alias.as_ref().map_or_else(
            || player_country.unwrap_or_else(|| "null".to_string()),
            |lookup_player| lookup_player.country.clone(),
        ))
        .civilisation(translated_civilisation_string)
        .requested(requested)
        .build();

    Ok(player_raw)
}
