//! Teams datastructures to be used with `aoc-reference-data` repository

use serde::{
    Deserialize,
    Serialize,
};

/// A Team from `aoc-ref-data`
#[derive(
    Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize,
)]
pub struct Team {
    /// (Optional) short form of the name
    pub abbreviation: Option<String>,
    /// Long form of the name
    pub name: String,
    /// A list of Players in a Team
    pub players: Vec<String>,
}

#[test]
fn ensure_teams_roundtrips() {
    let t = <Vec<Team>>::default();
    let j = serde_json::to_string(&t).unwrap();
    let r: Vec<Team> = serde_json::from_str(&j).unwrap();
    assert_eq!(t, r);
}

#[test]
fn ensure_teams_from_sample() {
    let sample = r#"
[
  {
    "name": "Aftermath",
    "abbreviation": "aM",
    "players": [
      "Hearttt",
      "MbL",
      "Nicov"
    ]
  },
  {
    "name": "Infinity Legends",
    "abbreviation": "iL",
    "players": [
      "BacT",
      "Capoch",
      "Chris",
      "RiuT"
    ]
  },
  {
    "name": "Team Savages",
    "abbreviation": null,
    "players": [
      "F1Re",
      "Goku",
      "St4rk"
    ]
  },
  {
    "name": "Heresy",
    "abbreviation": null,
    "players": [
      "dogao",
      "Edie",
      "Feanor",
      "Kasva",
      "LaaaaaN",
      "Melkor",
      "Miguel",
      "Sitaux"
    ]
  },
  {
    "name": "GamerLegion",
    "abbreviation": "GL",
    "players": [
      "DauT",
      "JorDan",
      "Nili",
      "Slam",
      "Tatoh",
      "TheViper"
    ]
  },
  {
    "name": "SalzZ",
    "abbreviation": "SalzZ",
    "players": [
      "classicpro",
      "Dark",
      "Vinch",
      "WaRRioR"
    ]
  },
  {
    "name": "Slavic Supremacy",
    "abbreviation": null,
    "players": [
      "Barles"
    ]
  },
  {
    "name": "Vietnam Legends",
    "abbreviation": "VNS",
    "players": [
      "ACCM",
      "BadBoy",
      "CooL",
      "Yellow"
    ]
  },
  {
    "name": "AreS",
    "abbreviation": null,
    "players": [
      "Ming",
      "StrayDog",
      "Whoop",
      "Xiaoxiong"
    ]
  },
  {
    "name": "Tempo Storm",
    "abbreviation": "Tempo",
    "players": [
      "Hera",
      "Liereyy"
    ]
  },
  {
    "name": "Suomi",
    "abbreviation": null,
    "players": [
      "Jupe",
      "Pike",
      "Rubenstock",
      "TheMax",
      "Villese",
      "Zuppi"
    ]
  },
  {
    "name": "PetGunPet",
    "abbreviation": "PGP",
    "players": [
      "AngelinaJolie",
      "Belgium",
      "Daniel",
      "Shed",
      "Stefan"
    ]
  },
  {
    "name": "Rulers of Rome",
    "abbreviation": "RoR",
    "players": [
      "Ganji",
      "GoldenEnd",
      "JonSlow",
      "Kamigawa",
      "Kellar",
      "Luca",
      "nanimaren",
      "Pete Martell",
      "Rise",
      "Shades",
      "Sobek",
      "Sommos",
      "Vodka"
    ]
  },
  {
    "name": "DS",
    "abbreviation": null,
    "players": [
      "BL4CK",
      "Carbo",
      "Ralber",
      "TaoPaiPai"
    ]
  },
  {
    "name": "QuEnDi",
    "abbreviation": null,
    "players": [
      "Clemensor",
      "cortical_",
      "ffavorite_1",
      "l2aGe",
      "L3af",
      "PaladiNz",
      "True",
      "uNLeAsHeD__",
      "_zaryab_"
    ]
  },
  {
    "name": "Brazookas",
    "abbreviation": "bK",
    "players": [
      "BruH"
    ]
  },
  {
    "name": "Fei Mei San Dao",
    "abbreviation": null,
    "players": [
      "Bad Koala",
      "Jibatong",
      "Tim",
      "Vivi"
    ]
  }
]

    "#;

    let _: Vec<Team> = serde_json::from_str(&sample).unwrap();
}
