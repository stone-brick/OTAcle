<script setup lang="ts">
import { mdiChartLine } from '@mdi/js'
import BaseIcon from '../../components/ui/BaseIcon.vue'
import CardBox from '../../components/ui/CardBox.vue'
import CardBoxComponentBody from '../../components/ui/CardBoxComponentBody.vue'
import MetricsChart from '../../components/think/MetricsChart.vue'
import type { DecisionLog, ThinkConfig } from '../../types'

defineProps<{
  config: ThinkConfig
  logs: DecisionLog[]
}>()
</script>

<template>
  <div class="monitor-panel p-4 h-full overflow-y-auto">
    <CardBox class="flex flex-col flex-1">
      <CardBoxComponentBody class="flex flex-col gap-4">
        <div
          v-if="config.display_fields.length > 0"
          class="charts-grid"
        >
          <MetricsChart
            v-for="field in config.display_fields"
            :key="field.name"
            :field="field"
            :logs="logs"
          />
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
            暂无图表字段，请先在「图表配置」中添加
          </p>
        </div>
      </CardBoxComponentBody>
    </CardBox>
  </div>
</template>

<style scoped>
.monitor-panel {
  height: 100%;
}

.charts-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 16px;
}
</style>
