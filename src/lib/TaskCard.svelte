<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import type { Task } from "../database";
  export let task: Task;
  export let groupId: string;
  export let refresh: () => void;

  let new_group_id: string;

  const removeTask = () => {
    invoke("remove_task", { groupId: groupId, taskId: task.id });
    refresh();
  };
  const moveTask = () => {
    invoke("change_task_group", {
      taskId: task.id,
      oldGroupId: groupId,
      newGroupId: new_group_id,
    });
    refresh();
  };
</script>

<div>
  {task.body}
  <button on:click={removeTask}>Remove</button>
  <input bind:value={new_group_id} />
  <button on:click={moveTask}>Switch</button>
</div>
