import { get_board } from "../backend";
import { writable } from "svelte/store";
import { DEFAULT_BOARD, type Board, type User } from ".";

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

export const user = (() => {
  const { subscribe, set } = writable<User | undefined>(undefined);

  return {
    subscribe,
    set: (user: User) => {
      set(user);
      board.reload();
    },
  };
})();

export const TASK_DND = "task-dnd";
export const GROUP_DND = "group-dnd";
