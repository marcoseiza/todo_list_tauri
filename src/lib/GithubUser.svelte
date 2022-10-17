<script lang="ts">
  export let avatarUrl = "";
  export let name = "";
  export let htmlUrl = "";
</script>

<a
  href={htmlUrl}
  target="_blank"
  class="flex items-center gap-3 p-2 pr-4 pb-3 bg-neutral-800 rounded-tr-xl opacity-70 hover:opacity-100 transition-opacity"
>
  <img src={avatarUrl} alt="Github Avatar Url" class="rounded-full w-6 h-6" />
  <h2 class="text-md font-bold">{name}</h2>
</a>
