<script setup lang="ts">
// Think 模块：模型训练数据展示（待实现）
</script>

<template>
  <div class="think-page">
    <div class="think-placeholder">
      <div class="placeholder-icon">🧠</div>
      <h2>Think 模块</h2>
      <p>模型训练数据展示（待实现）</p>
    </div>
  </div>
</template>

<style scoped>
.think-page {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  background: var(--color-background);
}

.think-placeholder {
  text-align: center;
  color: var(--color-text-muted);
}

.placeholder-icon {
  font-size: 48px;
  margin-bottom: 16px;
}

.think-placeholder h2 {
  margin: 0 0 8px 0;
  font-size: 20px;
  font-weight: 600;
  color: var(--color-text);
}

.think-placeholder p {
  margin: 0;
  font-size: 14px;
}
</style>
