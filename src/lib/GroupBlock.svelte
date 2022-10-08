<script lang="ts">
  import type { Group } from "src/database";
  import TaskCard from "./TaskCard.svelte";
  import { Plus } from "phosphor-svelte";
  import { dragContainer } from "./drag/UseContainerAction";
  import { immovableItem } from "./drag/UseImmovableItem";
  import { dropData } from "./drag/UseDropData";
  import { invoke } from "@tauri-apps/api/tauri";
  import { board } from "../database/store";

  export let group: Group;

  let tasks = group.tasks.map((task) => ({ task, edit: false }));

  const addTask = () => {
    invoke("add_task", { groupId: group.id, body: "" });
    board.reload();
  };
</script>

<div class="group-container">
  <h1 class="group-name">{group.name}</h1>
  <div use:dragContainer use:dropData={group.id} class="task-list">
    {#each tasks as { task, edit }}
      <TaskCard groupId={group.id} {task} {edit} />
    {/each}
    <div class="add-task-container" use:immovableItem>
      <button on:click={addTask} class="add-task">
        <Plus size={20} weight="fill" color="var(--icon-color)" />
      </button>
    </div>
  </div>
</div>

<style>
  .group-container {
    position: relative;
    display: flex;
    flex-direction: column;
    gap: 0.5em;
    background-color: rgba(0, 0, 0, 0.1);
    padding: 1.5em;
  }

  .group-name {
    margin: 0.3em 0;
  }

  .add-task-container {
    position: absolute;
    padding: 1em 0;
    bottom: 0;
    left: 0;
    width: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .add-task {
    display: flex;
    align-items: center;
    background-color: transparent;
    border: none;
    --icon-color: white;
  }

  .task-list {
    padding-bottom: 2em;
  }
</style>
