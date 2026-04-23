<script setup lang="ts">
import { ref } from 'vue'
import ThinkTabNav from '../../components/nav/ThinkTabNav.vue'
import StatusTabPanel from '../../components/think/StatusTabPanel.vue'
import ConfigTabPanel from '../../components/think/ConfigTabPanel.vue'
import AddFieldDialog from '../../components/think/AddFieldDialog.vue'
import { useThink } from '../../composables/think/useThink'
import type { DisplayField, ThinkConfig } from '../../types'

const {
  decisionLogs,
  config,
} = useThink()

const activeTab = ref<'status' | 'config'>('status')
const showAddFieldDialog = ref(false)

function handleLoadDemoConfig() {
  const demoConfig: ThinkConfig = {
    max_data_points: 500,
    display_fields: [
      { name: 'reward', title: '奖励', chart_type: 'line' },
      { name: 'loss', title: '损失', chart_type: 'area' },
      { name: 'epsilon', title: '探索率', chart_type: 'line' },
    ],
  }
  config.value = demoConfig
}

function handleAddField() {
  showAddFieldDialog.value = true
}

function handleConfirmAddField(field: DisplayField) {
  config.value.display_fields.push(field)
  showAddFieldDialog.value = false
}

function handleDeleteField(name: string) {
  const index = config.value.display_fields.findIndex(f => f.name === name)
  if (index !== -1) {
    config.value.display_fields.splice(index, 1)
  }
}
</script>

<template>
  <div class="think-page">
    <ThinkTabNav v-model:active-tab="activeTab" />

    <div class="think-content">
      <StatusTabPanel
        v-if="activeTab === 'status'"
      />

      <ConfigTabPanel
        v-else-if="activeTab === 'config'"
        :config="config"
        :logs="decisionLogs"
        @add-field="handleAddField"
        @delete-field="handleDeleteField"
        @load-demo="handleLoadDemoConfig"
      />
    </div>

    <AddFieldDialog
      :show="showAddFieldDialog"
      @confirm="handleConfirmAddField"
      @cancel="showAddFieldDialog = false"
    />
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
