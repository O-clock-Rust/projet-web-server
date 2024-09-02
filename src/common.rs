#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub status: bool,
}

pub struct AppState {
    pub app_name: String,
    pub tasks: Option<Vec<Task>>,
}
