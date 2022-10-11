<script lang="ts">
  import { makeDrake } from "./lib/drag/Dragula";
  import BoardBlock from "./lib/BoardBlock.svelte";
  import AddGroup from "./lib/AddGroup.svelte";
  import { GROUP_DND, TASK_DND } from "./database/store";
  import { change_task_group, update_group_pos } from "./backend";

  const onTaskDrop = (
    task: HTMLElement,
    newGroup: HTMLElement,
    oldGroup: HTMLElement,
    siblingTask: HTMLElement
  ) => {
    change_task_group(
      task.dataset.drop,
      siblingTask?.dataset?.drop,
      oldGroup.dataset.drop,
      newGroup.dataset.drop
    );
  };

  const onGroupDrop = (
    group: HTMLElement,
    _c: HTMLElement,
    _s: HTMLElement,
    neighborGroup: HTMLElement
  ) => {
    update_group_pos(group.dataset.drop, neighborGroup?.dataset?.drop);
  };

  makeDrake(TASK_DND).on("drop", onTaskDrop);
  makeDrake(GROUP_DND, {
    checkHandle: true,
    direction: "horizontal",
  }).on("drop", onGroupDrop);
</script>

<main class="flex overflow-scroll h-full p-4">
  <BoardBlock />
  <AddGroup />
</main>
