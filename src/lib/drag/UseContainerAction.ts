import { getContext } from "svelte";
import { DragContextKey, type DragContext } from "./DragContext";

export const dragContainer = (node: HTMLElement) => {
  const { addContainer } = getContext<DragContext>(DragContextKey);
  addContainer(node);
};
