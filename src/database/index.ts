export interface User {
  avatar_url: string;
  html_url: string;
  firebase_auth_token: string;
  firebase_uid: string;
  full_name: string;
  board: Board;
}

export interface Board {
  groups: Group[];
}

export const DEFAULT_BOARD: Board = { groups: [] };
export const DEFAULT_USER: User = {
  avatar_url: "",
  html_url: "",
  firebase_auth_token: "",
  firebase_uid: "",
  full_name: "",
  board: DEFAULT_BOARD,
};

export enum GroupColor {
  BLUE = "BLUE",
  GREEN = "GREEN",
  RED = "RED",
  ORANGE = "ORANGE",
  YELLOW = "YELLOW",
}

export const GroupColorToValue = (color: GroupColor): string => {
  switch (color) {
    case GroupColor.BLUE:
      return "bg-blue-500 dark:bg-blue-700";
    case GroupColor.GREEN:
      return "bg-green-500 dark:bg-green-700";
    case GroupColor.RED:
      return "bg-red-500 dark:bg-red-700";
    case GroupColor.ORANGE:
      return "bg-orange-500 dark:bg-orange-700";
    case GroupColor.YELLOW:
      return "bg-yellow-500 dark:bg-yellow-600";
  }
};

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
