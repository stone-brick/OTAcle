<script setup>
import { computed } from 'vue'
import { RouterLink } from 'vue-router'
import { getButtonColor, colorsTransparent } from '../../utils/colors.js'
import BaseIcon from '@/components/ui/BaseIcon.vue'

const props = defineProps({
  label: [String, Number],
  icon: String,
  iconSize: [String, Number],
  href: String,
  target: String,
  to: [String, Object],
  type: String,
  color: {
    type: String,
    default: 'white',
  },
  as: String,
  small: Boolean,
  outline: Boolean,
  active: Boolean,
  disabled: Boolean,
  roundedFull: Boolean,
  transparentBg: Boolean,
})

const is = computed(() => {
  if (props.as) {
    return props.as
  }

  if (props.to) {
    return RouterLink
  }

  if (props.href) {
    return 'a'
  }

  return 'button'
})

const computedType = computed(() => {
  if (is.value === 'button') {
    return props.type ?? 'button'
  }

  return null
})

const labelClass = computed(() => (props.small && props.icon ? 'px-1' : 'px-2'))

const componentClass = computed(() => {
  const base = [
    'inline-flex',
    'justify-center',
    'items-center',
    'whitespace-nowrap',
    'focus:outline-hidden',
    'transition-colors',
    'border',
    props.disabled ? 'cursor-not-allowed' : 'cursor-pointer',
    props.roundedFull ? 'rounded-full' : 'rounded-sm',
  ]

  if (props.transparentBg) {
    base.push(colorsTransparent.border, colorsTransparent.ring, colorsTransparent.bg)
    if (!props.disabled) {
      base.push(colorsTransparent.bgHover)
    }
  } else {
    base.push(getButtonColor(props.color, props.outline, !props.disabled, props.active))
  }

  if (!props.label && props.icon) {
    base.push('p-1')
  } else if (props.small) {
    base.push('text-xs', props.roundedFull ? 'px-3 py-0.5' : 'p-1')
  } else {
    base.push('py-2', props.roundedFull ? 'px-6' : 'px-3')
  }

  if (props.disabled) {
    base.push('opacity-40')
  }

  return base
})
</script>

<template>
  <component
    :is="is"
    :class="componentClass"
    :href="href"
    :type="computedType"
    :to="to"
    :target="target"
    :disabled="disabled"
  >
    <BaseIcon
      v-if="icon"
      :path="icon"
      :size="iconSize"
    />
    <span
      v-if="label"
      :class="labelClass"
    >{{ label }}</span>
  </component>
</template>
