#![allow(clippy::used_underscore_binding)]
#![allow(clippy::empty_enum)]
//! The data structures we return to the client
//! when calling the `match_info` endpoint

use ron::ser::{
    to_writer_pretty,
    PrettyConfig,
};

use derive_getters::Getters;
use serde::Serialize;
use std::{
    fs,
    io::BufWriter,
};
use typed_builder::TypedBuilder;

/// An enum describing the different `MatchSizes` we support on our overlay
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum MatchSize {
    NoGame = -1,
    Custom = 0,
    G1v1 = 2,
    G2v2 = 4,
    G3v3 = 6,
    G4v4 = 8,
    G2v2v2,
    G2v2v2v2,
}
/// Convenience type
type Time = usize;

/// Status of a match derived from `Last_match` AoE2.net endpoint
/// if a game has no finished time, we threat it as running
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum MatchStatus {
    Running,
    Finished(Time),
}

type ErrorMessage = String;

/// Head struct to assemble `MatchInfo` into and save `error_messages` within to
/// delegate to the frontend
#[derive(Clone, Debug, TypedBuilder, PartialEq, Serialize)]
pub struct MatchInfoResult {
    pub match_info: MatchInfo,
    #[builder(default=None, setter(strip_option))]
    pub error_message: Option<ErrorMessage>,
}

impl MatchInfoResult {
    /// Write a RON file of MatchInfoResult to `logs/match_info_result.ron` for
    /// debugging purposes
    pub fn export_data_to_file(&self) {
        let ron_config = PrettyConfig::new()
            .with_depth_limit(8)
            .with_separate_tuple_members(true)
            .with_enumerate_arrays(true)
            .with_indentor("\t".to_owned());

        // Open the file in writable mode with buffer.
        let file = fs::File::create("logs/match_info_result.ron").unwrap();
        let writer = BufWriter::new(file);

        // Write data to file
        to_writer_pretty(writer, &self, ron_config)
            .expect("Unable to write data");
    }
}

/// Basic information needed in the `MatchInfo`
/// Used to aggregate all the other data inside
/// a single struct
#[derive(Clone, Debug, TypedBuilder, PartialEq, Serialize)]
pub struct MatchInfo {
    game_type: String,
    rating_type: String,
    match_size: MatchSize,
    match_status: MatchStatus,
    map_name: String,
    server: String,
    teams: Teams,
}

/// Wrapper struct around `PlayerRaw` for `Players`
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Players(pub Vec<PlayerRaw>);

#[derive(Clone, TypedBuilder, Getters, Debug, PartialEq, Serialize)]
pub struct PlayerRaw {
    rating: Rating,
    player_number: i64,
    team_number: i64,
    name: String,
    country: String,
    civilisation: String,
    requested: bool,
}

/// Wrapper around `TeamRaw` for `Teams`
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Teams(pub Vec<TeamRaw>);

/// A single Team used for Builder pattern and later
/// for assemblance of the Teams(T) wrapper
#[derive(Clone, Debug, TypedBuilder, PartialEq, Serialize)]
pub struct TeamRaw {
    players: Players,
    team_number: i64,
    #[builder(default, setter(strip_option))]
    team_name: Option<String>,
}

/// Rating part of the our `matchinfo` endpoint
#[derive(Clone, Debug, TypedBuilder, PartialEq, Serialize)]
pub struct Rating {
    mmr: u32,
    rank: u64,
    pub wins: u64,
    pub losses: u64,
    streak: i32,
    #[builder(default=Some(0.0), setter(strip_option))]
    pub win_rate: Option<f32>,
    #[builder(setter(strip_option))]
    highest_mmr: Option<u32>,
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
