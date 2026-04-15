import { ref } from 'vue';
import type { LogEntry } from '../types';

// ============================================================================
// Module-level Singleton State
// ============================================================================
const logs = ref<LogEntry[]>([]);

function formatTime(): string {
  const now = new Date();
  return now.toLocaleTimeString('zh-CN', { hour12: false });
}

function addLog(message: string, type: LogEntry['type'] = 'info'): void {
  logs.value.push({
    time: formatTime(),
    message,
    type,
  });
}

function clearLogs(): void {
  logs.value = [];
}

// ============================================================================
// Singleton Instance
// ============================================================================
const logService = {
  /**
   * Logs array - module-level singleton exposed directly.
   *
   * NOTE: This is intentionally a direct ref (not wrapped in readonly)
   * to maintain compatibility with existing component prop types.
   * The singleton nature ensures all components share the same state.
   *
   * External modification should be avoided - use addLog() and clearLogs() instead.
   */
  logs,

  addLog,

  clearLogs,
};

// ============================================================================
// Composable Wrapper - 保持向后兼容
// ============================================================================

/**
 * useLog - Vue composable compatible interface
 *
 * Returns the singleton log service instance.
 * Despite the "use" prefix suggesting new instances, this always returns
 * the same singleton, ensuring centralized logging across the application.
 *
 * @example
 * // For reading and clearing logs
 * const { logs, clearLogs } = useLog()
 *
 * @example
 * // For adding logs (common in other composables)
 * const { addLog } = useLog()
 * addLog('Operation completed', 'success')
 */
export function useLog() {
  return logService;
}

// ============================================================================
// Named Export - 可选的直接访问方式
// ============================================================================

/**
 * LogService - Direct singleton access without composable wrapper.
 *
 * Alternative to useLog() for cases where the composable pattern is not needed.
 * Provides the same interface as useLog().
 *
 * @example
 * import { LogService } from '@/composables/useLog'
 * LogService.addLog('Direct call', 'info')
 */
export { logService as LogService };
