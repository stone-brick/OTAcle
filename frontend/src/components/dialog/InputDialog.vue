<script setup lang="ts">
import { ref, watch } from 'vue'
import BaseButton from '@/components/ui/BaseButton.vue'
import FormControl from '@/components/ui/FormControl.vue'

const props = defineProps<{
  show: boolean
  title?: string
  message: string
  defaultValue?: string
  placeholder?: string
}>()

const emit = defineEmits<{
  confirm: [value: string]
  cancel: []
}>()

const inputValue = ref('')

watch(() => props.show, (visible) => {
  if (visible) {
    inputValue.value = props.defaultValue ?? ''
  }
})

function handleConfirm() {
  if (inputValue.value.trim()) {
    emit('confirm', inputValue.value.trim())
  }
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === 'Enter') {
    handleConfirm()
  } else if (e.key === 'Escape') {
    emit('cancel')
  }
}
</script>

<template>
  <Teleport to="body">
    <div
      v-if="show"
      class="fixed inset-0 bg-black/50 flex items-center justify-center z-[1000]"
      @click.self="emit('cancel')"
    >
      <div class="bg-white dark:bg-slate-900 rounded-lg p-6 max-w-[400px] w-[90%]">
        <h4
          v-if="title"
          class="mb-3 text-base"
        >
          {{ title }}
        </h4>
        <p class="mb-4 text-sm text-gray-500 dark:text-slate-400 leading-relaxed">
          {{ message }}
        </p>
        <FormControl
          v-model="inputValue"
          type="text"
          class="mb-4"
          :placeholder="placeholder"
          autofocus
          @keydown="handleKeydown"
        />
        <div class="flex justify-end gap-2">
          <BaseButton
            label="取消"
            color="whiteDark"
            @click="emit('cancel')"
          />
          <BaseButton
            label="确定"
            color="info"
            @click="handleConfirm"
          />
        </div>
      </div>
    </div>
  </Teleport>
</template>
