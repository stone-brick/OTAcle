import { ref, effectScope } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { useLog } from './useLog'
import { useProjectEvents, type ProjectEvent } from './useProjectEvents'
import { useProject } from './useProject'

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

  async function start(address_: string) {
    try {
      await invoke('comm_set_pull_address', { addr: address_ })
      await invoke('comm_start_pull')
      address.value = address_
      addLog(`通信连接已启动: ${address_}`, 'success', 'comm')
      await fetchStatus()
    } catch (e) {
      addLog('启动通信接收失败，请检查端口是否被占用', 'error', 'comm')
      throw e
    }
  }

  async function stop() {
    try {
      await invoke('comm_stop_pull')
      await fetchStatus()
    } catch (e) {
      addLog(`停止通信连接失败: ${e}`, 'error', 'comm')
    }
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
      // 使用默认配置并保存
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
  }
}