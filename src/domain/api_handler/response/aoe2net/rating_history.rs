use ::serde::{
    Deserialize,
    Serialize,
};

#[derive(
    Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize,
)]
pub struct RatingHistoryList {
    #[serde(rename = "RatingHistory")]
    pub list: Vec<RatingHistory>,
}

#[derive(
    Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize,
)]
pub struct RatingHistory {
    pub drops: i64,
    pub num_losses: i64,
    pub num_wins: i64,
    pub rating: i64,
    pub streak: i64,
    pub timestamp: i64,
}

#[test]
fn ensure_rating_history_roundtrips() {
    let t = <Vec<RatingHistory>>::default();
    let j = serde_json::to_string(&t).unwrap();
    let r: Vec<RatingHistory> = serde_json::from_str(&j).unwrap();
    assert_eq!(t, r);
}

#[test]
fn ensure_rating_history_from_sample() {
    let sample = r#"
[{"rating":1263,"num_wins":80,"num_losses":14,"streak":-5,"drops":1,"timestamp":1611394467},{"rating":1263,"num_wins":80,"num_losses":13,"streak":-4,"drops":1,"timestamp":1611393418}]

    "#;

    let _: Vec<RatingHistory> = serde_json::from_str(&sample).unwrap();
}
