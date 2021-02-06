use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

/// So we don't have to tackle how different database work, we'll just use
/// a simple in-memory DB, a vector synchronized by a mutex.
pub type Db = Arc<Mutex<Vec<Todo>>>;

pub fn blank_db() -> Db {
    Arc::new(Mutex::new(Vec::new()))
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Todo {
    pub id: u64,
    pub text: String,
    pub completed: bool,
}

#[derive(Debug, Deserialize)]
pub struct MatchInfoRequest {
    pub id_type: Option<String>,
    pub id_number: Option<String>,
}

// The query parameters for list_todos.
#[derive(Debug, Deserialize)]
pub struct ListOptions {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}
