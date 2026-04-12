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

// Scroll direction type
export type ScrollDirection = 'up' | 'down' | 'left' | 'right';

// Action types from action config
export interface KeyAction {
  type: 'key';
  key: string;
  hold_time_ms?: number;
}

export interface KeySequenceItem {
  key: string;
  hold_time_ms?: number;
  interval_ms?: number;
}

export interface KeySequenceAction {
  type: 'key_sequence';
  keys: KeySequenceItem[];
  default_interval_ms: number;
}

export interface MouseClickAction {
  type: 'mouse_click';
  button: MouseButton;
  count: string;
  interval_ms?: number;
}

export interface MouseMoveAction {
  type: 'mouse_move';
  x: string;
  y: string;
  duration_ms?: number;
}

export interface MouseScrollAction {
  type: 'mouse_scroll';
  direction: ScrollDirection;
  amount: number;
}

export interface DelayAction {
  type: 'delay';
  duration_ms: number;
}

export interface TextAction {
  type: 'text';
  content: string;
}

export type Action = KeyAction | KeySequenceAction | MouseClickAction | MouseMoveAction | MouseScrollAction | DelayAction | TextAction;

// Action config map (JSON format)
export type ActionConfig = Record<string, Action>;

// Action item with index and name (matches backend ActionItem)
export interface ActionItem {
  index: number;
  name: string | null;
  data: Action;
}

// Action config data (full config structure)
export interface ActionConfigData {
  default_backend: InputBackend;
  actions: ActionItem[];
}

// Action type for editor
export type ActionType = 'key' | 'key_sequence' | 'mouse_click' | 'mouse_move' | 'mouse_scroll' | 'delay' | 'text';

// Log entry
export interface LogEntry {
  time: string;
  message: string;
  type: 'info' | 'success' | 'error';
}

// App status
export type AppStatus = 'ready' | 'sending' | 'error';
