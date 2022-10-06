use crate::database::board::Board;
use crate::database::group::Group;
use crate::database::task::Task;

pub fn find_group(group_id: String, board: &mut Board) -> Option<&mut Group> {
    board.groups.iter_mut().find(|group| *group.id == group_id)
}

pub fn find_task(task_id: String, group: &Group) -> Option<(&Task, usize)> {
    let task_pos = group
        .tasks
        .iter()
        .position(|task| task.id == task_id)
        .expect("Task to exist");
    Some((&group.tasks[task_pos], task_pos))
}

pub fn remove_task_from_group(task_id: String, group: &mut Group) -> Option<Task> {
    let (task, task_pos) = match find_task(task_id, group) {
        Some(found_task) => found_task,
        None => return None,
    };
    let task_clone = task.clone();
    group.tasks.remove(task_pos);
    Some(task_clone)
}
