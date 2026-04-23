import { ref, effectScope } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { useLog } from './useLog'
import { useProjectEvents, type ProjectEvent } from './useProjectEvents'
import { useProject } from './useProject'
import type { ObserveConfig } from '../types'

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
    } catch (e) {
      addLog(`获取通信状态失败: ${e}`, 'error', 'comm')
    }
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
      addLog(`已加载通信配置: ${path}`, 'success', 'comm')
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
  }
}