use crate::domain::types::{
    aoc_ref::{
        players,
        RefDataLists,
    },
    api::{
        match_info_response::*,
        MatchInfoRequest,
        MatchInfoResult,
    },
    requests::ApiRequest,
    InMemoryDb,
    MatchDataResponses,
};
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

    pub fn get_rating_type(&self) -> Result<String> {
        self.aoe2net.player_last_match.as_ref().map_or_else(
            || Err(ResponderError::NotFound("rating type".to_string())),
            |val| Ok(val["last_match"]["rating_type"].to_string()),
        )
    }

    pub fn get_server_location(&self) -> Result<String> {
        self.aoe2net.player_last_match.as_ref().map_or_else(
            || Err(ResponderError::NotFound("server location".to_string())),
            |val| Ok(val["last_match"]["server"].to_string()),
        )
    }

    pub fn get_current_rating(&self) -> Result<u32> {
        self.aoe2net.leaderboard.as_ref().map_or_else(
            || {
                Err(ResponderError::NotFound(
                    "rating in rating history".to_string(),
                ))
            },
            |val| {
                Ok(val["rating_history"]["rating"]
                    .to_string()
                    .parse::<u32>()?)
            },
        )
    }

    pub fn get_highest_rating(&self) -> Result<u32> {
        self.aoe2net.leaderboard.as_ref().map_or_else(
            || Err(ResponderError::NotFound("highest rating".to_string())),
            |val| {
                Ok(val["leaderboard"]["highest_rating"]
                    .to_string()
                    .parse::<u32>()?)
            },
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

    pub fn get_rank(&self) -> Result<u64> {
        self.aoe2net.leaderboard.as_ref().map_or_else(
            || Err(ResponderError::NotFound("rank".to_string())),
            |val| Ok(val["leaderboard"]["rank"].to_string().parse::<u64>()?),
        )
    }

    pub fn get_wins(&self) -> Result<u64> {
        self.aoe2net.leaderboard.as_ref().map_or_else(
            || Err(ResponderError::NotFound("wins".to_string())),
            |val| Ok(val["leaderboard"]["wins"].to_string().parse::<u64>()?),
        )
    }

    pub fn get_losses(&self) -> Result<u64> {
        self.aoe2net.leaderboard.as_ref().map_or_else(
            || Err(ResponderError::NotFound("losses".to_string())),
            |val| {
                Ok(val["leaderboard"]["losses"].to_string().parse::<u64>()?)
            },
        )
    }

    pub fn get_streak(&self) -> Result<i32> {
        self.aoe2net.leaderboard.as_ref().map_or_else(
            || Err(ResponderError::NotFound("streak".to_string())),
            |val| {
                Ok(val["rating_history"]["streak"]
                    .to_string()
                    .parse::<i32>()?)
            },
        )
    }

    pub fn get_country_from_leaderboard(&self) -> Result<String> {
        self.aoe2net.leaderboard.as_ref().map_or_else(
            || Err(ResponderError::NotFound("country".to_string())),
            |val| Ok(val["leaderboard"]["country"].to_string()),
        )
    }

    pub fn get_last_match_profile_id(&self) -> Result<String> {
        self.aoe2net.leaderboard.as_ref().map_or_else(
            || {
                Err(ResponderError::NotFound(
                    "Last match profile id".to_string(),
                ))
            },
            |val| Ok(val["last_match"]["profile_id"].to_string()),
        )
    }

    pub fn get_clan_from_leaderboard(&self) -> Result<String> {
        self.aoe2net.leaderboard.as_ref().map_or_else(
            || Err(ResponderError::NotFound("clan".to_string())),
            |val| Ok(val["leaderboard"]["clan"].to_string()),
        )
    }

    pub fn get_rating(&self) -> Result<serde_json::Value> {
        self.aoe2net.rating_history.as_ref().map_or_else(
            || Err(ResponderError::NotFound("rating history".to_string())),
            |val| Ok(val[0].clone()),
        )
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
        let mut api_requests: Vec<(String, ApiRequest)> = Vec::with_capacity(5);

        let mut language: String = STANDARD_LANGUAGE.to_string();
        let mut game: String = STANDARD_GAME.to_string();

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
            db: in_memory_db.lock().await.with_language(&language),
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

        // GET `Leaderboard` data
        api_requests.push((
            "leaderboard".to_string(),
            ApiRequest::builder()
                .client(client.clone())
                .root("https://aoe2.net/api")
                .endpoint("leaderboard")
                .query(vec![
                    ("game".to_string(), game.clone()),
                    (par.id_type.clone(), par.id_number.clone()),
                    ("leaderboard_id".to_string(), leaderboard_id.clone()),
                ])
                .build(),
        ));

        // GET `RatingHistory` data
        api_requests.push((
            "rating_history".to_string(),
            ApiRequest::builder()
                .client(client.clone())
                .root("https://aoe2.net/api")
                .endpoint("player/ratinghistory")
                .query(vec![
                    ("game".to_string(), game.clone()),
                    (par.id_type.clone(), par.id_number.clone()),
                    ("leaderboard_id".to_string(), leaderboard_id),
                    ("count".to_string(), "1".to_string()),
                ])
                .build(),
        ));

        for (response_name, req) in &api_requests {
            match response_name.as_str() {
                "leaderboard" => {
                    responses.aoe2net.leaderboard = req.execute().await?;
                }
                "rating_history" => {
                    responses.aoe2net.rating_history = req.execute().await?;
                }
                _ => {
                    return Err(ResponderError::RequestNotMatching {
                        name: response_name.to_string(),
                        req: req.clone(),
                    })
                }
            }
        }

        Ok(responses)
    }
}
