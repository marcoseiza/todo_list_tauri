use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use crate::database::crypto::Secure;
use crate::database::{Color, Group, Reset};

use super::EncryptedGroup;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Board {
    pub groups: Vec<Group>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EncryptedBoard {
    pub groups: Vec<EncryptedGroup>,
}

impl Default for Board {
    fn default() -> Self {
        Board {
            groups: vec![
                Group::default_from_color("To Do".to_string(), Color::RED),
                Group::default_from_color("Doing".to_string(), Color::YELLOW),
                Group::default_from_color("Done".to_string(), Color::GREEN),
            ],
        }
    }
}

impl Reset for Board {
    fn reset(&mut self) {
        self.groups = Board::default().groups;
    }
}

impl Secure for Board {
    type Encrypted = EncryptedBoard;

    fn encrypt(&self, cocoon: &cocoon::Cocoon<cocoon::Creation>) -> Self::Encrypted {
        Self::Encrypted {
            groups: self
                .groups
                .iter()
                .map(|t| t.encrypt(cocoon))
                .collect::<Vec<EncryptedGroup>>(),
        }
    }

    fn decrypt(cocoon: &cocoon::Cocoon<cocoon::Creation>, container: &Self::Encrypted) -> Self {
        Self {
            groups: container
                .groups
                .iter()
                .map(|t| Group::decrypt(cocoon, t))
                .collect(),
        }
    }
}

pub struct BoardState(pub Mutex<Board>);
