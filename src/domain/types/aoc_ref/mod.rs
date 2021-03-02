pub mod platforms;
pub mod players;
pub mod teams;

use ::serde::{
    Deserialize,
    Serialize,
};
use serde::de::DeserializeOwned;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AoePlayers(Vec<players::Player>);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AoeTeams(Vec<teams::Teams>);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AoePlatforms(Vec<platforms::Platforms>);

#[derive(Debug, Clone, Default, Serialize)]
pub struct RefDataLists {
    pub players: AoePlayers,
    pub teams: AoeTeams,
    pub platforms: AoePlatforms,
}

impl RefDataLists {
    #[must_use]
    pub fn new() -> Self {
        RefDataLists::default()
    }

    // pub fn get_alias_from_profile_id() -> Option<String> {
    //     todo!();

    // What could we need for showing on the overlay/twitch extension?
    // Name,
    // aka,
    // discord,
    // esportsearnings,
    // liquipedia,
    // platforms[de] (maybe in the future platforms[voobly] as well)
    // twitch
    // youtube

    // What do we need to search for?
    // platforms[de]
    // platforms[voobly]

    // TODO
    // - Let a thread (or directly after downloading in the same thread) index
    //   all the `player_ids` of the platforms
    // - create a HashMap from it with the ID as a key
    // }
}

// impl Iterator for Foreigns {
//     type Item = Package;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.locals.next().and_then(|p| {
//             let name = p.name();
//             match self.syncs.iter().find_map(|db| db.pkg(name).ok()) {
//                 None => Some(p),
//                 Some(_) => self.next(),
//             }
//         })
//     }
// }
