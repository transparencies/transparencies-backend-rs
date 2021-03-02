pub mod aoc_ref;
pub mod aoe2net;
pub mod api;
pub mod match_data;
pub mod requests;

pub use match_data::MatchDataResponses;
pub use requests::*;

use std::{
    collections::HashMap,
    sync::Arc,
};

use tokio::sync::Mutex;

use self::aoc_ref::RefDataLists;
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
pub struct InMemoryDb {
    pub aoe2net_languages: HashMap<String, serde_json::Value>,
    pub github_file_content: RefDataLists,
}
