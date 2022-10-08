import { invoke } from "@tauri-apps/api";
import { writable } from "svelte/store";
import type { Board } from ".";

export const board = (() => {
  const initialBoard = new Promise<Board>(() => ({ groups: [] }));

  const { subscribe, set } = writable(initialBoard, () => reload());
  const reload = () => {
    set(invoke<Board>("get_board"));
  };

  return {
    subscribe,
    reload,
  };
})();
