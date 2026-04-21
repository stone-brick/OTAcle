<script setup lang="ts">
import { computed } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { mdiEye, mdiMessageTextOutline, mdiTarget } from '@mdi/js'
import BaseIcon from '../ui/BaseIcon.vue'
import ProjectSelector from '../ProjectSelector.vue'

const router = useRouter()
const route = useRoute()

interface NavItem {
  id: string
  label: string
  icon: string
  path: string
}

const menuItems: NavItem[] = [
  { id: 'observe', label: '观察', icon: mdiEye, path: '/observe' },
  { id: 'think', label: '思考', icon: mdiMessageTextOutline, path: '/think' },
  { id: 'act', label: '执行', icon: mdiTarget, path: '/act' },
]

const currentPath = computed(() => route.path)

function navigateTo(path: string) {
  router.push(path)
}

function isActive(path: string): boolean {
  return currentPath.value === path
}
</script>

<template>
  <nav class="w-[200px] h-full bg-white dark:bg-slate-900 border-r border-gray-100 dark:border-slate-800 flex flex-col">
    <ProjectSelector />

    <ul class="list-none p-2 m-0">
      <li
        v-for="item in menuItems"
        :key="item.id"
        class="flex items-center gap-2.5 px-4 py-3 mb-1 rounded-lg cursor-pointer transition-colors duration-150 text-sm"
        :class="isActive(item.path)
          ? 'bg-blue-50 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400 font-semibold'
          : 'text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-slate-800 hover:text-gray-700 dark:hover:text-gray-200'"
        @click="navigateTo(item.path)"
      >
        <BaseIcon
          :path="item.icon"
          :size="20"
        />
        <span>{{ item.label }}</span>
      </li>
    </ul>
  </nav>
</template>
