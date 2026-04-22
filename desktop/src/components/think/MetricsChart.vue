<script setup lang="ts">
import { computed } from 'vue'
import VChart from 'vue-echarts'
import { use } from 'echarts/core'
import { CanvasRenderer } from 'echarts/renderers'
import { LineChart, BarChart, GaugeChart } from 'echarts/charts'
import {
  TitleComponent,
  TooltipComponent,
  GridComponent,
  LegendComponent,
} from 'echarts/components'
import type { DecisionLog, DisplayField } from '../../types'

use([
  CanvasRenderer,
  LineChart,
  BarChart,
  GaugeChart,
  TitleComponent,
  TooltipComponent,
  GridComponent,
  LegendComponent,
])

const props = defineProps<{
  field: DisplayField
  logs: DecisionLog[]
}>()

function getFieldValue(log: DecisionLog): number | null {
  // 首先尝试从 custom 字段获取
  if (log.custom && props.field.name in log.custom) {
    const val = log.custom[props.field.name]
    if (typeof val === 'number') return val
    if (typeof val === 'string') return parseFloat(val)
  }
  // 然后尝试直接从 log 属性获取
  const directVal = (log as any)[props.field.name]
  if (typeof directVal === 'number') return directVal
  if (typeof directVal === 'string') return parseFloat(directVal)
  return null
}

const chartOption = computed(() => {
  const data: [number, number][] = []
  props.logs.forEach((log, index) => {
    const value = getFieldValue(log)
    if (value !== null && !isNaN(value)) {
      data.push([index, value])
    }
  })

  const baseOption = {
    tooltip: {
      trigger: 'axis',
      formatter: (params: any) => {
        if (params && params.length > 0) {
          const p = params[0]
          return `${props.field.title}<br/>${p.value[1].toFixed(4)}`
        }
        return ''
      },
    },
    grid: {
      left: 50,
      right: 20,
      top: 30,
      bottom: 30,
    },
    xAxis: {
      type: 'value',
      show: false,
    },
    yAxis: {
      type: 'value',
    },
    series: [] as any[],
  }

  switch (props.field.chart_type) {
    case 'line':
      return {
        ...baseOption,
        series: [
          {
            type: 'line',
            data,
            smooth: true,
            showSymbol: false,
            lineStyle: {
              width: 2,
            },
          },
        ],
      }
    case 'bar':
      return {
        ...baseOption,
        series: [
          {
            type: 'bar',
            data,
            itemStyle: {
              borderRadius: [4, 4, 0, 0],
            },
          },
        ],
      }
    case 'area':
      return {
        ...baseOption,
        series: [
          {
            type: 'line',
            data,
            smooth: true,
            showSymbol: false,
            areaStyle: {
              opacity: 0.3,
            },
            lineStyle: {
              width: 2,
            },
          },
        ],
      }
    case 'gauge': {
      const latest = data.length > 0 ? data[data.length - 1][1] : 0
      return {
        series: [
          {
            type: 'gauge',
            startAngle: 180,
            endAngle: 0,
            min: 0,
            max: 100,
            splitNumber: 5,
            axisLine: {
              lineStyle: {
                width: 20,
              },
            },
            pointer: {
              length: '60%',
            },
            detail: {
              formatter: '{value}',
              fontSize: 14,
            },
            data: [{ value: latest }],
          },
        ],
      }
    }
    default:
      return baseOption
  }
})
</script>

<template>
  <div class="metrics-chart">
    <h3 class="chart-title">
      {{ field.title }}
    </h3>
    <v-chart
      class="chart"
      :option="chartOption"
      autoresize
    />
  </div>
</template>

<style scoped>
.metrics-chart {
  background: var(--color-base-200);
  border-radius: 8px;
  padding: 12px;
}

.chart-title {
  font-size: 14px;
  font-weight: 600;
  margin-bottom: 8px;
  color: var(--color-text);
}

.chart {
  width: 100%;
  height: 200px;
}
</style>