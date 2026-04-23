import { ref, watchEffect, type Ref, type ComputedRef } from 'vue'
import type { FullFrameMessage, CropRegion } from '../types'
import { useLog } from './useLog'

const PREVIEW_INTERVAL_MS = 100

// 最大显示尺寸（按原图比例放大）
const MAX_DISPLAY_WIDTH = 800
const MAX_DISPLAY_HEIGHT = 600

export function useObserveCanvas(
  canvasRef: Ref<HTMLCanvasElement | null>,
  fullPreviewFrame: Ref<FullFrameMessage | null>,
  cropRegions: ComputedRef<CropRegion[]>
) {
  const { addLog } = useLog()
  const lastUpdateTime = ref(0)

  // 实际 FPS 追踪
  const actualFps = ref(0)
  let lastFrameTime = 0
  let frameCount = 0
  let fpsUpdateTime = 0

  // 解码 Base64 图像数据
  function decodeBase64Image(base64: string, width: number, height: number): Uint8Array {
    const binaryString = atob(base64)
    const bytes = new Uint8Array(binaryString.length)
    for (let i = 0; i < binaryString.length; i++) {
      bytes[i] = binaryString.charCodeAt(i)
    }
    const expectedLen = width * height * 4
    if (bytes.length !== expectedLen) {
      addLog(`图像尺寸不匹配: 期望 ${expectedLen} 字节, 实际 ${bytes.length} 字节`, 'error', 'observe')
      return new Uint8Array(0)
    }
    return bytes
  }

  // 渲染完整帧（用于前端预览，带 ROI 遮罩）
  function renderFullFrame(frame: FullFrameMessage) {
    const canvas = canvasRef.value
    if (!canvas) return

    // 计算显示尺寸：按原图比例放大到填满最大尺寸
    const scale = Math.max(
      MAX_DISPLAY_WIDTH / frame.width,
      MAX_DISPLAY_HEIGHT / frame.height
    )
    const displayWidth = Math.round(frame.width * scale)
    const displayHeight = Math.round(frame.height * scale)

    try {
      const ctx = canvas.getContext('2d')
      if (!ctx) return

      // Canvas 尺寸设为计算出的显示尺寸
      canvas.width = displayWidth
      canvas.height = displayHeight

      // 偏移为 0，居中由 CSS 处理
      const offsetX = 0
      const offsetY = 0

      // 绘制黑色背景
      ctx.fillStyle = '#000000'
      ctx.fillRect(0, 0, canvas.width, canvas.height)

      // 解码完整图像
      const bytes = decodeBase64Image(frame.image, frame.width, frame.height)
      if (bytes.length === 0) return

      // 绘制完整图像
      const offscreen = new OffscreenCanvas(frame.width, frame.height)
      const offCtx = offscreen.getContext('2d')
      if (!offCtx) return

      const imageData = offCtx.createImageData(frame.width, frame.height)
      imageData.data.set(bytes)
      offCtx.putImageData(imageData, 0, 0)

      ctx.drawImage(
        offscreen,
        0, 0, frame.width, frame.height,
        offsetX, offsetY,
        displayWidth, displayHeight
      )

      const regions = cropRegions.value

      if (regions.length === 0) {
        // 无 ROI 配置，只绘制图像
        return
      }

      // 创建临时 canvas 绘制遮罩
      const maskCanvas = document.createElement('canvas')
      maskCanvas.width = canvas.width
      maskCanvas.height = canvas.height
      const maskCtx = maskCanvas.getContext('2d')
      if (!maskCtx) return

      // 整个区域填充半透明灰色
      maskCtx.fillStyle = 'rgba(128, 128, 128, 0.5)'
      maskCtx.fillRect(
        offsetX,
        offsetY,
        displayWidth,
        displayHeight
      )

      // 用 destination-out 挖出 ROI 区域（ROI 变透明）
      maskCtx.globalCompositeOperation = 'destination-out'
      maskCtx.fillStyle = 'rgba(0, 0, 0, 1)'
      for (const region of regions) {
        maskCtx.fillRect(
          offsetX + region.x * scale,
          offsetY + region.y * scale,
          region.w * scale,
          region.h * scale
        )
      }

      // 将遮罩叠加到主 canvas
      ctx.drawImage(maskCanvas, 0, 0)

      // 绘制 ROI 边框
      ctx.strokeStyle = '#00ff00'
      ctx.lineWidth = 2 / scale
      ctx.setLineDash([5 / scale, 5 / scale])
      for (const region of regions) {
        ctx.strokeRect(
          offsetX + region.x * scale,
          offsetY + region.y * scale,
          region.w * scale,
          region.h * scale
        )
      }
      ctx.setLineDash([])

    } catch (e) {
      addLog(`完整帧渲染异常: ${e}`, 'error', 'observe')
    }
  }

  function resetCanvas() {
    const canvas = canvasRef.value
    if (!canvas) return

    actualFps.value = 0
    lastFrameTime = 0
    frameCount = 0
  }

  // 监听帧变化，节流渲染
  watchEffect(() => {
    const fullFrame = fullPreviewFrame.value
    const canvas = canvasRef.value
    if (!canvas) return

    const now = Date.now()
    if (now - lastUpdateTime.value < PREVIEW_INTERVAL_MS) return
    lastUpdateTime.value = now

    // 计算实际 FPS
    if (lastFrameTime > 0) {
      const elapsed = now - lastFrameTime
      if (elapsed > 0) {
        frameCount++
        // 每 500ms 更新一次 FPS
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

    // 只渲染完整帧（事件驱动）
    if (fullFrame) {
      renderFullFrame(fullFrame)
    }
  })

  return {
    actualFps,
    resetCanvas,
  }
}
