<script lang="ts">
  export let avatarUrl = "";
  export let name = "";
  export let htmlUrl = "";
</script>

<a
  href={htmlUrl}
  target="_blank"
  class="flex items-center gap-3 p-3 pr-5 bg-neutral-500 rounded-tr-xl"
>
  <img src={avatarUrl} alt="Github Avatar Url" class="rounded-full w-7 h-7" />
  <h2 class="text-lg font-bold">{name}</h2>
</a>
