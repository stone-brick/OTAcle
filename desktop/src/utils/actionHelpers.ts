import type { ActionItem } from '../types';

/**
 * 获取动作类型的中文标签
 */
export function getActionTypeLabel(item: ActionItem | undefined): string {
  if (!item) return '未知';
  switch (item.type) {
    case 'key':
      return '按键';
    case 'key_sequence':
      return '序列';
    case 'mouse_click':
      return '点击';
    case 'mouse_move':
      return '移动';
    case 'mouse_scroll':
      return '滚动';
    case 'delay':
      return '延迟';
    case 'text':
      return '文本';
    default:
      return '未知';
  }
}

/**
 * 获取动作详情描述
 */
export function formatActionDetail(item: ActionItem | undefined): string {
  if (!item) return '(无数据)';
  switch (item.type) {
    case 'key':
      return item.key || '(未设置)';
    case 'key_sequence':
      return `${item.keys.length} 个按键`;
    case 'mouse_click':
      const pos = item.x !== undefined || item.y !== undefined ? ` @(${item.x ?? 0},${item.y ?? 0})` : '';
      return `${item.button}键 x${item.count}${pos}`;
    case 'mouse_move':
      return `(${item.x}, ${item.y})`;
    case 'mouse_scroll':
      return `${item.direction} ${item.amount}`;
    case 'delay':
      return `${item.duration_ms}ms`;
    case 'text': {
      const content = item.content.length > 15 ? item.content.substring(0, 15) + '...' : item.content;
      return `"${content}"`;
    }
    default:
      return '';
  }
}

