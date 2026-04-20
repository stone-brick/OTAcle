<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useObserve } from '../../composables/useObserve'

const {
  isObserving,
  previewFrame,
  config,
  loadConfig,
  saveConfig,
  startListening,
  stopListening,
  getFirstSessionStats,
  formatBytes,
} = useObserve()

// ZMQ 地址（来自 communication 模块）
const zmqAddress = ref('tcp://127.0.0.1:5556')

// 当前会话统计
const sessionStats = computed(() => getFirstSessionStats())

// Canvas ref
const canvasRef = ref<HTMLCanvasElement | null>(null)

// FPS 追踪
const actualFps = ref(0)
let lastFrameTime = 0
let frameCount = 0
let fpsUpdateTime = 0
const lastUpdateTime = ref(0)
const PREVIEW_INTERVAL_MS = 100

// 加载 ZMQ 地址
async function loadZmqAddress() {
  try {
    const addr = await invoke<string>('comm_get_pub_address')
    zmqAddress.value = addr
  } catch (e) {
    console.error('Failed to load ZMQ address:', e)
  }
}

// 保存 ZMQ 地址
async function saveZmqAddress() {
  try {
    await invoke('comm_set_pub_address', { addr: zmqAddress.value })
  } catch (e) {
    console.error('Failed to save ZMQ address:', e)
    throw e
  }
}

// 配置路径状态
const configPath = ref('')

// 监听预览帧
watch(previewFrame, (frame) => {
  if (!frame) return

  const now = Date.now()
  if (now - lastUpdateTime.value < PREVIEW_INTERVAL_MS) return
  lastUpdateTime.value = now

  // 计算实际 FPS
  if (lastFrameTime > 0) {
    const elapsed = now - lastFrameTime
    if (elapsed > 0) {
      frameCount++
      if (now - fpsUpdateTime >= 500) {
        actualFps.value = Math.round((frameCount * 1000) / (now - fpsUpdateTime))
        frameCount = 0
        fpsUpdateTime = now
      }
    }
  } else {
    fpsUpdateTime = now
  }
  lastFrameTime = now

  renderFrame(frame)
})

function renderFrame(frame: typeof previewFrame.value) {
  if (!frame || !canvasRef.value) return

  const canvas = canvasRef.value
  const ctx = canvas.getContext('2d')
  if (!ctx) return

  const targetWidth = config.value.capture.target_width
  const targetHeight = config.value.capture.target_height

  canvas.width = targetWidth
  canvas.height = targetHeight

  const scaleX = targetWidth / frame.width
  const scaleY = targetHeight / frame.height
  const scale = Math.min(scaleX, scaleY)

  const offsetX = (targetWidth - frame.width * scale) / 2
  const offsetY = (targetHeight - frame.height * scale) / 2

  ctx.fillStyle = '#000000'
  ctx.fillRect(0, 0, canvas.width, canvas.height)

  for (const block of frame.data) {
    if (!block || !block.image) continue

    const binaryString = atob(block.image)
    const bytes = new Uint8Array(binaryString.length)
    for (let i = 0; i < binaryString.length; i++) {
      bytes[i] = binaryString.charCodeAt(i)
    }

    const expectedLen = block.w * block.h * 4
    if (bytes.length !== expectedLen) continue

    const offscreen = new OffscreenCanvas(block.w, block.h)
    const offCtx = offscreen.getContext('2d')
    if (!offCtx) continue

    const imageData = offCtx.createImageData(block.w, block.h)
    imageData.data.set(bytes)
    offCtx.putImageData(imageData, 0, 0)

    ctx.drawImage(
      offscreen,
      0, 0, block.w, block.h,
      offsetX + block.x * scale,
      offsetY + block.y * scale,
      block.w * scale,
      block.h * scale
    )
  }

  // Draw crop regions
  ctx.strokeStyle = '#00ff00'
  ctx.lineWidth = 2 / scale
  ctx.setLineDash([5 / scale, 5 / scale])

  for (const block of frame.data) {
    ctx.strokeRect(
      offsetX + block.x * scale,
      offsetY + block.y * scale,
      block.w * scale,
      block.h * scale
    )
  }

  ctx.setLineDash([])
}

async function handleLoad() {
  const path = prompt('输入配置文件路径:', configPath.value || 'observe.json')
  if (path) {
    configPath.value = path
    await loadConfig(path)
  }
}

async function handleSave() {
  if (configPath.value) {
    await saveConfig(configPath.value)
  } else {
    handleSaveAs()
  }
}

async function handleSaveAs() {
  const path = prompt('输入新配置文件路径:', 'observe.json')
  if (path) {
    configPath.value = path
    await saveConfig(path)
  }
}

onMounted(async () => {
  await loadZmqAddress()
  await startListening()
})

onUnmounted(() => {
  stopListening()
})
</script>

<template>
  <div class="transport-panel">
    <!-- 预览区域（左侧） -->
    <div class="preview-area">
      <canvas
        ref="canvasRef"
        class="preview-canvas"
      />

      <!-- 未观察时的提示 -->
      <div
        v-if="!isObserving"
        class="preview-placeholder"
      >
        <p>请先在"截图配置"中启动观察</p>
      </div>
    </div>

    <!-- 配置面板（右侧） -->
    <div class="config-panel">
      <h3>传输配置</h3>

      <!-- ZMQ 地址 -->
      <div class="config-group">
        <label>ZMQ 地址</label>
        <input
          v-model="zmqAddress"
          type="text"
          placeholder="tcp://127.0.0.1:5556"
          @change="saveZmqAddress"
        >
      </div>

      <!-- FPS 指示器 -->
      <div class="config-group">
        <div class="status-row">
          <span class="status-label">实际帧率</span>
          <span class="status-value">{{ actualFps }} fps</span>
        </div>
      </div>

      <!-- 裁切区域信息 -->
      <div class="config-group">
        <label>裁切区域</label>
        <div class="crop-info">
          <span>{{ config.crop_regions.length }} 个区域</span>
          <span
            v-if="config.crop_regions.length > 0"
            class="crop-hint"
          >
            （在"裁切配置"中编辑）
          </span>
        </div>
      </div>

      <!-- 配置文件操作 -->
      <div class="config-group">
        <label>配置文件</label>
        <div class="config-actions">
          <button
            class="config-btn"
            @click="handleLoad"
          >
            加载
          </button>
          <button
            class="config-btn"
            @click="handleSave"
          >
            保存
          </button>
          <button
            class="config-btn"
            @click="handleSaveAs"
          >
            另存为
          </button>
        </div>
      </div>

      <!-- 连接状态 -->
      <div class="config-group">
        <div class="status-row">
          <span class="status-label">观察状态</span>
          <span :class="['status-value', isObserving ? 'active' : 'inactive']">
            {{ isObserving ? '采集中' : '已停止' }}
          </span>
        </div>
      </div>

      <!-- ZMQ 统计 -->
      <div class="config-group">
        <label>ZMQ 传输</label>
        <div class="stats-grid">
          <div class="stat-item">
            <span class="stat-label">连接状态</span>
            <span :class="['stat-value', sessionStats?.zmq_connected ? 'success' : 'error']">
              {{ sessionStats?.zmq_connected ? '已连接' : '未连接' }}
            </span>
          </div>
          <div class="stat-item">
            <span class="stat-label">发送消息</span>
            <span class="stat-value">{{ sessionStats?.zmq_messages_sent ?? 0 }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">发送数据</span>
            <span class="stat-value">{{ formatBytes(sessionStats?.bytes_sent ?? 0) }}</span>
          </div>
          <div class="stat-item">
            <span class="stat-label">发送帧数</span>
            <span class="stat-value">{{ sessionStats?.frames_captured ?? 0 }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- 操作栏 -->
    <div class="action-bar">
      <div class="action-info">
        <span class="info-label">ZMQ PUB → Python SUB</span>
        <span class="info-hint">图像帧通过 ZMQ 实时传输到下游处理程序</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.transport-panel {
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

.config-group label {
  font-size: 12px;
  color: var(--color-text-secondary);
}

.config-group input[type="text"] {
  padding: 6px 10px;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  font-size: 13px;
  background: var(--color-surface);
  color: var(--color-text);
}

.config-group input:focus {
  outline: none;
  border-color: var(--color-primary);
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

.crop-info {
  padding: 8px;
  background: var(--color-surface-secondary);
  border-radius: var(--radius-md);
  font-size: 12px;
  color: var(--color-text-secondary);
}

.crop-hint {
  color: var(--color-text-muted);
  font-size: 11px;
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

.stat-value.success {
  color: var(--color-success, #22c55e);
}

.stat-value.error {
  color: var(--color-error, #dc2626);
}

.config-actions {
  display: flex;
  gap: 8px;
}

.config-btn {
  flex: 1;
  padding: 6px 10px;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  font-size: 12px;
  background: var(--color-surface);
  color: var(--color-text);
  cursor: pointer;
  transition: background-color 0.15s;
}

.config-btn:hover {
  background: var(--color-hover);
}

.action-bar {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 280px;
  display: flex;
  align-items: center;
  padding: 12px 16px;
  border-top: 1px solid var(--color-border);
  background: var(--color-surface);
}

.action-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.info-label {
  font-size: 13px;
  font-weight: 500;
  color: var(--color-text);
}

.info-hint {
  font-size: 11px;
  color: var(--color-text-muted);
}
</style>
