use serde::{Deserialize, Serialize};
use titlecase::titlecase;

use crate::database::color::Color;
use crate::database::crypto::Secure;
use crate::database::id::Id;
use crate::database::task::{EncryptedTask, Task};
use crate::helpers::{join_bytes, separate_bytes, trim_whitespace};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Group {
    pub id: Id,
    pub name: String,
    pub color: Color,
    #[serde(default)]
    pub tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct EncryptedGroup {
    pub id: Id,
    pub name: String,
    pub color: Color,
    #[serde(default)]
    pub tasks: Vec<EncryptedTask>,
}

impl Group {
    pub fn default_from(name: String) -> Self {
        Group {
            name: trim_whitespace(&*titlecase(name.trim())),
            ..Default::default()
        }
    }

    pub fn default_from_color(name: String, color: Color) -> Self {
        Group {
            name: trim_whitespace(&*titlecase(name.trim())),
            color,
            ..Default::default()
        }
    }
}

impl Secure for Group {
    type Encrypted = EncryptedGroup;

    fn encrypt(&self, cocoon: &cocoon::Cocoon<cocoon::Creation>) -> Self::Encrypted {
        Self::Encrypted {
            id: self.id.clone(),
            name: join_bytes(&cocoon.wrap(self.name.as_bytes()).unwrap(), ","),
            color: self.color,
            tasks: self
                .tasks
                .iter()
                .map(|t| t.encrypt(cocoon))
                .collect::<Vec<EncryptedTask>>(),
        }
    }

    fn decrypt(cocoon: &cocoon::Cocoon<cocoon::Creation>, container: &Self::Encrypted) -> Self {
        Self {
            id: container.id.clone(),
            name: String::from_utf8(
                cocoon
                    .unwrap(&separate_bytes(&container.name, ","))
                    .unwrap(),
            )
            .unwrap(),
            color: container.color,
            tasks: container
                .tasks
                .iter()
                .map(|t| Task::decrypt(cocoon, t))
                .collect(),
        }
    }
}
