import { invoke } from "@tauri-apps/api";
import type { Board, User } from "../database";
import { board } from "../database/store";

export const save = async () => {
  return invoke<void>("save");
};
(global as any).window.save = save;

export const load = async () => {
  invoke<Board>("load");
  return board.reload();
};
(global as any).window.load = load;

export const sign_up_with_github = async () => {
  return invoke<User>("sign_up_with_github");
};

export const login = async () => {
  return invoke<User>("login");
};

export const get_user = async () => {
  return invoke<User>("get_user");
};

export const refresh_user_token = async () => {
  return invoke<User>("refresh_user_token");
};
(global as any).window.refresh_user_token = refresh_user_token;

export const get_board = async () => {
  return invoke<Board>("get_board");
};
(global as any).window.get_board = get_board;

export const add_group = async (name: string) => {
  await invoke("add_group", { name });
  board.reload();
};

export const remove_group = async (groupId: string) => {
  await invoke("remove_group", { groupId });
  board.reload();
};

export const update_group_pos = async (
  groupId: string,
  neighborGroupId?: string
) => {
  await invoke("update_group_pos", {
    groupId,
    neighborGroupId: neighborGroupId,
  });
  board.reload();
};

export const update_group_name = async (groupId: string, name: string) => {
  await invoke("update_group_name", { groupId, name });
  board.reload();
};

export const update_group_color = async (groupId: string, color: string) => {
  await invoke("update_group_color", { groupId, color });
};

export const add_task = async (groupId: string, body: string) => {
  await invoke("add_task", { groupId, body });
  board.reload();
};

export const remove_task = async (groupId: string, taskId: string) => {
  await invoke("remove_task", { groupId, taskId });
  board.reload();
};

export const change_task_group = async (
  taskId: string,
  neighborTaskId: string | undefined,
  oldGroupId: string,
  newGroupId: string
) => {
  await invoke("change_task_group", {
    taskId,
    neighborTaskId: neighborTaskId,
    oldGroupId,
    newGroupId,
  });
  board.reload();
};

export const change_task_body_or_create = async (
  groupId: string,
  taskId: string | undefined,
  body: string
) => {
  await invoke("change_task_body_or_create", {
    groupId,
    taskId: taskId,
    body,
  });
  board.reload();
};

export const reset = async () => {
  invoke("reset");
  board.reload();
};
