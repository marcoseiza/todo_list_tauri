use crate::database::board::Board;
use crate::database::group::Group;
use crate::database::id::Id;
use crate::database::task::Task;

#[macro_export]
macro_rules! unwrap_or {
    ($res: expr, $code: expr) => {
        match $res {
            Some(v) => v,
            None => $code,
        }
    };
}

pub fn trim_whitespace(s: &str) -> String {
    // first attempt: allocates a vector and a string
    let words: Vec<_> = s.split_whitespace().collect();
    words.join(" ")
}

pub fn find_group(group_id: Id, board: &mut Board) -> Option<&mut Group> {
    board.groups.iter_mut().find(|group| group.id == group_id)
}

pub fn find_group_with_name(name: &String, board: &mut Board) -> Option<usize> {
    board
        .groups
        .iter()
        .position(|group| group.name.to_lowercase().eq(&name.to_lowercase()))
}

pub fn find_group_pos(group_id: Id, board: &Board) -> Option<usize> {
    board.groups.iter().position(|group| group.id == group_id)
}

pub fn find_task(task_id: Id, group: &Group) -> Option<usize> {
    group.tasks.iter().position(|task| task.id == task_id)
}

pub fn create_task(group: &mut Group) -> usize {
    let task = Task::default();
    group.tasks.push(task);
    group.tasks.len() - 1
}

pub fn find_task_or_create(task_id: Option<Id>, group: &mut Group) -> usize {
    let task_id = unwrap_or!(task_id, return create_task(group));
    find_task(task_id, group).unwrap_or_else(|| create_task(group))
}

pub fn remove_task_from_group(task_id: Id, group: &mut Group) -> Option<Task> {
    let task_pos = unwrap_or!(find_task(task_id, group), return None);
    let task_clone = group.tasks[task_pos].clone();
    group.tasks.remove(task_pos);
    Some(task_clone)
}
