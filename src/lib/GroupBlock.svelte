<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import type { Group } from "src/database";

  export let group: Group;
  export let refresh: () => void;

  let new_task_body: string;

  $: id = group.id;

  const addTask = () => {
    invoke("add_task", { groupId: id, body: new_task_body });
    refresh();
  };
</script>

<div>
  <h1>{group.name}</h1>
  {#each group.tasks as task}
    <div>{task.body}</div>
  {/each}
  <input bind:value={new_task_body} />
  <button on:click={addTask}>Add Task</button>
</div>
