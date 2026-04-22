<script setup lang="ts">
import type { InputBackend } from '../../types';
import BackendSelector from '../../components/act/editor/BackendSelector.vue';
import BaseButton from '@/components/ui/BaseButton.vue';
import BaseLevel from '@/components/ui/BaseLevel.vue';
import { mdiContentSave, mdiPlus, mdiUndo, mdiRedo, mdiDeleteSweep } from '@mdi/js';

defineProps<{
  isProjectLoaded: boolean;
  isLoaded: boolean;
  defaultBackend: InputBackend;
  hasChanges: boolean;
  canUndo: boolean;
  canRedo: boolean;
}>();

const emit = defineEmits<{
  backendChange: [backend: InputBackend];
  save: [];
  openNewActionModal: [];
  discardAll: [];
  undo: [];
  redo: [];
}>();
</script>

<template>
  <div class="bg-white dark:bg-slate-900/70 rounded-xl p-3 flex flex-col md:flex-row items-start md:items-center gap-3">
    <div class="flex items-center gap-3">
      <span class="text-sm font-medium text-gray-500 dark:text-slate-400">执行后台</span>
      <BackendSelector
        :model-value="defaultBackend"
        :hide-default="true"
        @update:model-value="emit('backendChange', $event as InputBackend)"
      />
    </div>

    <BaseLevel class="flex-1 justify-end">
      <BaseButton
        :icon="mdiContentSave"
        color="whiteDark"
        :disabled="!isProjectLoaded || !isLoaded || !hasChanges"
        @click="emit('save')"
      />
      <BaseButton
        :icon="mdiPlus"
        :disabled="!isProjectLoaded"
        @click="emit('openNewActionModal')"
      />
      <BaseButton
        :icon="mdiDeleteSweep"
        color="whiteDark"
        :disabled="!isProjectLoaded || !isLoaded || !hasChanges"
        @click="emit('discardAll')"
      />
      <BaseButton
        :icon="mdiUndo"
        color="whiteDark"
        :disabled="!isProjectLoaded || !isLoaded || !canUndo"
        @click="emit('undo')"
      />
      <BaseButton
        :icon="mdiRedo"
        color="whiteDark"
        :disabled="!isProjectLoaded || !isLoaded || !canRedo"
        @click="emit('redo')"
      />
    </BaseLevel>
  </div>
</template>
