export interface Board {
  groups: Group[];
}

export type GroupColor =
  | "bg-blue-700"
  | "bg-red-700"
  | "bg-green-700"
  | "bg-orange-700"
  | "bg-yellow-600";
export const GROUP_COLOR_VALUES = [
  "bg-blue-700",
  "bg-red-700",
  "bg-green-700",
  "bg-orange-700",
  "bg-yellow-600",
];

export interface Group {
  id: string;
  name: string;
  color: GroupColor;
  tasks: Task[];
}

export interface Task {
  id: string;
  body: string;
}

export interface TaskCardInfo extends Task {
  edit: boolean;
}

export const taskToTaskCardInfo = (task: Task): TaskCardInfo => ({
  ...task,
  edit: false,
});

export const newEditingTaskCardInfo = (): TaskCardInfo => ({
  id: undefined,
  body: undefined,
  edit: true,
});
