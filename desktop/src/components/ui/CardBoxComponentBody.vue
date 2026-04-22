<script setup>
defineProps({
  noPadding: Boolean,
})
</script>

<template>
  <div
    class="flex flex-col h-full"
    :class="{ 'p-2': !noPadding }"
  >
    <slot />
  </div>
</template>
