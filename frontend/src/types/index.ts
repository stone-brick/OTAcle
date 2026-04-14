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

// Variable definition - maps a parameter name to an action field name
export interface Variable {
  param_name: string;
  field_name: string;
}

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
  count: number;
  interval_ms?: number;
  variables?: Variable[];
}

export interface MouseMoveAction {
  type: 'mouse_move';
  x: number;
  y: number;
  duration_ms?: number;
  variables?: Variable[];
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

// Action item types - flattened to match Rust backend's #[serde(flatten)]
// Each variant has index, name, type, and action-specific fields at top level

export interface KeyActionItem {
  name: string | null;
  type: 'key';
  key: string;
  hold_time_ms?: number;
  backend?: InputBackend;
}

export interface KeySequenceActionItem {
  name: string | null;
  type: 'key_sequence';
  keys: KeySequenceItem[];
  default_interval_ms: number;
  backend?: InputBackend;
}

export interface MouseClickActionItem {
  name: string | null;
  type: 'mouse_click';
  button: MouseButton;
  count: number;
  interval_ms?: number;
  hold_time_ms?: number;
  backend?: InputBackend;
  variables?: Variable[];
}

export interface MouseMoveActionItem {
  name: string | null;
  type: 'mouse_move';
  x: number;
  y: number;
  duration_ms?: number;
  backend?: InputBackend;
  variables?: Variable[];
}

export interface MouseScrollActionItem {
  name: string | null;
  type: 'mouse_scroll';
  direction: ScrollDirection;
  amount: number;
  backend?: InputBackend;
}

export interface DelayActionItem {
  name: string | null;
  type: 'delay';
  duration_ms: number;
}

export interface TextActionItem {
  name: string | null;
  type: 'text';
  content: string;
  backend?: InputBackend;
}

export type ActionItem =
  | KeyActionItem
  | KeySequenceActionItem
  | MouseClickActionItem
  | MouseMoveActionItem
  | MouseScrollActionItem
  | DelayActionItem
  | TextActionItem;

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

// Window search mode
export type SearchMode = 'title' | 'titleContains' | 'class' | 'pid' | 'exe' | 'hwnd';
