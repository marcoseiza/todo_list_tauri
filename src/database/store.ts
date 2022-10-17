import * as backend from "../backend";
import { writable } from "svelte/store";
import { DEFAULT_BOARD, type Board, type User } from ".";
import { timeout } from "../helpers";

export const board = (() => {
  const { subscribe, set } = writable<Board>(DEFAULT_BOARD, () => {
    reload();
  });

  const reload = async () => {
    const board = await backend.get_board();
    set(board);
  };

  return {
    subscribe,
    reload,
  };
})();

export const saving = writable(false);

const saveController = (() => {
  const INTERVAL = 30_000;
  const MIN_INTERVAL_SAVING = 500;

  const save = async () => {
    saving.set(false);
    await timeout(INTERVAL);
    saving.set(true);
    await backend.save();
    await timeout(MIN_INTERVAL_SAVING);
    save();
  };

  return {
    start: save,
  };
})();

export const user = (() => {
  const { subscribe, set } = writable<User | undefined>(undefined, () => {
    initial();
  });

  const initial = async () => {
    try {
      let user = await backend.get_user();
      set(user);
      console.log(user);
    } catch (e) {
      console.error(e);
    }
  };

  return {
    subscribe,
    set: (user: User) => {
      set(user);
      board.reload();
      saveController.start();
    },
    save: async () => {
      saving.set(true);
      await backend.save();
      board.reload();
      saving.set(false);
    },
  };
})();

export const TASK_DND = "task-dnd";
export const GROUP_DND = "group-dnd";
