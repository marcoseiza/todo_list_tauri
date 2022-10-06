use crate::database::board::Board;
use crate::database::group::Group;

pub fn find_group(group_id: String, board: &mut Board) -> Option<&mut Group> {
    board.groups.iter_mut().find(|group| *group.id == group_id)
}
