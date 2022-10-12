<script lang="ts">
  import type { TaskCardInfo } from "../database";
  import { movableItem } from "./drag/UseMovableItem";
  import { dropData } from "./drag/UseDropData";
  import { TASK_DND } from "../database/store";
  import EditActions from "./EditActions.svelte";
  import TaskBody from "./TaskBody.svelte";
  import { change_task_body_or_create, remove_task } from "../backend";
  import EditingTag from "./EditingTag.svelte";

  export let info: TaskCardInfo;
  export let groupId: string;

  const onEdit = () => (info.edit = true);
  const onRemove = () => remove_task(groupId, info.id);

  function onSubmit(this: HTMLTextAreaElement) {
    change_task_body_or_create(groupId, info.id, this.value);
    info.edit = false;
  }
</script>

<div
  class="group relative bg-neutral-300 dark:bg-neutral-500 p-4 rounded-md {!info.edit &&
    'cursor-grab active:cursor-grabbing'} shadow-md"
  use:dropData={info.id || ""}
  use:movableItem={{ key: TASK_DND, movable: !info.edit }}
>
  <EditingTag show={info.edit} />
  <TaskBody body={info.body} edit={info.edit} {onEdit} {onSubmit} />
  <EditActions {onEdit} {onRemove} show={!info.edit} />
</div>
