<script setup lang="ts">
import { ref, watch } from 'vue'
import FormControl from '@/components/ui/FormControl.vue'
import BaseButton from '@/components/ui/BaseButton.vue'
import SegmentedControl from '@/components/ui/SegmentedControl.vue'
import type { DisplayField } from '../../../types'

const props = defineProps<{
  field: DisplayField
  fieldIndex: number
}>()

const emit = defineEmits<{
  update: [index: number, field: DisplayField]
  cancel: []
}>()

const editedField = ref<DisplayField>({ ...props.field })

watch(() => props.field, (newField) => {
  editedField.value = { ...newField }
}, { deep: true })

const chartTypeOptions = [
  { value: 'line', label: '折线' },
  { value: 'bar', label: '柱状' },
  { value: 'area', label: '面积' },
  { value: 'gauge', label: '仪表' },
] as const

function handleUpdate() {
  emit('update', props.fieldIndex, { ...editedField.value })
}

function handleCancel() {
  emit('cancel')
}
</script>

<template>
  <div class="field-form">
    <div class="form-section">
      <label class="form-label">字段名</label>
      <FormControl
        v-model="editedField.name"
        placeholder="如: reward, loss, epsilon"
      />
      <p class="form-hint">
        决策日志中的字段名，支持嵌套如 custom.q_value
      </p>
    </div>

    <div class="form-section">
      <label class="form-label">显示标题</label>
      <FormControl
        v-model="editedField.title"
        placeholder="如: 奖励值, 损失, 探索率"
      />
    </div>

    <div class="form-section">
      <label class="form-label">图表类型</label>
      <SegmentedControl
        v-model="editedField.chart_type"
        :options="chartTypeOptions"
      />
    </div>

    <div class="form-actions">
      <BaseButton
        label="取消"
        @click="handleCancel"
      />
      <BaseButton
        label="保存"
        variant="primary"
        @click="handleUpdate"
      />
    </div>
  </div>
</template>

<style scoped>
.field-form {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.form-section {
  display: flex;
  flex-direction: column;
  gap: 8px;
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

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  padding-top: 8px;
  border-top: 1px solid var(--color-border-light);
}
</style>
