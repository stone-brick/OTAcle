import { ref, effectScope } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { useLog } from './useLog'
import { MAX_ZMQ_MESSAGES } from '../utils/constants'

export interface ZmqMessage {
  time: string
  type: 'info' | 'success' | 'error'
  content: string
}

const isConnected = ref(false)
const address = ref('')
const messages = ref<ZmqMessage[]>([])

// Create a single effect scope for all reactive effects managed by this composable
const scope = effectScope()

function formatTime(): string {
  const now = new Date()
  return now.toLocaleTimeString('zh-CN', { hour12: false })
}

// Listener handles - stored at module level for cleanup access
let unlistenLog: UnlistenFn | null = null
let unlistenError: UnlistenFn | null = null

export function useZmq() {
  const { addLog } = useLog()

  async function fetchStatus() {
    try {
      const [connected, addr] = await invoke<[boolean, string]>('act_zmq_status')
      isConnected.value = connected
      address.value = addr
    } catch (e) {
      addLog(`获取 ZMQ 状态失败: ${e}`, 'error')
    }
  }

  async function start(address_: string) {
    try {
      await invoke('act_zmq_start', { addr: address_ })
      address.value = address_
      addLog(`ZMQ 连接已启动: ${address_}`, 'success')
      await fetchStatus()
    } catch (e) {
      addLog(`启动 ZMQ 连接失败: ${e}`, 'error')
      throw e
    }
  }

  async function stop() {
    try {
      await invoke('act_zmq_stop')
      addLog('ZMQ 连接已停止', 'info')
      await fetchStatus()
    } catch (e) {
      addLog(`停止 ZMQ 连接失败: ${e}`, 'error')
    }
  }

  function addMessage(content: string, type: ZmqMessage['type'] = 'info') {
    messages.value.push({
      time: formatTime(),
      type,
      content,
    })
    // Keep only last MAX_ZMQ_MESSAGES messages
    if (messages.value.length > MAX_ZMQ_MESSAGES) {
      messages.value.shift()
    }
  }

  function clearMessages() {
    messages.value = []
  }

  /**
   * Start listening for ZMQ events. Call this in component's onMounted.
   */
  async function startListening() {
    // Fetch initial status
    await fetchStatus()

    // Listen for ZMQ log events
    unlistenLog = await listen<string>('zmq:log', (event) => {
      addMessage(event.payload, 'success')
    })

    unlistenError = await listen<string>('zmq:error', (event) => {
      addMessage(event.payload, 'error')
    })
  }

  /**
   * Stop listening for ZMQ events. Call this in component's onUnmounted.
   */
  function stopListening() {
    unlistenLog?.()
    unlistenError?.()
    unlistenLog = null
    unlistenError = null
  }

  /**
   * Cleanup all resources. Call this when permanently disposing of this composable.
   */
  function cleanup() {
    stopListening()
    scope.stop()
  }

  return {
    isConnected,
    address,
    messages,
    fetchStatus,
    start,
    stop,
    addMessage,
    clearMessages,
    startListening,
    stopListening,
    cleanup,
  }
}
