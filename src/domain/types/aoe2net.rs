//! Datastructures used to deal with the aoe2net API

use serde::{
    Deserialize,
    Serialize,
};

use derive_getters::Getters;

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
    pub civ: i64,
    #[serde(skip)]
    pub clan: ::serde_json::Value,
    pub color: i64,
    pub country: ::serde_json::Value,
    #[serde(skip)]
    pub drops: ::serde_json::Value,
    #[serde(skip)]
    pub games: ::serde_json::Value,
    pub name: String,
    pub profile_id: i64,
    pub rating: i64,
    #[serde(skip)]
    pub rating_change: ::serde_json::Value,
    pub slot: i64,
    pub slot_type: i64,
    #[serde(skip)]
    pub steam_id: String,
    #[serde(skip)]
    pub streak: ::serde_json::Value,
    pub team: i64,
    #[serde(skip)]
    pub wins: ::serde_json::Value,
    pub won: bool,
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
