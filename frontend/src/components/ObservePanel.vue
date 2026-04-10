<script setup lang="ts">
// ObservePanel - Placeholder for future Observe module
// This will display captured frames and annotations in the future
</script>

<template>
  <div class="observe-panel">
    <div class="placeholder">
      <div class="placeholder-icon">🎯</div>
      <h3>观察区（预留）</h3>
      <p>未来将集成画面捕获和标注功能</p>
      <div class="placeholder-hint">
        <span>ZMQ SUB 接收图像帧</span>
        <span>实时画面渲染</span>
        <span>状态数据可视化</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.observe-panel {
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--color-surface);
}

.placeholder {
  text-align: center;
  padding: 40px;
}

.placeholder-icon {
  font-size: 48px;
  margin-bottom: 16px;
  opacity: 0.6;
}

.placeholder h3 {
  margin: 0 0 8px 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--color-text);
}

.placeholder p {
  margin: 0 0 20px 0;
  font-size: 13px;
  color: var(--color-text-secondary);
}

.placeholder-hint {
  display: flex;
  gap: 16px;
  justify-content: center;
  flex-wrap: wrap;
}

.placeholder-hint span {
  padding: 6px 12px;
  font-size: 11px;
  background: var(--color-surface-secondary);
  color: var(--color-text-muted);
  border-radius: var(--radius-lg);
}
</style>
