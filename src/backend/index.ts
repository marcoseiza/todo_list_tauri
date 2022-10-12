import { invoke } from "@tauri-apps/api";
import type { Board } from "../database";
import { board } from "../database/store";

export const get_board = async () => {
  return invoke<Board>("get_board");
};

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
  // board.reload();
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