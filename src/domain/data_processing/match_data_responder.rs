use crate::domain::types::{
    aoc_ref::RefDataLists,
    api::{
        match_info_response::*,
        MatchInfoRequest,
        MatchInfoResult,
    },
    requests::*,
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
use std::{
    fs,
    io::BufWriter,
    sync::Arc,
    time::Duration,
};
use tokio::sync::Mutex;

impl MatchDataResponses {
    /// Return serde_json::Value for `leaderboard_id` for future requests
    pub fn get_leaderboard_id_for_request(&self) -> String {
        let (_response_name, values) = self
            .aoe2net
            .get_key_value("player_last_match")
            .expect("PlayerLastMatch information must not be missing.");

        println!("player_last_match: {:?}", values);

        values["last_match"]["leaderboard_id"].to_string()
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
        ref_data: Arc<Mutex<RefDataLists>>,
    ) -> MatchDataResponses {
        let mut api_requests: Vec<(String, ApiRequest)> = Vec::with_capacity(5);
        let mut responses = MatchDataResponses::default();

        // GET `PlayerLastMatch` data
        let last_match_request = ApiRequest::builder()
            .client(client.clone())
            .root("https://aoe2.net/api")
            .endpoint("player/lastmatch")
            .query(vec![
                ("game".to_string(), "aoe2de".to_string()),
                (par.id_type.clone(), par.id_number.clone()),
            ])
            .build();

        responses.aoe2net.insert(
            "player_last_match".to_string(),
            last_match_request.execute().await.unwrap(),
        );

        // Get `leaderboard_id` for future requests
        let leaderboard_id =
            responses.get_leaderboard_id_for_request().to_string();

        // GET `Leaderboard` data
        api_requests.push((
            "leaderboard".to_string(),
            ApiRequest::builder()
                .client(client.clone())
                .root("https://aoe2.net/api")
                .endpoint("leaderboard")
                .query(vec![
                    ("game".to_string(), "aoe2de".to_string()),
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
                    ("game".to_string(), "aoe2de".to_string()),
                    (par.id_type.clone(), par.id_number.clone()),
                    ("leaderboard_id".to_string(), leaderboard_id),
                    ("count".to_string(), "1".to_string()),
                ])
                .build(),
        ));

        for (response_name, req) in &api_requests {
            responses.aoe2net.insert(
                response_name.to_string(),
                req.execute().await.unwrap(),
            );
        }

        // Include github response
        responses.github = ref_data.lock().await.clone();

        responses
    }
}
