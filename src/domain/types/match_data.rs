//! Wrapper structs for usages within the in-memory DB

use aoe2net::types::api::Player as aoe2net_Player;
use dashmap::DashMap;
use serde::Serialize;
use serde_json::Value as JsonValue;

use super::InMemoryDb;

#[derive(Debug, Clone, Default, Serialize)]
pub struct MatchDataResponses {
    pub aoe2net: Aoe2NetResponses,
    pub db: InMemoryDb,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Aoe2NetResponses {
    pub leaderboard_id: Option<String>,
    pub player_last_match: Option<JsonValue>,
    pub leaderboard: DashMap<String, JsonValue>,
    pub rating_history: DashMap<String, JsonValue>,
    pub players_temp: Vec<aoe2net_Player>,
    pub match_id: Option<JsonValue>,
}
