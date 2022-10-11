<script lang="ts">
  import { Pencil } from "phosphor-svelte";

  export let show: boolean = true;
</script>

{#if show}
  <div
    class="absolute -top-2 left-1 flex items-center gap-1 bg-neutral-600 p-1 pl-2 pr-3 rounded-md"
  >
    <Pencil size={15} weight="fill" {...{ class: "fill-white" }} />
    <p class="text-xs font-bold">EDITING</p>
  </div>
{/if}
