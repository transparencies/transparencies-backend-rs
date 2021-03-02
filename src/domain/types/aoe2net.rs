use serde::{
    Deserialize,
    Serialize,
};

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

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Players {
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
