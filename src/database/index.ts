export interface Board {
  groups: Group[];
}

export interface Group {
  id: string;
  name: string;
  tasks: Task[];
}

export interface Task {
  id: string;
  body: string;
}
