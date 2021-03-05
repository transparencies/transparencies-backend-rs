//! Wrapper structs for usages within the in-memory DB

use serde::Serialize;
use std::collections::HashMap;

use crate::domain::types::aoe2net;

use super::InMemoryDb;

#[derive(Debug, Clone, Default, Serialize)]
pub struct MatchDataResponses {
    pub aoe2net: Aoe2NetResponses,
    pub db: InMemoryDb,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Aoe2NetResponses {
    pub player_last_match: Option<serde_json::Value>,
    pub leaderboard: HashMap<String, serde_json::Value>,
    pub rating_history: HashMap<String, serde_json::Value>,
    pub players_temp: Vec<aoe2net::Player>,
}
