import { ref } from 'vue'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import type { DecisionLog, ThinkConfig, ThinkStatus } from '../../types'
import { useProjectEvents, type ProjectEvent } from '../useProjectEvents'
import { useProject } from '../useProject'
import { useLog } from '../useLog'
import { useComm } from '../useComm'

const isThinking = ref(false)
const decisionLogs = ref<DecisionLog[]>([])
const config = ref<ThinkConfig>({ max_data_points: 1000, display_fields: [] })
const status = ref<ThinkStatus | null>(null)

let unlistenDecisionLog: UnlistenFn | null = null
let unlistenError: UnlistenFn | null = null
let isListeningActive = false
let unsubscribeProject: (() => void) | null = null

export function useThink() {
  const { addLog } = useLog()

  async function setThinkPullAddress(addr: string) {
    const comm = useComm()
    comm.setThinkPullAddress(addr)
  }

  async function start() {
    const comm = useComm()
    await comm.startThinkPull()
    isThinking.value = true
    await fetchStatus()
    await getLogs()
  }

  async function stop() {
    const comm = useComm()
    await comm.stopThinkPull()
    isThinking.value = false
    await fetchStatus()
  }

  async function loadConfig(path: string) {
    const comm = useComm();
    try {
      const cfg = await comm.thinkLoadConfig(path);
      config.value = cfg
      addLog(`Think 配置已加载: ${path}`, 'success', 'think')
      return cfg
    } catch {
      // 配置文件不存在时创建默认配置
      const defaultConfig: ThinkConfig = {
        max_data_points: 1000,
        display_fields: []
      }
      await comm.thinkSaveConfig(path, defaultConfig);
      config.value = defaultConfig
      addLog(`已创建默认 think 配置文件: ${path}`, 'info', 'think')
      return defaultConfig
    }
  }

  async function saveConfig(path: string) {
    const comm = useComm();
    await comm.thinkSaveConfig(path, config.value);
  }

  async function getLogs(limit?: number) {
    const comm = useComm();
    const logs = await comm.thinkGetLogs(limit);
    decisionLogs.value = logs
    return logs
  }

  async function fetchStatus() {
    const comm = useComm();
    status.value = await comm.thinkGetStatus() as ThinkStatus;
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
        config.value = { max_data_points: 1000, display_fields: [] }
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
    setThinkPullAddress,
    loadConfig,
    saveConfig,
    getLogs,
    fetchStatus,
    startListening,
    stopListening,
    initProjectEventListener,
  }
}