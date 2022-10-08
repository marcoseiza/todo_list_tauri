<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { Plus } from "phosphor-svelte";

  export let updateBoard: () => void;

  const onSubmit = (e) => {
    e.preventDefault();
    const formData = new FormData(e.target);
    const data = new Map(Array.from(formData.entries()));
    const name = data.get("group-name") as string;
    if (name != "") {
      invoke("add_group", { name });
      updateBoard();
    }
    e.target.reset();
  };
</script>

<form on:submit={onSubmit} class="new-group-form">
  <input type="text" name="group-name" />
  <button type="submit" class="add-group">
    <Plus size={32} weight="fill" color="var(--icon-color)" />
  </button>
</form>

<style>
  .new-group-form {
    padding: 1em;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1em;
  }

  .add-group {
    display: flex;
    align-items: center;
    background-color: rgba(71, 71, 71, 1);
    border-radius: 0.3em;
    padding: 0.5em;
    border: none;
    --icon-color: white;
  }
</style>
