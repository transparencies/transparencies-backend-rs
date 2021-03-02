pub mod match_info_response;
pub use match_info_response::*;

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Deserialize)]
pub struct MatchInfoRequest {
    pub language: Option<String>,
    pub game: Option<String>,
    pub id_type: String,
    pub id_number: String,
}
