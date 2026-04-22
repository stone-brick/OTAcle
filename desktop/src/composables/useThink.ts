import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import type { DecisionLog, ThinkConfig, ThinkStatus } from '../types'
import { useProjectEvents, type ProjectEvent } from './useProjectEvents'
import { useProject } from './useProject'

const isThinking = ref(false)
const decisionLogs = ref<DecisionLog[]>([])
const config = ref<ThinkConfig>({ max_data_points: 500, display_fields: [] })
const status = ref<ThinkStatus | null>(null)

let unlistenDecisionLog: UnlistenFn | null = null
let unlistenError: UnlistenFn | null = null
let isListeningActive = false
let unsubscribeProject: (() => void) | null = null

export function useThink() {
  async function start() {
    await invoke('think_start')
    isThinking.value = true
    await fetchStatus()
    await getLogs()
  }

  async function stop() {
    await invoke('think_stop')
    isThinking.value = false
    await fetchStatus()
  }

  async function loadConfig(path: string) {
    try {
      const cfg = await invoke<ThinkConfig>('think_load_config', { path })
      config.value = cfg
      return cfg
    } catch {
      // 配置文件不存在时创建默认配置
      const defaultConfig: ThinkConfig = {
        max_data_points: 500,
        display_fields: []
      }
      await invoke('think_save_config', { path, config: defaultConfig })
      config.value = defaultConfig
      console.warn(`已创建默认 think 配置文件: ${path}`)
      return defaultConfig
    }
  }

  async function saveConfig(path: string) {
    await invoke('think_save_config', { path, config: config.value })
  }

  async function getLogs(limit?: number) {
    const logs = await invoke<DecisionLog[]>('think_get_logs', { limit })
    decisionLogs.value = logs
    return logs
  }

  async function fetchStatus() {
    status.value = await invoke<ThinkStatus>('think_get_status')
  }

  async function startListening() {
    if (isListeningActive) return
    isListeningActive = true

    unlistenDecisionLog = await listen<DecisionLog>('think:decision_log', (event) => {
      decisionLogs.value.push(event.payload)
      const maxPoints = config.value.max_data_points
      if (decisionLogs.value.length > maxPoints) {
        decisionLogs.value.shift()
      }
    })

    unlistenError = await listen<{ error: string }>('think:error', (event) => {
      console.error('Think error:', event.payload.error)
    })
  }

  function stopListening() {
    if (unlistenDecisionLog) {
      unlistenDecisionLog()
      unlistenDecisionLog = null
    }
    if (unlistenError) {
      unlistenError()
      unlistenError = null
    }
    isListeningActive = false
  }

  function initProjectEventListener(): void {
    if (unsubscribeProject) {
      unsubscribeProject()
    }

    const { onProjectEvent } = useProjectEvents()
    const { getProjectThinkConfigPath } = useProject()

    unsubscribeProject = onProjectEvent(async (event: ProjectEvent) => {
      if (event.type === 'opened' && event.project) {
        const cfgPath = await getProjectThinkConfigPath()
        if (cfgPath) {
          try {
            await loadConfig(cfgPath)
          } catch {
            console.warn('自动加载 think 配置失败')
          }
        }
      } else if (event.type === 'closed') {
        config.value = { max_data_points: 500, display_fields: [] }
      }
    })
  }

  return {
    isThinking,
    decisionLogs,
    config,
    status,
    start,
    stop,
    loadConfig,
    saveConfig,
    getLogs,
    fetchStatus,
    startListening,
    stopListening,
    initProjectEventListener,
  }
}