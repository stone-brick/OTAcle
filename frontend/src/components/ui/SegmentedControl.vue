<script setup>
import { computed } from 'vue'

const props = defineProps({
  modelValue: [String, Number, Boolean],
  options: {
    type: Array,
    required: true
  },
  hideDefault: Boolean,
})

const emit = defineEmits(['update:modelValue'])

const visibleOptions = computed(() =>
  props.hideDefault
    ? props.options.filter(o => o.value !== 'default')
    : props.options
)
</script>

<template>
  <div class="flex gap-2">
    <button
      v-for="opt in visibleOptions"
      :key="opt.value"
      type="button"
      class="px-3 py-1.5 text-xs border rounded-lg cursor-pointer transition-colors"
      :class="modelValue === opt.value
        ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/30 text-blue-600 dark:text-blue-400'
        : 'border-gray-300 dark:border-slate-600 text-gray-700 dark:text-slate-300 hover:border-blue-400'"
      @click="emit('update:modelValue', opt.value)"
    >
      {{ opt.label }}
    </button>
  </div>
</template>
