import { ref } from 'vue';
import type { LogEntry } from '../types';

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

export function useLog() {
  return {
    logs,
    addLog,
    clearLogs,
  };
}
