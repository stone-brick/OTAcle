<script setup lang="ts">
import { mdiCloudOutline, mdiCloudCheckOutline, mdiAccountOutline, mdiAccountGroup, mdiCog } from '@mdi/js'
import BaseIcon from '../ui/BaseIcon.vue'
import { useRemote } from '../../composables/useRemote'

defineProps<{
  activeTab: 'auth' | 'groups' | 'config'
}>()

const emit = defineEmits<{
  'update:activeTab': [tab: 'auth' | 'groups' | 'config']
}>()

const { isLoggedIn } = useRemote()

interface TabItem {
  id: 'auth' | 'groups' | 'config'
  label: string
  icon: string
}

const authTab: TabItem[] = [{ id: 'auth', label: '登录/注册', icon: mdiAccountOutline }]
const loggedInTabs: TabItem[] = [
  { id: 'groups', label: '分组', icon: mdiAccountGroup },
  { id: 'config', label: '配置', icon: mdiCog },
]

function selectTab(tab: TabItem) {
  emit('update:activeTab', tab.id)
}
</script>

<template>
  <nav class="flex items-center gap-6 px-4 h-12 bg-white dark:bg-slate-900 border-b border-gray-100 dark:border-slate-800">
    <div class="flex items-center gap-2 pr-4 border-r border-gray-200 dark:border-slate-700">
      <BaseIcon
        :path="isLoggedIn ? mdiCloudCheckOutline : mdiCloudOutline"
        :size="20"
      />
      <span class="text-base font-bold text-blue-500">Remote</span>
    </div>
    <ul class="flex list-none m-0 p-0 gap-1">
      <li
        v-for="tab in isLoggedIn ? loggedInTabs : authTab"
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