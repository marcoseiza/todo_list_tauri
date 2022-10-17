<script lang="ts">
  import { DotsSixVertical } from "phosphor-svelte";
  import { GroupColor, GroupColorToValue } from "../database";
  import ColorPicker from "./ColorPicker.svelte";
  import { dragHandle } from "./drag/UseDragHandle";
  import TextAreaInput from "./TextAreaInput.svelte";

  export let name: string = "";
  export let color: GroupColor = GroupColor.BLUE;
  export let edit: boolean = false;
  export let onEdit: () => void = () => {};
  export let onSubmit: (this: HTMLTextAreaElement) => void = () => {};
  export let onColor: (color: string) => void = () => {};

  let editColor = false;
  const onMouseEnter = () => (editColor = edit && true);
  const onMouseLeave = () => (editColor = edit && false);

  let onColorInternal = (color: GroupColor) => {
    onColor(color);
    editColor = false;
  };
</script>

<div class="flex items-start font-bold gap-3">
  <div
    class="relative {GroupColorToValue(color)} p-1 px-0 rounded-lg {!edit &&
      'cursor-grab active:cursor-grabbing'}"
    on:mouseenter={onMouseEnter}
    on:mouseleave={onMouseLeave}
  >
    <div use:dragHandle class="{edit && 'opacity-0'} transition-opacity">
      <DotsSixVertical size={32} />
    </div>
    {#if editColor}<ColorPicker
        onColor={onColorInternal}
        exclude={color}
      />{/if}
  </div>
  <h1 class="flex text-2xl break-all mt-1">
    {#if !edit}
      <span on:dblclick={onEdit}>{name}</span>
    {:else}
      <TextAreaInput {onSubmit} defaultVal={name} />
    {/if}
  </h1>
</div>
