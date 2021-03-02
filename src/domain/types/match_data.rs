use super::aoc_ref::RefDataLists;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

use super::InMemoryDb;

#[derive(Debug, Clone, Default, Serialize)]
pub struct MatchDataResponses {
    pub aoe2net: Aoe2NetResponses,
    pub db: InMemoryDb,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct Aoe2NetResponses {
    pub player_last_match: Option<serde_json::Value>,
    pub leaderboard: Option<serde_json::Value>,
    pub rating_history: Option<serde_json::Value>,
}
