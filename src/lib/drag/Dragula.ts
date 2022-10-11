import dragula, { type DragulaOptions } from "dragula";
import { setContext } from "svelte";
import {
  DRAG_HANDLE_KEY,
  keyToItemDataAttribute,
  type DragContext,
} from "./DragContext";

interface CustomDragulaOptions extends DragulaOptions {
  checkHandle?: boolean;
}

export const makeDrake = (key: string, options: CustomDragulaOptions = {}) => {
  const moves = (el: Element, _: Element, handle: Element) => {
    return (
      el.hasAttribute(keyToItemDataAttribute(key)) &&
      (options.checkHandle ? handle.hasAttribute(DRAG_HANDLE_KEY) : true)
    );
  };

  setContext<DragContext>(key, {
    getDrake: () => drake,
    addContainer: (container: HTMLElement) => {
      drake.containers.push(container);
    },
  });

  const drake = dragula({
    ...options,
    moves: (e, c, h, s) =>
      moves(e, c, h) &&
      (options.moves !== undefined ? options.moves(e, c, h, s) : true),
  });

  return drake;
};
