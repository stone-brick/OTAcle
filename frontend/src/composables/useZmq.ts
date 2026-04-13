import { ref, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { useLog } from './useLog'

export interface ZmqMessage {
  time: string
  type: 'info' | 'success' | 'error'
  content: string
}

const isConnected = ref(false)
const address = ref('')
const messages = ref<ZmqMessage[]>([])

function formatTime(): string {
  const now = new Date()
  return now.toLocaleTimeString('zh-CN', { hour12: false })
}

export function useZmq() {
  const { addLog } = useLog()

  // Listener refs - inside useZmq() so each component instance has its own
  const unlistenLog = ref<UnlistenFn | null>(null)
  const unlistenError = ref<UnlistenFn | null>(null)

  async function fetchStatus() {
    try {
      const [connected, addr] = await invoke<[boolean, string]>('zmq_status')
      isConnected.value = connected
      address.value = addr
    } catch (e) {
      addLog(`获取 ZMQ 状态失败: ${e}`, 'error')
    }
  }

  async function start(address_: string) {
    try {
      await invoke('zmq_start', { addr: address_ })
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
      await invoke('zmq_stop')
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
    // Keep only last 100 messages
    if (messages.value.length > 100) {
      messages.value.shift()
    }
  }

  function clearMessages() {
    messages.value = []
  }

  onMounted(async () => {
    await fetchStatus()

    // Listen for ZMQ log events
    unlistenLog.value = await listen<string>('zmq:log', (event) => {
      addMessage(event.payload, 'success')
    })

    unlistenError.value = await listen<string>('zmq:error', (event) => {
      addMessage(event.payload, 'error')
    })
  })

  onUnmounted(() => {
    unlistenLog.value?.()
    unlistenError.value?.()
  })

  return {
    isConnected,
    address,
    messages,
    fetchStatus,
    start,
    stop,
    addMessage,
    clearMessages,
  }
}