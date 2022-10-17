use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use crate::database::{Color, Group, Reset};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Board {
    pub groups: Vec<Group>,
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

pub struct BoardState(pub Mutex<Board>);
