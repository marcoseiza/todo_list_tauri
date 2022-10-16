use serde::{Deserialize, Serialize};

use crate::{database::id::Id, helpers::trim_whitespace};

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
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
