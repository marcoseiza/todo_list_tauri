<script>
  import AppIcon from "./AppIcon.svelte";
  import GithubLogin from "./GithubLogin.svelte";
</script>

<div
  class="fixed w-full h-full flex flex-col justify-center items-center gap-10 backdrop-blur bg-neutral-800/10 z-50"
>
  <div class="flex flex-col justify-center items-center">
    <AppIcon size={150} />
    <h1 class="text-3xl">todo_list</h1>
  </div>
  <GithubLogin />
</div>
