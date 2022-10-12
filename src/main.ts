import "./style.css";
import "dragula/dist/dragula.css";
import App from "./App.svelte";
import { window, event } from "@tauri-apps/api";
import { reset } from "./backend";

const app = new App({
  target: document.getElementById("app"),
});

window.appWindow.onFocusChanged(({ payload: focus }: event.Event<boolean>) => {
  if (!focus) document.getElementById("app").classList.add("window-blur");
  else document.getElementById("app").classList.remove("window-blur");
});

(global as any).window.reset = reset;

export default app;
