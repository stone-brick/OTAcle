import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import type { ObserveConfig, CropRegion, FrameMessage } from '../types'

// 状态
const isObserving = ref(false)
const previewFrame = ref<FrameMessage | null>(null)
const config = ref<ObserveConfig>({
  capture: { frame_rate: 20, target_width: 640, target_height: 480 },
  zmq: { address: 'tcp://127.0.0.1:5556' },
  crop_regions: []
})

// 用于事件监听的清理函数
let unlistenFrame: UnlistenFn | null = null
let unlistenError: UnlistenFn | null = null
// 标记是否正在监听
let isListeningActive = false

export function useObserve() {
  // 开始观察
  async function startObserve(windowId: string): Promise<void> {
    try {
      await invoke('observe_start', {
        window: windowId,
        config: config.value
      })
      isObserving.value = true
    } catch (e) {
      isObserving.value = false
      throw e
    }
  }

  // 停止观察
  async function stopObserve(): Promise<void> {
    try {
      await invoke('observe_stop')
    } finally {
      isObserving.value = false
      // 确保停止后清理监听器
      stopListening()
    }
  }

  // 获取观察状态
  async function fetchStatus(): Promise<void> {
    try {
      const status = await invoke<boolean>('observe_get_status')
      isObserving.value = status
    } catch (e) {
      console.error('Failed to get observe status:', e)
    }
  }

  // 加载配置
  async function loadConfig(path: string): Promise<void> {
    try {
      const loaded = await invoke<ObserveConfig>('observe_load_config', { path })
      config.value = loaded
    } catch (e) {
      console.error('Failed to load observe config:', e)
      throw e
    }
  }

  // 保存配置
  async function saveConfig(path: string): Promise<void> {
    try {
      await invoke('observe_save_config', { path, config: config.value })
    } catch (e) {
      console.error('Failed to save observe config:', e)
      throw e
    }
  }

  // 更新裁切区域
  function updateCropRegions(regions: CropRegion[]): void {
    config.value.crop_regions = regions
  }

  // 添加裁切区域
  function addCropRegion(region: CropRegion): void {
    config.value.crop_regions.push(region)
  }

  // 删除裁切区域
  function removeCropRegion(index: number): void {
    config.value.crop_regions.splice(index, 1)
  }

  // 开始监听事件
  async function startListening(): Promise<void> {
    if (isListeningActive) return

    // 获取初始状态
    await fetchStatus()

    // 监听预览帧事件
    unlistenFrame = await listen<FrameMessage>('observe:frame', (event) => {
      previewFrame.value = event.payload
    })

    // 监听错误事件
    unlistenError = await listen<{ error: string }>('observe:error', (event) => {
      console.error('Observe error:', event.payload.error)
    })

    isListeningActive = true
  }

  // 停止监听事件
  function stopListening(): void {
    if (!isListeningActive) return
    unlistenFrame?.()
    unlistenError?.()
    unlistenFrame = null
    unlistenError = null
    isListeningActive = false
  }

  return {
    // 状态
    isObserving,
    previewFrame,
    config,

    // 方法
    startObserve,
    stopObserve,
    fetchStatus,
    loadConfig,
    saveConfig,
    updateCropRegions,
    addCropRegion,
    removeCropRegion,
    startListening,
    stopListening,
  }
}
