<script setup lang="ts">
import { ref } from 'vue'
import ThinkTabNav from '../../components/nav/ThinkTabNav.vue'
import ThinkCommPanel from '../../components/think/ThinkCommPanel.vue'
import ThinkConfigPanel from './ThinkConfigPanel.vue'
import ThinkMonitorPanel from './ThinkMonitorPanel.vue'
import { useThink } from '../../composables/think/useThink'
import { useProject } from '../../composables/useProject'
import { useLog } from '../../composables/useLog'
import type { DisplayField } from '../../types'

const {
  decisionLogs,
  config,
  saveConfig,
} = useThink()

const { getProjectThinkConfigPath, isProjectLoaded } = useProject()
const { addLog } = useLog()

const activeTab = ref<'comm' | 'config' | 'monitor'>('monitor')

function handleUpdateField(index: number, field: DisplayField) {
  config.value.display_fields[index] = field
}

function handleAddField(field: DisplayField) {
  config.value.display_fields.push(field)
}

function handleDeleteField(index: number) {
  config.value.display_fields.splice(index, 1)
}

async function handleSaveConfig() {
  try {
    const path = await getProjectThinkConfigPath()
    if (path) {
      await saveConfig(path)
      addLog('Think 配置已保存', 'success', 'think')
    }
  } catch {
    addLog('保存 Think 配置失败', 'error', 'think')
  }
}
</script>

<template>
  <div class="think-page">
    <ThinkTabNav v-model:active-tab="activeTab" />

    <div class="think-content">
      <ThinkCommPanel
        v-if="activeTab === 'comm'"
      />

      <ThinkConfigPanel
        v-else-if="activeTab === 'config'"
        :config="config"
        :is-project-loaded="isProjectLoaded"
        @save="handleSaveConfig"
        @update-field="handleUpdateField"
        @add-field="handleAddField"
        @delete-field="handleDeleteField"
      />

      <ThinkMonitorPanel
        v-else-if="activeTab === 'monitor'"
        :config="config"
        :logs="decisionLogs"
      />
    </div>
  </div>
</template>

<style scoped>
.think-page {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
}

.think-content {
  flex: 1;
  overflow: hidden;
}

.think-content > :deep(*) {
  height: 100%;
  overflow: auto;
}
</style>
