<script lang="ts">
  import { onMount } from "svelte";
  import { onEnter } from "./actions/UseOnEnter";
  import { autoHeight } from "./UseAutoHeight";

  export let defaultVal: string = "";
  export let onSubmit: (this: HTMLTextAreaElement) => void = () => {};

  let textAreaElement: HTMLTextAreaElement;

  function handleOnEnter(e: KeyboardEvent) {
    onSubmit.bind(this)();
  }
  const onBlur = (e: FocusEvent) => {
    onSubmit.bind(e.currentTarget as HTMLTextAreaElement)();
  };

  onMount(() => {
    textAreaElement.focus();
  });
</script>

<textarea
  bind:this={textAreaElement}
  class="w-full text-inherit font-inherit bg-transparent outline-none resize-none text-[inherit]"
  type="text"
  value={defaultVal}
  use:onEnter={handleOnEnter}
  on:blur={onBlur}
  use:autoHeight
/>
