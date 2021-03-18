//! Datastructures of the aoe2net API (incomplete)

use serde::{
    Deserialize,
    Serialize,
};

use serde_json::Value as JsonValue;

/// Convenience datastructure for the `RatingHistory` endpoint to
/// easily parse the data into our struct
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
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
    pub profile_id: JsonValue,
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
