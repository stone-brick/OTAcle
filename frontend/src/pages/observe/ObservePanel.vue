<script setup lang="ts">
import { inject, onMounted, onUnmounted, type Ref } from 'vue'
import CardBox from '@/components/ui/CardBox.vue'
import CardBoxComponentBody from '@/components/ui/CardBoxComponentBody.vue'
import { useObserve } from '../../composables/useObserve'

const { startListening, stopListening } = useObserve()

// 使用全局 canvas ref（注入给子组件）
const globalCanvasRef = inject<Ref<HTMLCanvasElement | null>>('globalCanvasRef')

onMounted(async () => {
  await startListening()
})

onUnmounted(() => {
  stopListening()
})
</script>

<template>
  <div class="flex flex-row h-full">
    <!-- 左侧占位，由父级 canvas-wrapper 覆盖 -->

    <!-- 插槽：右侧配置面板 -->
    <CardBox class="w-72 flex-shrink-0 border-l border-gray-100 dark:border-slate-800 overflow-y-auto">
      <CardBoxComponentBody
        no-padding
        class="flex flex-col gap-4"
      >
        <slot />
      </CardBoxComponentBody>
    </CardBox>
  </div>
</template>