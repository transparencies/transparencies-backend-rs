use ::serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerLastMatch {
    pub clan: String,
    pub country: String,
    pub last_match: LastMatch,
    pub name: String,
    pub profile_id: i64,
    pub steam_id: String,
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LastMatch {
    pub average_rating: ::serde_json::Value,
    pub cheats: bool,
    pub ending_age: i64,
    pub expansion: ::serde_json::Value,
    pub finished: i64,
    pub full_tech_tree: bool,
    pub game_type: i64,
    pub has_custom_content: ::serde_json::Value,
    pub has_password: bool,
    pub leaderboard_id: i64,
    pub lobby_id: ::serde_json::Value,
    pub lock_speed: bool,
    pub lock_teams: bool,
    pub map_size: i64,
    pub map_type: i64,
    pub match_id: String,
    pub match_uuid: String,
    pub name: String,
    pub num_players: i64,
    pub num_slots: i64,
    pub opened: i64,
    pub players: Vec<Players>,
    pub pop: i64,
    pub ranked: bool,
    pub rating_type: i64,
    pub resources: i64,
    pub rms: ::serde_json::Value,
    pub scenario: ::serde_json::Value,
    pub server: String,
    pub shared_exploration: bool,
    pub speed: i64,
    pub started: i64,
    pub starting_age: i64,
    pub team_positions: bool,
    pub team_together: bool,
    pub treaty_length: i64,
    pub turbo: bool,
    pub version: String,
    pub victory: i64,
    pub victory_time: i64,
    pub visibility: i64,
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Players {
    pub civ: i64,
    pub clan: ::serde_json::Value,
    pub color: i64,
    pub country: ::serde_json::Value,
    pub drops: ::serde_json::Value,
    pub games: ::serde_json::Value,
    pub name: String,
    pub profile_id: i64,
    pub rating: ::serde_json::Value,
    pub rating_change: ::serde_json::Value,
    pub slot: i64,
    pub slot_type: i64,
    pub steam_id: String,
    pub streak: ::serde_json::Value,
    pub team: i64,
    pub wins: ::serde_json::Value,
    pub won: bool,
}

#[test]
fn ensure_player_last_match_roundtrips() {
    let t = <PlayerLastMatch>::default();
    let j = serde_json::to_string(&t).unwrap();
    let r: PlayerLastMatch = serde_json::from_str(&j).unwrap();
    assert_eq!(t, r);
}

#[test]
fn ensure_player_last_match_from_sample() {
    let sample = r#"
{
  "profile_id":459658,
  "steam_id":"76561199003184910",
  "name":"DS_HOANG |AOEBuilds.com",
  "clan":"Biry",
  "country":"VN",
  "last_match":{
    "match_id":"70872238",
    "lobby_id":null,
    "match_uuid":"0d6ea86e-b917-cd43-a27f-836dfb6aa37d",
    "version":"45340",
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
    "map_type":9,
    "pop":200,
    "ranked":true,
    "leaderboard_id":3,
    "rating_type":2,
    "resources":1,
    "rms":null,
    "scenario":null,
    "server":"southeastasia",
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
    "opened":1613401916,
    "started":1613401916,
    "finished":1613403371,
    "players":[
      {
        "profile_id":226575,
        "steam_id":"76561198313422112",
        "name":"_WWP_lyx",
        "clan":null,
        "country":null,
        "slot":1,
        "slot_type":1,
        "rating":null,
        "rating_change":null,
        "games":null,
        "wins":null,
        "streak":null,
        "drops":null,
        "color":2,
        "team":1,
        "civ":25,
        "won":true
      },
      {
        "profile_id":459658,
        "steam_id":"76561199003184910",
        "name":"DS_HOANG |AOEBuilds.com",
        "clan":null,
        "country":null,
        "slot":2,
        "slot_type":1,
        "rating":null,
        "rating_change":null,
        "games":null,
        "wins":null,
        "streak":null,
        "drops":null,
        "color":1,
        "team":2,
        "civ":7,
        "won":false
      }
    ]
  }
}
    "#;

    let _: PlayerLastMatch = serde_json::from_str(&sample).unwrap();
}
