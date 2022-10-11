<script lang="ts">
  import ColorButton from "./ColorButton.svelte";
  import { slide, fade } from "svelte/transition";
  import { GROUP_COLOR_VALUES } from "../database";

  export let onColor: (color: string) => void;
  export let exclude: string = "";

  const colors = GROUP_COLOR_VALUES.filter((v) => v != exclude);
</script>

<ul
  class="absolute top-full inset-x-0 pt-0.5 flex flex-col gap-0.5 list-none z-50 rounded-full backdrop-blur"
  transition:fade={{ duration: 200 }}
>
  {#each colors as color}
    <li transition:slide={{ duration: 200 }}>
      <ColorButton {color} onClick={onColor} />
    </li>
  {/each}
</ul>
