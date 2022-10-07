<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import type { Task } from "../database";
  import { Trash, Pencil } from "phosphor-svelte";
  export let task: Task;
  export let edit: boolean = false;
  export let groupId: string;
  export let refresh: () => void;

  let new_group_id: string;

  const removeTask = () => {
    invoke("remove_task", { groupId: groupId, taskId: task.id });
    refresh();
  };
  // TODO: Fix DnD
  // const moveTask = () => {
  //   invoke("change_task_group", {
  //     taskId: task.id,
  //     oldGroupId: groupId,
  //     newGroupId: new_group_id,
  //   });
  //   refresh();
  // };

  const onSubmit = (e: any) => {
    const formData = new FormData(e.target);
    const data = new Map(Array.from(formData.entries()));
    const body = data.get("body");
    invoke("change_task_body", { groupId, taskId: task?.id || "", body });
    refresh();
    edit = false;
  };

  const onEdit = () => (edit = true);
</script>

<div class="task-container">
  {#if edit}
    <form class="task-edit-container" on:submit|preventDefault={onSubmit}>
      <input type="text" name="body" id="body" value={task?.body || ""} />
      <button type="submit" class="action edit">
        <Pencil size={20} weight="fill" color="white" />
      </button>
    </form>
  {:else}
    <span class="task-body">{task.body}</span>
    <div class="action-container">
      <button class="action" on:click={onEdit}>
        <Pencil size={20} weight="fill" color="var(--icon-color)" />
      </button>
      <button on:click={removeTask} class="action">
        <Trash size={20} weight="fill" color="var(--icon-color)" />
      </button>
    </div>
  {/if}
</div>

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
