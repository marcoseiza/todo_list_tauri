use serde::Serialize;
use titlecase::titlecase;

use crate::database::color::Color;
use crate::database::id::Id;
use crate::database::task::Task;
use crate::helpers::trim_whitespace;

#[derive(Serialize, Clone, Default)]
pub struct Group {
    pub id: Id,
    pub name: String,
    pub color: Color,
    pub tasks: Vec<Task>,
}

impl Group {
    pub fn default_from(name: String) -> Self {
        Group {
            name: trim_whitespace(&*titlecase(name.trim())),
            ..Default::default()
        }
    }
}
