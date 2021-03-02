pub mod match_info_response;
pub use match_info_response::*;

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Deserialize)]
pub struct MatchInfoRequest {
    pub id_type: String,
    pub id_number: String,
    pub language: String,
    pub game: String,
}

// pub enum Language {
//     En = "en",
//     De = "de",
//     El = "el",
//     Es = "es",
//     EsMx= "es-MX",
//     Fr = "fr",
//     Hi = "hi",
//     It = "it",
//     Ja = "ja",
//     Ko = "ko",
//     Ms = "ms",
//     Nl = "nl",
//     Pt = "pt",
//     Ru = "ru",
//     Tr = "tr",
//     Vi = "vi",
//     Zh = "zh",
//     ZhTw = "zh-TW",
// }
