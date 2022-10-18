import { window, dialog } from "@tauri-apps/api";
import { get } from "svelte/store";
import { saving, SavingStates, user } from "../database/store";

export const registerWindowClosePrompt = async () => {
  const unlisten = await window.appWindow.onCloseRequested(async (event) => {
    if (get(saving) == SavingStates.SAVING_AND_BLOCKING) {
      event.preventDefault();
      return;
    }
    const confirmed = await dialog.ask("Save before you exit?", {
      title: "Save before you exit?",
    });
    if (confirmed) await user.saveAndBlock();
  });
  return unlisten;
};
