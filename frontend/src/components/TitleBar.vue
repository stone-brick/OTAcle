<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'

const appWindow = getCurrentWindow()
const isMaximized = ref(false)

onMounted(async () => {
  isMaximized.value = await appWindow.isMaximized()
})

async function handleDrag() {
  await appWindow.startDragging()
}

async function handleMinimize() {
  await appWindow.minimize()
}

async function handleToggleMaximize() {
  if (isMaximized.value) {
    await appWindow.unmaximize()
  } else {
    await appWindow.maximize()
  }
  isMaximized.value = !isMaximized.value
}

async function handleClose() {
  await appWindow.close()
}
</script>

<template>
  <div
    class="flex items-center justify-between h-8 bg-white dark:bg-slate-900 border-b border-gray-100 dark:border-slate-800"
    style="-webkit-app-region: drag; user-select: none;"
  >
    <div
      class="flex-1 flex items-center pl-3 h-full cursor-default"
      @mousedown="handleDrag"
    >
      <span class="text-sm font-semibold text-gray-700 dark:text-gray-200">OTAcle</span>
    </div>
    <div
      class="flex h-full"
      style="-webkit-app-region: no-drag;"
    >
      <button
        class="w-[46px] h-full flex items-center justify-center border-none bg-transparent text-gray-400 dark:text-gray-500 cursor-pointer transition-colors duration-150 hover:bg-gray-100 dark:hover:bg-slate-800 hover:text-gray-700 dark:hover:text-gray-200"
        title="最小化"
        @click="handleMinimize"
      >
        <svg
          width="12"
          height="12"
          viewBox="0 0 12 12"
        >
          <rect
            x="1"
            y="5.5"
            width="10"
            height="1"
            fill="currentColor"
          />
        </svg>
      </button>
      <button
        class="w-[46px] h-full flex items-center justify-center border-none bg-transparent text-gray-400 dark:text-gray-500 cursor-pointer transition-colors duration-150 hover:bg-gray-100 dark:hover:bg-slate-800 hover:text-gray-700 dark:hover:text-gray-200"
        :title="isMaximized ? '还原' : '最大化'"
        @click="handleToggleMaximize"
      >
        <svg
          v-if="isMaximized"
          width="12"
          height="12"
          viewBox="0 0 12 12"
        >
          <rect
            x="2"
            y="4"
            width="6"
            height="6"
            fill="none"
            stroke="currentColor"
            stroke-width="1"
          />
          <path
            d="M4 4V2h6v6h-2"
            fill="none"
            stroke="currentColor"
            stroke-width="1"
          />
        </svg>
        <svg
          v-else
          width="12"
          height="12"
          viewBox="0 0 12 12"
        >
          <rect
            x="1.5"
            y="1.5"
            width="9"
            height="9"
            fill="none"
            stroke="currentColor"
            stroke-width="1"
          />
        </svg>
      </button>
      <button
        class="w-[46px] h-full flex items-center justify-center border-none bg-transparent text-gray-400 dark:text-gray-500 cursor-pointer transition-colors duration-150 hover:bg-red-500 hover:text-white"
        title="关闭"
        @click="handleClose"
      >
        <svg
          width="12"
          height="12"
          viewBox="0 0 12 12"
        >
          <path
            d="M1 1L11 11M11 1L1 11"
            stroke="currentColor"
            stroke-width="1.5"
            stroke-linecap="round"
          />
        </svg>
      </button>
    </div>
  </div>
</template>
