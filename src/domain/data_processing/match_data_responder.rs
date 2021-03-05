//! Everything around `MatchDataResponses` resembles in here
//! Beware, there is a close connection to the `MatchInfoProcessing`
//! in many places

use crate::domain::types::{
    aoe2net,
    aoe2net::Aoe2netStringObj,
    api::{
        MatchInfoRequest,
        Rating,
        Server,
    },
    error::ResponderError,
    InMemoryDb,
    MatchDataResponses,
};

use crate::domain::util;

use ron::ser::{
    to_writer_pretty,
    PrettyConfig,
};
use serde_json::Value;
use std::{
    fs,
    io::BufWriter,
    result,
    sync::Arc,
};
use tokio::sync::Mutex;
use tracing::{
    debug,
    trace,
};

use crate::STANDARD;

type Result<T> = result::Result<T, ResponderError>;

impl MatchDataResponses {
    /// Return `serde_json::Value` for `leaderboard_id` for future requests
    ///
    /// # Errors
    /// Will return an error if the `leaderboard_id` could not be found
    pub fn get_leaderboard_id_for_request(&self) -> Result<String> {
        self.aoe2net.player_last_match.as_ref().map_or_else(
            || Err(ResponderError::NotFound("leaderboard_id".to_string())),
            |val| Ok(val["last_match"]["leaderboard_id"].to_string()),
        )
    }

    /// Parses all the players into a `type T`
    /// from the `last_match` response for convenience
    ///
    /// # Errors
    /// Will return an error if either the `players array` can not be found
    /// or will `panic!` if the deserialisation failed or the parsing of the
    /// `Player` struct failed
    pub fn parse_all_players<T>(&self) -> Result<T>
    where T: for<'de> serde::Deserialize<'de> {
        self.aoe2net.player_last_match.as_ref().map_or_else(
            || Err(ResponderError::NotFound("players array".to_string())),
            |val| {
                Ok(serde_json::from_str::<T>(
                    &serde_json::to_string(&val["last_match"]["players"])
                        .expect("Conversion of players to string failed."),
                )
                .expect("Parsing of player struct failed."))
            },
        )
    }

    /// Returns the number of players from the `last_match` response
    ///
    /// # Errors
    /// Will return an error if the `last_match` could not be found
    pub fn get_number_of_players(&self) -> Result<String> {
        self.aoe2net.player_last_match.as_ref().map_or_else(
            || Err(ResponderError::NotFound("number of players".to_string())),
            |val| Ok(val["last_match"]["num_players"].to_string()),
        )
    }

    /// Returns the finishing time of a match
    /// We use that to see if a match is currently running (`Null`)
    /// or already finished
    ///
    /// # Errors
    /// Will return an error if the `last_match` could not be found
    pub fn get_finished_time(&self) -> Result<String> {
        self.aoe2net.player_last_match.as_ref().map_or_else(
            || Err(ResponderError::NotFound("finished time".to_string())),
            |val| Ok(val["last_match"]["finished"].to_string()),
        )
    }

    /// Returns the rating type id for a match
    /// We use that to check if a game is rated/unrated
    /// or in which ladder to look for the Rating
    ///
    /// # Errors
    /// Will return an error if the `last_match` could not be found
    pub fn get_rating_type_id(&self) -> Result<usize> {
        self.aoe2net.player_last_match.as_ref().map_or_else(
            || Err(ResponderError::NotFound("rating type".to_string())),
            |val| {
                Ok(val["last_match"]["rating_type"]
                    .to_string()
                    .parse::<usize>()?)
            },
        )
    }

    /// Get a `Rating` datastructure from a `response` for a given player
    ///
    /// # Errors
    /// Won't throw an error, but `Option` is set to `None` resulting in an
    /// empty player name that is taken if the `looked_up_player` doesn't
    /// give any value
    pub fn get_country(looked_up_leaderboard: &Value) -> Option<String> {
        if let Some(mut country) =
            Some(looked_up_leaderboard["country"].to_string().to_lowercase())
        {
            country.retain(|s| s != '\\');
            country.retain(|s| s != '\"');
            Some(country)
        }
        else {
            None
        }
    }

    /// Get a `Rating` datastructure from a `response` for a given player
    ///
    /// # Arguments
    /// * `looked_up_rating` - a [serde_json::Value] type that holds the
    ///   [aoe2net::RatingHistory] of an [aoe2net::Player]
    /// * `looked_up_leaderboard` - a [serde_json::Value] type that holds
    ///   Leaderboard data of an [aoe2net::Player]
    ///
    /// # Errors
    /// Function will throw errors in cases the deserialisation and conversion
    /// to the corresponding types is not successful
    pub fn get_rating(
        looked_up_rating: &Value,
        looked_up_leaderboard: &Value,
    ) -> Result<Rating> {
        // TODO Get rid of expect and gracefully handle errors
        let player_rating = Rating::builder()
            .mmr(serde_json::from_str::<u32>(&serde_json::to_string(
                &looked_up_rating["rating"],
            )?)?)
            .rank(serde_json::from_str::<u64>(&serde_json::to_string(
                &looked_up_leaderboard["rank"],
            )?)?)
            .wins(serde_json::from_str::<u64>(&serde_json::to_string(
                &looked_up_rating["num_wins"],
            )?)?)
            .losses(serde_json::from_str::<u64>(&serde_json::to_string(
                &looked_up_rating["num_losses"],
            )?)?)
            .streak(serde_json::from_str::<i32>(&serde_json::to_string(
                &looked_up_rating["streak"],
            )?)?)
            .highest_mmr(serde_json::from_str::<u32>(&serde_json::to_string(
                &looked_up_leaderboard["highest_rating"],
            )?)?)
            .build();

        Ok(player_rating)
    }

    /// Returns a`serde_json::Value`of the downloaded translation
    /// strings from AoE2.net
    ///
    /// # Errors
    /// Will throw an error if the translation has moved due to consuming a
    /// value from the in-memory database. Errors there, due to threaded
    /// behaviour, will result in `runtime` errors, no compile-time checks
    /// possible.
    pub fn get_translation_for_language(&mut self) -> Result<Value> {
        let mut translation: Option<serde_json::Value> = None;

        trace!(
            "Length of self.db.aoe2net_languages is: {:?}",
            self.db.aoe2net_languages.len()
        );

        if self.db.aoe2net_languages.len() == 1 {
            for (language, translation_value) in
                self.db.aoe2net_languages.clone().drain().take(1)
            {
                translation = Some(translation_value);
                trace!("Translation that was used: {:?}", language);
            }
        }
        else {
            return Err(ResponderError::TranslationHasBeenMovedError);
        }
        Ok(translation.expect("Translation should never be None value."))
    }

    /// Returns the corresponding `String` for our convenience
    /// by searching through [Aoe2netStringObj]'s field `id`
    ///
    /// # Arguments
    /// * `first` - A string slice that holds the element name to be used to
    ///   lookup inside the `language` (e.g. `civ` or `game_type`).
    /// * `id` - An integer holding the number that we get from `last_match` or
    ///   `leaderboard` to translate and be looked up
    ///
    /// # Errors
    /// This will error if the `translation data` cannot be found. Most likely
    /// due to being moved/consumed. See also
    /// [Self::get_translation_for_language()].
    pub fn get_translated_string_from_id(
        &self,
        first: &str,
        id: usize,
    ) -> Result<String> {
        trace!("Getting translated string in {:?} with id: {:?}", first, id);
        let language =
            if let Ok(lang) = self.clone().get_translation_for_language() {
                lang
            }
            else {
                return Err(ResponderError::NotFound(
                    "translation data".to_string(),
                ));
            };

        let translated_vec = serde_json::from_str::<Vec<Aoe2netStringObj>>(
            &serde_json::to_string(&language[first]).unwrap_or_else(|_| {
                panic!(format!(
                    "Conversion of language [{:?}] to string failed.",
                    first.to_string(),
                ))
            }),
        )
        .expect("Conversion from translated string failed.");

        let mut translated_string: Option<String> = None;

        for obj_string in &translated_vec {
            if *obj_string.id() == id {
                translated_string = Some(obj_string.string().to_string())
            }
        }

        if translated_string.is_none() {
            return Err(ResponderError::TranslationPosError(
                format!("[{:?}]", first.to_string(),),
                id,
            ));
        }

        Ok(translated_string.unwrap())
    }

    /// Returns the `map_type` id from a match
    /// used only for translation purposes to get the id
    /// for the translation lookup function
    ///
    /// # Errors
    /// Will return an error if the `last_match` could not be found
    pub fn get_map_type_id(&self) -> Result<usize> {
        self.aoe2net.player_last_match.as_ref().map_or_else(
            || Err(ResponderError::NotFound("map type".to_string())),
            |val| {
                Ok(val["last_match"]["map_type"]
                    .to_string()
                    .parse::<usize>()?)
            },
        )
    }

    /// Returns the `game_type` id from a match
    /// used only for translation purposes to get the id
    /// for the translation lookup function
    ///
    /// # Errors
    /// Will return an error if the `last_match` could not be found
    pub fn get_game_type_id(&self) -> Result<usize> {
        self.aoe2net.player_last_match.as_ref().map_or_else(
            || Err(ResponderError::NotFound("game type".to_string())),
            |val| {
                Ok(val["last_match"]["game_type"]
                    .to_string()
                    .parse::<usize>()?)
            },
        )
    }

    /// Returns the server location of the match while looking it up from
    /// aoe2.net data and matching against [Server]
    ///
    /// # Errors
    /// Will return an error if the `last_match` could not be found
    pub fn get_server_location(&self) -> Result<Server> {
        self.aoe2net.player_last_match.as_ref().map_or_else(
            || Err(ResponderError::NotFound("server location".to_string())),
            |val| {
                let mut string = val["last_match"]["server"].to_string();
                string.retain(|char| char != '\\');
                string.retain(|char| char != '\"');

                Ok(match string.as_str() {
                    "australiasoutheast" => Server::Australia,
                    "brazilsouth" => Server::Brazil,
                    "ukwest" => Server::UK,
                    "westindia" => Server::India,
                    "southeastasia" => Server::SoutheastAsia,
                    "westeurope" => Server::WesternEurope,
                    "eastus" => Server::UsEast,
                    "koreacentral" => Server::Korea,
                    "westus2" => Server::UsWest,
                    _ => Server::NotFound,
                })
            },
        )
    }

    /// Looks up a player rating list in a [std::collections::HashMap] for
    /// `player_id` and returns a `serde_json::Value` with the `rating
    /// history` for that corresponding player
    ///
    /// # Arguments
    /// * `profile_id` - A string slice that contains the Aoe2.net profile ID of
    ///   a player
    ///
    /// # Errors
    /// Doesn't throw any errors, but returns an Option type that is `None` if
    /// the profile ID couldn't be found in the index
    #[must_use]
    pub fn lookup_player_rating_for_profile_id(
        &self,
        profile_id: &str,
    ) -> Option<serde_json::Value> {
        match self.aoe2net.rating_history.get(profile_id) {
            Some(rating_history) => Some(rating_history.clone()),
            None => None,
        }
    }

    /// Looks up a leaderboard entry in a [std::collections::HashMap] for
    /// `player_id` and returns a `serde_json::Value` with the `leaderboard` for
    /// that corresponding player
    ///
    /// # Arguments
    /// * `profile_id` - A string slice that contains the Aoe2.net profile ID of
    ///   a player
    ///
    /// # Errors
    /// Doesn't throw any errors, but returns an Option type that is `None` if
    /// the profile ID couldn't be found in the index
    #[must_use]
    pub fn lookup_leaderboard_for_profile_id(
        &self,
        profile_id: &str,
    ) -> Option<serde_json::Value> {
        match self.aoe2net.leaderboard.get(profile_id) {
            Some(leaderboard) => Some(leaderboard.clone()),
            None => None,
        }
    }

    /// Print debug information of this data structure
    pub fn print_debug_information(&self) {
        debug!("DEBUG: {:#?}", self)
    }

    /// Write the data in RON format to a file for debugging purposes
    pub fn export_data_to_file(&self) {
        let ron_config = PrettyConfig::new()
            .with_depth_limit(8)
            .with_separate_tuple_members(true)
            .with_enumerate_arrays(true)
            .with_indentor("\t".to_owned());

        // Open the file in writable mode with buffer.
        let file = fs::File::create("logs/match_data_responses.ron").unwrap();
        let writer = BufWriter::new(file);

        // Write data to file
        to_writer_pretty(writer, &self, ron_config)
            .expect("Unable to write data");
    }

    /// Creates a new [`MatchDataResponses`] struct by executing requests for
    /// match data
    ///
    /// # Arguments
    /// * `par` - holds a [MatchInfoRequest] that contains all the request
    ///   parameters to our backend
    /// * `client` - holds a clone of a [reqwest::Client] for connection pooling
    ///   purposes
    /// * `in_memory_db` - holds [InMemoryDb] wrapped inside an [Arc] with a
    ///   [Mutex] due to threading
    ///
    /// # Errors
    pub async fn new_with_match_data(
        par: MatchInfoRequest,
        client: reqwest::Client,
        in_memory_db: Arc<Mutex<InMemoryDb>>,
    ) -> Result<MatchDataResponses> {
        let mut language: String =
            STANDARD.get(&"language").unwrap().to_string();
        let mut game: String = STANDARD.get(&"game").unwrap().to_string();

        // API root for aoe2net
        let root = "https://aoe2.net/api";

        let mut db_cloned: InMemoryDb;
        {
            // Just clone and drop the lock
            db_cloned = in_memory_db.lock().await.clone();
        }
        // Set `language` to Query value if specified
        if let Some(lang) = par.language {
            language = lang;
        }

        // Set `game` to Query value if specified
        if let Some(val) = par.game {
            game = val;
        }

        // Include github response
        let mut responses = MatchDataResponses {
            db: db_cloned.retain_language(&language),
            ..Default::default()
        };

        // GET `PlayerLastMatch` data
        responses.aoe2net.player_last_match = {
            util::build_api_request(
                client.clone(),
                root,
                "player/lastmatch",
                vec![
                    ("game".to_string(), game.clone()),
                    (par.id_type.clone(), par.id_number.clone()),
                ],
            )
            .execute()
            .await?
        };

        // Get `leaderboard_id` for future requests
        let leaderboard_id = responses.get_leaderboard_id_for_request()?;

        // Get all players from `LastMatch` response
        responses.aoe2net.players_temp =
            responses.parse_all_players::<Vec<aoe2net::Player>>()?;

        for player in &responses.aoe2net.players_temp {
            // Get Rating `HistoryData` for each player
            let req_rating = util::build_api_request(
                client.clone(),
                root,
                "player/ratinghistory",
                vec![
                    ("game".to_string(), game.clone()),
                    ("profile_id".to_string(), player.profile_id.to_string()),
                    ("leaderboard_id".to_string(), leaderboard_id.clone()),
                    ("count".to_string(), "1".to_string()),
                ],
            );

            // GET `Leaderboard` data
            let req_lead = util::build_api_request(
                client.clone(),
                root,
                "leaderboard",
                vec![
                    ("game".to_string(), game.clone()),
                    ("profile_id".to_string(), player.profile_id.to_string()),
                    ("leaderboard_id".to_string(), leaderboard_id.clone()),
                ],
            );

            responses.aoe2net.rating_history.insert(
                player.profile_id.to_string(),
                req_rating.execute().await?,
            );

            responses.aoe2net.leaderboard.insert(
                player.profile_id.to_string(),
                req_lead.execute().await?,
            );
        }
        Ok(responses)
    }
}
