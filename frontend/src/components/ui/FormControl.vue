<script setup>
import { computed } from 'vue'
import FormControlIcon from '@/components/ui/FormControlIcon.vue'

const props = defineProps({
  name: String,
  id: String,
  autocomplete: String,
  maxlength: String,
  placeholder: String,
  inputmode: String,
  icon: String,
  options: Array,
  type: {
    type: String,
    default: 'text',
  },
  modelValue: {
    type: [String, Number, Boolean, Array, Object],
    default: '',
  },
  required: Boolean,
  borderless: Boolean,
  transparent: Boolean,
})

const emit = defineEmits(['update:modelValue', 'setRef'])

const computedValue = computed({
  get: () => props.modelValue,
  set: (value) => {
    emit('update:modelValue', value)
  },
})

const inputElClass = computed(() => {
  const base = [
    'px-3 py-1 max-w-full focus:outline-none border border-slate-400 rounded-lg w-full focus:border-slate-700',
    'dark:placeholder-gray-400',
    computedType.value === 'textarea' ? 'h-24' : 'py-1',
    props.borderless ? 'border-0' : 'border',
    props.transparent ? 'bg-transparent' : 'bg-white dark:bg-slate-800',
  ]

  if (props.icon) {
    base.push('pl-10')
  }

  return base
})

const computedType = computed(() => (props.options ? 'select' : props.type))

const controlIconH = computed(() => (props.type === 'textarea' ? 'h-full' : 'h-12'))
</script>

<template>
  <div class="relative">
    <select
      v-if="computedType === 'select'"
      :id="id"
      v-model="computedValue"
      :name="name"
      :class="inputElClass"
    >
      <option
        v-for="option in options"
        :key="option.value ?? option"
        :value="option.value ?? option"
      >
        {{ option.label ?? option }}
      </option>
    </select>
    <textarea
      v-else-if="computedType === 'textarea'"
      :id="id"
      v-model="computedValue"
      :class="inputElClass"
      :name="name"
      :maxlength="maxlength"
      :placeholder="placeholder"
      :required="required"
    />
    <input
      v-else
      :id="id"
      v-model="computedValue"
      :name="name"
      :maxlength="maxlength"
      :inputmode="inputmode"
      :autocomplete="autocomplete"
      :required="required"
      :placeholder="placeholder"
      :type="computedType"
      :class="inputElClass"
    >
    <FormControlIcon
      v-if="icon"
      :icon="icon"
      :h="controlIconH"
    />
  </div>
</template>
