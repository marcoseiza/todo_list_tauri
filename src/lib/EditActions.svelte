<script lang="ts">
  import { Pencil, Trash } from "phosphor-svelte";

  export let show: boolean = true;
  export let onEdit: () => void = () => {};
  export let onRemove: () => void = () => {};
  export let classext: string = "";
</script>

{#if show}
  <div
    class="flex absolute gap-2 top-1 right-1 p-1 bg-zinc-900/50 rounded-md opacity-0 group-hover:opacity-100 transition-opacity duration-200 {classext}"
  >
    <button on:click={onEdit}>
      <Pencil
        size={20}
        weight="fill"
        {...{ class: "fill-white hover:fill-blue-400 transition-colors" }}
      />
    </button>
    <button on:click={onRemove}>
      <Trash
        size={20}
        weight="fill"
        {...{ class: "fill-white hover:fill-red-400 transition-colors" }}
      />
    </button>
  </div>
{/if}
