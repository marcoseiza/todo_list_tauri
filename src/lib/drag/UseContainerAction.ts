import { getContext } from "svelte";
import type { DragContext } from "./DragContext";

export const dragContainer = (node: HTMLElement, key: string) => {
  const { addContainer } = getContext<DragContext>(key);
  addContainer(node);
};
