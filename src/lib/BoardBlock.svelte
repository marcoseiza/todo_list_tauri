<script lang="ts">
  import { board, GROUP_DND } from "../database/store";
  import { dragContainer } from "./drag/UseContainerAction";
  import { dropData } from "./drag/UseDropData";
  import { movableItem } from "./drag/UseMovableItem";
  import GroupBlock from "./GroupBlock.svelte";
  import AddGroup from "./AddGroup.svelte";
</script>

<div class="overflow-scroll w-full h-full flex p-4 pt-[var(--toolbar-size)]">
  <div class="relative flex gap-4 h-fit" use:dragContainer={GROUP_DND}>
    {#await $board then board}
      {#each board.groups as group (group.id)}
        <div
          class="min-w-[300px] max-w-[300px]"
          use:movableItem={GROUP_DND}
          use:dropData={group.id}
        >
          <GroupBlock {group} />
        </div>
      {/each}
    {/await}
  </div>
  <AddGroup />
</div>
