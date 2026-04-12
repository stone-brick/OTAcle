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
  <div class="title-bar">
    <div class="title-content" @mousedown="handleDrag">
      <span class="title-text">OTAcle</span>
    </div>
    <div class="window-controls">
      <button class="control-btn minimize" @click="handleMinimize" title="最小化">
        <svg width="12" height="12" viewBox="0 0 12 12">
          <rect x="1" y="5.5" width="10" height="1" fill="currentColor"/>
        </svg>
      </button>
      <button class="control-btn maximize" @click="handleToggleMaximize" :title="isMaximized ? '还原' : '最大化'">
        <svg v-if="isMaximized" width="12" height="12" viewBox="0 0 12 12">
          <rect x="2" y="4" width="6" height="6" fill="none" stroke="currentColor" stroke-width="1"/>
          <path d="M4 4V2h6v6h-2" fill="none" stroke="currentColor" stroke-width="1"/>
        </svg>
        <svg v-else width="12" height="12" viewBox="0 0 12 12">
          <rect x="1.5" y="1.5" width="9" height="9" fill="none" stroke="currentColor" stroke-width="1"/>
        </svg>
      </button>
      <button class="control-btn close" @click="handleClose" title="关闭">
        <svg width="12" height="12" viewBox="0 0 12 12">
          <path d="M1 1L11 11M11 1L1 11" stroke="currentColor" stroke-width="1.5" stroke-linecap="round"/>
        </svg>
      </button>
    </div>
  </div>
</template>

<style scoped>
.title-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 32px;
  background: var(--color-surface);
  border-bottom: 1px solid var(--color-border);
  -webkit-app-region: drag;
  user-select: none;
}

.title-content {
  flex: 1;
  display: flex;
  align-items: center;
  padding-left: 12px;
  height: 100%;
  cursor: default;
}

.title-text {
  font-size: 13px;
  font-weight: 600;
  color: var(--color-text);
}

.window-controls {
  display: flex;
  height: 100%;
  -webkit-app-region: no-drag;
}

.control-btn {
  width: 46px;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  border: none;
  background: transparent;
  color: var(--color-text-secondary);
  cursor: pointer;
  transition: background 0.15s, color 0.15s;
}

.control-btn:hover {
  background: var(--color-hover);
  color: var(--color-text);
}

.control-btn.close:hover {
  background: var(--color-error);
  color: white;
}
</style>
