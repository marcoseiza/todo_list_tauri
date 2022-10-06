use serde::Serialize;
use uuid::Uuid;

#[derive(Default, Serialize, Clone)]
pub struct Task {
    pub id: String,
    pub body: String,
}

impl Task {
    pub fn default_from(body: String) -> Self {
        Task {
            id: Uuid::new_v4().simple().to_string(),
            body,
        }
    }
}
