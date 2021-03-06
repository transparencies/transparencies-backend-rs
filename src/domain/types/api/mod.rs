//! Our API root module
pub mod match_info_response;
pub use match_info_response::*;

use serde::Deserialize;

/// Datastructure for an incoming `request` on our api
/// on the `matchinfo` endpoint
#[derive(Debug, Deserialize)]
pub struct MatchInfoRequest {
    /// Requested language (Optional), Standard value is "en"
    pub language: Option<String>,
    /// Requested game (Optional), Standard value is "aoe2de"
    pub game: Option<String>,
    /// Requested type of ID, possible values are ["steam_id", "profile_id"]
    pub id_type: String,
    /// The ID itself as a String
    pub id_number: String,
}
