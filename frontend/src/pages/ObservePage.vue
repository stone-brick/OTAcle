<script setup lang="ts">
import { ref } from 'vue'
import ObserveTabNav from '../components/nav/ObserveTabNav.vue'
import CaptureTabPanel from './observe/CaptureTabPanel.vue'
import CropTabPanel from './observe/CropTabPanel.vue'
import TransportTabPanel from './observe/TransportTabPanel.vue'

const activeTab = ref<'capture' | 'crop' | 'transport'>('capture')
</script>

<template>
  <div class="observe-page">
    <ObserveTabNav v-model:activeTab="activeTab" />

    <div class="observe-content">
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
  overflow: hidden;
}

.observe-content > :deep(*) {
  height: 100%;
  overflow: auto;
}
</style>
