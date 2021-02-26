pub mod match_info;

pub use match_info::*;

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Deserialize)]
pub struct MatchInfoRequest {
    pub id_type: String,
    pub id_number: String,
}
