use serde::{
    Deserialize,
    Serialize,
};
use std::collections::HashMap;

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "MatchInfo")]
pub struct MatchInfo {
    #[serde(rename = "match")]
    pub match_: Match,
    pub running: bool,
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Match {
    pub average_rating: ::serde_json::Value,
    pub cheats: bool,
    pub ending_age: i64,
    pub ending_age_string: String,
    pub expansion: ::serde_json::Value,
    pub finished: i64,
    pub full_tech_tree: bool,
    pub game_type: i64,
    pub game_type_string: String,
    pub has_custom_content: ::serde_json::Value,
    pub has_password: bool,
    pub leaderboard_id: i64,
    pub leaderboard_id_string: String,
    pub lobby_id: ::serde_json::Value,
    pub lock_speed: bool,
    pub lock_teams: bool,
    pub map_name_string: String,
    pub map_size: i64,
    pub map_size_string: String,
    pub map_type: i64,
    pub map_type_string: String,
    pub match_id: String,
    pub match_uuid: String,
    pub max_team_size: i64,
    pub name: String,
    pub num_players: i64,
    pub num_slots: i64,
    pub opened: i64,
    pub pop: i64,
    pub ranked: bool,
    pub rating_type: i64,
    pub resources: i64,
    pub resources_string: String,
    pub rms: ::serde_json::Value,
    pub scenario: ::serde_json::Value,
    pub server: String,
    pub shared_exploration: bool,
    pub speed: i64,
    pub speed_string: String,
    pub started: i64,
    pub starting_age: i64,
    pub starting_age_string: String,
    pub team_positions: bool,
    pub team_together: bool,
    pub teams: Teams,
    pub treaty_length: i64,
    pub turbo: bool,
    pub version: String,
    pub victory: i64,
    pub victory_string: String,
    pub victory_time: i64,
    pub visibility: i64,
    pub visibility_string: String,
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Teams {
    pub players: Vec<Players>,
    pub team: i64,
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Players {
    pub civ: i64,
    pub civ_string: String,
    pub clan: ::serde_json::Value,
    pub color: i64,
    pub country: ::serde_json::Value,
    pub country_aliased: String,
    pub drops: ::serde_json::Value,
    pub games: ::serde_json::Value,
    pub name: String,
    pub name_aliased: String,
    pub profile_id: i64,
    pub rating: Rating,
    pub rating_change: ::serde_json::Value,
    pub reference: HashMap<String, ::serde_json::Value>,
    pub slot: i64,
    pub slot_type: i64,
    pub steam_id: String,
    pub streak: ::serde_json::Value,
    pub team: i64,
    pub wins: ::serde_json::Value,
    pub won: bool,
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rating {
    pub clan: Option<String>,
    pub country: String,
    pub drops: i64,
    pub games: i64,
    pub highest_rating: i64,
    pub highest_streak: i64,
    pub historic: bool,
    pub icon: ::serde_json::Value,
    pub last_match: i64,
    pub last_match_time: i64,
    pub losses: i64,
    pub lowest_streak: i64,
    pub name: String,
    pub previous_rating: i64,
    pub profile_id: i64,
    pub rank: i64,
    pub rating: i64,
    pub steam_id: String,
    pub streak: i64,
    pub winrate: String,
    pub wins: i64,
}

#[test]
fn ensure_match_info_roundtrips() {
    let t = <MatchInfo>::default();
    let j = serde_json::to_string(&t).unwrap();
    let r: MatchInfo = serde_json::from_str(&j).unwrap();
    assert_eq!(t, r);
}

#[test]
fn ensure_match_info_from_sample() {
    let sample = r#"
{
  "match":{
    "match_id":"68875962",
    "lobby_id":null,
    "match_uuid":"baa51af1-f79f-6845-896d-9b85bd1242f8",
    "version":"44834",
    "name":"AUTOMATCH",
    "num_players":2,
    "num_slots":2,
    "average_rating":null,
    "cheats":false,
    "full_tech_tree":false,
    "ending_age":5,
    "expansion":null,
    "game_type":0,
    "has_custom_content":null,
    "has_password":true,
    "lock_speed":true,
    "lock_teams":true,
    "map_size":0,
    "map_type":149,
    "pop":200,
    "ranked":true,
    "leaderboard_id":3,
    "rating_type":2,
    "resources":1,
    "rms":null,
    "scenario":null,
    "server":"ukwest",
    "shared_exploration":false,
    "speed":2,
    "starting_age":2,
    "team_together":true,
    "team_positions":true,
    "treaty_length":0,
    "turbo":false,
    "victory":1,
    "victory_time":0,
    "visibility":0,
    "opened":1612705034,
    "started":1612705034,
    "finished":1612706393,
    "starting_age_string":"Dark Age",
    "ending_age_string":"Imperial Age",
    "game_type_string":"Random Map",
    "leaderboard_id_string":"1v1 Random Map",
    "map_size_string":"Tiny (2 player)",
    "map_type_string":"African Clearing",
    "map_name_string":"African Clearing",
    "resources_string":"Low",
    "speed_string":"Normal",
    "victory_string":"Conquest",
    "visibility_string":"Normal",
    "max_team_size":1,
    "teams":{
      "team":-1,
      "players":[
        [
          {
            "profile_id":197751,
            "steam_id":"76561197964180873",
            "name":"GL.Nili",
            "clan":null,
            "country":null,
            "slot":2,
            "slot_type":1,
            "rating":{
              "profile_id":197751,
              "rank":81,
              "rating":2178,
              "steam_id":"76561197964180873",
              "icon":null,
              "name":"GL.Nili",
              "clan":"Liiit",
              "country":"DE",
              "previous_rating":2167,
              "highest_rating":2252,
              "streak":1,
              "lowest_streak":-8,
              "highest_streak":10,
              "games":968,
              "wins":535,
              "losses":433,
              "drops":5,
              "last_match":1612706393,
              "last_match_time":1612706393,
              "winrate":"55.3",
              "historic":false
            },
            "rating_change":null,
            "games":null,
            "wins":null,
            "streak":null,
            "drops":null,
            "color":5,
            "team":2,
            "civ":13,
            "won":true,
            "civ_string":"Huns",
            "reference":{
              "name":"Nili",
              "country":"de",
              "aoeelo":11,
              "esportsearnings":41832,
              "liquipedia":"Nili_AoE",
              "twitch":"https://www.twitch.tv/nili_aoe",
              "youtube":"https://www.youtube.com/channel/UCXeY7zz-1LsyZdnpFbffdqA/videos",
              "discord":"https://discordapp.com/invite/pQG8txM",
              "platforms":{
                "voobly":[
                  "9267",
                  "124423938"
                ],
                "de":[
                  "197751"
                ],
                "zone":[
                  "nili"
                ],
                "gamepark":[
                  "nili"
                ]
              }
            },
            "name_aliased":"Nili",
            "country_aliased":"de"
          },
          {
            "profile_id":3457608,
            "steam_id":"76561199092185117",
            "name":"HeadlessHalloumi",
            "clan":null,
            "country":null,
            "slot":1,
            "slot_type":1,
            "rating":{
              "profile_id":3457608,
              "rank":128,
              "rating":2101,
              "steam_id":"76561199092185117",
              "icon":null,
              "name":"HeadlessHalloumi",
              "clan":null,
              "country":"FI",
              "previous_rating":2109,
              "highest_rating":2168,
              "streak":-4,
              "lowest_streak":-4,
              "highest_streak":21,
              "games":139,
              "wins":92,
              "losses":47,
              "drops":4,
              "last_match":1612790340,
              "last_match_time":1612790340,
              "winrate":"66.2",
              "historic":false
            },
            "rating_change":null,
            "games":null,
            "wins":null,
            "streak":null,
            "drops":null,
            "color":2,
            "team":1,
            "civ":24,
            "won":false,
            "civ_string":"Mayans",
            "reference":{
              "name":"Pike",
              "country":"fi",
              "aoeelo":44,
              "esportsearnings":15935,
              "liquipedia":"NorthernPike",
              "twitch":"https://www.twitch.tv/pikeaoc/",
              "platforms":{
                "voobly":[
                  "123162283",
                  "123947524",
                  "123950294"
                ],
                "de":[
                  "229063",
                  "3457608"
                ]
              }
            },
            "name_aliased":"Pike",
            "country_aliased":"fi"
          }
        ]
      ]
    }
  },
  "running":false
}
"#;

    let _: MatchInfo = serde_json::from_str(&sample).unwrap();
}
