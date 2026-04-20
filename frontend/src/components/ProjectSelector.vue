<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { useProject, type RecentProject } from '../composables/useProject';

const {
  currentProject,
  recentProjects,
  isProjectLoaded,
  isLoading,
  openProjectDialog,
  createProjectDialog,
  closeProject,
  refreshRecentProjects,
  removeRecentProject,
  togglePinProject,
  openProject,
} = useProject();

const showRecentList = ref(false);
const hoveredProject = ref<string | null>(null);

onMounted(() => {
  refreshRecentProjects();
});

function formatDate(dateStr: string): string {
  const date = new Date(dateStr);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));

  if (diffDays === 0) return '今天';
  if (diffDays === 1) return '昨天';
  if (diffDays < 7) return `${diffDays} 天前`;
  return date.toLocaleDateString('zh-CN');
}

const pinnedProjects = computed(() => recentProjects.value.filter(p => p.pinned));
const unpinnedProjects = computed(() => recentProjects.value.filter(p => !p.pinned));

function handleOpenRecent(project: RecentProject) {
  openProject(project.path);
  showRecentList.value = false;
}

function handleTogglePin(e: Event, path: string) {
  e.stopPropagation();
  togglePinProject(path);
}

function handleRemoveRecent(e: Event, path: string) {
  e.stopPropagation();
  removeRecentProject(path);
}
</script>

<template>
  <div class="project-selector">
    <div class="current-project" v-if="isProjectLoaded && currentProject">
      <div class="project-info">
        <span class="project-icon">📁</span>
        <span class="project-name" :title="currentProject.path">{{ currentProject.name }}</span>
      </div>
      <button class="close-btn" @click="closeProject" title="关闭项目">×</button>
    </div>

    <div class="no-project" v-else>
      <div class="project-actions">
        <button class="action-btn primary" @click="openProjectDialog" :disabled="isLoading">
          📂 打开项目
        </button>
        <button class="action-btn" @click="createProjectDialog" :disabled="isLoading">
          ➕ 新建项目
        </button>
      </div>

      <div class="recent-dropdown" v-if="recentProjects.length > 0">
        <button class="recent-toggle" @click="showRecentList = !showRecentList">
          📌 最近项目
          <span class="arrow">{{ showRecentList ? '▲' : '▼' }}</span>
        </button>

        <div class="recent-list" v-if="showRecentList">
          <div
            v-for="project in [...pinnedProjects, ...unpinnedProjects]"
            :key="project.path"
            class="recent-item"
            :class="{ pinned: project.pinned }"
            @click="handleOpenRecent(project)"
            @mouseenter="hoveredProject = project.path"
            @mouseleave="hoveredProject = null"
          >
            <span class="recent-icon">{{ project.pinned ? '📌' : '📂' }}</span>
            <div class="recent-info">
              <span class="recent-name">{{ project.name }}</span>
              <span class="recent-date">{{ formatDate(project.last_opened) }}</span>
            </div>
            <div class="recent-actions" v-if="hoveredProject === project.path">
              <button
                class="icon-btn"
                @click="(e) => handleTogglePin(e, project.path)"
                :title="project.pinned ? '取消固定' : '固定'"
              >
                {{ project.pinned ? '📍' : '📌' }}
              </button>
              <button
                class="icon-btn remove"
                @click="(e) => handleRemoveRecent(e, project.path)"
                title="移除"
              >
                ×
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.project-selector {
  padding: 12px;
  border-bottom: 1px solid var(--color-border);
}

.current-project {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px;
  background: var(--color-primary-bg);
  border-radius: var(--radius-md);
}

.project-info {
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: 0;
}

.project-icon {
  font-size: 16px;
}

.project-name {
  font-size: 13px;
  font-weight: 600;
  color: var(--color-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.close-btn {
  background: none;
  border: none;
  font-size: 16px;
  color: var(--color-text-secondary);
  cursor: pointer;
  padding: 2px 6px;
  border-radius: var(--radius-sm);
}

.close-btn:hover {
  background: var(--color-hover);
  color: var(--color-text);
}

.no-project {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.project-actions {
  display: flex;
  gap: 6px;
}

.action-btn {
  flex: 1;
  padding: 8px 12px;
  font-size: 12px;
  border: 1px solid var(--color-border);
  background: var(--color-surface);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all var(--transition-duration);
  white-space: nowrap;
}

.action-btn:hover:not(:disabled) {
  background: var(--color-hover);
  border-color: var(--color-primary);
}

.action-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.action-btn.primary {
  background: var(--color-primary);
  color: white;
  border-color: var(--color-primary);
}

.action-btn.primary:hover:not(:disabled) {
  background: var(--color-primary-dark);
}

.recent-dropdown {
  position: relative;
}

.recent-toggle {
  width: 100%;
  padding: 6px 10px;
  font-size: 11px;
  border: 1px solid var(--color-border);
  background: var(--color-surface);
  border-radius: var(--radius-md);
  cursor: pointer;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.recent-toggle:hover {
  background: var(--color-hover);
}

.arrow {
  font-size: 8px;
  color: var(--color-text-secondary);
}

.recent-list {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  margin-top: 4px;
  background: var(--color-surface);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-lg);
  max-height: 200px;
  overflow-y: auto;
  z-index: 100;
}

.recent-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 10px;
  cursor: pointer;
  transition: background var(--transition-duration);
}

.recent-item:hover {
  background: var(--color-hover);
}

.recent-item.pinned {
  background: var(--color-primary-bg);
}

.recent-icon {
  font-size: 14px;
}

.recent-info {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
}

.recent-name {
  font-size: 12px;
  color: var(--color-text);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.recent-date {
  font-size: 10px;
  color: var(--color-text-secondary);
}

.recent-actions {
  display: flex;
  gap: 4px;
}

.icon-btn {
  background: none;
  border: none;
  font-size: 12px;
  cursor: pointer;
  padding: 2px 4px;
  border-radius: var(--radius-sm);
}

.icon-btn:hover {
  background: var(--color-hover);
}

.icon-btn.remove {
  color: var(--color-error);
}
</style>
