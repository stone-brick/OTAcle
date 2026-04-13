<script setup lang="ts">
import { ref, computed } from 'vue'
import ActTabNav from '../components/nav/ActTabNav.vue'
import ZmqMonitor from '../components/ZmqMonitor.vue'
import ActionListEditor from '../components/editor/ActionListEditor.vue'
import ActionForm from '../components/editor/ActionForm.vue'
import WindowSidebar from '../components/WindowSidebar.vue'
import ActionTestPanel from '../components/ActionTestPanel.vue'
import ActionListReadonly from '../components/ActionListReadonly.vue'
import { useWindows } from '../composables/useWindows'
import { useActionEditor } from '../composables/useActionEditor'
import type { InputBackend } from '../types'

const activeTab = ref<'monitor' | 'config' | 'windows'>('monitor')

const { windows, selectedWindow, isLoading, refreshWindows, selectWindow } = useWindows()

const {
  actions,
  defaultBackend,
  configPath,
  isDirty,
  selectedIndex,
  isLoaded,
  selectedAction,
  hasChanges,
  loadConfig,
  saveConfig,
  createAction,
  updateAction,
  deleteAction,
  selectAction,
  setDefaultBackend,
  setExecutionBackend,
  executeActionWithParams,
  setTargetWindow,
  getChangedIndices,
  discardChanges,
  discardAction,
  undo,
  redo,
  canUndo,
  canRedo,
} = useActionEditor()

const pathInput = ref('')
const pathBeforeEdit = ref('')
const isPathFocused = ref(false)
const selectedTestActionIndex = ref<number | null>(null)
const changedIndices = computed(() => getChangedIndices())

// New action modal state
const showNewActionModal = ref(false)
const selectedType = ref<string | null>(null)
const newActionName = ref('')

const actionTypes = [
  { value: 'key', label: '按键' },
  { value: 'key_sequence', label: '序列' },
  { value: 'mouse_click', label: '点击' },
  { value: 'mouse_move', label: '移动' },
  { value: 'mouse_scroll', label: '滚动' },
  { value: 'delay', label: '延迟' },
  { value: 'text', label: '文本' },
]

async function handleLoad() {
  if (pathInput.value.trim()) {
    await loadConfig(pathInput.value.trim())
    pathInput.value = configPath.value
    pathBeforeEdit.value = configPath.value
  }
}

async function handleSave() {
  await saveConfig()
}

async function handleSaveAs() {
  const path = prompt('输入新配置文件路径:', configPath.value || 'actions.json')
  if (path) {
    await saveConfig(path)
  }
}

function openNewActionModal() {
  selectedType.value = null
  newActionName.value = ''
  showNewActionModal.value = true
}

async function confirmCreateAction() {
  if (!selectedType.value) return
  const index = await createAction(selectedType.value, newActionName.value || undefined)
  selectAction(index)
  showNewActionModal.value = false
  selectedType.value = null
  newActionName.value = ''
}

function cancelCreateAction() {
  showNewActionModal.value = false
  selectedType.value = null
  newActionName.value = ''
}

function handleSelectAction(index: number) {
  selectAction(index)
}

async function handleUpdateAction(index: number, action: any, name: string | null) {
  await updateAction(index, action, name || undefined)
}

async function handleDeleteAction(index: number) {
  await deleteAction(index)
}

function handleBackendChange(backend: InputBackend) {
  setDefaultBackend(backend)
  setExecutionBackend(backend)
}

async function handleExecuteAction(payload: { actionId: number; params: Record<string, any> }) {
  await executeActionWithParams(payload.actionId, payload.params);
}

function handleSelectTestAction(index: number) {
  selectedTestActionIndex.value = index;
}

function handleSelectWindow(hwnd: number | null) {
  if (hwnd !== null) {
    selectWindow(hwnd);
  }
  // Sync to backend global state
  const windowSpec = hwnd !== null ? `id:${hwnd}` : null;
  setTargetWindow(windowSpec);
}

function handleDiscardAction(index: number) {
  discardAction(index);
}

function handleDiscardAll() {
  discardChanges();
}

async function handleUndo() {
  await undo();
}

async function handleRedo() {
  await redo();
}
</script>

<template>
  <div class="act-page">
    <!-- Top Navigation -->
    <ActTabNav v-model:activeTab="activeTab" />

    <!-- Tab Content -->
    <div class="act-content">
      <!-- Monitor Tab -->
      <div v-if="activeTab === 'monitor'" class="tab-panel">
        <ZmqMonitor />
      </div>

      <!-- Config Tab -->
      <div v-else-if="activeTab === 'config'" class="tab-panel config-panel">
        <!-- Header -->
        <div class="editor-header">
          <div class="header-left">
            <h3>编辑动作配置</h3>
          </div>
          <div class="header-actions">
            <div class="header-actions-left">
              <input
                v-model="pathInput"
                type="text"
                class="path-input"
                :class="{ 'path-input--faded': !isPathFocused && configPath }"
                placeholder="输入配置文件路径..."
                @focus="isPathFocused = true; pathBeforeEdit = pathInput"
                @blur="(e) => { if (!e.relatedTarget || !(e.relatedTarget as HTMLElement).closest('.btn-primary')) pathInput = pathBeforeEdit; isPathFocused = false }"
                @keydown.enter="handleLoad"
              />
              <span v-if="isDirty" class="dirty-indicator">● 已修改</span>
            </div>
            <div class="header-actions-right">
              <button class="btn-primary" @click="handleLoad">加载</button>
              <button class="btn-secondary" @click="handleSave" :disabled="!isLoaded || !isDirty">
                保存
              </button>
              <button class="btn-secondary" @click="handleSaveAs" :disabled="!isLoaded">
                另存为
              </button>
            </div>
          </div>
        </div>

        <!-- Main Content -->
        <div class="editor-content">
          <!-- Left Panel: Action List -->
          <div class="left-panel">
            <ActionListEditor
              v-if="isLoaded"
              :actions="actions"
              :selectedIndex="selectedIndex"
              :changedIndices="changedIndices"
              @select="handleSelectAction"
              @delete="handleDeleteAction"
              @discard="handleDiscardAction"
            />
            <div v-else class="empty-state">
              <p>请先加载配置文件</p>
            </div>
          </div>

          <!-- Right Panel: Config & Action Form -->
          <div class="right-panel">
            <!-- Top Bar: Backend, New Action -->
            <div class="top-bar">
              <div class="top-bar-left">
                <span class="top-bar-label">执行后台</span>
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

              <div class="top-bar-right">
                <button class="btn-primary" @click="openNewActionModal">
                  + 新建动作
                </button>
                <button
                  class="btn-secondary"
                  :disabled="!hasChanges"
                  @click="handleDiscardAll"
                >
                  撤销全部
                </button>
                <button
                  class="btn-secondary"
                  :disabled="!canUndo()"
                  @click="handleUndo"
                >
                  撤销
                </button>
                <button
                  class="btn-secondary"
                  :disabled="!canRedo()"
                  @click="handleRedo"
                >
                  重做
                </button>
              </div>
            </div>

            <!-- Action Form -->
            <div class="form-area">
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
        </div>

        <!-- New Action Modal -->
        <div v-if="showNewActionModal" class="modal-overlay" @click.self="cancelCreateAction">
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
                  @keydown.enter="confirmCreateAction"
                />
              </div>
            </div>
            <div class="modal-actions">
              <button class="btn-secondary" @click="cancelCreateAction">取消</button>
              <button class="btn-primary" :disabled="!selectedType" @click="confirmCreateAction">创建</button>
            </div>
          </div>
        </div>
      </div>

      <!-- Windows Tab -->
      <div v-else-if="activeTab === 'windows'" class="tab-panel windows-tab-panel">
        <WindowSidebar
          :windows="windows"
          :selectedHwnd="selectedWindow?.hwnd ?? null"
          :isLoading="isLoading"
          @select="handleSelectWindow"
          @refresh="refreshWindows"
        />
        <ActionListReadonly
          :actions="actions"
          :selectedIndex="selectedTestActionIndex"
          @select="handleSelectTestAction"
        />
        <ActionTestPanel
          :selectedWindow="selectedWindow"
          :actions="actions"
          :isLoaded="isLoaded"
          :selectedActionIndex="selectedTestActionIndex"
          @execute="handleExecuteAction"
        />
      </div>
    </div>
  </div>
</template>

<style scoped>
.act-page {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
}

.act-content {
  flex: 1;
  overflow: hidden;
}

.tab-panel {
  height: 100%;
  overflow: auto;
  background: var(--color-background);
}

.windows-tab-panel {
  display: flex;
  flex-direction: row;
  height: 100%;
  overflow: hidden;
}

.windows-tab-panel > :first-child {
  flex: 0 0 380px;
  width: 380px;
}

.windows-tab-panel > :nth-child(2) {
  flex: 1 1 auto;
  min-width: 0;
}

.windows-tab-panel > :last-child {
  flex: 0 0 380px;
  width: 380px;
}

.tab-panel.scrollbar-thin::-webkit-scrollbar {
  width: 6px;
}

.tab-panel.scrollbar-thin::-webkit-scrollbar-thumb {
  background: var(--color-border);
  border-radius: 3px;
}

/* Config Panel */
.config-panel {
  display: flex;
  flex-direction: column;
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

.header-actions {
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex: 1;
  gap: 8px;
  margin-left: 24px;
}

.header-actions-left {
  flex: 1;
  display: flex;
  align-items: center;
  gap: 12px;
}

.header-actions-right {
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
  width: 280px;
  flex-shrink: 0;
  background: var(--color-surface);
  border-radius: var(--radius-md);
  overflow-y: auto;
}

.right-panel {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 12px;
  min-width: 0;
}

.top-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  padding: 12px 16px;
  background: var(--color-surface);
  border-radius: var(--radius-md);
  flex-wrap: wrap;
}

.top-bar-left {
  display: flex;
  align-items: center;
  gap: 12px;
}

.top-bar-right {
  display: flex;
  gap: 8px;
  align-items: center;
}

.top-bar-label {
  font-size: 13px;
  color: var(--color-text-secondary);
  font-weight: 500;
}

.path-input {
  width: 420px;
  flex-shrink: 0;
  padding: 8px 12px;
  font-size: 13px;
  background: var(--color-background);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  font-family: monospace;
  box-sizing: border-box;
}

.path-input:focus {
  outline: none;
  border-color: var(--color-primary);
}

.path-input--faded {
  color: var(--color-text-muted);
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

.new-action-select {
  padding: 6px 12px;
  font-size: 13px;
  background: var(--color-primary);
  color: white;
  border: none;
  border-radius: var(--radius-sm);
  cursor: pointer;
}

.new-action-select:hover {
  background: var(--color-primary-hover);
}

.form-area {
  flex: 1;
  background: var(--color-surface);
  border-radius: var(--radius-md);
  padding: 16px;
  overflow-y: auto;
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

.btn-warning {
  padding: 8px 16px;
  font-size: 13px;
  font-weight: 500;
  background: var(--color-warning-bg);
  color: var(--color-warning);
  border: 1px solid var(--color-warning);
  border-radius: var(--radius-sm);
  cursor: pointer;
}

.btn-warning:hover:not(:disabled) {
  background: var(--color-warning);
  color: white;
}

.btn-warning:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.btn-warning.active {
  background: var(--color-warning);
  color: white;
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
  max-width: 480px;
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

.action-type-display {
  padding: 8px 12px;
  background: var(--color-surface-secondary);
  border-radius: var(--radius-sm);
  font-size: 13px;
  color: var(--color-text);
  text-transform: capitalize;
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
</style>
