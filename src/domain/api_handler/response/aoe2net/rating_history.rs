use ::serde::{
    Deserialize,
    Serialize,
};

#[derive(
    Default,
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
pub struct RatingHistoryList {
    #[serde(rename = "RatingHistory")]
    pub list: Vec<RatingHistory>,
}

#[derive(
    Default,
    Clone,
    Debug,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
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
[
   {
      "rating":2345,
      "num_wins":1715,
      "num_losses":1260,
      "streak":7,
      "drops":41,
      "timestamp":1608795056
   },
   {
      "rating":2344,
      "num_wins":1714,
      "num_losses":1260,
      "streak":6,
      "drops":41,
      "timestamp":1608791810
   },
   {
      "rating":2330,
      "num_wins":1713,
      "num_losses":1260,
      "streak":5,
      "drops":41,
      "timestamp":1608789352
   },
   {
      "rating":2315,
      "num_wins":1712,
      "num_losses":1260,
      "streak":4,
      "drops":41,
      "timestamp":1608784971
   },
   {
      "rating":2310,
      "num_wins":1711,
      "num_losses":1260,
      "streak":3,
      "drops":41,
      "timestamp":1608782368
   }
]
    "#;

    let _: Vec<RatingHistory> = serde_json::from_str(&sample).unwrap();
}
