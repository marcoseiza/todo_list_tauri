use serde::Serialize;
use uuid::Uuid;

use crate::database::task::Task;

#[derive(Serialize, Clone)]
pub struct Group {
    pub id: String,
    pub name: String,
    pub tasks: Vec<Task>,
}

impl Default for Group {
    fn default() -> Self {
        Group {
            id: Uuid::new_v4().simple().to_string(),
            name: Default::default(),
            tasks: Default::default(),
        }
    }
}

impl Group {
    pub fn default_from(name: String) -> Self {
        Group {
            name,
            ..Default::default()
        }
    }
}
