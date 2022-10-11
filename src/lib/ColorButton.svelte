<script lang="ts">
  export let color: string = "";
  export let onClick: (color: string) => void;
</script>

<button
  class="block {color} w-[32px] h-[calc(32px+0.5em)] rounded-lg"
  on:mousedown={() => onClick(color)}
/>
