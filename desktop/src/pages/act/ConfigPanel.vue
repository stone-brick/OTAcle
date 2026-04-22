<script setup lang="ts">
import { ref } from 'vue';
import type { ActionItem, InputBackend } from '../../types';
import EditorTopBar from './EditorTopBar.vue';
import ActionList from '../../components/act/ActionList.vue';
import ActionForm from '../../components/act/editor/ActionForm.vue';
import NewActionModal from './NewActionModal.vue';

defineProps<{
  isProjectLoaded: boolean;
  actions: ActionItem[];
  defaultBackend: InputBackend;
  selectedIndex: number | null;
  isLoaded: boolean;
  selectedAction: ActionItem | null;
  hasChanges: boolean;
  canUndo: boolean;
  canRedo: boolean;
}>();

const emit = defineEmits<{
  save: [];
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
  <div class="flex flex-col p-3 gap-3 h-full box-border">
    <div class="flex flex-1 gap-3 min-h-0">
      <!-- 左侧面板：动作列表 -->
      <div class="w-72 flex-shrink-0 flex flex-col rounded-xl overflow-hidden">
        <ActionList
          :actions="isLoaded ? actions : []"
          :selected-index="selectedIndex"
          :show-delete="true"
          @select="handleSelectAction"
          @delete="handleDeleteAction"
        >
          <template #empty>
            <div>
              暂无动作，请加载或创建
            </div>
          </template>
        </ActionList>
      </div>

      <!-- 右侧面板：配置与动作表单 -->
      <div class="flex-1 flex flex-col gap-3 min-w-0">
        <EditorTopBar
          :is-project-loaded="isProjectLoaded"
          :is-loaded="isLoaded"
          :default-backend="defaultBackend"
          :has-changes="hasChanges"
          :can-undo="canUndo"
          :can-redo="canRedo"
          @backend-change="handleBackendChange"
          @save="emit('save')"
          @open-new-action-modal="openNewActionModal"
          @discard-all="emit('discardChanges')"
          @undo="emit('undo')"
          @redo="emit('redo')"
        />

        <!-- 动作表单 -->
        <div class="flex-1 bg-white dark:bg-slate-900/70 rounded-xl p-3 overflow-y-auto">
          <ActionForm
            v-if="selectedAction"
            :action="selectedAction"
            :action-index="selectedIndex ?? 0"
            @update="handleUpdateAction"
          />
          <div
            v-else
            class="flex flex-col items-center justify-center h-full gap-1"
          >
            <p class="text-gray-500 text-sm">
              选择左侧列表中的动作进行编辑
            </p>
            <p class="text-gray-400 text-sm">
              或点击「新建动作」创建新动作
            </p>
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
