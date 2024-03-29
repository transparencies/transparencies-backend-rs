//! Platforms datastructures to be used with `aoc-reference-data` repository

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Clone,
           Debug,
           PartialEq,
           PartialOrd,
           Eq,
           Ord,
           Hash,
           Serialize,
           Deserialize)]
pub struct Platforms {
    pub id: String,
    pub match_url: String,
    pub name: String,
    pub url: String,
}

#[test]
fn ensure_platforms_roundtrips() {
    let t = <Vec<Platforms>>::default();
    let j = serde_json::to_string(&t).unwrap();
    let r: Vec<Platforms> = serde_json::from_str(&j).unwrap();
    assert_eq!(t, r);
}

#[test]
fn ensure_platforms_from_sample() {
    let sample = r#"
[
  {
    "id": "voobly",
    "name": "Voobly",
    "url": "https://www.voobly.com",
    "match_url": "https://www.voobly.com/match/view/"
  },
  {
    "id": "vooblycn",
    "name": "Voobly China",
    "url": "http://www.vooblycn.com",
    "match_url": "http://www.vooblycn.com/match/view/"
  },
  {
    "id": "qq",
    "name": "QQ AoC",
    "url": "http://aocrec.com",
    "match_url": "http://aocrec.com/"
  },
  {
    "id": "de",
    "name": "Definitive Edition",
    "url": "https://www.ageofempires.com/",
    "match_url": "https://www.ageofempires.com/stats/match-details/?game=age2&gameId="
  },
  {
    "id": "zone",
    "name": "MSN Gaming Zone",
    "url": "https://zone.msn.com/",
    "match_url": ""
  },
  {
    "id": "lan",
    "name": "LAN",
    "url": "",
    "match_url": ""
  },
  {
    "id": "gamepark",
    "name": "GamePark",
    "url": "http://www.gamepark.eu/",
    "match_url": ""
  },
  {
    "id": "igz",
    "name": "International Gaming Zones",
    "url": "http://www.igzones.com/",
    "match_url": ""
  },
  {
    "id": "gameranger",
    "name": "GameRanger",
    "url": "https://www.gameranger.com/",
    "match_url": ""
  },
  {
    "id": "ibp",
    "name": "互动游戏对战平台",
    "url": "http://hd.fxt365.com/",
    "match_url": ""
  }
]

    "#;

    let _: Vec<Platforms> = serde_json::from_str(&sample).unwrap();
}
