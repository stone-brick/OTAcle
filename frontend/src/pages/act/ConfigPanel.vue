<script setup lang="ts">
import { ref } from 'vue';
import type { ActionItem, InputBackend } from '../../types';
import EditorHeader from './EditorHeader.vue';
import EditorTopBar from './EditorTopBar.vue';
import ActionList from '../../components/ActionList.vue';
import ActionForm from '../../components/editor/ActionForm.vue';
import NewActionModal from './NewActionModal.vue';

defineProps<{
  actions: ActionItem[];
  defaultBackend: InputBackend;
  configPath: string;
  selectedIndex: number | null;
  isLoaded: boolean;
  selectedAction: ActionItem | null;
  hasChanges: boolean;
  canUndo: boolean;
  canRedo: boolean;
}>();

const emit = defineEmits<{
  load: [path: string];
  save: [];
  saveAs: [];
  selectAction: [index: number | null];
  updateAction: [index: number, action: ActionItem];
  deleteAction: [index: number];
  discardChanges: [];
  undo: [];
  redo: [];
  setDefaultBackend: [backend: InputBackend];
  createAction: [type: string, name: string | undefined];
}>();

// 模态框状态
const showNewActionModal = ref(false);

function openNewActionModal() {
  showNewActionModal.value = true;
}

function handleConfirmCreateAction(type: string, name: string | undefined) {
  emit('createAction', type, name);
  showNewActionModal.value = false;
}

function handleCancelCreateAction() {
  showNewActionModal.value = false;
}

function handleBackendChange(backend: InputBackend) {
  emit('setDefaultBackend', backend);
}

function handleSelectAction(index: number) {
  emit('selectAction', index);
}

function handleUpdateAction(index: number, action: ActionItem) {
  emit('updateAction', index, action);
}

function handleDeleteAction(index: number) {
  emit('deleteAction', index);
}
</script>

<template>
  <div class="config-panel">
    <EditorHeader
      :config-path="configPath"
      :is-dirty="hasChanges"
      :is-loaded="isLoaded"
      @load="emit('load', $event)"
      @save="emit('save')"
      @save-as="emit('saveAs')"
    />

    <div class="editor-content">
      <!-- 左侧面板：动作列表 -->
      <div class="left-panel">
        <ActionList
          v-if="isLoaded"
          :actions="actions"
          :selected-index="selectedIndex"
          :show-delete="true"
          @select="handleSelectAction"
          @delete="handleDeleteAction"
        >
          <template #empty>
            <div class="empty-state-custom">
              暂无动作，请点击上方「新建动作」创建
            </div>
          </template>
        </ActionList>
        <div
          v-else
          class="empty-state"
        >
          <p>请先加载配置文件</p>
        </div>
      </div>

      <!-- 右侧面板：配置与动作表单 -->
      <div class="right-panel">
        <EditorTopBar
          :default-backend="defaultBackend"
          :has-changes="hasChanges"
          :can-undo="canUndo"
          :can-redo="canRedo"
          @backend-change="handleBackendChange"
          @open-new-action-modal="openNewActionModal"
          @discard-all="emit('discardChanges')"
          @undo="emit('undo')"
          @redo="emit('redo')"
        />

        <!-- 动作表单 -->
        <div class="form-area">
          <ActionForm
            v-if="selectedAction"
            :action="selectedAction"
            :action-index="selectedIndex ?? 0"
            @update="handleUpdateAction"
          />
          <div
            v-else
            class="empty-state"
          >
            <p>选择左侧列表中的动作进行编辑</p>
            <p>或点击「新建动作」创建新动作</p>
          </div>
        </div>
      </div>
    </div>

    <NewActionModal
      :show="showNewActionModal"
      @confirm="handleConfirmCreateAction"
      @cancel="handleCancelCreateAction"
    />
  </div>
</template>

<style scoped>
.config-panel {
  display: flex;
  flex-direction: column;
  padding: 16px;
  gap: 16px;
  height: 100%;
  box-sizing: border-box;
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

.form-area {
  flex: 1;
  background: var(--color-surface);
  border-radius: var(--radius-md);
  padding: 16px;
  overflow-y: auto;
}

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

.empty-state-custom {
  text-align: center;
  padding: 24px 12px;
  color: var(--color-text-muted);
  font-size: 13px;
}
</style>
