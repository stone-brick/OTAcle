import { ref, watchEffect, type Ref, type ComputedRef } from 'vue'
import type { FullFrameMessage, CropRegion } from '../types'
import { useLog } from './useLog'

const PREVIEW_INTERVAL_MS = 100

export function useObserveCanvas(
  canvasRef: Ref<HTMLCanvasElement | null>,
  fullPreviewFrame: Ref<FullFrameMessage | null>,
  cropRegions: ComputedRef<CropRegion[]>,
  targetWidth: ComputedRef<number>,
  targetHeight: ComputedRef<number>
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

    const tw = targetWidth.value
    const th = targetHeight.value

    try {
      const ctx = canvas.getContext('2d')
      if (!ctx) return

      // Canvas 尺寸固定为目标分辨率
      canvas.width = tw
      canvas.height = th

      // 计算缩放比例（最长边等长）
      const scaleX = tw / frame.width
      const scaleY = th / frame.height
      const scale = Math.min(scaleX, scaleY)

      // 居中偏移
      const offsetX = (tw - frame.width * scale) / 2
      const offsetY = (th - frame.height * scale) / 2

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
        frame.width * scale, frame.height * scale
      )

      // 绘制 ROI 遮罩：非选中区域叠加半透明灰色，选中区域绘制绿色边框
      const regions = cropRegions.value

      if (regions.length === 0) {
        // 无 ROI 配置，不做遮罩
        return
      }

      // 绘制 ROI 区域绿色边框
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

      // 绘制非 ROI 区域的半透明灰色遮罩
      // 思路：绘制整个画面的半透明灰色遮罩，然后挖掉 ROI 区域
      ctx.fillStyle = 'rgba(128, 128, 128, 0.5)'

      // 方式：绘制整个画面，然后在每个 ROI 区域"挖洞"（用 clearRect 或绘制原图）
      // 为了简化，我们用 destination-out 合成来挖洞

      // 首先，创建一个临时 canvas 来存储完整图像
      const tempCanvas = document.createElement('canvas')
      tempCanvas.width = canvas.width
      tempCanvas.height = canvas.height
      const tempCtx = tempCanvas.getContext('2d')
      if (!tempCtx) return

      // 绘制完整图像到临时 canvas
      tempCtx.drawImage(offscreen, 0, 0, frame.width, frame.height, offsetX, offsetY, frame.width * scale, frame.height * scale)

      // 在临时 canvas 上绘制 ROI 边框
      tempCtx.strokeStyle = '#00ff00'
      tempCtx.lineWidth = 2 / scale
      tempCtx.setLineDash([5 / scale, 5 / scale])
      for (const region of regions) {
        tempCtx.strokeRect(
          offsetX + region.x * scale,
          offsetY + region.y * scale,
          region.w * scale,
          region.h * scale
        )
      }
      tempCtx.setLineDash([])

      // 在临时 canvas 上绘制半透明灰色遮罩
      tempCtx.fillStyle = 'rgba(128, 128, 128, 0.5)'
      tempCtx.fillRect(0, 0, tempCanvas.width, tempCanvas.height)

      // 用 destination-out 挖掉 ROI 区域
      tempCtx.globalCompositeOperation = 'destination-out'
      tempCtx.fillStyle = 'rgba(0, 0, 0, 1)'
      for (const region of regions) {
        tempCtx.fillRect(
          offsetX + region.x * scale,
          offsetY + region.y * scale,
          region.w * scale,
          region.h * scale
        )
      }
      tempCtx.globalCompositeOperation = 'source-over'

      // 将处理后的临时 canvas 绘制到主 canvas
      ctx.drawImage(tempCanvas, 0, 0)

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
