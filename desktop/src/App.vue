<script setup lang="ts">
import { onMounted } from 'vue'
import { useLog } from './composables/useLog'
import { useProject } from './composables/useProject'
import { useActionEditor } from './composables/act/useActionEditor'
import { useObserve } from './composables/observe/useObserve'
import { useThink } from './composables/think/useThink'
import { useComm } from './composables/useComm'
import { useDialog } from './composables/useDialog'
import AppSideMenu from './components/nav/AppSideMenu.vue'
import TitleBar from './components/TitleBar.vue'
import LogPanel from './components/LogPanel.vue'
import AlertDialog from './components/dialog/AlertDialog.vue'
import InputDialog from './components/dialog/InputDialog.vue'
import ConfirmDialog from './components/dialog/ConfirmDialog.vue'

const { logs, clearLogs } = useLog()
const { checkCurrentProject, refreshRecentProjects } = useProject()
const {
  alertState,
  inputState,
  confirmState,
  handleAlertOk,
  handleInputConfirm,
  handleInputCancel,
  handleConfirmOk,
  handleConfirmCancel,
} = useDialog()

onMounted(async () => {
  // 先初始化项目事件监听，再检查当前项目（确保监听器已注册）
  useActionEditor().initProjectEventListener()
  useObserve().initProjectEventListener()
  useThink().initProjectEventListener()
  useComm().initProjectEventListener()

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

    <AlertDialog
      :show="alertState.show"
      :title="alertState.title"
      :message="alertState.message"
      @ok="handleAlertOk"
    />

    <InputDialog
      :show="inputState.show"
      :title="inputState.title"
      :message="inputState.message"
      :default-value="inputState.defaultValue"
      :placeholder="inputState.placeholder"
      @confirm="handleInputConfirm"
      @cancel="handleInputCancel"
    />

    <ConfirmDialog
      :show="confirmState.show"
      :title="confirmState.title"
      :message="confirmState.message"
      @confirm="handleConfirmOk"
      @cancel="handleConfirmCancel"
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
