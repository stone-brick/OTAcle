<script setup lang="ts">
defineProps<{
  activeTab: 'capture' | 'crop' | 'transport'
}>()

const emit = defineEmits<{
  'update:activeTab': [tab: 'capture' | 'crop' | 'transport']
}>()

interface TabItem {
  id: 'capture' | 'crop' | 'transport'
  label: string
  icon: string
}

const tabs: TabItem[] = [
  { id: 'capture', label: '截图配置', icon: '📷' },
  { id: 'crop', label: '裁切配置', icon: '✂️' },
  { id: 'transport', label: '传输配置', icon: '📡' },
]

function selectTab(tab: TabItem) {
  emit('update:activeTab', tab.id)
}
</script>

<template>
  <nav class="observe-tab-nav">
    <div class="nav-brand">
      <span class="brand-icon">👁️</span>
      <span class="brand-text">Observe</span>
    </div>
    <ul class="tab-list">
      <li
        v-for="tab in tabs"
        :key="tab.id"
        class="tab-item"
        :class="{ active: activeTab === tab.id }"
        @click="selectTab(tab)"
      >
        <span class="tab-icon">{{ tab.icon }}</span>
        <span class="tab-label">{{ tab.label }}</span>
      </li>
    </ul>
  </nav>
</template>

<style scoped>
.observe-tab-nav {
  display: flex;
  align-items: center;
  gap: 24px;
  padding: 0 16px;
  height: 48px;
  background: var(--color-surface);
  border-bottom: 1px solid var(--color-border);
}

.nav-brand {
  display: flex;
  align-items: center;
  gap: 8px;
  padding-right: 16px;
  border-right: 1px solid var(--color-border);
}

.brand-icon {
  font-size: 18px;
}

.brand-text {
  font-size: 16px;
  font-weight: 700;
  color: var(--color-primary);
}

.tab-list {
  display: flex;
  list-style: none;
  margin: 0;
  padding: 0;
  gap: 4px;
}

.tab-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 16px;
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: background var(--transition-duration), color var(--transition-duration);
  color: var(--color-text-secondary);
}

.tab-item:hover {
  background: var(--color-hover);
  color: var(--color-text);
}

.tab-item.active {
  background: var(--color-primary-bg);
  color: var(--color-primary);
  font-weight: 600;
}

.tab-icon {
  font-size: 16px;
}

.tab-label {
  font-size: 14px;
}
</style>
