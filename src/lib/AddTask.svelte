<script lang="ts">
  import { Plus } from "phosphor-svelte";

  export let addTask: () => void = () => {};
</script>

<button
  on:click={addTask}
  class="absolute inset-x-0 bottom-0 flex items-center gap-3 p-5 text-neutral-400 hover:text-neutral-50 transition-colors cursor-pointer"
>
  <Plus size={20} weight="fill" {...{ class: "text-inherit" }} />
  <h3>New</h3>
</button>
