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

export enum SavingStates {
  SAVING,
  IDLE,
  SAVING_AND_BLOCKING,
}

export const saving = writable(SavingStates.IDLE);

export const saveController = (() => {
  const INTERVAL = 60_000;
  const MIN_INTERVAL_SAVING = 500;

  const save = async () => {
    saving.set(SavingStates.IDLE);
    await timeout(INTERVAL);
    saving.set(SavingStates.SAVING);
    await backend.save();
    await timeout(MIN_INTERVAL_SAVING);
    save();
  };

  return {
    start: save,
  };
})();

export enum UserLoginState {
  UNKOWN,
  LOADING,
  NEEDS_SIGN_UP,
  ERROR,
  GOOD_TO_GO,
}

export const userLoginState = (() => {
  const { subscribe, set } = writable<UserLoginState>(UserLoginState.UNKOWN);

  return {
    subscribe,
    set,
    error: () => {
      userLoginState.set(UserLoginState.ERROR);
      setTimeout(() => {
        userLoginState.set(UserLoginState.NEEDS_SIGN_UP);
      }, 2000);
    },
  };
})();

export const user = (() => {
  const { subscribe, set } = writable<User | undefined>(undefined, () => {
    initial();
  });

  const initial = async () => {
    try {
      userLoginState.set(UserLoginState.LOADING);
      let user = await backend.login();
      set(user);
      await board.reload();
      saveController.start();
      userLoginState.set(UserLoginState.GOOD_TO_GO);
    } catch (e) {
      userLoginState.set(UserLoginState.NEEDS_SIGN_UP);
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
      saving.set(SavingStates.SAVING);
      await backend.save();
      board.reload();
      saving.set(SavingStates.IDLE);
    },
    saveAndBlock: async () => {
      saving.set(SavingStates.SAVING_AND_BLOCKING);
      await backend.save();
      saving.set(SavingStates.IDLE);
    },
  };
})();

export const TASK_DND = "task-dnd";
export const GROUP_DND = "group-dnd";
