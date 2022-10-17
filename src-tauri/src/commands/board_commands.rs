use crate::database::{parse_user_state, Board, UserState};

#[tauri::command]
pub async fn get_board(state: tauri::State<'_, UserState>) -> Result<Board, ()> {
    match &*parse_user_state(&state).await {
        Some(user) => Ok(user.board.clone()),
        None => Ok(Board::default()),
    }
}
