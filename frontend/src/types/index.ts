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

// =============================================================================
// ActionItem Types (Unified - replaces separate Action and ActionItem types)
// =============================================================================

// Base interface for all action types - common fields
export interface ActionItemBase {
  name?: string | null;
  backend?: InputBackend | null;
}

// Key Action - single key press
export interface KeyActionItem extends ActionItemBase {
  type: 'key';
  key: string;
  hold_time_ms?: number;
}

// Key Sequence Item
export interface KeySequenceItem {
  key: string;
  hold_time_ms?: number;
  interval_ms?: number | null;
}

// Key Sequence Action - multiple key presses
export interface KeySequenceActionItem extends ActionItemBase {
  type: 'key_sequence';
  keys: KeySequenceItem[];
  default_interval_ms: number;
}

// Mouse Click Action
export interface MouseClickActionItem extends ActionItemBase {
  type: 'mouse_click';
  button: MouseButton;
  count: number;
  interval_ms?: number | null;
  hold_time_ms?: number;
  variables?: Variable[];
}

// Mouse Move Action
export interface MouseMoveActionItem extends ActionItemBase {
  type: 'mouse_move';
  x: number;
  y: number;
  duration_ms?: number | null;
  variables?: Variable[];
}

// Mouse Scroll Action
export interface MouseScrollActionItem extends ActionItemBase {
  type: 'mouse_scroll';
  direction: ScrollDirection;
  amount: number;
}

// Delay Action
export interface DelayActionItem extends ActionItemBase {
  type: 'delay';
  duration_ms: number;
}

// Text Action
export interface TextActionItem extends ActionItemBase {
  type: 'text';
  content: string;
}

// Union type for all action items
export type ActionItem =
  | KeyActionItem
  | KeySequenceActionItem
  | MouseClickActionItem
  | MouseMoveActionItem
  | MouseScrollActionItem
  | DelayActionItem
  | TextActionItem;

// ActionItem type for editor (same as ActionItem - kept for backwards compatibility)
export type ActionType = ActionItem['type'];

// Action config data (full config structure)
export interface ActionConfigData {
  default_backend: InputBackend;
  actions: ActionItem[];
}

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
