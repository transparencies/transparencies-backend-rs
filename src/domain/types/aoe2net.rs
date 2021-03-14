//! Datastructures used to deal with the aoe2net API

use serde::{
    Deserialize,
    Serialize,
};

use serde_json::Value as JsonValue;

use derive_getters::Getters;
use displaydoc::Display;

#[derive(Clone, Copy, PartialEq)]
pub enum RecoveredRating {
    Recovered,
    Original,
}

/// Convenience datastructure for the `RatingHistory` endpoint to
/// easily parse the data into our struct
#[derive(
    Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize,
)]
pub struct RatingHistory {
    pub drops: i64,
    pub num_losses: i64,
    pub num_wins: i64,
    pub rating: i64,
    pub streak: i64,
    pub timestamp: i64,
}

/// Convenient `Player` datastructure for aoe2net part of the `last_match`
/// endpoint Used to easily deal with the assembly of the player vectors
#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Player {
    pub civ: JsonValue,
    #[serde(skip)]
    pub clan: JsonValue,
    pub color: JsonValue,
    pub country: JsonValue,
    #[serde(skip)]
    pub drops: JsonValue,
    #[serde(skip)]
    pub games: JsonValue,
    pub name: JsonValue,
    pub profile_id: u64,
    pub rating: JsonValue,
    #[serde(skip)]
    pub rating_change: JsonValue,
    pub slot: JsonValue,
    pub slot_type: JsonValue,
    #[serde(skip)]
    pub steam_id: JsonValue,
    #[serde(skip)]
    pub streak: JsonValue,
    pub team: i64,
    #[serde(skip)]
    pub wins: JsonValue,
    pub won: JsonValue,
}

/// Helper datastructure to easily parse parts of array
/// from aoe2net
#[derive(
    Default,
    Clone,
    Debug,
    Getters,
    PartialEq,
    PartialOrd,
    Ord,
    Eq,
    Serialize,
    Deserialize,
)]
pub struct Aoe2netStringObj {
    id: usize,
    string: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display)]
pub enum Aoe2netIdType {
    /// steam_id
    Steam,
    /// profile_id
    Profile,
    /// match_id
    Match,
}
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Display)]
#[non_exhaustive]
pub enum Aoe2netRequestType {
    /// Last_Match
    LastMatch,
    /// Match_ID
    MatchId,
}
