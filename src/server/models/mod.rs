// pub mod match_info;

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Deserialize)]
pub struct MatchInfoRequest {
    pub id_type: Option<String>,
    pub id_number: Option<String>,
}

enum MatchInfoRequestType {
    SteamId((String, String)),
    AoeNetProfile((String, String)),
    Invalid,
}
