pub mod platforms;
pub mod players;
pub mod teams;

use ::serde::Serialize;
use serde::de::DeserializeOwned;

#[derive(Debug, Clone, Default, Serialize)]
pub struct RefDataLists {
    pub players: Vec<players::Players>,
    pub teams: Vec<teams::Teams>,
    pub platforms: Vec<platforms::Platforms>,
}

impl RefDataLists {
    #[must_use]
    pub fn new() -> Self {
        RefDataLists::default()
    }
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
