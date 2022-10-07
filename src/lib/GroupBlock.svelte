<script lang="ts">
  import type { Group } from "src/database";
  import TaskCard from "./TaskCard.svelte";
  import { Plus } from "phosphor-svelte";

  export let group: Group;
  export let refresh: () => void;

  $: id = group.id;
  let tasks = group.tasks.map((task) => ({ task, edit: false }));

  const addTask = () => {
    tasks = tasks.concat({ task: undefined, edit: true });
  };
</script>

<div class="group-container">
  <h1>{group.name}</h1>
  {#each tasks as { task, edit }}
    <TaskCard groupId={id} {task} {edit} {refresh} />
  {/each}
  <div>
    <button on:click={addTask} class="add-task">
      <Plus size={20} weight="fill" color="var(--icon-color)" />
    </button>
  </div>
  <div class="drop" />
</div>

<style>
  .drop {
    height: 5em;
    width: 100%;
  }

  .group-container {
    display: flex;
    flex-direction: column;
    gap: 0.5em;
    background-color: rgba(0, 0, 0, 0.1);
  }

  .add-task {
    display: flex;
    align-items: center;
    background-color: transparent;
    border: none;
    --icon-color: white;
  }
</style>
