<script setup lang="ts">
import { ref } from 'vue';

defineProps<{
  show: boolean;
}>();

const emit = defineEmits<{
  confirm: [type: string, name: string | undefined];
  cancel: [];
}>();

const selectedType = ref<string | null>(null);
const newActionName = ref('');

const actionTypes = [
  { value: 'key', label: '按键' },
  { value: 'key_sequence', label: '序列' },
  { value: 'mouse_click', label: '点击' },
  { value: 'mouse_move', label: '移动' },
  { value: 'mouse_scroll', label: '滚动' },
  { value: 'delay', label: '延迟' },
  { value: 'text', label: '文本' },
];

function handleConfirm() {
  if (!selectedType.value) return;
  emit('confirm', selectedType.value, newActionName.value || undefined);
  resetState();
}

function handleCancel() {
  emit('cancel');
  resetState();
}

function resetState() {
  selectedType.value = null;
  newActionName.value = '';
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter' && selectedType.value) {
    handleConfirm();
  }
}
</script>

<template>
  <div v-if="show" class="modal-overlay" @click.self="handleCancel">
    <div class="modal">
      <h4>新建动作</h4>
      <div class="modal-form">
        <div class="form-field">
          <label>动作类型</label>
          <div class="action-type-grid">
            <button
              v-for="type in actionTypes"
              :key="type.value"
              class="type-btn"
              :class="{ active: selectedType === type.value }"
              @click="selectedType = type.value"
            >
              {{ type.label }}
            </button>
          </div>
        </div>
        <div class="form-field">
          <label>名称（可选）</label>
          <input
            v-model="newActionName"
            type="text"
            class="text-input"
            placeholder="输入动作名称..."
            @keydown="handleKeydown"
          />
        </div>
      </div>
      <div class="modal-actions">
        <button class="btn-secondary" @click="handleCancel">取消</button>
        <button class="btn-primary" :disabled="!selectedType" @click="handleConfirm">创建</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal {
  background: var(--color-surface);
  border-radius: var(--radius-md);
  padding: 24px;
  max-width: 480px;
  width: 90%;
}

.modal h4 {
  margin: 0 0 12px 0;
  font-size: 16px;
}

.modal-form {
  margin-bottom: 20px;
}

.modal-form .form-field {
  margin-bottom: 12px;
}

.modal-form label {
  display: block;
  font-size: 12px;
  font-weight: 500;
  color: var(--color-text-secondary);
  margin-bottom: 4px;
}

.action-type-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 8px;
}

.type-btn {
  padding: 12px 8px;
  background: var(--color-surface-secondary);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  cursor: pointer;
  font-size: 13px;
  color: var(--color-text);
  transition: all 0.15s;
}

.type-btn:hover {
  background: var(--color-surface-hover);
}

.type-btn.active {
  background: var(--color-primary-bg);
  border-color: var(--color-primary);
  color: var(--color-primary);
}

.text-input {
  width: 100%;
  padding: 8px 12px;
  font-size: 13px;
  background: var(--color-background);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  box-sizing: border-box;
}

.text-input:focus {
  outline: none;
  border-color: var(--color-primary);
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}

.btn-primary {
  padding: 8px 16px;
  font-size: 13px;
  font-weight: 500;
  background: var(--color-primary);
  color: white;
  border: none;
  border-radius: var(--radius-sm);
  cursor: pointer;
}

.btn-primary:hover:not(:disabled) {
  background: var(--color-primary-hover);
}

.btn-primary:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.btn-secondary {
  padding: 8px 16px;
  font-size: 13px;
  font-weight: 500;
  background: var(--color-surface-secondary);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  cursor: pointer;
}

.btn-secondary:hover {
  background: var(--color-surface-hover);
}
</style>
