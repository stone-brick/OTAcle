import { ref } from 'vue';
import type { LogEntry } from '../types';
import { MAX_LOG_ENTRIES } from '../utils/constants';

// ============================================================================
// 模块级单例状态
// ============================================================================
const logs = ref<LogEntry[]>([]);

function formatTime(): string {
  const now = new Date();
  return now.toLocaleTimeString('zh-CN', { hour12: false });
}

function addLog(message: string, type: LogEntry['type'] = 'info', source: LogEntry['source'] = 'system'): void {
  logs.value.push({
    time: formatTime(),
    message,
    type,
    source,
  });
  if (logs.value.length > MAX_LOG_ENTRIES) {
    logs.value.shift();
  }
}

function clearLogs(): void {
  logs.value = [];
}

// ============================================================================
// 单例实例
// ============================================================================
const logService = {
  /**
   * 日志数组 - 模块级单例，直接暴露。
   *
   * 注意：这里有意使用直接 ref（而非包装在 readonly 中）
   * 以保持与现有组件 prop 类型兼容。
   * 单例性质确保所有组件共享同一状态。
   *
   * 应避免外部修改 - 请使用 addLog() 和 clearLogs()。
   */
  logs,

  addLog,

  clearLogs,
};

// ============================================================================
// Composable 包装器 - 保持向后兼容
// ============================================================================

/**
 * useLog - Vue composable 兼容接口
 *
 * 返回单例日志服务实例。
 * 尽管"use"前缀暗示创建新实例，但此函数始终返回
 * 同一单例，确保应用程序中的集中式日志记录。
 *
 * @example
 * // 用于读取和清除日志
 * const { logs, clearLogs } = useLog()
 *
 * @example
 * // 用于添加日志（在其他 composables 中常用）
 * const { addLog } = useLog()
 * addLog('操作完成', 'success')
 */
export function useLog() {
  return logService;
}

// ============================================================================
// 命名导出 - 可选的直接访问方式
// ============================================================================

/**
 * LogService - 无需 composable 包装的直接单例访问。
 *
 * 在不需要 composable 模式时使用 useLog() 的替代方案。
 * 提供与 useLog() 相同的接口。
 *
 * @example
 * import { LogService } from '@/composables/useLog'
 * LogService.addLog('直接调用', 'info')
 */
export const LogService = logService;
