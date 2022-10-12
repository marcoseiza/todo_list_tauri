<script lang="ts">
  import { type GroupColor, GroupColorToValue } from "../database";

  export let color: GroupColor;
  export let onClick: () => void;
</script>

<button
  class="block {GroupColorToValue(
    color
  )} w-[32px] h-[calc(32px+0.5em)] rounded-lg"
  on:mousedown={onClick}
/>
