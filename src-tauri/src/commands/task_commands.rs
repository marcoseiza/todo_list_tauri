use crate::database::{
    Reset, Task, {parse_user_state_expected, UserState},
};
use crate::helpers::{
    find_group, find_task, find_task_or_create, remove_task_from_group, trim_whitespace,
};

#[tauri::command]
pub async fn add_task(
    group_id: String,
    body: String,
    state: tauri::State<'_, UserState>,
) -> Result<(), ()> {
    let mut user = parse_user_state_expected(&state).await;

    let group = find_group(group_id.into(), &mut user.board).expect("Group exists");
    group.tasks.push(Task::default_from(body));
    Ok(())
}

#[tauri::command]
pub async fn remove_task(
    group_id: String,
    task_id: String,
    state: tauri::State<'_, UserState>,
) -> Result<(), ()> {
    let mut user = parse_user_state_expected(&state).await;

    let group = find_group(group_id.into(), &mut user.board).expect("Group to exist");
    let task_pos = find_task(task_id.into(), group).expect("Task to exist");
    group.tasks.remove(task_pos);
    Ok(())
}

#[tauri::command]
pub async fn change_task_group(
    task_id: String,
    neighbor_task_id: Option<String>,
    old_group_id: String,
    new_group_id: String,
    state: tauri::State<'_, UserState>,
) -> Result<(), ()> {
    let mut user = parse_user_state_expected(&state).await;

    let task_clone: Task;
    {
        let old_group = find_group(old_group_id.into(), &mut user.board).expect("Group to exist");
        task_clone = remove_task_from_group(task_id.into(), old_group).expect("Task to exist");
    }
    {
        let new_group = find_group(new_group_id.into(), &mut user.board).expect("Group to exist");
        let neighbor_index = neighbor_task_id
            .map(|str| str.into())
            .and_then(|id| find_task(id, new_group))
            .unwrap_or(new_group.tasks.len());
        new_group.tasks.insert(neighbor_index, task_clone);
    }
    Ok(())
}

#[tauri::command]
pub async fn change_task_body_or_create(
    group_id: String,
    task_id: Option<String>,
    body: String,
    state: tauri::State<'_, UserState>,
) -> Result<(), ()> {
    let mut user = parse_user_state_expected(&state).await;
    let group = find_group(group_id.into(), &mut user.board).expect("Group to exist");
    let task_pos = find_task_or_create(task_id.map(|str| str.into()), group);
    group.tasks[task_pos].body = trim_whitespace(body.trim());
    Ok(())
}

#[tauri::command]
pub async fn reset(state: tauri::State<'_, UserState>) -> Result<(), ()> {
    let mut user = parse_user_state_expected(&state).await;
    user.reset();
    Ok(())
}
