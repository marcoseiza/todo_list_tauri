import "./style.css";
import "dragula/dist/dragula.css";
import App from "./App.svelte";

const app = new App({
  target: document.getElementById("app"),
});

export default app;
