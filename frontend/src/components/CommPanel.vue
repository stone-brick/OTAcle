<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useComm } from '../composables/useComm'
import { useLog } from '../composables/useLog'
import CardBox from '@/components/ui/CardBox.vue'
import BaseButton from '@/components/ui/BaseButton.vue'
import FormControl from '@/components/ui/FormControl.vue'
import { getStatusTextColor } from '@/utils/colors'

const {
  isConnected,
  address,
  fetchStatus,
  start,
  stop,
  startListening,
  cleanup,
} = useComm()

const { logs } = useLog()

const commLogs = computed(() =>
  logs.value.filter(log => log.source === 'comm')
)

const inputAddress = ref('tcp://127.0.0.1:5555')

onMounted(async () => {
  await startListening()
})

onUnmounted(() => {
  cleanup()
})

async function handleStart() {
  await start(inputAddress.value)
}

async function handleStop() {
  await stop()
}
</script>

<template>
  <CardBox class="flex flex-col gap-4">
    <!-- Connection Status -->
    <div class="flex items-center gap-6 flex-wrap">
      <div class="flex flex-col gap-1">
        <span class="text-xs text-gray-500 uppercase tracking-wide">连接状态</span>
        <span :class="['text-sm font-semibold', isConnected ? 'text-green-500' : 'text-gray-600']">
          {{ isConnected ? '已连接' : '未连接' }}
        </span>
      </div>
      <div class="flex flex-col gap-1">
        <span class="text-xs text-gray-500 uppercase tracking-wide">地址</span>
        <span class="text-sm font-mono text-gray-600">{{ address || '-' }}</span>
      </div>
      <div class="flex items-center gap-2 ml-auto">
        <FormControl
          v-model="inputAddress"
          type="text"
          placeholder="tcp://127.0.0.1:5555"
          :disabled="isConnected"
        />
        <BaseButton
          v-if="!isConnected"
          label="连接"
          color="info"
          @click="handleStart"
        />
        <BaseButton
          v-else
          label="断开"
          color="danger"
          @click="handleStop"
        />
        <BaseButton
          label="刷新"
          color="whiteDark"
          @click="fetchStatus"
        />
      </div>
    </div>

    <!-- Message Log -->
    <div class="flex flex-col flex-1 overflow-hidden rounded-xl bg-white dark:bg-slate-900/70">
      <div class="flex justify-between items-center px-4 py-3 border-b border-gray-100 dark:border-slate-800">
        <h4 class="m-0 text-xs font-semibold text-gray-500 dark:text-slate-400 uppercase tracking-wide">
          消息日志
        </h4>
      </div>
      <div class="flex-1 overflow-y-auto px-4 py-2 font-mono text-xs">
        <div
          v-for="(msg, index) in commLogs"
          :key="index"
          class="flex gap-3 py-1.5 border-b border-gray-50 dark:border-slate-800 last:border-0"
        >
          <span class="text-gray-400 flex-shrink-0">{{ msg.time }}</span>
          <span
            :class="['break-all', getStatusTextColor(msg.type)]"
          >{{ msg.message }}</span>
        </div>
        <div
          v-if="commLogs.length === 0"
          class="py-8 text-center text-gray-400"
        >
          暂无消息
        </div>
      </div>
    </div>
  </CardBox>
</template>
