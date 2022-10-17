<script lang="ts">
  import AppIcon from "./AppIcon.svelte";
  import GithubLogin from "./GithubLogin.svelte";
  import { UserLoginState, userLoginState } from "../database/store";
  import { CircleNotch, Warning } from "phosphor-svelte";
</script>

<div
  class="fixed w-full h-full flex flex-col justify-center items-center gap-10 backdrop-blur bg-neutral-800/10 z-40"
>
  <div class="flex flex-col justify-center items-center">
    <AppIcon size={150} />
    <h1 class="text-3xl">todo_list</h1>
  </div>
  {#if $userLoginState == UserLoginState.NEEDS_SIGN_UP}
    <GithubLogin />
  {:else if $userLoginState == UserLoginState.LOADING}
    <CircleNotch size={40} {...{ class: "animate-spin" }} />
  {:else if $userLoginState == UserLoginState.ERROR}
    <Warning size={40} />
  {/if}
</div>
