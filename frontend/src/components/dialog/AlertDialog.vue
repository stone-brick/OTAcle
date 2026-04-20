<script setup lang="ts">
import BaseButton from '@/components/ui/BaseButton.vue'

defineProps<{
  show: boolean
  title?: string
  message: string
}>()

const emit = defineEmits<{
  ok: []
}>()

function handleOk() {
  emit('ok')
}
</script>

<template>
  <Teleport to="body">
    <div
      v-if="show"
      class="fixed inset-0 bg-black/50 flex items-center justify-center z-[1000]"
      @click.self="handleOk"
    >
      <div class="bg-white dark:bg-slate-900 rounded-lg p-6 max-w-[400px] w-[90%]">
        <h4
          v-if="title"
          class="mb-3 text-base"
        >
          {{ title }}
        </h4>
        <p class="mb-5 text-sm text-gray-500 dark:text-slate-400 leading-relaxed">
          {{ message }}
        </p>
        <div class="flex justify-end">
          <BaseButton
            label="确定"
            color="info"
            @click="handleOk"
          />
        </div>
      </div>
    </div>
  </Teleport>
</template>
