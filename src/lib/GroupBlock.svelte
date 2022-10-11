<script lang="ts">
  import {
    newEditingTaskCardInfo,
    taskToTaskCardInfo,
    type Group,
  } from "../database";
  import TaskCard from "./TaskCard.svelte";
  import { dragContainer } from "./drag/UseContainerAction";
  import { dropData } from "./drag/UseDropData";
  import { TASK_DND } from "../database/store";
  import AddTask from "./AddTask.svelte";
  import GroupTitle from "./GroupTitle.svelte";
  import EditActions from "./EditActions.svelte";
  import {
    remove_group,
    update_group_color,
    update_group_name,
  } from "../backend";

  export let group: Group;
  let edit = false;

  let tasksCardInfos = group.tasks.map(taskToTaskCardInfo);

  const addTask = () => {
    tasksCardInfos = tasksCardInfos.concat(newEditingTaskCardInfo());
  };

  const onRemove = () => remove_group(group.id);
  const onEdit = () => (edit = true);

  function onSubmit(this: HTMLTextAreaElement) {
    update_group_name(group.id, this.value);
    edit = false;
  }

  const onColor = (color: string) => {
    update_group_color(group.id, color);
    edit = false;
  };
</script>

<div
  class="relative w-full grid grid-rows-[auto_1fr] bg-neutral-800 p-5 gap-4 rounded-lg"
>
  <div class="group">
    <GroupTitle
      name={group.name}
      color={group.color}
      {edit}
      {onEdit}
      {onSubmit}
      {onColor}
    />
    <EditActions show={!edit} {onEdit} {onRemove} classext="top-3 right-3" />
  </div>
  <div
    use:dragContainer={TASK_DND}
    use:dropData={group.id}
    class="flex flex-col gap-2 pb-10 min-w-0"
  >
    {#each tasksCardInfos as info}
      <TaskCard groupId={group.id} {info} />
    {/each}
    <AddTask {addTask} />
  </div>
</div>
