<script lang="ts">
  import { makeDrake } from "./lib/drag/Dragula";
  import BoardBlock from "./lib/BoardBlock.svelte";
  import { GROUP_DND, saving, TASK_DND } from "./database/store";
  import { change_task_group, update_group_pos } from "./backend";
  import { user } from "./database/store";
  import LoginScreen from "./lib/LoginScreen.svelte";
  import SaveSpinner from "./lib/SaveSpinner.svelte";
  import GithubUser from "./lib/GithubUser.svelte";

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

<main class="relative flex flex-col h-full w-full">
  <div
    data-tauri-drag-region
    class="fixed top-0 left-0 w-full h-[var(--toolbar-size)] z-50"
  />
  {#if $user == undefined}
    <LoginScreen />
  {:else}
    <div class="fixed bottom-0 left-0">
      <GithubUser
        avatarUrl={$user.user_info.avatar_url}
        name={$user.user_info.full_name}
        htmlUrl={$user.user_info.html_url}
      />
    </div>
  {/if}
  <BoardBlock />
  {#if $saving}
    <div class="fixed bottom-2 right-2">
      <SaveSpinner />
    </div>
  {/if}
</main>
