import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import type { ObserveConfig, CropRegion, FrameMessage, FullFrameMessage, ObserveStatus, SessionStatus, ObserveGlobalStats } from '../types'
import { useLog } from './useLog'
import { useProjectEvents, type ProjectEvent } from './useProjectEvents'
import { useProject } from './useProject'

// 状态
const isObserving = ref(false)
const previewFrame = ref<FrameMessage | null>(null)
const fullPreviewFrame = ref<FullFrameMessage | null>(null)
const config = ref<ObserveConfig>({
  capture: { frame_rate: 3, target_width: 640, target_height: 480 },
  crop_regions: []
})
const fullStatus = ref<ObserveStatus | null>(null)

// 用于事件监听的清理函数
let unlistenFullFrame: UnlistenFn | null = null
let unlistenFrame: UnlistenFn | null = null
let unlistenError: UnlistenFn | null = null
let unlistenPubStarted: UnlistenFn | null = null
let unlistenPubError: UnlistenFn | null = null
// 标记是否正在监听
let isListeningActive = false

// 项目事件监听器
let unsubscribeProject: (() => void) | null = null

// 状态轮询
let statusPollInterval: ReturnType<typeof setInterval> | null = null

export function useObserve() {
  const { addLog } = useLog()

  // 开始状态轮询
  function startStatusPolling() {
    if (statusPollInterval) return
    statusPollInterval = setInterval(async () => {
      if (isObserving.value) {
        await fetchStatus()
      }
    }, 1000) // 每秒刷新一次
  }

  // 停止状态轮询
  function stopStatusPolling() {
    if (statusPollInterval) {
      clearInterval(statusPollInterval)
      statusPollInterval = null
    }
  }

  // 获取第一个会话的统计信息（目前仅支持单会话）
  function getFirstSessionStats(): SessionStatus | null {
    const sessions = fullStatus.value?.sessions
    if (!sessions) return null
    const keys = Object.keys(sessions)
    if (keys.length === 0) return null
    return sessions[Number(keys[0])]
  }

  // 格式化字节数
  function formatBytes(bytes: number): string {
    if (bytes < 1024) return `${bytes} B`
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
    return `${(bytes / 1024 / 1024).toFixed(1)} MB`
  }

  // 格式化运行时长
  function formatUptime(seconds: number): string {
    const h = Math.floor(seconds / 3600)
    const m = Math.floor((seconds % 3600) / 60)
    const s = seconds % 60
    if (h > 0) return `${h}:${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`
    return `${m}:${s.toString().padStart(2, '0')}`
  }

  // 捕获单帧预览
  async function capturePreview(windowId: string): Promise<void> {
    try {
      const frame = await invoke<FrameMessage>('observe_capture_preview', {
        window: windowId,
        config: config.value,
      })
      previewFrame.value = frame
      addLog(`预览已捕获: ${frame.width}x${frame.height}`, 'success', 'observe')
    } catch (e) {
      addLog(`预览捕获失败: ${e}`, 'error', 'observe')
    }
  }

  // 捕获完整帧预览（用于前端展示完整窗口 + ROI 遮罩）
  async function captureFullFrame(windowId: string): Promise<void> {
    try {
      const frame = await invoke<FullFrameMessage>('observe_capture_full_frame', {
        window: windowId,
        config: config.value,
      })
      fullPreviewFrame.value = frame
      addLog(`完整帧预览已捕获: ${frame.width}x${frame.height}`, 'success', 'observe')
    } catch (e) {
      addLog(`完整帧预览捕获失败: ${e}`, 'error', 'observe')
    }
  }

  // 开始观察
  async function startObserve(windowId: string): Promise<void> {
    previewFrame.value = null
    try {
      await invoke('observe_start', {
        window: windowId,
        config: config.value
      })
      isObserving.value = true
      await fetchStatus()
      startStatusPolling()
      addLog(`观察已启动，窗口: ${windowId}`, 'success', 'observe')
    } catch (e) {
      isObserving.value = false
      addLog(`启动观察失败: ${e}`, 'error', 'observe')
      throw e
    }
  }

  // 停止观察
  async function stopObserve(): Promise<void> {
    try {
      await invoke('observe_stop')
    } catch (e) {
      addLog(`停止观察失败: ${e}`, 'error', 'observe')
    } finally {
      isObserving.value = false
      stopStatusPolling()
      await fetchStatus()
      stopListening()
    }
  }

  // 获取观察状态
  async function fetchStatus(): Promise<ObserveStatus | null> {
    try {
      const status = await invoke<ObserveStatus>('observe_get_status')
      fullStatus.value = status
      // 兼容旧逻辑：检查是否有活跃会话
      isObserving.value = Object.values(status.sessions).some(s => s.running)
      return status
    } catch {
      addLog('获取观察状态失败', 'error', 'observe')
      return null
    }
  }

  // 获取指定会话的统计信息
  function getSessionStats(hwnd: number): SessionStatus | null {
    return fullStatus.value?.sessions[hwnd] ?? null
  }

  // 获取全局统计信息
  function getGlobalStats(): ObserveGlobalStats | null {
    return fullStatus.value?.global_stats ?? null
  }

  // 加载配置
  async function loadConfig(path: string): Promise<void> {
    try {
      const loaded = await invoke<ObserveConfig>('observe_load_config', { path })
      config.value = loaded
      addLog(`已加载观察配置: ${path}`, 'success', 'observe')
    } catch {
      // 配置文件不存在时使用默认配置（自动加载场景下不抛出错误）
      config.value = {
        capture: { frame_rate: 3, target_width: 640, target_height: 480 },
        crop_regions: []
      }
      addLog('观察配置文件不存在，使用默认配置', 'info', 'observe')
    }
  }

  // 保存配置
  async function saveConfig(path: string): Promise<void> {
    try {
      await invoke('observe_save_config', { path, config: config.value })
      addLog(`已保存观察配置: ${path}`, 'success', 'observe')
    } catch (e) {
      addLog(`保存观察配置失败: ${e}`, 'error', 'observe')
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

    // 监听完整帧事件（事件驱动，替代轮询）
    unlistenFullFrame = await listen<FullFrameMessage>('observe:full_frame', (event) => {
      fullPreviewFrame.value = event.payload
    })

    // 监听错误事件
    unlistenError = await listen<{ error: string }>('observe:error', () => {
      addLog('观察捕获发生错误', 'error', 'observe')
    })

    // 监听帧传输启动事件
    unlistenPubStarted = await listen('observe:pub_started', () => {
      addLog('图像帧传输已启动', 'success', 'observe')
    })

    // 监听帧传输错误事件
    unlistenPubError = await listen('observe:pub_error', () => {
      addLog('图像帧传输发生错误', 'error', 'observe')
    })

    isListeningActive = true
  }

  // 停止监听事件
  function stopListening(): void {
    if (!isListeningActive) return
    unlistenFullFrame?.()
    unlistenFrame?.()
    unlistenError?.()
    unlistenPubStarted?.()
    unlistenPubError?.()
    unlistenFullFrame = null
    unlistenFrame = null
    unlistenError = null
    unlistenPubStarted = null
    unlistenPubError = null
    isListeningActive = false
  }

  // 初始化项目事件监听（项目打开时自动加载配置）
  function initProjectEventListener(): void {
    if (unsubscribeProject) {
      unsubscribeProject()
    }

    const { onProjectEvent } = useProjectEvents()
    const { getProjectObserveConfigPath } = useProject()

    unsubscribeProject = onProjectEvent(async (event: ProjectEvent) => {
      if (event.type === 'opened' && event.project) {
        const cfgPath = await getProjectObserveConfigPath()
        if (cfgPath) {
          try {
            await loadConfig(cfgPath)
          } catch {
            addLog('自动加载观察配置文件失败，使用默认配置', 'warn', 'observe');
          }
        }
      } else if (event.type === 'closed') {
        config.value = {
          capture: { frame_rate: 3, target_width: 640, target_height: 480 },
          crop_regions: []
        }
      }
    })
  }

  return {
    // 状态
    isObserving,
    previewFrame,
    fullPreviewFrame,
    config,
    fullStatus,

    // 方法
    capturePreview,
    captureFullFrame,
    startObserve,
    stopObserve,
    fetchStatus,
    getSessionStats,
    getGlobalStats,
    getFirstSessionStats,
    formatBytes,
    formatUptime,
    loadConfig,
    saveConfig,
    updateCropRegions,
    addCropRegion,
    removeCropRegion,
    startListening,
    stopListening,
    initProjectEventListener,
  }
}
