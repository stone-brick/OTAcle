<script setup lang="ts">
import { mdiCamera, mdiContentCut, mdiAccessPoint, mdiEye } from '@mdi/js'
import BaseIcon from '../ui/BaseIcon.vue'

defineProps<{
  activeTab: 'capture' | 'crop' | 'transport'
}>()

const emit = defineEmits<{
  'update:activeTab': [tab: 'capture' | 'crop' | 'transport']
}>()

interface TabItem {
  id: 'capture' | 'crop' | 'transport'
  label: string
  icon: string
}

const tabs: TabItem[] = [
  { id: 'capture', label: '截图配置', icon: mdiCamera },
  { id: 'crop', label: '裁切配置', icon: mdiContentCut },
  { id: 'transport', label: '传输配置', icon: mdiAccessPoint },
]

function selectTab(tab: TabItem) {
  emit('update:activeTab', tab.id)
}
</script>

<template>
  <nav class="flex items-center gap-6 px-4 h-12 bg-white dark:bg-slate-900 border-b border-gray-100 dark:border-slate-800">
    <div class="flex items-center gap-2 pr-4 border-r border-gray-200 dark:border-slate-700">
      <BaseIcon
        :path="mdiEye"
        :size="20"
      />
      <span class="text-base font-bold text-blue-500">Observe</span>
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
