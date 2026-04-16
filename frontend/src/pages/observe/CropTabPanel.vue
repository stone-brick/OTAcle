<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'
import { useObserve } from '../../composables/useObserve'
import type { CropRegion } from '../../types'

const {
  isObserving,
  previewFrame,
  config,
  addCropRegion,
  removeCropRegion,
  startListening,
  stopListening,
} = useObserve()

// Canvas ref
const canvasRef = ref<HTMLCanvasElement | null>(null)

// ROI drawing state
const isDrawing = ref(false)
const drawStart = ref({ x: 0, y: 0 })
const currentRect = ref({ x: 0, y: 0, w: 0, h: 0 })
const selectedRegionIndex = ref<number | null>(null)

// FPS tracking
const actualFps = ref(0)
let lastFrameTime = 0
let frameCount = 0
let fpsUpdateTime = 0
const lastUpdateTime = ref(0)
const PREVIEW_INTERVAL_MS = 100

// Watch preview frames
watch(previewFrame, (frame) => {
  if (!frame) return

  const now = Date.now()
  if (now - lastUpdateTime.value < PREVIEW_INTERVAL_MS) return
  lastUpdateTime.value = now

  // Calculate actual FPS
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

  // Draw frame data
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

  // Draw existing crop regions
  ctx.strokeStyle = '#00ff00'
  ctx.lineWidth = 2 / scale
  ctx.setLineDash([5 / scale, 5 / scale])

  config.value.crop_regions.forEach((region, index) => {
    ctx.strokeStyle = selectedRegionIndex.value === index ? '#ffff00' : '#00ff00'
    ctx.strokeRect(
      offsetX + region.x * scale,
      offsetY + region.y * scale,
      region.w * scale,
      region.h * scale
    )
  })

  // Draw current selection highlight
  if (selectedRegionIndex.value !== null) {
    const region = config.value.crop_regions[selectedRegionIndex.value]
    if (region) {
      ctx.strokeStyle = '#ffff00'
      ctx.lineWidth = 3 / scale
      ctx.strokeRect(
        offsetX + region.x * scale,
        offsetY + region.y * scale,
        region.w * scale,
        region.h * scale
      )
    }
  }

  // Draw current drawing rect
  if (isDrawing.value && (currentRect.value.w > 0 || currentRect.value.h > 0)) {
    ctx.strokeStyle = '#ff0000'
    ctx.lineWidth = 2 / scale
    ctx.setLineDash([])
    ctx.strokeRect(
      currentRect.value.x,
      currentRect.value.y,
      currentRect.value.w,
      currentRect.value.h
    )
  }

  ctx.setLineDash([])
}

// Canvas mouse events for ROI drawing
function getCanvasCoords(e: MouseEvent) {
  if (!canvasRef.value) return { x: 0, y: 0 }
  const rect = canvasRef.value.getBoundingClientRect()
  return {
    x: e.clientX - rect.left,
    y: e.clientY - rect.top
  }
}

function handleMouseDown(e: MouseEvent) {
  if (!isObserving.value) return
  const coords = getCanvasCoords(e)
  isDrawing.value = true
  drawStart.value = coords
  currentRect.value = { x: coords.x, y: coords.y, w: 0, h: 0 }
}

function handleMouseMove(e: MouseEvent) {
  if (!isDrawing.value) return
  const coords = getCanvasCoords(e)
  const x = Math.min(drawStart.value.x, coords.x)
  const y = Math.min(drawStart.value.y, coords.y)
  const w = Math.abs(coords.x - drawStart.value.x)
  const h = Math.abs(coords.y - drawStart.value.y)
  currentRect.value = { x, y, w, h }
}

function handleMouseUp() {
  if (!isDrawing.value) return
  isDrawing.value = false

  // Only add if rect is big enough
  if (currentRect.value.w > 10 && currentRect.value.h > 10) {
    // Convert canvas coords to frame coords
    const canvas = canvasRef.value
    if (!canvas) return

    const targetWidth = config.value.capture.target_width
    const targetHeight = config.value.capture.target_height
    const scaleX = targetWidth / canvas.width
    const scaleY = targetHeight / canvas.height
    const scale = Math.min(scaleX, scaleY)

    const offsetX = (targetWidth - canvas.width * scale) / 2
    const offsetY = (targetHeight - canvas.height * scale) / 2

    const newRegion: CropRegion = {
      x: Math.round((currentRect.value.x - offsetX) / scale),
      y: Math.round((currentRect.value.y - offsetY) / scale),
      w: Math.round(currentRect.value.w / scale),
      h: Math.round(currentRect.value.h / scale),
    }

    addCropRegion(newRegion)
  }

  currentRect.value = { x: 0, y: 0, w: 0, h: 0 }
}

function handleCanvasClick(e: MouseEvent) {
  if (isObserving.value) return // Don't select when drawing

  const coords = getCanvasCoords(e)
  const targetWidth = config.value.capture.target_width
  const targetHeight = config.value.capture.target_height
  const canvas = canvasRef.value
  if (!canvas) return

  const scaleX = targetWidth / canvas.width
  const scaleY = targetHeight / canvas.height
  const scale = Math.min(scaleX, scaleY)
  const offsetX = (targetWidth - canvas.width * scale) / 2
  const offsetY = (targetHeight - canvas.height * scale) / 2

  const clickX = (coords.x - offsetX) / scale
  const clickY = (coords.y - offsetY) / scale

  // Find clicked region
  const clickedIndex = config.value.crop_regions.findIndex(region => {
    return clickX >= region.x && clickX <= region.x + region.w &&
           clickY >= region.y && clickY <= region.y + region.h
  })

  selectedRegionIndex.value = clickedIndex !== -1 ? clickedIndex : null
}

function handleAddRegion() {
  // Enter draw mode hint
  if (!isObserving.value) {
    alert('请先在"截图配置"中启动观察')
    return
  }
  selectedRegionIndex.value = null
}

function handleDeleteRegion() {
  if (selectedRegionIndex.value !== null) {
    removeCropRegion(selectedRegionIndex.value)
    selectedRegionIndex.value = null
  }
}

onMounted(async () => {
  await startListening()
})

onUnmounted(() => {
  stopListening()
})
</script>

<template>
  <div class="crop-panel">
    <!-- Preview area (left side) -->
    <div class="preview-area">
      <canvas
        ref="canvasRef"
        class="preview-canvas"
        @mousedown="handleMouseDown"
        @mousemove="handleMouseMove"
        @mouseup="handleMouseUp"
        @mouseleave="handleMouseUp"
        @click="handleCanvasClick"
        :class="{ drawing: isDrawing }"
      />

      <!-- Hint when not observing -->
      <div v-if="!isObserving" class="preview-placeholder">
        <p>请先在"截图配置"中启动观察</p>
      </div>
    </div>

    <!-- Config panel (right side) -->
    <div class="config-panel">
      <h3>裁切配置</h3>

      <!-- FPS indicator -->
      <div class="config-group">
        <div class="status-row">
          <span class="status-label">实际帧率</span>
          <span class="status-value">{{ actualFps }} fps</span>
        </div>
      </div>

      <!-- Crop regions list -->
      <div class="config-group">
        <label>裁切区域列表</label>
        <div class="crop-regions">
          <div
            v-for="(region, index) in config.crop_regions"
            :key="index"
            class="crop-region-item"
            :class="{ selected: selectedRegionIndex === index }"
            @click="selectedRegionIndex = index"
          >
            <span class="region-index">#{{ index }}</span>
            <span class="region-coords">{{ region.x }}, {{ region.y }}</span>
            <span class="region-size">{{ region.w }} × {{ region.h }}</span>
          </div>
          <p v-if="config.crop_regions.length === 0" class="no-regions">
            暂无裁切区域
          </p>
        </div>
      </div>

      <!-- Instructions -->
      <div class="config-group">
        <div class="instructions">
          <p v-if="isObserving">在预览区拖拽绘制矩形添加裁切区域</p>
          <p v-else>启动观察后可在预览区绘制裁切区域</p>
        </div>
      </div>
    </div>

    <!-- Action bar -->
    <div class="action-bar">
      <button @click="handleAddRegion">
        添加区域
      </button>
      <button @click="handleDeleteRegion" :disabled="selectedRegionIndex === null">
        删除选中
      </button>
    </div>
  </div>
</template>

<style scoped>
.crop-panel {
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
  cursor: crosshair;
}

.preview-canvas.drawing {
  cursor: crosshair;
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

.crop-regions {
  background: var(--color-surface-secondary);
  border-radius: var(--radius-md);
  padding: 8px;
  max-height: 200px;
  overflow-y: auto;
}

.crop-region-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 8px;
  font-size: 12px;
  color: var(--color-text-secondary);
  border-bottom: 1px solid var(--color-border);
  cursor: pointer;
  border-radius: var(--radius-sm);
}

.crop-region-item:last-child {
  border-bottom: none;
}

.crop-region-item:hover {
  background: var(--color-hover);
}

.crop-region-item.selected {
  background: var(--color-primary-bg);
  border-color: var(--color-primary);
}

.region-index {
  font-weight: 600;
  color: var(--color-primary);
}

.region-coords {
  flex: 1;
}

.region-size {
  color: var(--color-text-muted);
}

.no-regions {
  margin: 0;
  padding: 8px;
  font-size: 12px;
  color: var(--color-text-muted);
  font-style: italic;
  text-align: center;
}

.instructions {
  padding: 8px;
  background: var(--color-surface-secondary);
  border-radius: var(--radius-md);
}

.instructions p {
  margin: 0;
  font-size: 12px;
  color: var(--color-text-secondary);
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
