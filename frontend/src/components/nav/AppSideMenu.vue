<script setup lang="ts">
import { computed } from 'vue'
import { useRouter, useRoute } from 'vue-router'

const router = useRouter()
const route = useRoute()

interface NavItem {
  id: string
  label: string
  icon: string
  path: string
}

const menuItems: NavItem[] = [
  { id: 'observe', label: '观察', icon: '👁', path: '/observe' },
  { id: 'think', label: '思考', icon: '💭', path: '/think' },
  { id: 'act', label: '执行', icon: '🎯', path: '/act' },
]

const currentPath = computed(() => route.path)

function navigateTo(path: string) {
  router.push(path)
}

function isActive(path: string): boolean {
  return currentPath.value === path
}
</script>

<template>
  <nav class="side-menu">
    <div class="menu-header">
      <span class="logo">OTAcle</span>
    </div>

    <ul class="menu-list">
      <li
        v-for="item in menuItems"
        :key="item.id"
        class="menu-item"
        :class="{ active: isActive(item.path) }"
        @click="navigateTo(item.path)"
      >
        <span class="menu-icon">{{ item.icon }}</span>
        <span class="menu-label">{{ item.label }}</span>
      </li>
    </ul>
  </nav>
</template>

<style scoped>
.side-menu {
  width: 160px;
  height: 100%;
  background: var(--color-surface);
  border-right: 1px solid var(--color-border);
  display: flex;
  flex-direction: column;
}

.menu-header {
  padding: 16px;
  border-bottom: 1px solid var(--color-border);
}

.logo {
  font-size: 18px;
  font-weight: 700;
  color: var(--color-primary);
}

.menu-list {
  list-style: none;
  padding: 8px;
  margin: 0;
}

.menu-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px 16px;
  margin-bottom: 4px;
  border-radius: 8px;
  cursor: pointer;
  transition: background 0.15s, color 0.15s;
  color: var(--color-text-secondary);
}

.menu-item:hover {
  background: var(--color-hover);
  color: var(--color-text);
}

.menu-item.active {
  background: var(--color-primary-bg);
  color: var(--color-primary);
  font-weight: 600;
}

.menu-icon {
  font-size: 18px;
}

.menu-label {
  font-size: 14px;
}
</style>
