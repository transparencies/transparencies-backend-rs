use super::aoc_ref::RefDataLists;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Default, Serialize)]
pub struct MatchDataResponses {
    pub aoe2net: HashMap<String, serde_json::Value>,
    pub github: RefDataLists,
}
