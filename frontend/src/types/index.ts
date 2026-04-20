// 窗口信息（来自 Rust 后端）
export interface WindowInfo {
  hwnd: number;
  title: string;
  class_name: string;
  process_name: string;
  pid: number;
  is_visible: boolean;
}

// 输入后端类型
export type InputBackend = 'enigo' | 'win32';

// 鼠标按钮类型
export type MouseButton = 'left' | 'right' | 'middle';

// 滚动方向类型
export type ScrollDirection = 'up' | 'down' | 'left' | 'right';

// 变量定义 - 将参数名映射到动作字段名
export interface Variable {
  param_name: string;
  field_name: string;
}

// =============================================================================
// ActionItem 类型（统一结构 - 替代分离的 Action 和 ActionItem 类型）
// =============================================================================

// 所有动作类型的基接口 - 公共字段
export interface ActionItemBase {
  name?: string | null;
  backend?: InputBackend | null;
}

// 按键动作 - 单次按键
export interface KeyActionItem extends ActionItemBase {
  type: 'key';
  key: string;
  hold_time_ms?: number;
}

// 按键序列项
export interface KeySequenceItem {
  key: string;
  hold_time_ms?: number;
  interval_ms?: number | null;
}

// 按键序列动作 - 多次按键
export interface KeySequenceActionItem extends ActionItemBase {
  type: 'key_sequence';
  keys: KeySequenceItem[];
  default_interval_ms: number;
}

// 鼠标点击动作
export interface MouseClickActionItem extends ActionItemBase {
  type: 'mouse_click';
  button: MouseButton;
  count: number;
  interval_ms?: number | null;
  hold_time_ms?: number;
  variables?: Variable[];
}

// 鼠标移动动作
export interface MouseMoveActionItem extends ActionItemBase {
  type: 'mouse_move';
  x: number;
  y: number;
  duration_ms?: number | null;
  variables?: Variable[];
}

// 鼠标滚动动作
export interface MouseScrollActionItem extends ActionItemBase {
  type: 'mouse_scroll';
  direction: ScrollDirection;
  amount: number;
}

// 延迟动作
export interface DelayActionItem extends ActionItemBase {
  type: 'delay';
  duration_ms: number;
}

// 文本动作
export interface TextActionItem extends ActionItemBase {
  type: 'text';
  content: string;
}

// 所有动作项的联合类型
export type ActionItem =
  | KeyActionItem
  | KeySequenceActionItem
  | MouseClickActionItem
  | MouseMoveActionItem
  | MouseScrollActionItem
  | DelayActionItem
  | TextActionItem;

// ActionItem 类型用于编辑器（与 ActionItem 相同 - 保持向后兼容）
export type ActionType = ActionItem['type'];

// 动作配置数据（完整配置结构）
export interface ActionConfigData {
  default_backend: InputBackend;
  actions: ActionItem[];
}

// 日志条目
export interface LogEntry {
  time: string;
  message: string;
  type: 'info' | 'success' | 'error';
}

// 应用状态
export type AppStatus = 'ready' | 'sending' | 'error';

// 窗口搜索模式
export type SearchMode = 'title' | 'title_contains' | 'class' | 'pid' | 'exe' | 'hwnd';

// =============================================================================
// Observe 模块类型
// =============================================================================

// 截图配置
export interface CaptureConfig {
  frame_rate: number;
  target_width: number;
  target_height: number;
}

// 帧内裁切区域
export interface CropRegion {
  x: number;
  y: number;
  w: number;
  h: number;
}

// Observe 模块配置
export interface ObserveConfig {
  capture: CaptureConfig;
  crop_regions: CropRegion[];
}

// 帧内单个裁切块
export interface CropBlock {
  x: number;
  y: number;
  w: number;
  h: number;
  image: string;  // Base64 编码
}

// 通过 ZMQ 和 Tauri 事件发送的帧消息
export interface FrameMessage {
  width: number;
  height: number;
  timestamp: number;
  frame_id: number;
  data: CropBlock[];
}

// 会话运行时状态
export interface SessionStatus {
  hwnd: number;
  running: boolean;
  uptime_seconds: number;
  config: ObserveConfig;
  frames_captured: number;
  bytes_sent: number;
  errors_count: number;
  zmq_connected: boolean;
  zmq_messages_sent: number;
}

// Observe 全局统计
export interface ObserveGlobalStats {
  total_frames_captured: number;
  total_bytes_sent: number;
  total_errors: number;
  active_sessions: number;
}

// Observe 完整状态
export interface ObserveStatus {
  sessions: Record<number, SessionStatus>;
  global_stats: ObserveGlobalStats;
}
