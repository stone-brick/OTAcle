<script setup lang="ts">
import { ref, computed } from 'vue'
import { mdiClose } from '@mdi/js'
import BaseButton from '../ui/BaseButton.vue'
import BaseIcon from '../ui/BaseIcon.vue'
import FormControl from '../ui/FormControl.vue'
import type { DisplayField } from '../../types'

defineProps<{
  show: boolean
}>()

const emit = defineEmits<{
  confirm: [field: DisplayField]
  cancel: []
}>()

const fieldName = ref('')
const fieldTitle = ref('')
const chartType = ref<DisplayField['chart_type']>('line')

const isValid = computed(() => {
  return fieldName.value.trim() !== '' && fieldTitle.value.trim() !== ''
})

function handleConfirm() {
  if (!isValid.value) return
  emit('confirm', {
    name: fieldName.value.trim(),
    title: fieldTitle.value.trim(),
    chart_type: chartType.value,
  })
  resetForm()
}

function handleCancel() {
  emit('cancel')
  resetForm()
}

function resetForm() {
  fieldName.value = ''
  fieldTitle.value = ''
  chartType.value = 'line'
}
</script>

<template>
  <Teleport to="body">
    <div
      v-if="show"
      class="dialog-overlay"
      @click.self="handleCancel"
    >
      <div class="dialog-panel">
        <div class="dialog-header">
          <h3 class="text-base font-semibold">
            添加图表
          </h3>
          <button
            class="p-1 rounded hover:bg-gray-100 dark:hover:bg-slate-700"
            @click="handleCancel"
          >
            <BaseIcon
              :path="mdiClose"
              :size="20"
            />
          </button>
        </div>

        <div class="dialog-body">
          <div class="form-group">
            <label class="form-label">字段名</label>
            <FormControl
              v-model="fieldName"
              placeholder="如: reward, loss, epsilon"
            />
            <p class="form-hint">
              决策日志中的字段名，支持嵌套如 custom.q_value
            </p>
          </div>

          <div class="form-group">
            <label class="form-label">显示标题</label>
            <FormControl
              v-model="fieldTitle"
              placeholder="如: 奖励值, 损失, 探索率"
            />
          </div>

          <div class="form-group">
            <label class="form-label">图表类型</label>
            <div class="chart-type-grid">
              <button
                v-for="type in ['line', 'bar', 'area', 'gauge'] as const"
                :key="type"
                :class="['chart-type-btn', { active: chartType === type }]"
                @click="chartType = type"
              >
                {{ type }}
              </button>
            </div>
          </div>
        </div>

        <div class="dialog-footer">
          <BaseButton
            label="取消"
            @click="handleCancel"
          />
          <BaseButton
            label="添加"
            variant="primary"
            :disabled="!isValid"
            @click="handleConfirm"
          />
        </div>
      </div>
    </div>
  </Teleport>
</template>

<style scoped>
.dialog-overlay {
  position: fixed;
  inset: 0;
  background: var(--color-overlay);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
}

.dialog-panel {
  background: var(--color-surface);
  border-radius: 12px;
  width: 400px;
  max-width: 90vw;
  box-shadow: 0 20px 40px var(--color-shadow);
}

.dialog-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid var(--color-border-light);
}

.dialog-body {
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding: 16px 20px;
  border-top: 1px solid var(--color-border-light);
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.form-label {
  font-size: 13px;
  font-weight: 500;
  color: var(--color-text);
}

.form-hint {
  font-size: 11px;
  color: var(--color-text-muted);
  margin: 0;
}

.chart-type-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 8px;
}

.chart-type-btn {
  padding: 8px 12px;
  border: 1px solid var(--color-border);
  border-radius: 6px;
  background: var(--color-surface);
  font-size: 13px;
  text-transform: capitalize;
  cursor: pointer;
  transition: all 0.15s;
}

.chart-type-btn:hover {
  border-color: var(--color-primary);
}

.chart-type-btn.active {
  background: var(--color-primary);
  color: white;
  border-color: var(--color-primary);
}
</style>