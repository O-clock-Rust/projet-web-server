use serde::Serialize;

use crate::models::Task;

pub trait HasId {
    fn get_id(&self) -> u32;
}

#[derive(Serialize)]
pub struct BasicResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct TasksResponse {
    pub status: String,
    pub results: usize,
    pub tasks: Vec<Task>,
}

#[derive(Serialize, Debug)]
pub struct SingleTaskResponse {
    pub status: String,
    pub task: Task,
}
