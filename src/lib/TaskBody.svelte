<script lang="ts">
  import TextAreaInput from "./TextAreaInput.svelte";

  export let onEdit: () => void = () => {};
  export let onSubmit: (this: HTMLTextAreaElement) => void = () => {};
  export let body: string;
  export let edit: boolean;
</script>

<h3
  class={`flex cursor-text break-all ${
    body || edit ? "text-white" : "text-stone-300"
  }`}
  on:dblclick={onEdit}
>
  {#if !edit}
    {body || "Untitled"}
  {:else}
    <TextAreaInput {onSubmit} defaultVal={body || ""} />
  {/if}
</h3>
