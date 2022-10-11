import { DRAG_HANDLE_KEY } from "./DragContext";

export const dragHandle = (node: HTMLElement) => {
  node.setAttribute(DRAG_HANDLE_KEY, "true");
  node.childNodes.forEach((child: Node) => {
    if (child.nodeType != child.COMMENT_NODE)
      (child as HTMLElement).setAttribute("style", "pointer-events: none");
  });
};
