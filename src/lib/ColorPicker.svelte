<script lang="ts">
  import ColorButton from "./ColorButton.svelte";
  import { slide, fade } from "svelte/transition";
  import { GroupColor } from "../database";

  export let onColor: (color: GroupColor) => void;
  export let exclude: GroupColor;

  const colors = Object.keys(GroupColor).filter((v) => v != exclude);
</script>

<ul
  class="absolute top-full inset-x-0 pt-0.5 flex flex-col gap-0.5 list-none z-50 rounded-full backdrop-blur"
  transition:fade|local={{ duration: 200 }}
>
  {#each colors as color}
    <li>
      <ColorButton
        color={GroupColor[color]}
        onClick={() => onColor(GroupColor[color])}
      />
    </li>
  {/each}
</ul>
