//! Everything around [`MatchDataResponses`] resembles in here
//! Beware, there is a close connection to the [`super::MatchInfoProcessor`]
//! in many places

use aoe2net::types::{
    api::Player as aoe2net_Player,
    helper::{
        Aoe2netRequestType,
        Aoe2netStringObj,
        RecoveredRating,
    },
};

use crate::domain::{
    types::{
        api::{
            MatchInfoRequest,
            Rating,
            Server,
        },
        error::{
            ApiRequestError,
            ResponderError,
        },
        File,
        FileFormat,
        InMemoryDb,
        MatchDataResponses,
    },
    util,
};

use url::Url;

use ron::ser::{
    to_writer_pretty,
    PrettyConfig,
};
use serde_json::{
    json,
    Value as JsonValue,
};
use std::{
    fs,
    io::BufWriter,
    path::PathBuf,
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
    /// Return `String` for `leaderboard_id` for future requests
    ///
    /// # Errors
    /// Will return an error if the `leaderboard_id` could not be found
    pub fn get_leaderboard_id_from_request(
        &mut self,
        req_type: Aoe2netRequestType,
    ) -> Result<String> {
        match req_type {
            Aoe2netRequestType::LastMatch => {
                self.aoe2net.player_last_match.as_ref().map_or_else(
                    || {
                        Err(ResponderError::NotFound(
                            "Leaderboard ID in last_match response".to_string(),
                        ))
                    },
                    |val| Ok(val["last_match"]["leaderboard_id"].to_string()),
                )
            }
            Aoe2netRequestType::MatchId => {
                self.aoe2net.match_id.as_ref().map_or_else(
                    || {
                        Err(ResponderError::NotFound(
                            "Leaderboard ID in match_id response".to_string(),
                        ))
                    },
                    |val| Ok(val["leaderboard_id"].to_string()),
                )
            }
            _ => Err(ResponderError::InvalidReqType(req_type.to_string())),
        }
    }

    /// Parses all the players into a `type T`
    /// from the `last_match` response for convenience
    ///
    /// # Errors
    /// Will return an error if either the `players array` can not be found
    /// or will `panic!` if the deserialisation failed or the parsing of the
    /// `Player` struct failed
    pub fn parse_players_into<T>(
        &self,
        req_type: Aoe2netRequestType,
    ) -> Result<T>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        match req_type {
            Aoe2netRequestType::LastMatch => {
                self.aoe2net.player_last_match.as_ref().map_or_else(
                    || {
                        Err(ResponderError::NotFound(
                            "last_match players array".to_string(),
                        ))
                    },
                    |val| {
                        Ok(serde_json::from_str::<T>(&serde_json::to_string(
                            &val["last_match"]["players"],
                        )?)?)
                    },
                )
            }
            Aoe2netRequestType::MatchId => {
                self.aoe2net.match_id.as_ref().map_or_else(
                    || {
                        Err(ResponderError::NotFound(
                            "match_id players array".to_string(),
                        ))
                    },
                    |val| {
                        Ok(serde_json::from_str::<T>(&serde_json::to_string(
                            &val["players"],
                        )?)?)
                    },
                )
            }
            _ => Err(ResponderError::InvalidReqType(req_type.to_string())),
        }
    }

    /// Returns the number of players from the `last_match` response
    ///
    /// # Errors
    /// Will return an error if the `last_match` could not be found
    pub fn get_player_amount(&self) -> Result<String> {
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
    pub fn get_finished_time(
        &self,
        req_type: Aoe2netRequestType,
    ) -> Result<String> {
        match req_type {
            Aoe2netRequestType::LastMatch => {
                self.aoe2net.player_last_match.as_ref().map_or_else(
                    || {
                        Err(ResponderError::NotFound(
                            "last_match finished time".to_string(),
                        ))
                    },
                    |val| Ok(val["last_match"]["finished"].to_string()),
                )
            }
            Aoe2netRequestType::MatchId => {
                self.aoe2net.match_id.as_ref().map_or_else(
                    || {
                        Err(ResponderError::NotFound(
                            "match_id finished time".to_string(),
                        ))
                    },
                    |val| Ok(val["finished"].to_string()),
                )
            }
            _ => Err(ResponderError::InvalidReqType(req_type.to_string())),
        }
    }

    /// Returns the rating type id for a match
    /// We use that to check if a game is rated/unrated
    /// or in which ladder to look for the Rating
    ///
    /// # Errors
    /// Will return an error if the `last_match` could not be found
    pub fn get_id_for_rating_type(
        &self,
        req_type: Aoe2netRequestType,
    ) -> Result<usize> {
        match req_type {
            Aoe2netRequestType::LastMatch => {
                self.aoe2net.player_last_match.as_ref().map_or_else(
                    || {
                        Err(ResponderError::NotFound(
                            "last match rating type".to_string(),
                        ))
                    },
                    |val| {
                        Ok(val["last_match"]["rating_type"]
                            .to_string()
                            .parse::<usize>()?)
                    },
                )
            }
            Aoe2netRequestType::MatchId => {
                self.aoe2net.match_id.as_ref().map_or_else(
                    || {
                        Err(ResponderError::NotFound(
                            "match id rating type".to_string(),
                        ))
                    },
                    |val| {
                        Ok(val["rating_type"].to_string().parse::<usize>()?)
                    },
                )
            }
            _ => Err(ResponderError::InvalidReqType(req_type.to_string())),
        }
    }

    /// Get a the country from the `leaderboard response` for a given player
    ///
    /// # Errors
    /// Won't throw an error, but `Option` is set to `None` resulting in an
    /// empty player name that is taken if the `looked_up_player` doesn't
    /// give any value
    #[must_use]
    pub fn get_country_code(
        looked_up_leaderboard: &(RecoveredRating, JsonValue)
    ) -> Option<String> {
        let (recover, value) = looked_up_leaderboard;

        if *recover == RecoveredRating::Original {
            Some(value["country"].to_string().to_lowercase()).map(
                |mut country| {
                    country = util::remove_escaping(country);
                    country
                },
            )
        }
        else {
            None
        }
    }

    /// Get a `Rating` datastructure from a `response` for a given player
    ///
    /// # Arguments
    /// * `looked_up_rating` - a [`serde_json::Value`] type that holds the
    ///   [`aoe2net::RatingHistory`] of an [`aoe2net::Player`]
    /// * `looked_up_leaderboard` - a [`serde_json::Value`] type that holds
    ///   Leaderboard data of an [`aoe2net::Player`]
    ///
    /// # Errors
    /// Function will throw errors in cases the deserialisation and conversion
    /// to the corresponding types is not successful
    pub fn create_rating(
        looked_up_rating: &JsonValue,
        looked_up_leaderboard: &(RecoveredRating, JsonValue),
    ) -> Result<Rating> {
        let player_rating = match looked_up_leaderboard {
            (RecoveredRating::Original, leaderboard) => Rating::builder()
                .mmr(serde_json::from_str::<u32>(&serde_json::to_string(
                    &looked_up_rating["rating"],
                )?)?)
                .rank(serde_json::from_str::<u64>(&serde_json::to_string(
                    &leaderboard["rank"],
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
                .highest_mmr(serde_json::from_str::<u32>(
                    &serde_json::to_string(&leaderboard["highest_rating"])?,
                )?)
                .build(),
            (RecoveredRating::Recovered, leaderboard) => Rating::builder()
                .mmr(serde_json::from_str::<u32>(&serde_json::to_string(
                    &looked_up_rating["rating"],
                )?)?)
                .rank(serde_json::from_str::<u64>(&serde_json::to_string(&{
                    let rank = if leaderboard["rank"] == JsonValue::Null {
                        json![0]
                    }
                    else {
                        leaderboard["rank"].clone()
                    };
                    rank
                })?)?)
                .wins(serde_json::from_str::<u64>(&serde_json::to_string(
                    &looked_up_rating["num_wins"],
                )?)?)
                .losses(serde_json::from_str::<u64>(&serde_json::to_string(
                    &looked_up_rating["num_losses"],
                )?)?)
                .streak(serde_json::from_str::<i32>(&serde_json::to_string(
                    &looked_up_rating["streak"],
                )?)?)
                .highest_mmr(serde_json::from_str::<u32>(
                    &serde_json::to_string(&{
                        let rating_high = if leaderboard["highest_rating"]
                            == JsonValue::Null
                        {
                            json![0]
                        }
                        else {
                            leaderboard["highest_rating"].clone()
                        };
                        rating_high
                    })?,
                )?)
                .build(),
        };

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
    pub fn get_translation_for_language(&mut self) -> Result<JsonValue> {
        let mut translation: Option<JsonValue> = None;

        trace!(
            "Length of self.db.aoe2net_languages is: {:?}",
            self.db.aoe2net_languages.len()
        );

        if self.db.aoe2net_languages.len() == 1 {
            for (language, translation_value) in
                self.db.aoe2net_languages.clone()
            {
                translation = Some(translation_value);
                trace!("Translation that was used: {:?}", language);
            }
        }
        else {
            return Err(ResponderError::TranslationHasBeenMoved);
        }
        Ok(translation.expect("Translation should never be None value."))
    }

    /// Returns the corresponding `String` for our convenience
    /// by searching through [`Aoe2netStringObj`]'s field `id`
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
    /// [`Self::get_translation_for_language()`].
    ///
    /// # Panics
    /// Panics if the conversion to a string failed
    // TODO: Get rid of panic and handle gracefully
    pub fn lookup_string_for_id(
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
                panic!(
                    "Conversion of language [{:?}] to string failed.",
                    first.to_string(),
                )
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
    pub fn get_map_type_id(
        &self,
        req_type: Aoe2netRequestType,
    ) -> Result<usize> {
        match req_type {
            Aoe2netRequestType::LastMatch => {
                self.aoe2net.player_last_match.as_ref().map_or_else(
                    || {
                        Err(ResponderError::NotFound(
                            "last_match map type".to_string(),
                        ))
                    },
                    |val| {
                        Ok(val["last_match"]["map_type"]
                            .to_string()
                            .parse::<usize>()?)
                    },
                )
            }
            Aoe2netRequestType::MatchId => {
                self.aoe2net.match_id.as_ref().map_or_else(
                    || {
                        Err(ResponderError::NotFound(
                            "match_id map type".to_string(),
                        ))
                    },
                    |val| Ok(val["map_type"].to_string().parse::<usize>()?),
                )
            }
            _ => Err(ResponderError::InvalidReqType(req_type.to_string())),
        }
    }

    /// Returns the `game_type` id from a match
    /// used only for translation purposes to get the id
    /// for the translation lookup function
    ///
    /// # Errors
    /// Will return an error if the `last_match` could not be found
    pub fn get_game_type_id(
        &self,
        req_type: Aoe2netRequestType,
    ) -> Result<usize> {
        match req_type {
            Aoe2netRequestType::LastMatch => {
                self.aoe2net.player_last_match.as_ref().map_or_else(
                    || {
                        Err(ResponderError::NotFound(
                            "last_match game type".to_string(),
                        ))
                    },
                    |val| {
                        Ok(val["last_match"]["game_type"]
                            .to_string()
                            .parse::<usize>()?)
                    },
                )
            }
            Aoe2netRequestType::MatchId => {
                self.aoe2net.match_id.as_ref().map_or_else(
                    || {
                        Err(ResponderError::NotFound(
                            "match_id game type".to_string(),
                        ))
                    },
                    |val| Ok(val["game_type"].to_string().parse::<usize>()?),
                )
            }
            _ => Err(ResponderError::InvalidReqType(req_type.to_string())),
        }
    }

    /// Returns the server location of the match while looking it up from
    /// aoe2.net data and matching against [Server]
    ///
    /// # Errors
    /// Will return an error if the `last_match` could not be found
    pub fn get_server_location(
        &self,
        req_type: Aoe2netRequestType,
    ) -> Result<Server> {
        let mut server = String::new();

        match req_type {
            Aoe2netRequestType::LastMatch => {
                if let Some(val) = self.aoe2net.player_last_match.as_ref() {
                    server = val["last_match"]["server"].to_string();
                }
            }
            Aoe2netRequestType::MatchId => {
                if let Some(val) = self.aoe2net.match_id.as_ref() {
                    server = val["server"].to_string();
                }
            }
            _ => {
                return Err(ResponderError::InvalidReqType(
                    req_type.to_string(),
                ))
            }
        };

        server = util::remove_escaping(server);

        let server_result = match server.as_str() {
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
        };

        Ok(server_result)
    }

    /// Looks up a player rating list in a [`dashmap::DashMap`] for
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
    ) -> Option<JsonValue> {
        self.aoe2net
            .rating_history
            .get(profile_id)
            .map(|val| val.value().clone())
    }

    /// Looks up a leaderboard entry in a [`dashmap::DashMap`] for
    /// `player_id` and returns a [`serde_json::Value`] with the `leaderboard`
    /// for that corresponding player
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
    ) -> Option<JsonValue> {
        self.aoe2net
            .leaderboard
            .get(profile_id)
            .map(|val| val.value().clone())
    }

    /// Print debug information of this data structure
    pub fn print_debug_information(&self) {
        debug!("DEBUG: {:#?}", self)
    }

    /// Write the data in RON format to a file for debugging purposes
    ///
    /// # Panics
    /// Will panic if data cannot be written or file can not be created in
    /// Filesystem
    pub fn export_to_file(&self) {
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
    /// match data and putting them into a [`dashmap::DashMap`]
    ///
    /// # Arguments
    /// * `par` - holds a [`MatchInfoRequest`] that contains all the request
    ///   parameters to our backend
    /// * `client` - holds a clone of a [`reqwest::Client`] for connection
    ///   pooling purposes
    /// * `in_memory_db` - holds [`InMemoryDb`] wrapped inside an [`Arc`] with a
    ///   [`Mutex`] due to threading
    ///
    /// # Errors
    /// This function may throw errors in the form of [`reqwest::Error`] when
    /// requests fail
    ///
    /// # Panics
    /// Function could panic if the [`dashmap::DashMap`] of static
    /// global variable [`static@crate::STANDARD`] delivers `None`
    #[allow(clippy::too_many_lines)]
    pub async fn with_match_data(
        par: MatchInfoRequest,
        client: reqwest::Client,
        in_memory_db: Arc<Mutex<InMemoryDb>>,
        export_path: Option<PathBuf>,
        root: Url,
    ) -> Result<MatchDataResponses> {
        let mut language: String =
            (*STANDARD.get(&"language").unwrap()).to_string();
        let mut game: String = (*STANDARD.get(&"game").unwrap()).to_string();

        #[allow(unused_assignments)]
        let mut db_cloned = InMemoryDb::default();
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
            db: db_cloned.retain_only_requested_language(&language),
            ..MatchDataResponses::default()
        };

        match par.id_type.as_str() {
            "steam_id" | "profile_id" => {
                // GET `PlayerLastMatch` data
                let match_data_response = util::build_api_request(
                    client.clone(),
                    root.clone(),
                    "player/lastmatch",
                    vec![
                        ("game".to_string(), game.clone()),
                        (par.id_type.clone(), par.id_number.clone()),
                    ],
                )
                .execute::<JsonValue>()
                .await;

                match match_data_response {
                    Err(err) => match err {
                        ApiRequestError::NotFoundResponse {
                            root: _,
                            endpoint: _,
                            query: _,
                        } => return Err(ResponderError::LastMatchNotFound),
                        _ => {
                            return Err(ResponderError::OtherApiRequestError(
                                err,
                            ))
                        }
                    },
                    Ok(value) => {
                        responses.aoe2net.player_last_match = Some(value);
                        // Get `leaderboard_id` for future requests
                        responses.aoe2net.leaderboard_id =
                            Some(responses.get_leaderboard_id_from_request(
                                Aoe2netRequestType::LastMatch,
                            )?);

                        // Get all players from `LastMatch` response
                        responses.aoe2net.players_temp = responses
                            .parse_players_into::<Vec<aoe2net_Player>>(
                                Aoe2netRequestType::LastMatch,
                            )?;
                    }
                }

                if let Some(mut path) = export_path.clone() {
                    path.push("aoe2net");
                    util::export_to_json(
                        &File {
                            name: "last_match".to_string(),
                            ext: FileFormat::Json,
                        },
                        &path,
                        &responses
                            .clone()
                            .aoe2net
                            .player_last_match
                            .map_or(JsonValue::Null, |x| x),
                    )
                }
            }
            "match_id" => {
                // GET `MatchID` data
                responses.aoe2net.match_id = Some(
                    util::build_api_request(
                        client.clone(),
                        root.clone(),
                        "match",
                        vec![(par.id_type.clone(), par.id_number.clone())],
                    )
                    .execute::<JsonValue>()
                    .await?,
                );

                // Get `leaderboard_id` for future requests
                responses.aoe2net.leaderboard_id =
                    Some(responses.get_leaderboard_id_from_request(
                        Aoe2netRequestType::MatchId,
                    )?);

                // Get all players from `LastMatch` response
                responses.aoe2net.players_temp = responses
                    .parse_players_into::<Vec<aoe2net_Player>>(
                        Aoe2netRequestType::MatchId,
                    )?;
            }
            _ => {
                return Err(ResponderError::InvalidIdType(
                    std::borrow::Cow::Owned(par.id_type),
                ))
            }
        }

        for player in &responses.aoe2net.players_temp {
            // Get Rating `HistoryData` for each player
            let req_rating = util::build_api_request(
                client.clone(),
                root.clone(),
                "player/ratinghistory",
                vec![
                    ("game".to_string(), game.clone()),
                    ("profile_id".to_string(), player.profile_id.to_string()),
                    (
                        "leaderboard_id".to_string(),
                        responses.aoe2net.leaderboard_id.clone().unwrap(),
                    ),
                    ("count".to_string(), "1".to_string()),
                ],
            );

            // GET `Leaderboard` data
            let req_lead = util::build_api_request(
                client.clone(),
                root.clone(),
                "leaderboard",
                vec![
                    ("game".to_string(), game.clone()),
                    ("profile_id".to_string(), player.profile_id.to_string()),
                    (
                        "leaderboard_id".to_string(),
                        responses.aoe2net.leaderboard_id.clone().unwrap(),
                    ),
                ],
            );

            let rating_response: JsonValue = req_rating.execute().await?;
            let leaderboard_response: JsonValue = req_lead.execute().await?;
            let mut leaderboard_recovery: Option<JsonValue> = None;

            if leaderboard_response["count"].to_string() == 0.to_string() {
                // GET `Rating` data as recovery data
                let req_lead_rating = util::build_api_request(
                    client.clone(),
                    root.clone(),
                    "player/rating",
                    vec![
                        ("game".to_string(), game.clone()),
                        (
                            "profile_id".to_string(),
                            player.profile_id.to_string(),
                        ),
                        (
                            "leaderboard_id".to_string(),
                            responses.aoe2net.leaderboard_id.clone().unwrap(),
                        ),
                    ],
                );

                leaderboard_recovery = Some(req_lead_rating.execute().await?);
            }

            if let Some(mut path) = export_path.clone() {
                // TODO: Do requests just one time in export path
                path.push("aoe2net");

                if leaderboard_recovery.is_some() {
                    util::export_to_json(
                        &File {
                            name: format!("{:?}_recovery", player.profile_id),
                            ext: FileFormat::Json,
                        },
                        &{
                            let mut p = path.clone();
                            p.push("leaderboard");
                            p
                        },
                        &rating_response,
                    );
                }

                util::export_to_json(
                    &File {
                        name: format!("{:?}", player.profile_id),
                        ext: FileFormat::Json,
                    },
                    &{
                        let mut p = path.clone();
                        p.push("rating_history");
                        p
                    },
                    &rating_response,
                );
                util::export_to_json(
                    &File {
                        name: format!("{:?}", player.profile_id),
                        ext: FileFormat::Json,
                    },
                    &{
                        let mut p = path.clone();
                        p.push("leaderboard");
                        p
                    },
                    &leaderboard_response,
                );
            }

            responses
                .aoe2net
                .rating_history
                .insert(player.profile_id.to_string(), rating_response);

            responses
                .aoe2net
                .leaderboard
                .insert(player.profile_id.to_string(), leaderboard_response);

            if leaderboard_recovery.is_some() {
                responses.aoe2net.leaderboard.insert(
                    format!("{}_recovery", &player.profile_id.to_string()),
                    leaderboard_recovery.unwrap(),
                );
            }
        }

        Ok(responses)
    }
}
