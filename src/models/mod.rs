pub mod queries;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::common::HasId;

#[derive(Debug, Deserialize, Serialize, Clone, FromRow)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub status: bool,
}

impl HasId for Task {
    fn get_id(&self) -> u32 {
        self.id
    }
}

#[derive(Debug, Deserialize)]
pub struct NewTask {
    pub title: String,
    pub description: String,
}
