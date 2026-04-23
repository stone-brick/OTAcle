<script setup lang="ts">
import { ref, provide } from 'vue'
import ObserveTabNav from '../../components/nav/ObserveTabNav.vue'
import CaptureTabPanel from './CaptureTabPanel.vue'
import CropTabPanel from './CropTabPanel.vue'
import TransportTabPanel from './TransportTabPanel.vue'

const activeTab = ref<'capture' | 'crop' | 'transport'>('capture')

// 全局共享的 Canvas ref
const globalCanvasRef = ref<HTMLCanvasElement | null>(null)
provide('globalCanvasRef', globalCanvasRef)
</script>

<template>
  <div class="observe-page">
    <ObserveTabNav v-model:active-tab="activeTab" />

    <div class="observe-content">
      <!-- 全局 canvas，始终保留在 DOM 中 -->
      <div class="canvas-wrapper">
        <canvas
          ref="globalCanvasRef"
          class="max-w-full max-h-full object-contain"
        />
      </div>

      <!-- Tab 面板，共享同一 canvas -->
      <CaptureTabPanel v-if="activeTab === 'capture'" />
      <CropTabPanel v-else-if="activeTab === 'crop'" />
      <TransportTabPanel v-else-if="activeTab === 'transport'" />
    </div>
  </div>
</template>

<style scoped>
.observe-page {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
}

.observe-content {
  flex: 1;
  display: flex;
  overflow: hidden;
  position: relative;
}

.canvas-wrapper {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--color-background);
  padding: 1rem;
  min-width: 0;
}

.canvas-wrapper canvas {
  max-width: 600px;
  max-height: 500px;
}

.observe-content > :deep(*) {
  height: 100%;
  overflow: auto;
}
</style>