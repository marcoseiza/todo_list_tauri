use serde::Serialize;
use std::sync::Mutex;

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
                Group::default_from("To Do".to_string()),
                Group::default_from("Doing".to_string()),
                Group::default_from("Done".to_string()),
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
