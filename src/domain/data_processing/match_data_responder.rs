use crate::domain::types::{
    aoc_ref::{
        players,
        RefDataLists,
    },
    aoe2net,
    api::{
        match_info_response::*,
        MatchInfoRequest,
        MatchInfoResult,
    },
    requests::ApiRequest,
    InMemoryDb,
    MatchDataResponses,
};
use aoe2net::Aoe2netStringObj;
use log::{
    debug,
    error,
    info,
    trace,
    warn,
};
use ron::ser::{
    to_writer_pretty,
    PrettyConfig,
};
use serde_json::Value;
use std::{
    collections::HashMap,
    fs,
    io::BufWriter,
    result,
    str::FromStr,
    sync::Arc,
    time::Duration,
};
use tokio::sync::Mutex;

use super::error::ResponderError;

use crate::{
    STANDARD_GAME,
    STANDARD_LANGUAGE,
};

type Result<T> = result::Result<T, ResponderError>;

impl MatchDataResponses {
    /// Return `serde_json::Value` for `leaderboard_id` for future requests
    pub fn get_leaderboard_id_for_request(&self) -> Result<String> {
        self.aoe2net.player_last_match.as_ref().map_or_else(
            || Err(ResponderError::NotFound("leaderboard_id".to_string())),
            |val| Ok(val["last_match"]["leaderboard_id"].to_string()),
        )
    }

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

    pub fn get_number_of_players(&self) -> Result<String> {
        self.aoe2net.player_last_match.as_ref().map_or_else(
            || Err(ResponderError::NotFound("number of players".to_string())),
            |val| Ok(val["last_match"]["num_players"].to_string()),
        )
    }

    pub fn get_finished_time(&self) -> Result<String> {
        self.aoe2net.player_last_match.as_ref().map_or_else(
            || Err(ResponderError::NotFound("finished time".to_string())),
            |val| Ok(val["last_match"]["finished"].to_string()),
        )
    }

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
                    "translation file".to_string(),
                ));
            };

        let translated_vec = serde_json::from_str::<Vec<Aoe2netStringObj>>(
            &serde_json::to_string(&language[first]).expect(
                &format!(
                    "Conversion of language[{:?}] to string failed.",
                    first.to_string(),
                )
                .to_string(),
            ),
        )
        .expect("Conversion from translated string failed.");

        let mut translated_string: Option<String> = None;

        for obj_string in translated_vec.iter() {
            if *obj_string.id() == id {
                translated_string = Some(obj_string.string().to_string())
            }
        }

        if let None = translated_string {
            return Err(ResponderError::TranslationPosError(
                format!("[{:?}]", first.to_string(),),
                id,
            ));
        }

        Ok(translated_string.unwrap())
    }

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

    pub fn get_server_location(&self) -> Result<String> {
        self.aoe2net.player_last_match.as_ref().map_or_else(
            || Err(ResponderError::NotFound("server location".to_string())),
            |val| Ok(val["last_match"]["server"].to_string()),
        )
    }

    // TODO Generic function stuff
    // pub fn get_match_data<T>(&self, obj: &str, field: &str) -> Result<T>
    // where
    // T: FromStr,
    //  {
    // TODO: Leaderboard
    //     self.aoe2net.leaderboard.as_ref().map_or_else(
    //         || Err(ResponderError::NotFound(concat!(obj, '-', field))),
    //         |val| Ok(val[obj][field].to_string().parse::<T>()?),
    //     )
    // }

    /// Lookup player rating list for `player_id` and return
    /// `serde_json::Value`
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

    /// Lookup leaderboard entry for `player_id` and return
    /// `serde_json::Value`
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

    pub fn print_debug_information(&self) {
        debug!("DEBUG: {:#?}", self)
    }

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

    /// Create new [`MatchDataResponses`] struct by calling for match data
    pub async fn new_with_match_data(
        par: MatchInfoRequest,
        client: reqwest::Client,
        in_memory_db: Arc<Mutex<InMemoryDb>>,
    ) -> Result<MatchDataResponses> {
        let mut language: String = STANDARD_LANGUAGE.to_string();
        let mut game: String = STANDARD_GAME.to_string();

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
        let last_match_request = ApiRequest::builder()
            .client(client.clone())
            .root("https://aoe2.net/api")
            .endpoint("player/lastmatch")
            .query(vec![
                ("game".to_string(), game.clone()),
                (par.id_type.clone(), par.id_number.clone()),
            ])
            .build();

        responses.aoe2net.player_last_match =
            last_match_request.execute().await?;

        // Get `leaderboard_id` for future requests
        let leaderboard_id = responses.get_leaderboard_id_for_request()?;

        // Get all players from `LastMatch` response
        responses.aoe2net.players_temp =
            responses.parse_all_players::<Vec<aoe2net::Player>>()?;

        for player in &responses.aoe2net.players_temp {
            // Get Rating `HistoryData` for each player
            let req_rating = ApiRequest::builder()
                .client(client.clone())
                .root("https://aoe2.net/api")
                .endpoint("player/ratinghistory")
                .query(vec![
                    ("game".to_string(), game.clone()),
                    ("profile_id".to_string(), player.profile_id.to_string()),
                    ("leaderboard_id".to_string(), leaderboard_id.clone()),
                    ("count".to_string(), "1".to_string()),
                ])
                .build();

            // GET `Leaderboard` data
            let req_lead = ApiRequest::builder()
                .client(client.clone())
                .root("https://aoe2.net/api")
                .endpoint("leaderboard")
                .query(vec![
                    ("game".to_string(), game.clone()),
                    ("profile_id".to_string(), player.profile_id.to_string()),
                    ("leaderboard_id".to_string(), leaderboard_id.clone()),
                ])
                .build();

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
