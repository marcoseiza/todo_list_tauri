use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Clone)]
pub struct Task {
    pub id: String,
    pub body: String,
}

impl Default for Task {
    fn default() -> Self {
        Task {
            id: Uuid::new_v4().simple().to_string(),
            body: Default::default(),
        }
    }
}

impl Task {
    pub fn default_from(body: String) -> Self {
        Task {
            body,
            ..Default::default()
        }
    }
}
