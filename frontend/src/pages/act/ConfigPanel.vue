<script setup lang="ts">
import { ref, computed } from 'vue';
import type { ActionItem, InputBackend, Action } from '../../types';
import EditorHeader from './EditorHeader.vue';
import EditorTopBar from './EditorTopBar.vue';
import ActionListEditor from '../../components/editor/ActionListEditor.vue';
import ActionForm from '../../components/editor/ActionForm.vue';
import NewActionModal from './NewActionModal.vue';

defineProps<{
  actions: ActionItem[];
  defaultBackend: InputBackend;
  configPath: string;
  isDirty: boolean;
  selectedIndex: number | null;
  isLoaded: boolean;
  selectedAction: ActionItem | null;
  hasChanges: boolean;
  canUndo: () => boolean;
  canRedo: () => boolean;
}>();

const emit = defineEmits<{
  load: [path: string];
  save: [];
  saveAs: [];
  selectAction: [index: number | null];
  updateAction: [index: number, action: Action, name: string | null | undefined];
  deleteAction: [index: number];
  discardAction: [index: number];
  discardChanges: [];
  undo: [];
  redo: [];
  setDefaultBackend: [backend: InputBackend];
  setExecutionBackend: [backend: InputBackend];
  createAction: [type: string, name: string | undefined];
}>();

const changedIndices = computed(() => {
  return []; // Change tracking is done in useActionEditor
});

// Modal state
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
  emit('setExecutionBackend', backend);
}

function handleSelectAction(index: number) {
  emit('selectAction', index);
}

function handleUpdateAction(index: number, action: Action, name: string | null) {
  emit('updateAction', index, action, name || undefined);
}

function handleDeleteAction(index: number) {
  emit('deleteAction', index);
}

function handleDiscardAction(index: number) {
  emit('discardAction', index);
}
</script>

<template>
  <div class="config-panel">
    <EditorHeader
      :configPath="configPath"
      :isDirty="isDirty"
      :isLoaded="isLoaded"
      @load="emit('load', $event)"
      @save="emit('save')"
      @saveAs="emit('saveAs')"
    />

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
        <EditorTopBar
          :defaultBackend="defaultBackend"
          :hasChanges="hasChanges"
          :canUndo="canUndo()"
          :canRedo="canRedo()"
          @backendChange="handleBackendChange"
          @openNewActionModal="openNewActionModal"
          @discardAll="emit('discardChanges')"
          @undo="emit('undo')"
          @redo="emit('redo')"
        />

        <!-- Action Form -->
        <div class="form-area">
          <ActionForm
            v-if="selectedAction"
            :action="selectedAction"
            :actionIndex="selectedIndex ?? 0"
            @update="handleUpdateAction"
          />
          <div v-else class="empty-state">
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
</style>
