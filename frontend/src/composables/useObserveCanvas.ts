import { ref, watch, type Ref } from 'vue'
import type { FrameMessage } from '../types'
import { useLog } from './useLog'

const PREVIEW_INTERVAL_MS = 100

export function useObserveCanvas(
  canvasRef: Ref<HTMLCanvasElement | null>,
  previewFrame: Ref<FrameMessage | null>,
  targetWidth: number,
  targetHeight: number
) {
  const { addLog } = useLog()
  const lastUpdateTime = ref(0)

  // 实际 FPS 追踪
  const actualFps = ref(0)
  let lastFrameTime = 0
  let frameCount = 0
  let fpsUpdateTime = 0

  function renderFrame(frame: FrameMessage) {
    const canvas = canvasRef.value
    if (!canvas) return

    try {
      const ctx = canvas.getContext('2d')
      if (!ctx) return

      // 1. Canvas 尺寸固定为目标分辨率
      canvas.width = targetWidth
      canvas.height = targetHeight

      // 2. 计算缩放比例（最长边等长）
      const scaleX = targetWidth / frame.width
      const scaleY = targetHeight / frame.height
      const scale = Math.min(scaleX, scaleY)

      // 3. 居中偏移
      const offsetX = (targetWidth - frame.width * scale) / 2
      const offsetY = (targetHeight - frame.height * scale) / 2

      // 4. 清空为黑色背景
      ctx.fillStyle = '#000000'
      ctx.fillRect(0, 0, canvas.width, canvas.height)

      // 5. 遍历所有 crop blocks（带缩放渲染）
      for (const block of frame.data) {
        if (!block || !block.image) continue

        // Base64 解码为原始 RGBA 字节
        const binaryString = atob(block.image)
        const bytes = new Uint8Array(binaryString.length)
        for (let i = 0; i < binaryString.length; i++) {
          bytes[i] = binaryString.charCodeAt(i)
        }

        // 尺寸一致性检查
        const expectedLen = block.w * block.h * 4
        if (bytes.length !== expectedLen) {
          addLog(`裁切块尺寸不匹配: 期望 ${expectedLen} 字节, 实际 ${bytes.length} 字节`, 'error', 'observe')
          continue
        }

        // 离屏 Canvas 解码（putImageData 不支持缩放）
        const offscreen = new OffscreenCanvas(block.w, block.h)
        const offCtx = offscreen.getContext('2d')
        if (!offCtx) continue

        const imageData = offCtx.createImageData(block.w, block.h)
        imageData.data.set(bytes)
        offCtx.putImageData(imageData, 0, 0)

        // 缩放绘制到主 Canvas
        ctx.drawImage(
          offscreen,
          0, 0, block.w, block.h,
          offsetX + block.x * scale,
          offsetY + block.y * scale,
          block.w * scale,
          block.h * scale
        )
      }

      // 6. 绘制裁切框叠加（带缩放）
      drawCropRegions(ctx, frame, scale, offsetX, offsetY)
    } catch (e) {
      addLog(`帧渲染异常: ${e}`, 'error', 'observe')
    }
  }

  function drawCropRegions(
    ctx: CanvasRenderingContext2D,
    frame: FrameMessage,
    scale: number,
    offsetX: number,
    offsetY: number
  ) {
    ctx.strokeStyle = '#00ff00'
    ctx.lineWidth = 2 / scale // 线宽按比例缩放保持视觉一致
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

  function resetCanvas() {
    const canvas = canvasRef.value
    if (!canvas) return

    actualFps.value = 0
    lastFrameTime = 0
    frameCount = 0
  }

  // 监听帧变化，节流渲染
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

    renderFrame(frame)
  })

  return {
    actualFps,
    resetCanvas,
  }
}
