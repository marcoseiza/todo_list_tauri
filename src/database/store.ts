import { get_board, get_user } from "../backend";
import { writable } from "svelte/store";
import { DEFAULT_BOARD, DEFAULT_USER, type Board, type User } from ".";

export const board = (() => {
  const initial = new Promise<Board>(() => DEFAULT_BOARD);
  const { subscribe, set } = writable(initial, () => reload());

  const reload = () => {
    set(get_board());
  };

  return {
    subscribe,
    reload,
  };
})();

export const user = writable<User | undefined>(undefined);

export const TASK_DND = "task-dnd";
export const GROUP_DND = "group-dnd";
