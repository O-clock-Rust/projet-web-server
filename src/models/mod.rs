use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

use crate::common::HasId;

#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug)]
pub struct AppState {
    pub app_name: String,
    pub tasks: Arc<Mutex<Vec<Task>>>,
}

impl AppState {
    pub fn init() -> Self {
        AppState {
            app_name: String::from("ToDo List"),
            tasks: Arc::new(Mutex::new(vec![Task {
                id: 1,
                title: String::from("Test init task"),
                description: String::from(
                    "What happens if a task is already initialized in the State?",
                ),
                status: false,
            }])),
        }
    }
}
