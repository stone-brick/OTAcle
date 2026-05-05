<script setup lang="ts">
import { computed, onMounted, onUnmounted } from 'vue'
import { useThink } from '../../composables/think/useThink'
import { useComm } from '../../composables/useComm'
import { useProject } from '../../composables/useProject'
import { useLog } from '../../composables/useLog'
import CardBox from '@/components/ui/CardBox.vue'
import BaseButton from '@/components/ui/BaseButton.vue'
import FormControl from '@/components/ui/FormControl.vue'
import { getStatusTextColor } from '@/utils/colors'
import { mdiPlay, mdiStop, mdiRefresh, mdiContentSave } from '@mdi/js'

const {
  isThinking,
  setThinkPullAddress,
  fetchStatus,
  start,
  stop,
  startListening,
  stopListening,
  loadConfig,
} = useThink()

const { config: commConfig } = useComm()
const { isProjectLoaded, getProjectThinkConfigPath } = useProject()
const { logs, addLog } = useLog()

const thinkLogs = computed(() =>
  logs.value.filter(log => log.source === 'think')
)

const inputAddress = computed({
  get: () => commConfig.value.think_pull_address,
  set: (val: string) => { commConfig.value.think_pull_address = val }
})

async function savePullAddress() {
  try {
    setThinkPullAddress(commConfig.value.think_pull_address)
    addLog('Think pull_address 已保存', 'info', 'think')
  } catch {
    addLog('保存 pull_address 失败', 'error', 'think')
  }
}

onMounted(async () => {
  await startListening()
  await fetchStatus()
  // 加载项目时自动加载 Think 配置
  if (isProjectLoaded.value) {
    const path = await getProjectThinkConfigPath()
    if (path) {
      try {
        await loadConfig(path)
      } catch {
        // 静默处理
      }
    }
  }
})

onUnmounted(() => {
  stopListening()
})

async function handleStart() {
  try {
    await start()
    addLog('Think PULL 已启动', 'success', 'think')
  } catch {
    addLog('启动 Think PULL 失败', 'error', 'think')
  }
}

async function handleStop() {
  try {
    await stop()
    addLog('Think PULL 已停止', 'info', 'think')
  } catch {
    addLog('停止 Think PULL 失败', 'error', 'think')
  }
}
</script>

<template>
  <div class="flex flex-col p-3 gap-3 h-full box-border">
    <CardBox class="flex flex-col gap-4 flex-1">
    <!-- Connection Status -->
    <div class="flex items-center gap-6 flex-wrap">
      <div class="flex flex-col pl-4">
        <span :class="['text-sm font-semibold', isThinking ? 'text-green-500' : 'text-gray-600']">
          {{ isThinking ? '运行中' : '已停止' }}
        </span>
      </div>
      <div class="flex items-center gap-2 ml-auto">
        <FormControl
          v-model="inputAddress"
          type="text"
          placeholder="tcp://127.0.0.1:5557"
          :disabled="isThinking"
        />
        <BaseButton
          :icon="mdiContentSave"
          color="whiteDark"
          small
          :disabled="!isProjectLoaded"
          @click="savePullAddress"
        />
        <BaseButton
          v-if="!isThinking"
          :icon="mdiPlay"
          color="whiteDark"
          small
          :disabled="!isProjectLoaded"
          @click="handleStart"
        />
        <BaseButton
          v-else
          :icon="mdiStop"
          color="whiteDark"
          small
          @click="handleStop"
        />
        <BaseButton
          :icon="mdiRefresh"
          color="whiteDark"
          small
          @click="fetchStatus"
        />
      </div>
    </div>

    <!-- Message Log -->
    <div class="flex flex-col flex-1 overflow-hidden rounded-xl bg-white dark:bg-slate-900/70">
      <div class="px-4 py-3 border-b border-gray-100 dark:border-slate-800">
        <h4 class="m-0 text-xs font-semibold text-gray-500 dark:text-slate-400 uppercase tracking-wide">
          决策日志
        </h4>
      </div>
      <div class="flex-1 overflow-y-auto px-4 py-2 font-mono text-xs">
        <div
          v-for="(msg, index) in thinkLogs"
          :key="index"
          class="flex gap-3 py-1.5 border-b border-gray-50 dark:border-slate-800 last:border-0"
        >
          <span class="text-gray-400 flex-shrink-0">{{ msg.time }}</span>
          <span
            :class="['break-all', getStatusTextColor(msg.type)]"
          >{{ msg.message }}</span>
        </div>
        <div
          v-if="thinkLogs.length === 0"
          class="py-8 text-center text-gray-400"
        >
          暂无日志
        </div>
      </div>
    </div>
  </CardBox>
  </div>
</template>
