pub mod platforms;
pub mod players;
pub mod teams;

use ::serde::Serialize;
use std::collections::HashMap;

use crate::domain::types::error::IndexingError;

/// A list of Players
pub type AoePlayers = Vec<players::Player>;

/// A list of Teams
pub type AoeTeams = Vec<teams::Team>;

/// A list of Platforms
pub type AoePlatforms = Vec<platforms::Platforms>;

/// The position a Player in the Players vector
/// used for mainly for indexing
pub type PositionInAoePlayers = usize;

/// A wrapper struct around all of the preloaded responses
#[derive(Debug, Clone, Default, Serialize)]
pub struct RefDataLists {
    /// from `players.yaml`
    pub players: AoePlayers,
    /// Index over `players.yaml` `profile_id` for Aoe2DE
    pub players_index_aoe2de: HashMap<String, PositionInAoePlayers>,
    /// from `teams.json`
    pub teams: AoeTeams,
    /// from `platforms.json`
    pub platforms: AoePlatforms,
}

impl RefDataLists {
    /// Create a new `RefDataLists` struct with `default` initialisation
    #[must_use]
    pub fn new() -> Self {
        RefDataLists::default()
    }

    /// Index `players` into `players_index` `HashMap`
    pub fn index(&mut self) -> std::result::Result<(), Vec<IndexingError>> {
        let mut index: HashMap<String, PositionInAoePlayers> = HashMap::new();

        let mut indexing_errors: Vec<IndexingError> = Vec::new();

        for (player_number, player) in self.players.iter().enumerate() {
            if !&player.platforms.de.is_empty() {
                for profile_id in &player.platforms.de {
                    let old_value =
                        index.insert(profile_id.to_string(), player_number);

                    if let Some(x) = old_value {
                        // Better error handling, we shouldn't fail to
                        // create an index, just because there is an doublet
                        //
                        // Better would be to `.collect()` this error and not
                        // `continue` the loop and write the rest of the data
                        // into the hashmap
                        indexing_errors.push(
                            IndexingError::PlayerAlreadyExisting {
                                name: player.name.clone(),
                                profile_id: profile_id.to_string(),
                                pos: player_number,
                                doublet: x,
                            },
                        );
                        continue;
                    }
                }
            }
        }

        // Fill index field in struct
        self.players_index_aoe2de = index;

        // Return `indexing_errors`
        if indexing_errors.len() > 0 {
            return Err(indexing_errors);
        }
        else {
            Ok(())
        }
    }

    /// Search through alias list for `player_id` and return `players::Player`
    #[must_use]
    pub fn lookup_player_alias_for_profile_id(
        &self,
        profile_id: &str,
    ) -> Option<players::Player> {
        match self.players_index_aoe2de.get(profile_id) {
            Some(alias_position) => Some(self.players[*alias_position].clone()),
            None => None,
        }
    }
}

// impl Iterator for X {
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
