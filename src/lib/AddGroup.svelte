<script lang="ts">
  import { Plus } from "phosphor-svelte";
  import { add_group } from "../backend";
  import { afterUpdate } from "svelte";
  import { slide } from "svelte/transition";
  import { onEnter } from "./actions/UseOnEnter";

  let showNameInput = false;
  let nameInput: HTMLInputElement;

  function handleOnEnter(this: HTMLInputElement, _: KeyboardEvent) {
    if (this.value != "") {
      add_group(this.value);
      this.value = "";
      showNameInput = false;
    }
  }

  const onClick = () => (showNameInput = true);
  const onBlur = () => (showNameInput = false);

  afterUpdate(() => {
    if (showNameInput) nameInput.focus();
  });
</script>

<div
  class="flex flex-col items-start min-w-[200px] px-4 gap-3"
  style={`--show-name-input: ${showNameInput}`}
>
  <button
    class="flex items-center gap-3 bg-neutral-700 p-2 pr-4 rounded-md text-white"
    on:click={onClick}
  >
    <Plus size={32} weight="fill" {...{ class: "text-inherit" }} />
    <h2 class="text-xl font-bold">New</h2>
  </button>
  {#if showNameInput}
    <input
      bind:this={nameInput}
      transition:slide={{ duration: 100 }}
      class="outline-none bg-neutral-500 placeholder:text-neutral-300/60 p-2 px-3 rounded-md w-full font-bold "
      type="text"
      name="group-name"
      placeholder="New Group"
      use:onEnter={handleOnEnter}
      on:blur={onBlur}
    />
  {/if}
</div>
