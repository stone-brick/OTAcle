import { ref, effectScope } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { useLog } from './useLog'
import { useProjectEvents, type ProjectEvent } from './useProjectEvents'
import { useProject } from './useProject'
import type { ObserveConfig, ActionItem, InputBackend, DecisionLog, ThinkConfig } from '../types'

interface CommConfig {
  act_pull_address: string
  observe_pub_address: string
  think_pull_address: string
}

interface CommStatus {
  pull_running: boolean
  pub_running: boolean
  pull_address: string
  pub_address: string
  pub_connected: string
  pub_messages_sent: number
  pub_bytes_sent: number
  pub_last_error: string | null
}

interface WindowInfo {
  hwnd: number
  title: string
  class_name: string
  process_name: string
  process_id: number
}

const isConnected = ref(false)
const address = ref('')
const config = ref<CommConfig>({
  act_pull_address: 'tcp://127.0.0.1:5555',
  observe_pub_address: 'tcp://127.0.0.1:5556',
  think_pull_address: 'tcp://127.0.0.1:5557',
})

const scope = effectScope()

let unlistenLog: UnlistenFn | null = null
let unlistenError: UnlistenFn | null = null
let unsubscribeProject: (() => void) | null = null

// 各通信通道运行状态
const actPullRunning = ref(false)
const observePubRunning = ref(false)
const thinkPullRunning = ref(false)

export function useComm() {
  const { addLog } = useLog()

  async function fetchStatus() {
    try {
      const status = await invoke<CommStatus>('comm_get_status')
      isConnected.value = status.pull_running
      address.value = status.pull_address
      return status
    } catch (e) {
      addLog(`获取通信状态失败: ${e}`, 'error', 'comm')
      return null
    }
  }

  // 地址设置方法
  async function setPullAddress(addr: string) {
    await invoke('comm_set_pull_address', { addr })
  }

  async function setPubAddress(addr: string) {
    await invoke('comm_set_pub_address', { addr })
  }

  async function setThinkPullAddress(addr: string) {
    config.value.think_pull_address = addr
  }

  // Act PULL 通信
  async function startActPull() {
    try {
      await invoke('comm_set_pull_address', { addr: config.value.act_pull_address })
      await invoke('comm_start_pull')
      actPullRunning.value = true
      addLog(`Act PULL 已启动: ${config.value.act_pull_address}`, 'success', 'comm')
      await fetchStatus()
    } catch (e) {
      addLog('启动 Act PULL 失败', 'error', 'comm')
      throw e
    }
  }

  async function stopActPull() {
    try {
      await invoke('comm_stop_pull')
      actPullRunning.value = false
      await fetchStatus()
    } catch (e) {
      addLog(`停止 Act PULL 失败: ${e}`, 'error', 'comm')
    }
  }

  // Observe PUB 通信
  async function startObserve(windowId: string, observeConfig: ObserveConfig) {
    try {
      await invoke('observe_start', { window: windowId, config: observeConfig })
      observePubRunning.value = true
      addLog(`Observe PUB 已启动, 窗口: ${windowId}`, 'success', 'observe')
    } catch (e) {
      addLog(`启动 Observe PUB 失败: ${e}`, 'error', 'observe')
      throw e
    }
  }

  async function stopObserve() {
    try {
      await invoke('observe_stop')
      observePubRunning.value = false
    } catch (e) {
      addLog(`停止 Observe PUB 失败: ${e}`, 'error', 'observe')
    }
  }

  // Think PULL 通信
  async function startThinkPull() {
    try {
      await invoke('think_start')
      thinkPullRunning.value = true
      addLog(`Think PULL 已启动: ${config.value.think_pull_address}`, 'success', 'think')
    } catch (e) {
      addLog('启动 Think PULL 失败', 'error', 'think')
      throw e
    }
  }

  async function stopThinkPull() {
    try {
      await invoke('think_stop')
      thinkPullRunning.value = false
    } catch (e) {
      addLog(`停止 Think PULL 失败: ${e}`, 'error', 'think')
    }
  }

  // 向后兼容：原有 start/stop 方法委托给 startActPull/stopActPull
  async function start(address_: string) {
    config.value.act_pull_address = address_
    await startActPull()
  }

  async function stop() {
    await stopActPull()
  }

  async function startListening() {
    await fetchStatus()

    unlistenLog = await listen<string>('comm:log', (event) => {
      addLog(event.payload, 'info', 'comm')
    })

    unlistenError = await listen<string>('comm:error', (event) => {
      addLog(event.payload, 'error', 'comm')
    })
  }

  function stopListening() {
    unlistenLog?.()
    unlistenError?.()
    unlistenLog = null
    unlistenError = null
  }

  function cleanup() {
    stopListening()
    if (unsubscribeProject) {
      unsubscribeProject()
      unsubscribeProject = null
    }
    scope.stop()
  }

  async function loadConfig(path: string) {
    try {
      const loaded = await invoke<CommConfig>('comm_load_config', { path })
      config.value = loaded
      addLog(`Comm 配置已加载: ${path}`, 'success', 'comm')
    } catch {
      config.value = {
        act_pull_address: 'tcp://127.0.0.1:5555',
        observe_pub_address: 'tcp://127.0.0.1:5556',
        think_pull_address: 'tcp://127.0.0.1:5557',
      }
      await invoke('comm_save_config', { path, config: config.value })
      addLog(`已创建默认通信配置文件: ${path}`, 'info', 'comm')
    }
  }

  async function saveConfig(path: string) {
    try {
      await invoke('comm_save_config', { path, config: config.value })
      addLog(`已保存通信配置: ${path}`, 'success', 'comm')
    } catch (e) {
      addLog(`保存通信配置失败: ${e}`, 'error', 'comm')
      throw e
    }
  }

  // ============================================================
  // Act 模块命令（统一通过 useComm 调用）
  // ============================================================

  // Act 配置
  async function actLoadConfig(path: string, backend: InputBackend): Promise<void> {
    await invoke('act_load_config', { path, backend })
  }

  async function actSaveConfig(path: string, defaultBackend: InputBackend, actions: ActionItem[]): Promise<void> {
    await invoke('act_save_config', { path, defaultBackend, actions })
  }

  async function actGetList(): Promise<ActionItem[]> {
    return await invoke<ActionItem[]>('act_get_list')
  }

  async function actGetDefaultBackend(): Promise<InputBackend> {
    return await invoke<InputBackend>('act_get_default_backend')
  }

  async function actSetDefaultBackend(backend: InputBackend): Promise<void> {
    await invoke('act_set_default_backend', { backend })
  }

  async function actClearHistory(): Promise<void> {
    await invoke('act_clear_history')
  }

  async function actGetHistoryStatus(): Promise<[number, number]> {
    return await invoke<[number, number]>('act_get_history_status')
  }

  async function actGetNextIndex(): Promise<number> {
    return await invoke<number>('act_get_next_index')
  }

  // Act 动作管理
  async function actCreate(action: ActionItem, name: string | null): Promise<number> {
    return await invoke<number>('act_create', { action, name })
  }

  async function actUpdate(index: number, action: ActionItem, name: string | null): Promise<void> {
    await invoke('act_update', { index, action, name })
  }

  async function actDelete(index: number): Promise<void> {
    await invoke('act_delete', { index })
  }

  // Act 执行
  async function actExecuteAction(actionIdx: number): Promise<void> {
    await invoke('act_execute_action', { actionIdx })
  }

  async function actExecuteActionWithParams(actionIdx: number, params: Record<string, number | string>): Promise<void> {
    await invoke('act_execute_action_with_params', { actionIdx, params })
  }

  // Act 撤销/重做
  async function actUndo(): Promise<void> {
    await invoke('act_undo')
  }

  async function actRedo(): Promise<void> {
    await invoke('act_redo')
  }

  async function actDiscardAll(): Promise<void> {
    await invoke('act_discard_all')
  }

  // ============================================================
  // Observe 模块命令（统一通过 useComm 调用）
  // ============================================================

  async function observeCapturePreview(window: string, observeConfig: ObserveConfig) {
    return await invoke('observe_capture_preview', { window, config: observeConfig })
  }

  async function observeCaptureFullFrame(window: string, observeConfig: ObserveConfig) {
    return await invoke('observe_capture_full_frame', { window, config: observeConfig })
  }

  async function observeGetStatus() {
    return await invoke('observe_get_status')
  }

  async function observeLoadConfig(path: string) {
    return await invoke('observe_load_config', { path })
  }

  async function observeSaveConfig(path: string, observeConfig: ObserveConfig) {
    return await invoke('observe_save_config', { path, config: observeConfig })
  }

  // ============================================================
  // Think 模块命令（统一通过 useComm 调用）
  // ============================================================

  async function thinkLoadConfig(path: string): Promise<ThinkConfig> {
    return await invoke<ThinkConfig>('think_load_config', { path })
  }

  async function thinkSaveConfig(path: string, thinkConfig: ThinkConfig): Promise<void> {
    await invoke('think_save_config', { path, config: thinkConfig })
  }

  async function thinkGetLogs(limit?: number): Promise<DecisionLog[]> {
    return await invoke<DecisionLog[]>('think_get_logs', { limit })
  }

  async function thinkGetStatus() {
    return await invoke('think_get_status')
  }

  // ============================================================
  // Window 模块命令（统一通过 useComm 调用）
  // ============================================================

  async function windowList(): Promise<WindowInfo[]> {
    return await invoke<WindowInfo[]>('window_list')
  }

  async function windowGetInfo(hwnd: number): Promise<WindowInfo> {
    return await invoke<WindowInfo>('window_get_info', { hwnd })
  }

  async function windowFind(title?: string, className?: string, processName?: string): Promise<WindowInfo[]> {
    return await invoke<WindowInfo[]>('window_find', { title, className, processName })
  }

  async function windowSetTarget(window: string | null): Promise<void> {
    await invoke('window_set_target', { window })
  }

  // ============================================================
  // 统一的 initProjectEventListener（合并三个重复方法）
  // ============================================================

  function initProjectEventListener() {
    if (unsubscribeProject) {
      unsubscribeProject()
    }

    const { onProjectEvent } = useProjectEvents()
    const { getProjectCommConfigPath } = useProject()

    unsubscribeProject = onProjectEvent(async (event: ProjectEvent) => {
      if (event.type === 'opened' && event.project) {
        const cfgPath = await getProjectCommConfigPath()
        if (cfgPath) {
          try {
            await loadConfig(cfgPath)
          } catch {
            // 静默处理
          }
        }
      }
    })
  }

  return {
    isConnected,
    address,
    config,
    fetchStatus,
    start,
    stop,
    startListening,
    stopListening,
    cleanup,
    loadConfig,
    saveConfig,
    initProjectEventListener,
    // 新增：各通信通道控制
    actPullRunning,
    observePubRunning,
    thinkPullRunning,
    startActPull,
    stopActPull,
    startObserve,
    stopObserve,
    startThinkPull,
    stopThinkPull,
    // 地址设置
    setPullAddress,
    setPubAddress,
    setThinkPullAddress,

    // Act 模块命令
    actLoadConfig,
    actSaveConfig,
    actGetList,
    actGetDefaultBackend,
    actSetDefaultBackend,
    actClearHistory,
    actGetHistoryStatus,
    actGetNextIndex,
    actCreate,
    actUpdate,
    actDelete,
    actExecuteAction,
    actExecuteActionWithParams,
    actUndo,
    actRedo,
    actDiscardAll,

    // Observe 模块命令
    observeCapturePreview,
    observeCaptureFullFrame,
    observeGetStatus,
    observeLoadConfig,
    observeSaveConfig,

    // Think 模块命令
    thinkLoadConfig,
    thinkSaveConfig,
    thinkGetLogs,
    thinkGetStatus,

    // Window 模块命令
    windowList,
    windowGetInfo,
    windowFind,
    windowSetTarget,
  }
}