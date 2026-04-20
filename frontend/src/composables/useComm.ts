import { ref, effectScope } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { useLog } from './useLog'

export interface CommStatus {
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

const scope = effectScope()

let unlistenLog: UnlistenFn | null = null
let unlistenError: UnlistenFn | null = null

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
    scope.stop()
  }

  return {
    isConnected,
    address,
    fetchStatus,
    start,
    stop,
    startListening,
    stopListening,
    cleanup,
  }
}