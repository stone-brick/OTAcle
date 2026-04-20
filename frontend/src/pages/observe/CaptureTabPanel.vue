<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useObserve } from '../../composables/useObserve'
import { useWindows } from '../../composables/useWindows'
import { useObserveCanvas } from '../../composables/useObserveCanvas'

const {
  isObserving,
  previewFrame,
  config,
  startObserve,
  stopObserve,
  startListening,
  stopListening,
  getFirstSessionStats,
  formatUptime,
} = useObserve()

// 当前会话统计
const sessionStats = computed(() => getFirstSessionStats())

const { windows, selectedWindow, refreshWindows, selectWindow } = useWindows()

// Canvas ref
const canvasRef = ref<HTMLCanvasElement | null>(null)

// 推导的目标尺寸
const targetWidth = computed(() => config.value.capture.target_width)
const targetHeight = computed(() => config.value.capture.target_height)

// Canvas 渲染
const { actualFps, resetCanvas } = useObserveCanvas(canvasRef, previewFrame, targetWidth.value, targetHeight.value)

// 错误状态
const errorMessage = ref<string | null>(null)

function clearError() {
  errorMessage.value = null
}

async function handleStart() {
  if (!selectedWindow.value) {
    errorMessage.value = '请先选择一个窗口'
    return
  }
  try {
    errorMessage.value = null
    resetCanvas()
    await startObserve(selectedWindow.value.hwnd.toString())
  } catch (e) {
    errorMessage.value = `启动观察失败: ${e}`
  }
}

async function handleStop() {
  try {
    await stopObserve()
  } catch (e) {
    errorMessage.value = `停止观察失败: ${e}`
  }
}

function handleWindowChange() {
  if (selectedWindow.value) {
    selectWindow(selectedWindow.value.hwnd)
  }
}

onMounted(async () => {
  await refreshWindows()
  await startListening()
})

onUnmounted(() => {
  stopListening()
})
</script>

<template>
  <div class="capture-panel">
    <!-- Preview area (left side) -->
    <div class="preview-area">
      <canvas
        ref="canvasRef"
        class="preview-canvas"
      />

      <!-- Error banner -->
      <div
        v-if="errorMessage"
        class="error-banner"
      >
        <span>{{ errorMessage }}</span>
        <button
          class="error-close"
          @click="clearError"
        >
          ×
        </button>
      </div>

      <!-- Placeholder when not observing -->
      <div
        v-if="!isObserving && !errorMessage"
        class="preview-placeholder"
      >
        <p>选择窗口后点击"开始观察"启动预览</p>
      </div>
    </div>

    <!-- Config panel (right side) -->
    <div class="config-panel">
      <h3>截图配置</h3>

      <!-- Window selection -->
      <div class="config-group">
        <div class="label-row">
          <label>目标窗口</label>
          <button
            class="refresh-btn"
            title="刷新窗口列表"
            @click="refreshWindows"
          >
            ↻
          </button>
        </div>
        <select
          v-model="selectedWindow"
          @change="handleWindowChange"
        >
          <option
            :value="null"
            disabled
          >
            选择窗口...
          </option>
          <option
            v-for="win in windows"
            :key="win.hwnd"
            :value="win"
          >
            {{ win.title || 'Untitled' }}
          </option>
        </select>
      </div>

      <!-- Frame rate -->
      <div class="config-group">
        <label>采集帧率</label>
        <input
          v-model.number="config.capture.frame_rate"
          type="number"
          min="1"
          max="60"
        >
      </div>

      <!-- Target resolution -->
      <div class="config-group">
        <label>目标分辨率</label>
        <div class="resolution-inputs">
          <input
            v-model.number="config.capture.target_width"
            type="number"
            placeholder="宽度"
          >
          <span>×</span>
          <input
            v-model.number="config.capture.target_height"
            type="number"
            placeholder="高度"
          >
        </div>
      </div>

      <!-- Status indicator -->
      <div class="config-group">
        <div class="status-row">
          <span class="status-label">状态</span>
          <span :class="['status-value', isObserving ? 'active' : 'inactive']">
            {{ isObserving ? '采集中' : '已停止' }}
          </span>
        </div>
      </div>

      <!-- FPS indicator -->
      <div class="config-group">
        <div class="status-row">
          <span class="status-label">实际帧率</span>
          <span class="status-value">{{ actualFps }} fps</span>
        </div>
      </div>

      <!-- Statistics -->
      <div class="config-group">
        <label>采集统计</label>
        <div class="stats-grid">
          <div class="stat-item">
            <span class="stat-label">帧数</span>
            <span class="stat-value">{{ sessionStats?.frames_captured ?? 0 }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">运行时长</span>
            <span class="stat-value">{{ formatUptime(sessionStats?.uptime_seconds ?? 0) }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">错误</span>
            <span class="stat-value">{{ sessionStats?.errors_count ?? 0 }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">发送数据</span>
            <span class="stat-value">{{ sessionStats?.bytes_sent ?? 0 }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Action bar -->
    <div class="action-bar">
      <button
        :disabled="isObserving"
        @click="handleStart"
      >
        开始观察
      </button>
      <button
        :disabled="!isObserving"
        @click="handleStop"
      >
        停止观察
      </button>
    </div>
  </div>
</template>

<style scoped>
.capture-panel {
  display: flex;
  flex-direction: row;
  height: 100%;
  position: relative;
  background: var(--color-surface);
}

.preview-area {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--color-background);
  position: relative;
  min-width: 0;
  padding: 16px;
}

.preview-canvas {
  max-width: 100%;
  max-height: 100%;
  object-fit: contain;
}

.error-banner {
  position: absolute;
  top: 8px;
  left: 8px;
  right: 8px;
  padding: 8px 12px;
  background: var(--color-error-bg, #fee2e2);
  color: var(--color-error, #dc2626);
  border-radius: var(--radius-md);
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 13px;
}

.error-close {
  background: none;
  border: none;
  font-size: 16px;
  cursor: pointer;
  padding: 0 4px;
  color: inherit;
}

.preview-placeholder {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--color-text-muted);
  pointer-events: none;
}

.config-panel {
  width: 280px;
  flex-shrink: 0;
  padding: 16px;
  border-left: 1px solid var(--color-border);
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.config-panel h3 {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--color-text);
}

.config-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.label-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.label-row label {
  margin: 0;
}

.refresh-btn {
  padding: 2px 6px;
  background: none;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  cursor: pointer;
  font-size: 12px;
  color: var(--color-text-secondary);
}

.refresh-btn:hover {
  background: var(--color-surface-secondary);
}

.config-group label {
  font-size: 12px;
  color: var(--color-text-secondary);
}

.config-group select,
.config-group input[type="text"],
.config-group input[type="number"] {
  padding: 6px 10px;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  font-size: 13px;
  background: var(--color-surface);
  color: var(--color-text);
}

.config-group select:focus,
.config-group input:focus {
  outline: none;
  border-color: var(--color-primary);
}

.resolution-inputs {
  display: flex;
  align-items: center;
  gap: 8px;
}

.resolution-inputs input {
  flex: 1;
  min-width: 0;
}

.resolution-inputs span {
  color: var(--color-text-muted);
  flex-shrink: 0;
}

.status-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px;
  background: var(--color-surface-secondary);
  border-radius: var(--radius-md);
}

.status-label {
  font-size: 12px;
  color: var(--color-text-secondary);
}

.status-value {
  font-size: 12px;
  font-weight: 500;
}

.status-value.active {
  color: var(--color-success, #22c55e);
}

.status-value.inactive {
  color: var(--color-text-muted);
}

.stats-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 8px;
}

.stat-item {
  display: flex;
  justify-content: space-between;
  padding: 6px 8px;
  background: var(--color-surface-secondary);
  border-radius: var(--radius-sm);
  font-size: 12px;
}

.stat-label {
  color: var(--color-text-secondary);
}

.stat-value {
  font-weight: 500;
}

.action-bar {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 280px;
  display: flex;
  gap: 8px;
  padding: 12px 16px;
  border-top: 1px solid var(--color-border);
  background: var(--color-surface);
}

.action-bar button {
  flex: 1;
  padding: 8px 16px;
  border: none;
  border-radius: var(--radius-md);
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: background-color 0.15s;
}

.action-bar button:first-child {
  background: var(--color-primary);
  color: white;
}

.action-bar button:first-child:hover:not(:disabled) {
  background: var(--color-primary-hover);
}

.action-bar button:last-child {
  background: var(--color-surface-secondary);
  color: var(--color-text);
}

.action-bar button:last-child:hover:not(:disabled) {
  background: var(--color-border);
}

.action-bar button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}
</style>
