<script setup lang="ts">
import { ref } from 'vue';
import type { InputBackend } from '../../types';
import { useActionEditor } from '../../composables/useActionEditor';
import ActionListEditor from './ActionListEditor.vue';
import ActionForm from './ActionForm.vue';

const {
  actions,
  defaultBackend,
  configPath,
  isDirty,
  selectedIndex,
  isLoaded,
  selectedAction,
  loadConfig,
  saveConfig,
  createAction,
  updateAction,
  deleteAction,
  selectAction,
  setDefaultBackend,
} = useActionEditor();

const pathInput = ref('');
const showDeleteConfirm = ref(false);
const actionToDelete = ref<number | null>(null);

async function handleLoad() {
  if (pathInput.value.trim()) {
    await loadConfig(pathInput.value.trim());
  }
}

async function handleSave() {
  await saveConfig();
}

async function handleSaveAs() {
  const path = prompt('输入新配置文件路径:', configPath.value || 'actions.json');
  if (path) {
    await saveConfig(path);
  }
}

async function handleCreateAction(type: string) {
  const name = prompt('输入动作名称（可选）:');
  const index = await createAction(type, name || undefined);
  selectAction(index);
}

function handleSelectAction(index: number) {
  selectAction(index);
}

async function handleUpdateAction(index: number, action: any, name: string | null) {
  await updateAction(index, action, name || undefined);
}

async function handleDeleteAction(index: number) {
  actionToDelete.value = index;
  showDeleteConfirm.value = true;
}

async function confirmDelete() {
  if (actionToDelete.value !== null) {
    await deleteAction(actionToDelete.value);
    actionToDelete.value = null;
  }
  showDeleteConfirm.value = false;
}

function cancelDelete() {
  actionToDelete.value = null;
  showDeleteConfirm.value = false;
}

function handleBackendChange(backend: InputBackend) {
  setDefaultBackend(backend);
}
</script>

<template>
  <div class="action-editor">
    <!-- Header -->
    <div class="editor-header">
      <div class="header-left">
        <h3>动作编辑器</h3>
        <span v-if="isDirty" class="dirty-indicator">● 已修改</span>
        <span v-if="configPath" class="config-path">{{ configPath }}</span>
      </div>
      <div class="header-actions">
        <button class="btn-secondary" @click="handleSave" :disabled="!isLoaded || !isDirty">
          保存
        </button>
        <button class="btn-secondary" @click="handleSaveAs" :disabled="!isLoaded">
          另存为
        </button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="editor-content">
      <!-- Left Panel: Action List -->
      <div class="left-panel">
        <!-- Config Path Input -->
        <div class="config-input-section">
          <div class="path-input-row">
            <input
              v-model="pathInput"
              type="text"
              class="path-input"
              placeholder="输入配置文件路径..."
            />
            <button class="btn-primary" @click="handleLoad">加载</button>
          </div>
        </div>

        <!-- Backend Selector -->
        <div class="backend-section">
          <h4>默认后端</h4>
          <div class="backend-selector">
            <label class="radio-label" :class="{ active: defaultBackend === 'win32' }">
              <input
                type="radio"
                :checked="defaultBackend === 'win32'"
                @change="handleBackendChange('win32')"
              />
              <span>Win32</span>
            </label>
            <label class="radio-label" :class="{ active: defaultBackend === 'enigo' }">
              <input
                type="radio"
                :checked="defaultBackend === 'enigo'"
                @change="handleBackendChange('enigo')"
              />
              <span>Enigo</span>
            </label>
          </div>
        </div>

        <!-- Action List -->
        <div class="action-list-section">
          <div class="section-header">
            <h4>动作列表</h4>
            <div class="action-type-selector">
              <select @change="(e) => handleCreateAction((e.target as HTMLSelectElement).value)">
                <option value="" disabled selected>+ 新建动作</option>
                <option value="key">按键</option>
                <option value="key_sequence">按键序列</option>
                <option value="mouse_click">鼠标点击</option>
                <option value="mouse_move">鼠标移动</option>
                <option value="mouse_scroll">鼠标滚动</option>
                <option value="delay">延迟</option>
                <option value="text">文本输入</option>
              </select>
            </div>
          </div>

          <ActionListEditor
            v-if="isLoaded"
            :actions="actions"
            :selectedIndex="selectedIndex"
            @select="handleSelectAction"
            @delete="handleDeleteAction"
          />
          <div v-else class="empty-state">
            <p>请先加载配置文件</p>
          </div>
        </div>
      </div>

      <!-- Right Panel: Action Form -->
      <div class="right-panel">
        <ActionForm
          v-if="selectedAction"
          :action="selectedAction"
          @update="handleUpdateAction"
        />
        <div v-else class="empty-state">
          <p>选择左侧列表中的动作进行编辑</p>
          <p>或点击「新建动作」创建新动作</p>
        </div>
      </div>
    </div>

    <!-- Delete Confirmation Modal -->
    <div v-if="showDeleteConfirm" class="modal-overlay" @click.self="cancelDelete">
      <div class="modal">
        <h4>确认删除</h4>
        <p>确定要删除动作 #{{ actionToDelete }} 吗？此操作无法撤销。</p>
        <div class="modal-actions">
          <button class="btn-secondary" @click="cancelDelete">取消</button>
          <button class="btn-danger" @click="confirmDelete">删除</button>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.action-editor {
  display: flex;
  flex-direction: column;
  height: 100%;
  padding: 16px;
  gap: 16px;
}

.editor-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  background: var(--color-surface);
  border-radius: var(--radius-md);
}

.header-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.header-left h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
}

.dirty-indicator {
  color: var(--color-warning);
  font-size: 12px;
}

.config-path {
  font-size: 11px;
  color: var(--color-text-muted);
  font-family: monospace;
}

.header-actions {
  display: flex;
  gap: 8px;
}

.editor-content {
  display: flex;
  flex: 1;
  gap: 16px;
  min-height: 0;
}

.left-panel {
  width: 320px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.right-panel {
  flex: 1;
  background: var(--color-surface);
  border-radius: var(--radius-md);
  padding: 16px;
  overflow-y: auto;
}

/* Config Input Section */
.config-input-section {
  padding: 12px;
  background: var(--color-surface);
  border-radius: var(--radius-md);
}

.path-input-row {
  display: flex;
  gap: 8px;
}

.path-input {
  flex: 1;
  padding: 8px 12px;
  font-size: 13px;
  background: var(--color-background);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
}

.path-input:focus {
  outline: none;
  border-color: var(--color-primary);
}

/* Backend Section */
.backend-section {
  padding: 12px;
  background: var(--color-surface);
  border-radius: var(--radius-md);
}

.backend-section h4 {
  margin: 0 0 8px 0;
  font-size: 11px;
  font-weight: 600;
  color: var(--color-text-secondary);
  text-transform: uppercase;
}

.backend-selector {
  display: flex;
  gap: 8px;
}

.radio-label {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  background: var(--color-surface-secondary);
  border-radius: var(--radius-sm);
  cursor: pointer;
  font-size: 13px;
}

.radio-label.active {
  background: var(--color-primary-bg);
  border: 1px solid var(--color-primary);
}

.radio-label input[type="radio"] {
  accent-color: var(--color-primary);
}

/* Action List Section */
.action-list-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  padding: 12px;
  background: var(--color-surface);
  border-radius: var(--radius-md);
  min-height: 0;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.section-header h4 {
  margin: 0;
  font-size: 11px;
  font-weight: 600;
  color: var(--color-text-secondary);
  text-transform: uppercase;
}

.action-type-selector select {
  padding: 4px 8px;
  font-size: 12px;
  background: var(--color-primary);
  color: white;
  border: none;
  border-radius: var(--radius-sm);
  cursor: pointer;
}

/* Empty State */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  flex: 1;
  color: var(--color-text-muted);
  font-size: 13px;
  text-align: center;
  padding: 24px;
}

.empty-state p {
  margin: 4px 0;
}

/* Buttons */
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

.btn-primary:hover {
  background: var(--color-primary-hover);
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

.btn-secondary:hover:not(:disabled) {
  background: var(--color-surface-hover);
}

.btn-secondary:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.btn-danger {
  padding: 8px 16px;
  font-size: 13px;
  font-weight: 500;
  background: var(--color-error);
  color: white;
  border: none;
  border-radius: var(--radius-sm);
  cursor: pointer;
}

.btn-danger:hover {
  opacity: 0.9;
}

/* Modal */
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
  max-width: 400px;
  width: 90%;
}

.modal h4 {
  margin: 0 0 12px 0;
  font-size: 16px;
}

.modal p {
  margin: 0 0 20px 0;
  color: var(--color-text-secondary);
  font-size: 14px;
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}
</style>
