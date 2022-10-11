use serde::Serialize;

use crate::{database::id::Id, helpers::trim_whitespace};

#[derive(Serialize, Default, Clone)]
pub struct Task {
    pub id: Id,
    pub body: String,
}

impl Task {
    pub fn default_from(body: String) -> Self {
        Task {
            body: trim_whitespace(body.trim()),
            ..Default::default()
        }
    }
}
