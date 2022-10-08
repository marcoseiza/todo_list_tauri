<script lang="ts">
  import dragula, { type DragulaOptions } from "dragula";
  import { setContext } from "svelte";
  import {
    DragContextKey,
    ImmovableItemKey,
    type DragContext,
  } from "./DragContext";

  const moves = (el: Element) => {
    return !el.classList.contains(ImmovableItemKey);
  };

  export let options: DragulaOptions = {};
  export let onDrop: (
    el: Element,
    target: Element,
    source: Element,
    sibling: Element
  ) => void = () => {};

  setContext<DragContext>(DragContextKey, {
    getDrake: () => drake,
    addContainer: (container: HTMLElement) => {
      drake.containers.push(container);
    },
  });

  const drake = dragula({
    ...options,
    moves: (e, c, h, s) =>
      moves(e) &&
      (options.moves !== undefined ? options.moves(e, c, h, s) : true),
  });

  drake.on("drop", onDrop);
</script>

<slot />
