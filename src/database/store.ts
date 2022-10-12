import { get_board } from "../backend";
import { writable } from "svelte/store";
import type { Board } from ".";

export const board = (() => {
  const initialBoardPromise = new Promise<Board>(() => ({ groups: [] }));
  const { subscribe, set } = writable(initialBoardPromise, () => reload());

  const reload = () => {
    set(get_board());
  };

  return {
    subscribe,
    reload,
  };
})();

export const TASK_DND = "task-dnd";
export const GROUP_DND = "group-dnd";
