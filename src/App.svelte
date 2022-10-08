<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import GroupBlock from "./lib/GroupBlock.svelte";
  import Dragula from "./lib/drag/Dragula.svelte";
  import AddGroup from "./lib/AddGroup.svelte";
  import { board } from "./database/store";

  const reset = async () => {
    await invoke("reset");
    board.reload();
  };

  const onDrop = (
    task: HTMLElement,
    newGroup: HTMLElement,
    oldGroup: HTMLElement,
    _siblingTask: HTMLElement
  ) => {
    invoke("change_task_group", {
      taskId: task.dataset.drop,
      oldGroupId: oldGroup.dataset.drop,
      newGroupId: newGroup.dataset.drop,
    });
    board.reload();
  };
</script>

<main class="container">
  <h1>Tasks</h1>
  <div class="board">
    <Dragula {onDrop}>
      {#await $board then board}
        {#each board.groups as group (group.id)}
          <div class="board-column">
            <GroupBlock {group} />
          </div>
        {/each}
      {/await}
    </Dragula>
    <div class="board-column new-group">
      <AddGroup />
    </div>
  </div>
  <button on:click={reset}>Reset Store</button>
</main>

<style>
  .container {
    padding: 0.5em 1em;
  }
  .board {
    display: flex;
    width: 100%;
    gap: 1em;
    padding: 1em;
    overflow: scroll;
  }

  .board-column {
    height: 100%;
    min-width: 300px;
    max-width: 300px;
  }

  .board-column.new-group {
    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>
