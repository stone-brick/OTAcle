<script setup lang="ts">
import { onMounted } from 'vue'
import { useLog } from './composables/useLog'
import { useProject } from './composables/useProject'
import { useActionEditor } from './composables/act/useActionEditor'
import { useObserve } from './composables/useObserve'
import AppSideMenu from './components/nav/AppSideMenu.vue'
import TitleBar from './components/TitleBar.vue'
import LogPanel from './components/LogPanel.vue'

const { logs, clearLogs } = useLog()
const { checkCurrentProject, refreshRecentProjects } = useProject()

onMounted(async () => {
  // 先初始化项目事件监听，再检查当前项目（确保监听器已注册）
  useActionEditor().initProjectEventListener()
  useObserve().initProjectEventListener()

  await checkCurrentProject()
  await refreshRecentProjects()
})
</script>

<template>
  <div class="app-container">
    <TitleBar />

    <div class="app-body">
      <AppSideMenu />
      <main class="main-content">
        <router-view />
      </main>
    </div>

    <LogPanel
      :logs="logs"
      @clear="clearLogs"
    />
  </div>
</template>

<style scoped>
.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: var(--color-background);
}

.app-body {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.main-content {
  flex: 1;
  overflow: auto;
}
</style>
