#![allow(clippy::used_underscore_binding)]
#![allow(clippy::empty_enum)]
//! The data structures we return to the client
//! when calling the `match_info` endpoint

use ron::ser::{
    to_writer_pretty,
    PrettyConfig,
};

use derive_getters::Getters;
use serde::{
    Deserialize,
    Serialize,
};
use std::{
    fs,
    io::BufWriter,
};
use typed_builder::TypedBuilder;

/// An enum describing the different `MatchSizes` we support on our overlay
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatchSize {
    /// (Unused)
    /// NoGame = -1,
    /// Custom Game
    Custom = 0,
    /// 1v1 Game (2 players, 2 teams)
    G1v1 = 2,
    /// 2v2 Game (4 players, 2 teams)
    G2v2 = 4,
    /// 3v3 Game (6 players, 2 teams)
    G3v3 = 6,
    /// 4v4 Game (8 players, 2 teams)
    G4v4 = 8,
    /// 2v2v2 Game (6 players, 3 teams)
    G2v2v2,
    /// 2v2v2v2 Game (8 players, 4 teams)
    G2v2v2v2,
}
/// Convenience type
type Time = usize;

/// Convenience type
type ErrorMessage = String;

/// Status of a match derived from `Last_match` AoE2.net endpoint
/// if a game has no finished time, we threat it as running
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatchStatus {
    /// Game is currently running
    Running,
    /// Game was finished at `Time` (Unix)
    Finished(Time),
}
/// The servers the games can be played on
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Server {
    /// Australia
    Australia,
    /// Brazil
    Brazil,
    /// U.K.
    UK,
    /// India
    India,
    /// Southeast Asia
    SoutheastAsia,
    /// Western Europe
    WesternEurope,
    /// U.S. (East)
    UsEast,
    /// U.S. (West)
    UsWest,
    /// Korea
    Korea,
    /// NotFound
    NotFound,
}

/// Head struct to assemble `MatchInfo` into and save `error_messages` within to
/// delegate to the frontend
#[derive(Clone, Debug, TypedBuilder, PartialEq, Serialize, Deserialize)]
pub struct MatchInfoResult {
    /// Contains all the data about the players and the match
    pub match_info: MatchInfo,
    /// Error message strings that are important to give to the frontend
    /// e.g. parsing errors to keep that in cache in the frontend,
    /// or also problems with the HTTP client in general, for example if
    /// the aoe2net API is not reachable
    #[builder(default=None, setter(strip_option))]
    pub error_message: Option<ErrorMessage>,
}

impl Default for MatchInfoResult {
    fn default() -> Self {
        Self {
            match_info: MatchInfo {
                game_type: "Random Map".to_string(),
                rating_type: "1v1 Random Map".to_string(),
                match_size: MatchSize::G1v1,
                match_status: MatchStatus::Finished(1614949859),
                map_name: "Arabia".to_string(),
                server: Server::India,
                teams: Teams({
                    let mut m = Vec::new();
                    m.push(TeamRaw {
                        players: Players({
                            let mut n = Vec::new();
                            n.push(PlayerRaw {
                                rating: Rating {
                                    mmr: 2399,
                                    rank: 24,
                                    wins: 437,
                                    losses: 325,
                                    streak: 7,
                                    win_rate: Some(57.34908),
                                    highest_mmr: Some(2400),
                                },
                                player_number: 3,
                                team_number: 2,
                                name: "Valas".to_string(),
                                country: "fi".to_string(),
                                civilisation: "Spanish".to_string(),
                                requested: false,
                            });
                            n
                        }),
                        team_number: 2,
                        team_name: None,
                    });
                    m.push(TeamRaw {
                        players: Players({
                            let mut n = Vec::new();
                            n.push(PlayerRaw {
                                rating: Rating {
                                    mmr: 2223,
                                    rank: 70,
                                    wins: 1905,
                                    losses: 1432,
                                    streak: -1,
                                    win_rate: Some(57.087208),
                                    highest_mmr: Some(2345),
                                },
                                player_number: 2,
                                team_number: 1,
                                name: "Hoang".to_string(),
                                country: "vn".to_string(),
                                civilisation: "Celts".to_string(),
                                requested: true,
                            });
                            n
                        }),
                        team_number: 1,
                        team_name: None,
                    });
                    m
                }),
            },
            error_message: None,
        }
    }
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
#[derive(Clone, Debug, TypedBuilder, PartialEq, Serialize, Deserialize)]
pub struct MatchInfo {
    /// TODO: If it's matchmaking or custom lobby games, what is the difference
    /// to rating_type? Look into translation file
    game_type: String,
    /// TODO: If it's matchmaking or custom lobby games
    rating_type: String,
    /// How many players are participating in the match
    match_size: MatchSize,
    /// Shows if the match is still running or
    /// when it has been finished
    match_status: MatchStatus,
    /// Name of the currently played map
    map_name: String,
    /// Server location
    server: Server,
    /// Vector of Teams
    teams: Teams,
}

/// Wrapper struct around `PlayerRaw` for `Players`
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Players(pub Vec<PlayerRaw>);

#[derive(
    Clone,
    Default,
    TypedBuilder,
    Getters,
    Debug,
    PartialEq,
    Serialize,
    Deserialize,
)]
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
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Teams(pub Vec<TeamRaw>);

/// A single Team used for Builder pattern and later
/// for assemblance of the Teams(T) wrapper
#[derive(
    Clone, Default, Debug, TypedBuilder, PartialEq, Serialize, Deserialize,
)]
pub struct TeamRaw {
    players: Players,
    team_number: i64,
    #[builder(default, setter(strip_option))]
    team_name: Option<String>,
}

/// Rating part of the our `matchinfo` endpoint
#[derive(
    Clone, Default, Debug, TypedBuilder, PartialEq, Serialize, Deserialize,
)]
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

#[test]
fn ensure_match_info_roundtrips() {
    let t = <MatchInfoResult>::default();
    let j = serde_json::to_string(&t).unwrap();
    let r: MatchInfoResult = serde_json::from_str(&j).unwrap();
    assert_eq!(t, r);
}

#[test]
fn ensure_match_info_from_sample() {
    let sample = r#"
{
  "match_info":{
    "game_type":"Random Map",
    "rating_type":"1v1 Random Map",
    "match_size":"G1v1",
    "match_status":{
      "Finished":1614949859
    },
    "map_name":"Arabia",
    "server":"India",
    "teams":[
      {
        "players":[
          {
            "rating":{
              "mmr":2399,
              "rank":24,
              "wins":437,
              "losses":325,
              "streak":7,
              "win_rate":57.34908,
              "highest_mmr":2400
            },
            "player_number":3,
            "team_number":2,
            "name":"Valas",
            "country":"fi",
            "civilisation":"Spanish",
            "requested":false
          }
        ],
        "team_number":2,
        "team_name":null
      },
      {
        "players":[
          {
            "rating":{
              "mmr":2223,
              "rank":70,
              "wins":1905,
              "losses":1432,
              "streak":-1,
              "win_rate":57.087208,
              "highest_mmr":2345
            },
            "player_number":2,
            "team_number":1,
            "name":"Hoang",
            "country":"vn",
            "civilisation":"Celts",
            "requested":true
          }
        ],
        "team_number":1,
        "team_name":null
      }
    ]
  },
  "error_message":null
}
"#;

    let _: MatchInfoResult = serde_json::from_str(&sample).unwrap();
}
