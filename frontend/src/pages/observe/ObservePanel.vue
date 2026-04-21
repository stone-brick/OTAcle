<script setup lang="ts">
import { inject, onMounted, onUnmounted, provide, computed, type Ref } from 'vue'
import { useObserve } from '../../composables/useObserve'
import { useObserveCanvas } from '../../composables/useObserveCanvas'

const { startListening, stopListening, fullPreviewFrame, config } = useObserve()

// 使用全局 canvas ref（注入给子组件）
const globalCanvasRef = inject<Ref<HTMLCanvasElement | null>>('globalCanvasRef')

// 统一管理 canvas 渲染
const { actualFps, resetCanvas } = useObserveCanvas(
  globalCanvasRef!,
  fullPreviewFrame,
  computed(() => config.value.crop_regions),
  computed(() => config.value.capture.target_width),
  computed(() => config.value.capture.target_height)
)

// 向子组件提供 FPS 和重置方法
provide('actualFps', actualFps)
provide('resetCanvas', resetCanvas)

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
    <div class="w-72 flex flex-coly flex-shrink-0 border-l border-gray-100 dark:border-slate-800 overflow-y-auto">
      <slot />
    </div>
  </div>
</template>