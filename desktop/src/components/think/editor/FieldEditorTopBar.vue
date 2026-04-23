<script setup lang="ts">
import BaseButton from '@/components/ui/BaseButton.vue'
import BaseLevel from '@/components/ui/BaseLevel.vue'
import { mdiContentSave, mdiPlus, mdiDelete } from '@mdi/js'

defineProps<{
  isProjectLoaded: boolean
  hasSelection: boolean
}>()

const emit = defineEmits<{
  save: []
  addField: []
  deleteField: []
}>()
</script>

<template>
  <div class="bg-white dark:bg-slate-900/70 rounded-xl p-3 flex items-center gap-3">
    <BaseLevel class="flex-1 justify-end">
      <BaseButton
        :icon="mdiContentSave"
        color="whiteDark"
        title="保存配置"
        :disabled="!isProjectLoaded"
        @click="emit('save')"
      />
      <BaseButton
        :icon="mdiPlus"
        color="whiteDark"
        title="添加字段"
        :disabled="!isProjectLoaded"
        @click="emit('addField')"
      />
      <BaseButton
        :icon="mdiDelete"
        color="whiteDark"
        title="删除选中字段"
        :disabled="!hasSelection"
        @click="emit('deleteField')"
      />
    </BaseLevel>
  </div>
</template>
