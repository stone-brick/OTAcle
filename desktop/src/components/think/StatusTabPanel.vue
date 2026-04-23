<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useThink } from '../../composables/think/useThink'
import { useProject } from '../../composables/useProject'
import { useLog } from '../../composables/useLog'
import CardBox from '@/components/ui/CardBox.vue'
import BaseButton from '@/components/ui/BaseButton.vue'
import { getStatusTextColor } from '@/utils/colors'
import { mdiPlay, mdiStop, mdiRefresh, mdiContentSave } from '@mdi/js'

const {
  isThinking,
  status,
  fetchStatus,
  start,
  stop,
  getLogs,
  saveConfig,
  startListening,
  stopListening,
} = useThink()

const { isProjectLoaded, getProjectThinkConfigPath } = useProject()
const { logs, addLog } = useLog()

const commLogs = computed(() =>
  logs.value.filter(log => log.source === 'think')
)

const inputAddress = ref('tcp://127.0.0.1:5557')

onMounted(async () => {
  await startListening()
  await fetchStatus()
})

onUnmounted(() => {
  stopListening()
})

async function handleStart() {
  await start()
}

async function handleStop() {
  await stop()
}

async function handleRefresh() {
  await getLogs(100)
}

async function handleSave() {
  const path = await getProjectThinkConfigPath()
  if (path) {
    await saveConfig(path)
    addLog('Think 配置已保存', 'success', 'think')
  }
}
</script>

<template>
  <CardBox class="flex flex-col gap-4">
    <!-- Connection Status -->
    <div class="flex items-center gap-6 flex-wrap">
      <div class="flex flex-col gap-1">
        <span class="text-xs text-gray-500 uppercase tracking-wide">运行状态</span>
        <span :class="['text-sm font-semibold', isThinking ? 'text-green-500' : 'text-gray-600']">
          {{ isThinking ? '运行中' : '已停止' }}
        </span>
      </div>
      <div class="flex flex-col gap-1">
        <span class="text-xs text-gray-500 uppercase tracking-wide">地址</span>
        <span class="text-sm font-mono text-gray-600">{{ inputAddress }}</span>
      </div>
      <div class="flex flex-col gap-1">
        <span class="text-xs text-gray-500 uppercase tracking-wide">日志数</span>
        <span class="text-sm font-mono font-semibold">{{ status?.logs_count ?? '-' }}</span>
      </div>
      <div class="flex flex-col gap-1">
        <span class="text-xs text-gray-500 uppercase tracking-wide">收到消息</span>
        <span class="text-sm font-mono font-semibold">{{ status?.messages_received ?? '-' }}</span>
      </div>
      <div class="flex items-center gap-2 ml-auto">
        <BaseButton
          v-if="!isThinking"
          :icon="mdiPlay"
          color="whiteDark"
          small
          @click="handleStart"
        >
          启动
        </BaseButton>
        <BaseButton
          v-else
          :icon="mdiStop"
          color="whiteDark"
          small
          @click="handleStop"
        >
          停止
        </BaseButton>
        <BaseButton
          :icon="mdiRefresh"
          color="whiteDark"
          small
          @click="handleRefresh"
        >
          刷新
        </BaseButton>
        <BaseButton
          :icon="mdiContentSave"
          color="whiteDark"
          small
          :disabled="!isProjectLoaded"
          @click="handleSave"
        >
          保存
        </BaseButton>
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
