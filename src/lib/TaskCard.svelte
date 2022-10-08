<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import type { Task } from "../database";
  import { Trash, Pencil } from "phosphor-svelte";
  import { onMount } from "svelte";
  import { immovableItem } from "./drag/UseImmovableItem";
  import { dropData } from "./drag/UseDropData";
  import { board } from "../database/store";

  export let task: Task;
  export let edit: boolean = false;
  export let groupId: string;

  let editBodyInput: HTMLInputElement;
  if (edit) onMount(() => editBodyInput.focus());

  const removeTask = () => {
    invoke("remove_task", { groupId: groupId, taskId: task.id });
    board.reload();
  };

  const change_or_add_task = (
    groupId: string,
    taskId: string,
    body: string
  ) => {
    invoke("change_task_body", { groupId, taskId, body });
    board.reload();
    edit = false;
  };

  const onSubmit = (e: any) => {
    const formData = new FormData(e.target);
    const data = new Map(Array.from(formData.entries()));
    const body = data.get("body") as string;
    change_or_add_task(groupId, task?.id || "", body);
  };

  const onEdit = () => (edit = true);

  const onBlur = (e) => {
    e.preventDefault();
    change_or_add_task(groupId, "", e.target.value);
  };
</script>

{#if edit}
  <div class="task-container" use:immovableItem>
    <form class="task-edit-container" on:submit|preventDefault={onSubmit}>
      <input
        bind:this={editBodyInput}
        type="text"
        name="body"
        id="body"
        value={task?.body || ""}
        on:blur={onBlur}
      />
      <button type="submit" class="action edit">
        <Pencil size={20} weight="fill" color="white" />
      </button>
    </form>
  </div>
{:else}
  <div class="task-container" use:dropData={task?.id}>
    <h3 class={`task-body ${task?.body || "untitled"}`}>
      {task?.body || "Untitled"}
    </h3>
    <div class="action-container">
      <button class="action" on:click={onEdit}>
        <Pencil size={20} weight="fill" color="var(--icon-color)" />
      </button>
      <button on:click={removeTask} class="action">
        <Trash size={20} weight="fill" color="var(--icon-color)" />
      </button>
    </div>
  </div>
{/if}

<style>
  .task-container {
    background-color: rgb(85, 85, 85);
    padding: 1em;
    display: flex;
    justify-content: space-between;
    border: 1px solid white;
  }

  .task-edit-container {
    width: 100%;
    display: flex;
    justify-content: space-between;
  }

  .action-container {
    display: flex;
    gap: 0.5em;
  }

  .task-body {
    margin: 0;
  }

  .task-body.untitled {
    color: rgba(255, 255, 255, 0.7);
  }

  button.action {
    display: flex;
    align-items: center;
    background: transparent;
    border: none;
    padding: 0;
    cursor: pointer;
    --icon-color: white;
  }

  button.action.edit {
    cursor: default;
  }
</style>
