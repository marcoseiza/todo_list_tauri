<script lang="ts">
  import TextAreaInput from "./TextAreaInput.svelte";

  export let onEdit: () => void = () => {};
  export let onSubmit: (this: HTMLTextAreaElement) => void = () => {};
  export let body: string;
  export let edit: boolean;
</script>

<h3
  class={`flex cursor-text min-w-0 font-bold ${
    body || edit ? "" : "text-stone-600 dark:text-stone-300"
  }`}
  style="word-break: break-word;"
  on:dblclick={onEdit}
>
  {#if !edit}
    {body || "Untitled"}
  {:else}
    <TextAreaInput {onSubmit} defaultVal={body || ""} />
  {/if}
</h3>
