import type {
  ActionItem,
  KeyActionItem,
  KeySequenceActionItem,
  MouseClickActionItem,
  MouseMoveActionItem,
  MouseScrollActionItem,
  DelayActionItem,
  TextActionItem,
} from './index';

export function isKeyAction(item: ActionItem): item is KeyActionItem {
  return item.type === 'key';
}

export function isKeySequenceAction(item: ActionItem): item is KeySequenceActionItem {
  return item.type === 'key_sequence';
}

export function isMouseClickAction(item: ActionItem): item is MouseClickActionItem {
  return item.type === 'mouse_click';
}

export function isMouseMoveAction(item: ActionItem): item is MouseMoveActionItem {
  return item.type === 'mouse_move';
}

export function isMouseScrollAction(item: ActionItem): item is MouseScrollActionItem {
  return item.type === 'mouse_scroll';
}

export function isDelayAction(item: ActionItem): item is DelayActionItem {
  return item.type === 'delay';
}

export function isTextAction(item: ActionItem): item is TextActionItem {
  return item.type === 'text';
}

export function getActionTypeLabel(type: ActionItem['type']): string {
  const labels: Record<ActionItem['type'], string> = {
    key: '按键',
    key_sequence: '按键序列',
    mouse_click: '鼠标点击',
    mouse_move: '鼠标移动',
    mouse_scroll: '鼠标滚动',
    delay: '延迟',
    text: '文本输入',
  };
  return labels[type] || type;
}
