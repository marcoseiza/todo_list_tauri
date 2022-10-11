use serde::Serialize;
use std::sync::Mutex;

use crate::database::color::Color;
use crate::database::group::Group;
use crate::database::reset::Reset;

#[derive(Serialize, Clone)]
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
        self.groups = Board::default().groups.clone();
    }
}

pub struct BoardState(pub Mutex<Board>);
