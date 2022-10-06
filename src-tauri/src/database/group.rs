use serde::Serialize;
use uuid::Uuid;

use crate::database::task::Task;

#[derive(Default, Serialize, Clone)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub tasks: Vec<Task>,
}

impl Group {
    pub fn default_from(name: String) -> Self {
        Group {
            id: Uuid::new_v4().simple().to_string(),
            name,
            tasks: Vec::new(),
        }
    }
}
