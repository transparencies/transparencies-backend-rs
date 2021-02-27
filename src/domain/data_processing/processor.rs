use serde_json::Value;

use crate::domain::{
    data_processing::MatchDataResponses,
    types::api::MatchInfoResult,
};

#[derive(Clone, Debug)]
pub struct MatchInfoProcessor {
    responses: MatchDataResponses,
}

impl MatchInfoProcessor {
    pub fn load_responses(responses: MatchDataResponses) -> Self {
        Self { responses }
    }

    /// Return serde_json::Value for `leaderboard_id` for future requests
    pub fn get_leaderboard_id_for_request(
        responses: &MatchDataResponses
    ) -> String {
        let (_response_name, values) = responses
            .aoe2net
            .get_key_value("player_last_match")
            .expect("PlayerLastMatch information must not be missing.");

        println!("player_last_match: {:?}", values);

        values["last_match"]["leaderboard_id"].to_string()
    }

    pub fn create_result() -> MatchInfoResult {
        todo!();
    }

    // Return Teams array
    // Return Ratings for `profile_id`
}
