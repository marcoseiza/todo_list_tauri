import type { Drake } from "dragula";

export const DragContextKey = "drag_context";

export const keyToItemDataAttribute = (key: string) => `data-dnd-item-${key}`;
export const keyToContainerDataAttribute = (key: string) =>
  `data-dnd-container-${key}`;
export const DRAG_HANDLE_KEY = "data-drag-handle";

export interface DragContext {
  getDrake: () => Drake;
  addContainer: (container: HTMLElement) => void;
}
