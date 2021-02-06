use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
#[serde(rename = "Leaderboard")]
pub struct Leaderboard {
    pub count: i64,
    pub leaderboard: Leaderboard2,
    pub leaderboard_id: i64,
    pub start: i64,
    pub total: i64,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct Leaderboard2 {
    pub clan: String,
    pub country: String,
    pub drops: i64,
    pub games: i64,
    pub highest_rating: i64,
    pub highest_streak: i64,
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
    pub wins: i64,
}

#[test]
fn ensure_leaderboard_roundtrips() {
    let t = <Leaderboard>::default();
    let j = serde_json::to_string(&t).unwrap();
    let r: Leaderboard = serde_json::from_str(&j).unwrap();
    assert_eq!(t, r);
}

#[test]
fn ensure_leaderboard_from_sample() {
    let sample = r#"
{
   "total":43092,
   "leaderboard_id":3,
   "start":1,
   "count":1,
   "leaderboard":[
      {
         "profile_id":459658,
         "rank":26,
         "rating":2345,
         "steam_id":"76561199003184910",
         "icon":null,
         "name":"DS_HOANG |AOEBuilds.com",
         "clan":"Biry",
         "country":"VN",
         "previous_rating":2344,
         "highest_rating":2345,
         "streak":7,
         "lowest_streak":-9,
         "highest_streak":12,
         "games":2975,
         "wins":1715,
         "losses":1260,
         "drops":41,
         "last_match":1608795056,
         "last_match_time":1608795056
      }
   ]
}
    "#;

    let _: Leaderboard = serde_json::from_str(&sample).unwrap();
}
