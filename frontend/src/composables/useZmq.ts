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
const messages = ref<ZmqMessage[]>([])

// 创建单个 effect scope 来管理此 composable 的所有响应式效果
const scope = effectScope()

function formatTime(): string {
  const now = new Date()
  return now.toLocaleTimeString('zh-CN', { hour12: false })
}

// 监听器句柄 - 存储在模块级别以便清理访问
let unlistenLog: UnlistenFn | null = null
let unlistenError: UnlistenFn | null = null

export function useZmq() {
  const { addLog } = useLog()

  async function fetchStatus() {
    try {
      const status = await invoke<CommStatus>('comm_get_status')
      isConnected.value = status.pull_running
      address.value = status.pull_address
    } catch (e) {
      addLog(`获取 ZMQ 状态失败: ${e}`, 'error')
    }
  }

  async function start(address_: string) {
    try {
      await invoke('comm_set_pull_address', { addr: address_ })
      await invoke('comm_start_pull')
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
      await invoke('comm_stop_pull')
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
    // 只保留最近 MAX_ZMQ_MESSAGES 条消息
    if (messages.value.length > MAX_ZMQ_MESSAGES) {
      messages.value.shift()
    }
  }

  function clearMessages() {
    messages.value = []
  }

  /**
   * 开始监听 ZMQ 事件。在组件的 onMounted 中调用。
   */
  async function startListening() {
    // 获取初始状态
    await fetchStatus()

    // 监听 ZMQ 日志事件
    unlistenLog = await listen<string>('zmq:log', (event) => {
      addMessage(event.payload, 'success')
    })

    unlistenError = await listen<string>('zmq:error', (event) => {
      addMessage(event.payload, 'error')
    })
  }

  /**
   * 停止监听 ZMQ 事件。在组件的 onUnmounted 中调用。
   */
  function stopListening() {
    unlistenLog?.()
    unlistenError?.()
    unlistenLog = null
    unlistenError = null
  }

  /**
   * 清理所有资源。当永久释放此 composable 时调用。
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