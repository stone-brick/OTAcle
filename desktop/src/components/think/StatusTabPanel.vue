<script setup lang="ts">
import { mdiPlay, mdiStop, mdiRefresh } from '@mdi/js'
import BaseIcon from '../ui/BaseIcon.vue'
import BaseButton from '../ui/BaseButton.vue'
import CardBox from '../ui/CardBox.vue'
import CardBoxComponentBody from '../ui/CardBoxComponentBody.vue'

defineProps<{
  isThinking: boolean
  status: { running: boolean; logs_count: number; messages_received: number; uptime_seconds: number } | null
}>()

const emit = defineEmits<{
  start: []
  stop: []
  refresh: []
}>()
</script>

<template>
  <div class="status-tab-panel">
    <CardBox class="flex flex-col flex-1">
      <div class="flex items-center justify-between px-4 py-3 border-b border-gray-100 dark:border-slate-800">
        <span class="text-sm font-semibold text-gray-500 dark:text-slate-400">状态监控</span>
        <div class="flex gap-2">
          <BaseButton
            v-if="!isThinking"
            variant="primary"
            small
            @click="emit('start')"
          >
            <BaseIcon :path="mdiPlay" />
            启动
          </BaseButton>
          <BaseButton
            v-else
            variant="danger"
            small
            @click="emit('stop')"
          >
            <BaseIcon :path="mdiStop" />
            停止
          </BaseButton>
        </div>
      </div>

      <CardBoxComponentBody class="flex flex-col gap-4">
        <div
          v-if="status"
          class="grid grid-cols-4 gap-4"
        >
          <div class="flex flex-col gap-1 px-3 py-2 bg-gray-50 dark:bg-slate-800 rounded-lg">
            <span class="text-xs text-gray-500 dark:text-slate-400">运行状态</span>
            <span :class="['text-sm font-semibold', status.running ? 'text-green-500' : 'text-gray-400']">
              {{ status.running ? '运行中' : '已停止' }}
            </span>
          </div>
          <div class="flex flex-col gap-1 px-3 py-2 bg-gray-50 dark:bg-slate-800 rounded-lg">
            <span class="text-xs text-gray-500 dark:text-slate-400">日志数</span>
            <span class="text-sm font-semibold">{{ status.logs_count }}</span>
          </div>
          <div class="flex flex-col gap-1 px-3 py-2 bg-gray-50 dark:bg-slate-800 rounded-lg">
            <span class="text-xs text-gray-500 dark:text-slate-400">收到消息</span>
            <span class="text-sm font-semibold">{{ status.messages_received }}</span>
          </div>
          <div class="flex flex-col gap-1 px-3 py-2 bg-gray-50 dark:bg-slate-800 rounded-lg">
            <span class="text-xs text-gray-500 dark:text-slate-400">运行时间</span>
            <span class="text-sm font-semibold">{{ status.uptime_seconds }}s</span>
          </div>
        </div>

        <div class="flex gap-2">
          <BaseButton @click="emit('refresh')">
            <BaseIcon :path="mdiRefresh" />
            刷新数据
          </BaseButton>
        </div>
      </CardBoxComponentBody>
    </CardBox>
  </div>
</template>

<style scoped>
.status-tab-panel {
  height: 100%;
}
</style>