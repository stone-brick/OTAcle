<script setup lang="ts">
import type { DisplayField } from '../../types'
import BaseButton from '@/components/ui/BaseButton.vue'
import CardBox from '@/components/ui/CardBox.vue'
import CardBoxComponentBody from '@/components/ui/CardBoxComponentBody.vue'
import { mdiClose } from '@mdi/js'

defineProps<{
  fields: DisplayField[]
  selectedIndex: number | null
  showDelete?: boolean
}>()

const emit = defineEmits<{
  select: [index: number]
  delete: [index: number]
}>()

const chartTypeColors: Record<DisplayField['chart_type'], string> = {
  line: 'bg-blue-500',
  bar: 'bg-orange-500',
  area: 'bg-green-500',
  gauge: 'bg-purple-500',
}

const chartTypeLabels: Record<DisplayField['chart_type'], string> = {
  line: '折线',
  bar: '柱状',
  area: '面积',
  gauge: '仪表',
}
</script>

<template>
  <CardBox class="flex flex-col flex-1 border-l border-gray-100 dark:border-slate-800">
    <div class="px-3 py-2 border-b border-gray-100 dark:border-slate-800">
      <h3 class="m-0 text-sm font-semibold text-gray-700 dark:text-slate-200">
        字段列表 ({{ fields.length }})
      </h3>
    </div>

    <CardBoxComponentBody
      v-if="fields.length === 0"
      class="flex flex-col items-center justify-center flex-1"
    >
      <p class="text-gray-500 dark:text-slate-400 text-sm">
        <slot name="empty">
          暂无字段
        </slot>
      </p>
    </CardBoxComponentBody>

    <CardBoxComponentBody
      v-else
      class="flex-1 overflow-hidden"
    >
      <div class="flex-1 overflow-y-auto p-1.5 flex flex-col gap-1">
        <div
          v-for="(item, idx) in fields"
          :key="idx"
          class="group flex items-center justify-between p-3 rounded-lg cursor-pointer transition-colors border border-transparent"
          :class="selectedIndex === idx
            ? 'bg-blue-50 dark:bg-blue-900/30 border-blue-500 dark:border-blue-400'
            : 'bg-gray-50 dark:bg-slate-800/50 hover:bg-gray-100 dark:hover:bg-slate-800'"
          @click="emit('select', idx)"
        >
          <div class="flex-1 min-w-0">
            <div class="flex items-center gap-2 mb-1">
              <span class="font-mono font-semibold text-xs text-blue-600 dark:text-blue-400">#{{ idx }}</span>
              <span
                class="inline-flex items-center px-2 py-0.5 text-xs font-semibold rounded"
                :class="chartTypeColors[item.chart_type] + ' text-white'"
              >
                {{ chartTypeLabels[item.chart_type] }}
              </span>
            </div>
            <div class="text-sm font-medium text-gray-700 dark:text-slate-200 mb-0.5">
              {{ item.title }}
            </div>
            <div class="text-xs text-gray-500 dark:text-slate-400 font-mono truncate">
              {{ item.name }}
            </div>
          </div>

          <div
            v-if="showDelete"
            class="ml-2 opacity-0 group-hover:opacity-100 transition-opacity"
          >
            <BaseButton
              :icon="mdiClose"
              small
              transparent-bg
              @click.stop="emit('delete', idx)"
            />
          </div>
        </div>
      </div>
    </CardBoxComponentBody>
  </CardBox>
</template>
