pub mod platforms;
pub mod players;
pub mod teams;

use ::serde::{
    Deserialize,
    Serialize,
};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

use crate::domain::data_processing::error::IndexingError;

pub type AoePlayers = Vec<players::Player>;
pub type AoeTeams = Vec<teams::Teams>;
pub type AoePlatforms = Vec<platforms::Platforms>;
pub type PositionInAoePlayers = usize;

#[derive(Debug, Clone, Default, Serialize)]
pub struct RefDataLists {
    pub players: AoePlayers,
    pub players_index_aoe2de: HashMap<String, PositionInAoePlayers>,
    pub teams: AoeTeams,
    pub platforms: AoePlatforms,
}

impl RefDataLists {
    #[must_use]
    pub fn new() -> Self {
        RefDataLists::default()
    }

    /// Index `players` into `players_index` HashMap
    pub fn index(&mut self) -> std::result::Result<(), IndexingError> {
        let mut index: HashMap<String, PositionInAoePlayers> = HashMap::new();

        for (player_number, player) in self.players.iter().enumerate() {
            if !&player.platforms.de.is_empty() {
                for profile_id in &player.platforms.de {
                    let old_value =
                        index.insert(profile_id.to_string(), player_number - 1);

                    if let Some(x) = old_value {
                        return Err(IndexingError::PlayerAlreadyExisting {
                            name: player.name.clone(),
                            profile_id: profile_id.to_string(),
                            pos: player_number - 1,
                            doublette: x,
                        });
                    }
                }
            }
        }

        // Fill index field in struct
        self.players_index_aoe2de = index;

        Ok(())
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
