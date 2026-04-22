<script setup lang="ts">
import { mdiChartLine, mdiPlus, mdiClose } from '@mdi/js'
import BaseIcon from '../ui/BaseIcon.vue'
import BaseButton from '../ui/BaseButton.vue'
import CardBox from '../ui/CardBox.vue'
import CardBoxComponentBody from '../ui/CardBoxComponentBody.vue'
import MetricsChart from './MetricsChart.vue'
import type { DecisionLog, DisplayField, ThinkConfig } from '../../types'

defineProps<{
  config: ThinkConfig
  logs: DecisionLog[]
}>()

const emit = defineEmits<{
  addField: [field: DisplayField]
  deleteField: [name: string]
  loadDemo: []
}>()
</script>

<template>
  <div class="config-tab-panel">
    <CardBox class="flex flex-col flex-1">
      <div class="flex items-center justify-between px-4 py-3 border-b border-gray-100 dark:border-slate-800">
        <span class="text-sm font-semibold text-gray-500 dark:text-slate-400">
          图表配置 ({{ config.display_fields.length }})
        </span>
        <div class="flex gap-2">
          <BaseButton
            :icon="mdiChartLine"
            color="whiteDark"
            title="加载演示配置"
            @click="emit('loadDemo')"
          />
          <BaseButton
            :icon="mdiPlus"
            color="whiteDark"
            @click="emit('addField')"
          />
        </div>
      </div>

      <CardBoxComponentBody class="flex flex-col gap-4">
        <div
          v-if="config.display_fields.length > 0"
          class="charts-grid"
        >
          <div
            v-for="field in config.display_fields"
            :key="field.name"
            class="group chart-wrapper"
          >
            <div class="ml-2 opacity-0 group-hover:opacity-100 transition-opacity">
              <BaseButton
                :icon="mdiClose"
                small
                transparent-bg
                @click.stop="emit('deleteField', field.name)"
              />
            </div>
            <MetricsChart
              :field="field"
              :logs="logs"
            />
          </div>
        </div>
        <div
          v-else
          class="flex flex-col items-center justify-center py-12"
        >
          <BaseIcon
            :path="mdiChartLine"
            :size="48"
            class="text-gray-300 dark:text-gray-600 mb-4"
          />
          <p class="text-gray-500 dark:text-gray-400 text-center">
            暂无图表字段
          </p>
          <BaseButton
            class="mt-4"
            :icon="mdiPlus"
            label="添加图表"
            color="whiteDark"
            @click="emit('addField')"
          />
        </div>
      </CardBoxComponentBody>
    </CardBox>
  </div>
</template>

<style scoped>
.config-tab-panel {
  height: 100%;
}

.charts-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 16px;
}

.chart-wrapper {
  position: relative;
}

.delete-btn {
  position: absolute;
  top: 8px;
  right: 8px;
  z-index: 10;
  padding: 4px;
  border-radius: 4px;
  background: rgba(0, 0, 0, 0.5);
  color: white;
  border: none;
  cursor: pointer;
  opacity: 0;
  transition: opacity 0.15s;
}

.chart-wrapper:hover .delete-btn {
  opacity: 1;
}

.delete-btn:hover {
  background: rgba(239, 68, 68, 0.8);
}
</style>