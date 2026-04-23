<script setup lang="ts">
import { mdiMessageTextOutline, mdiCog, mdiAccessPoint, mdiChartLine } from '@mdi/js'
import BaseIcon from '../ui/BaseIcon.vue'

defineProps<{
  activeTab: 'comm' | 'config' | 'monitor'
}>()

const emit = defineEmits<{
  'update:activeTab': [tab: 'comm' | 'config' | 'monitor']
}>()

interface TabItem {
  id: 'comm' | 'config' | 'monitor'
  label: string
  icon: string
}

const tabs: TabItem[] = [
  { id: 'monitor', label: '状态监控', icon: mdiChartLine },
  { id: 'config', label: '图表配置', icon: mdiCog },
  { id: 'comm', label: '通信日志', icon: mdiAccessPoint },
]

function selectTab(tab: TabItem) {
  emit('update:activeTab', tab.id)
}
</script>

<template>
  <nav class="flex items-center gap-6 px-4 h-12 bg-white dark:bg-slate-900 border-b border-gray-100 dark:border-slate-800">
    <div class="flex items-center gap-2 pr-4 border-r border-gray-200 dark:border-slate-700">
      <BaseIcon
        :path="mdiMessageTextOutline"
        :size="20"
      />
      <span class="text-base font-bold text-blue-500">Think</span>
    </div>
    <ul class="flex list-none m-0 p-0 gap-1">
      <li
        v-for="tab in tabs"
        :key="tab.id"
        class="flex items-center gap-2 px-4 py-2 rounded-md cursor-pointer text-sm transition-colors duration-150"
        :class="activeTab === tab.id
          ? 'bg-blue-100 dark:bg-blue-900/50 text-blue-600 dark:text-blue-400 font-semibold'
          : 'text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-slate-800 hover:text-gray-700 dark:hover:text-gray-200'"
        @click="selectTab(tab)"
      >
        <BaseIcon
          :path="tab.icon"
          :size="18"
        />
        <span>{{ tab.label }}</span>
      </li>
    </ul>
  </nav>
</template>
