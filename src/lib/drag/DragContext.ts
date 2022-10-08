import type { Drake } from "dragula";

export const DragContextKey = "drag_context";
export const ImmovableItemKey = "dnd-immovable-item";

export interface DragContext {
  getDrake: () => Drake;
  addContainer: (container: HTMLElement) => void;
}
