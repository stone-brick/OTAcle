// Window information from Rust backend
export interface WindowInfo {
  hwnd: number;
  title: string;
  className: string;
  processName: string;
  pid: number;
  isVisible: boolean;
}

// Input backend type
export type InputBackend = 'enigo' | 'win32';

// Mouse button type
export type MouseButton = 'left' | 'right' | 'middle';

// Action types from action config
export interface KeyAction {
  type: 'key';
  key: string;
  holdTimeMs?: number;
}

export interface KeySequenceAction {
  type: 'key_sequence';
  keys: string[];
  intervalMs?: number;
}

export interface MouseClickAction {
  type: 'mouse_click';
  button: MouseButton;
  count: number;
  intervalMs?: number;
}

export interface MouseMoveAction {
  type: 'mouse_move';
  x: number;
  y: number;
  durationMs?: number;
}

export interface TextAction {
  type: 'text';
  content: string;
}

export type Action = KeyAction | KeySequenceAction | MouseClickAction | MouseMoveAction | TextAction;

// Action config map (JSON format)
export type ActionConfig = Record<string, Action>;

// Log entry
export interface LogEntry {
  time: string;
  message: string;
  type: 'info' | 'success' | 'error';
}

// App status
export type AppStatus = 'ready' | 'sending' | 'error';
