<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useComm } from '../composables/useComm'
import { useLog } from '../composables/useLog'

const {
  isConnected,
  address,
  fetchStatus,
  start,
  stop,
  startListening,
  cleanup,
} = useComm()

const { logs } = useLog()

const commLogs = computed(() =>
  logs.value.filter(log => log.source === 'comm')
)

const inputAddress = ref('tcp://127.0.0.1:5555')

onMounted(async () => {
  await startListening()
})

onUnmounted(() => {
  cleanup()
})

async function handleStart() {
  await start(inputAddress.value)
}

async function handleStop() {
  await stop()
}
</script>

<template>
  <div class="comm-panel">
    <!-- Connection Status -->
    <div class="status-section">
      <div class="status-row">
        <div class="status-item">
          <span class="status-label">连接状态</span>
          <span
            class="status-value"
            :class="{ connected: isConnected }"
          >
            {{ isConnected ? '已连接' : '未连接' }}
          </span>
        </div>
        <div class="status-item">
          <span class="status-label">地址</span>
          <span class="status-value address">{{ address || '-' }}</span>
        </div>
        <div class="status-actions">
          <input
            v-model="inputAddress"
            type="text"
            class="address-input"
            placeholder="tcp://127.0.0.1:5555"
            :disabled="isConnected"
          >
          <button
            v-if="!isConnected"
            class="btn btn-connect"
            @click="handleStart"
          >
            连接
          </button>
          <button
            v-else
            class="btn btn-disconnect"
            @click="handleStop"
          >
            断开
          </button>
          <button
            class="btn btn-refresh"
            @click="fetchStatus"
          >
            刷新
          </button>
        </div>
      </div>
    </div>

    <!-- Message Log -->
    <div class="log-section">
      <div class="log-header">
        <h4>消息日志</h4>
      </div>
      <div class="message-list">
        <div
          v-for="(msg, index) in commLogs"
          :key="index"
          class="message-item"
          :class="msg.type"
        >
          <span class="msg-time">{{ msg.time }}</span>
          <span class="msg-content">{{ msg.message }}</span>
        </div>
        <div
          v-if="commLogs.length === 0"
          class="empty-state"
        >
          暂无消息
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.comm-panel {
  display: flex;
  flex-direction: column;
  height: 100%;
  gap: 16px;
}

.status-section {
  background: var(--color-surface);
  border-radius: var(--radius-md);
  padding: 16px;
}

.status-row {
  display: flex;
  align-items: center;
  gap: 24px;
  flex-wrap: wrap;
}

.status-item {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.status-label {
  font-size: 11px;
  color: var(--color-text-muted);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.status-value {
  font-size: 14px;
  font-weight: 600;
  color: var(--color-text);
}

.status-value.connected {
  color: var(--color-success);
}

.status-value.address {
  font-family: monospace;
  color: var(--color-text-secondary);
}

.status-actions {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-left: auto;
}

.address-input {
  width: 180px;
  padding: 6px 12px;
  font-size: 13px;
  font-family: monospace;
  background: var(--color-background);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
}

.address-input:focus {
  outline: none;
  border-color: var(--color-primary);
}

.address-input:disabled {
  opacity: 0.6;
}

.btn {
  padding: 6px 16px;
  font-size: 13px;
  font-weight: 500;
  border: none;
  border-radius: var(--radius-sm);
  cursor: pointer;
  transition: opacity var(--transition-duration);
}

.btn:hover {
  opacity: 0.85;
}

.btn-connect {
  background: var(--color-success);
  color: white;
}

.btn-disconnect {
  background: var(--color-error);
  color: white;
}

.btn-refresh {
  background: var(--color-surface-secondary);
  color: var(--color-text);
  border: 1px solid var(--color-border);
}

.log-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  background: var(--color-surface);
  border-radius: var(--radius-md);
  overflow: hidden;
}

.log-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  border-bottom: 1px solid var(--color-border);
}

.log-header h4 {
  margin: 0;
  font-size: 13px;
  font-weight: 600;
  color: var(--color-text-secondary);
}

.btn-clear {
  padding: 4px 10px;
  font-size: 11px;
  background: transparent;
  color: var(--color-text-muted);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  cursor: pointer;
}

.btn-clear:hover {
  background: var(--color-hover);
}

.message-list {
  flex: 1;
  overflow-y: auto;
  padding: 8px 16px;
  font-family: 'Consolas', 'Monaco', monospace;
  font-size: 12px;
}

.message-item {
  display: flex;
  gap: 12px;
  padding: 6px 0;
  border-bottom: 1px solid var(--color-border-light);
}

.message-item:last-child {
  border-bottom: none;
}

.msg-time {
  color: var(--color-text-muted);
  flex-shrink: 0;
}

.msg-content {
  color: var(--color-text);
  word-break: break-all;
}

.message-item.success .msg-content {
  color: var(--color-success);
}

.message-item.error .msg-content {
  color: var(--color-error);
}

.empty-state {
  text-align: center;
  padding: 32px;
  color: var(--color-text-muted);
  font-size: 13px;
}
</style>