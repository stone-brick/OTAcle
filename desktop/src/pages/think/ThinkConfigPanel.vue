<script setup lang="ts">
import { ref, computed } from 'vue'
import type { DisplayField, ThinkConfig } from '../../types'
import FieldEditorTopBar from '../../components/think/editor/FieldEditorTopBar.vue'
import FieldList from '../../components/think/FieldList.vue'
import FieldForm from '../../components/think/editor/FieldForm.vue'
import AddFieldDialog from '../../components/think/AddFieldDialog.vue'

const props = defineProps<{
  config: ThinkConfig
  isProjectLoaded: boolean
}>()

const emit = defineEmits<{
  save: []
  updateField: [index: number, field: DisplayField]
  addField: [field: DisplayField]
  deleteField: [index: number]
}>()

const selectedIndex = ref<number | null>(null)
const showAddFieldDialog = ref(false)

const selectedField = computed(() =>
  selectedIndex.value !== null ? props.config.display_fields[selectedIndex.value] : null
)

function handleSelectField(index: number) {
  selectedIndex.value = index
}

function handleUpdateField(index: number, field: DisplayField) {
  emit('updateField', index, field)
}

function handleDeleteField(index: number) {
  emit('deleteField', index)
  if (selectedIndex.value === index) {
    selectedIndex.value = null
  }
}

function handleDiscardChanges() {
  selectedIndex.value = null
}

function handleAddFieldConfirm(field: DisplayField) {
  emit('addField', field)
  showAddFieldDialog.value = false
}
</script>

<template>
  <div class="flex flex-col p-3 gap-3 h-full box-border">
    <div class="flex flex-1 gap-3 min-h-0">
      <!-- 左侧面板：字段列表 w-72 -->
      <div class="w-72 flex-shrink-0 flex flex-col rounded-xl overflow-hidden">
        <FieldList
          :fields="config.display_fields"
          :selected-index="selectedIndex"
          :show-delete="true"
          @select="handleSelectField"
          @delete="handleDeleteField"
        >
          <template #empty>
            <div class="text-sm text-gray-500 dark:text-slate-400">
              暂无字段，请添加
            </div>
          </template>
        </FieldList>
      </div>

      <!-- 右侧面板：工具栏 + 表单 -->
      <div class="flex-1 flex flex-col gap-3 min-w-0">
        <FieldEditorTopBar
          :is-project-loaded="isProjectLoaded"
          :has-selection="selectedIndex !== null"
          @save="emit('save')"
          @add-field="showAddFieldDialog = true"
          @delete-field="handleDeleteField(selectedIndex!)"
        />

        <div class="flex-1 bg-white dark:bg-slate-900/70 rounded-xl p-4 overflow-y-auto">
          <FieldForm
            v-if="selectedField"
            :field="selectedField"
            :field-index="selectedIndex ?? 0"
            @update="handleUpdateField"
            @cancel="handleDiscardChanges"
          />
          <div
            v-else
            class="flex flex-col items-center justify-center h-full gap-1"
          >
            <p class="text-gray-500 text-sm">
              选择左侧列表中的字段进行编辑
            </p>
            <p class="text-gray-400 text-sm">
              或点击「添加字段」创建新字段
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>

  <AddFieldDialog
    :show="showAddFieldDialog"
    @confirm="handleAddFieldConfirm"
    @cancel="showAddFieldDialog = false"
  />
</template>
