import { keyToItemDataAttribute } from "./DragContext";

interface Options {
  key: string;
  movable?: boolean;
}

export const movableItem = (
  node: HTMLElement,
  keyOrOptions: string | Options
) => {
  if (typeof keyOrOptions == "string")
    node.setAttribute(keyToItemDataAttribute(keyOrOptions), "true");
  else if (keyOrOptions.movable)
    node.setAttribute(keyToItemDataAttribute(keyOrOptions.key), "true");
};
