//! The data structures we return to the client
//! when calling the `match_info` endpoint

use serde::{
    Deserialize,
    Serialize,
};
use std::collections::HashMap;
use typed_builder::TypedBuilder;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
enum MatchSize {
    G1v1,
    G2v2,
    G3v3,
    G4v4,
    Custom,
    NoGame,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
enum MatchType {
    RM,
    DM,
    CSTM,
}

type Time = String;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
enum MatchStatus {
    Running,
    Finished(Time),
}

type ErrorMessage = String;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MatchInfoResult {
    pub match_info: MatchInfo,
    pub error_message: Option<ErrorMessage>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct MatchInfo {
    match_type: MatchType,
    match_size: MatchSize,
    match_status: MatchStatus,
    map_name: String,
    teams: Teams,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Players(pub Vec<PlayersRaw>);

#[allow(clippy::empty_enum)]
#[derive(Clone, TypedBuilder, Debug, PartialEq, Serialize)]
pub struct PlayersRaw {
    rating: Rating,
    player_number: i64,
    name: String,
    country: String,
    civilisation: Civilisation,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Teams(Vec<TeamsRaw>);

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct TeamsRaw {
    players: Players,
    team_number: i64,
    team_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Rating {
    mmr: u32,
    rank: u64,
    wins: u64,
    losses: u64,
    win_rate: f32,
    streak: i32,
    highest_average_mmr: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum Civilisation {
    Aztecs = 0,
    Berbers,
    Britons,
    Bulgarians,
    Burgundians,
    Burmese,
    Byzantines,
    Celts,
    Chinese,
    Cumans,
    Ethiopians,
    Franks,
    Goths,
    Huns,
    Incas,
    Indians,
    Italians,
    Japanese,
    Khmer,
    Koreans,
    Lithuanians,
    Magyars,
    Malay,
    Malians,
    Mayans,
    Mongols,
    Persians,
    Portuguese,
    Saracens,
    Sicilians,
    Slavs,
    Spanish,
    Tatars,
    Teutons,
    Turks,
    Vietnamese,
    Vikings,
}

// #[test]
// fn ensure_match_info_roundtrips() {
//     let t = <MatchInfo>::default();
//     let j = serde_json::to_string(&t).unwrap();
//     let r: MatchInfo = serde_json::from_str(&j).unwrap();
//     assert_eq!(t, r);
// }

// #[test]
// fn ensure_match_info_from_sample() {
//     let sample = r#"
//
//    TODO: Include sample for testing!
//
// "#;

//     let _: MatchInfo = serde_json::from_str(&sample).unwrap();
// }
