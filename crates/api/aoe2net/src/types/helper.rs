//! Helper datastructures to easily handle some stuff more easily

use serde::{
    Deserialize,
    Serialize,
};

use derive_getters::Getters;
use displaydoc::Display;

#[derive(Clone, Copy, PartialEq)]
pub enum RecoveredRating {
    Recovered,
    Original,
}

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

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Display)]
#[non_exhaustive]
pub enum Aoe2netRequestType {
    /// Last_Match
    LastMatch,
    /// Match_ID
    MatchId,
}

impl Default for Aoe2netRequestType {
    fn default() -> Self {
        Aoe2netRequestType::LastMatch
    }
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

impl Default for Aoe2netIdType {
    fn default() -> Self {
        Aoe2netIdType::Profile
    }
}
