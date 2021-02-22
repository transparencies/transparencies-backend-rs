use ::serde::{
    Deserialize,
    Serialize,
};

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "LeaderboardInfo")]
pub struct LeaderboardInfo {
    pub count: i64,
    pub leaderboard: Vec<Leaderboard>,
    pub leaderboard_id: i64,
    pub start: i64,
    pub total: i64,
}

#[derive(Default, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Leaderboard {
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
fn ensure_leaderboard_info_roundtrips() {
    let t = <LeaderboardInfo>::default();
    let j = serde_json::to_string(&t).unwrap();
    let r: LeaderboardInfo = serde_json::from_str(&j).unwrap();
    assert_eq!(t, r);
}

#[test]
fn ensure_leaderboard_info_from_sample() {
    let sample = r#"
{"total":242505,"leaderboard_id":0,"start":1,"count":1,"leaderboard":[{"profile_id":459658,"rank":71420,"rating":1263,"steam_id":"76561199003184910","icon":null,"name":"DS_HOANG |AOEBuilds.com","clan":"Biry","country":"VN","previous_rating":1233,"highest_rating":1263,"streak":-5,"lowest_streak":-5,"highest_streak":22,"games":94,"wins":80,"losses":14,"drops":1,"last_match":1611394467,"last_match_time":1611394467}]}

    "#;

    let _: LeaderboardInfo = serde_json::from_str(&sample).unwrap();
}
