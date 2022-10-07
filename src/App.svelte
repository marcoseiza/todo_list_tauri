<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import GroupBlock from "./lib/GroupBlock.svelte";
  import type { Board } from "./database";

  let new_group_name: string;
  let promiseBoard: Promise<Board>;

  const updateBoard = () => (promiseBoard = invoke("get_board"));

  const reset = () => {
    invoke("reset");
    updateBoard();
  };
  const addGroup = () => {
    invoke("add_group", { name: new_group_name });
    updateBoard();
  };

  updateBoard();
</script>

<main class="container">
  <h1>Tasks</h1>
  <div class="board">
    {#await promiseBoard then board}
      {#each board.groups as group}
        <GroupBlock {group} refresh={updateBoard} />
      {/each}
    {/await}
  </div>
  <button on:click={reset}>Reset Store</button>
  <input bind:value={new_group_name} />
  <button on:click={addGroup}>Add Group</button>
</main>

<style>
  .container {
    padding: 0.5em 1em;
  }
  .board {
    display: grid;
    grid-template-columns: 1fr 1fr 1fr;
    width: 100%;
    gap: 1em;
    padding: 1em;
  }
</style>
