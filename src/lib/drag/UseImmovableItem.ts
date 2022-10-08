import { ImmovableItemKey } from "./DragContext";

export const immovableItem = (node: HTMLElement) => {
  node.classList.add(ImmovableItemKey);
};
