<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { mdiFolderOpen, mdiPlus, mdiPin, mdiMapMarker } from '@mdi/js'
import BaseIcon from './ui/BaseIcon.vue'
import { useProject, type RecentProject } from '../composables/useProject';
import { useDialog } from '../composables/useDialog';

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

const { prompt } = useDialog();

const showRecentList = ref(false);
const hoveredProject = ref<string | null>(null);
const isHoveringProject = ref(false);

onMounted(() => {
  refreshRecentProjects();
});

async function handleCreateProject() {
  const name = await prompt('输入项目名称:', 'my-otacle-project');
  if (name) {
    await createProjectDialog(name);
  }
}

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
  <div class="p-3 border-b border-gray-100 dark:border-slate-800">
    <!-- Current Project Loaded -->
    <div
      v-if="isProjectLoaded && currentProject"
      class="relative flex items-center justify-between px-3 py-2 pr-8 rounded-md bg-blue-50 dark:bg-blue-900/30"
      @mouseenter="isHoveringProject = true"
      @mouseleave="isHoveringProject = false"
    >
      <div class="flex items-center gap-2 min-w-0">
        <span
          class="text-sm font-semibold text-blue-600 dark:text-blue-400 truncate"
          :title="currentProject.path"
        >{{ currentProject.name }}</span>
      </div>
      <button
        class="absolute right-2 top-1/2 -translate-y-1/2 bg-none border-none text-base
               text-gray-400 dark:text-gray-500 cursor-pointer px-1 py-0.5 rounded
               opacity-0 hover:opacity-100 hover:bg-gray-100 dark:hover:bg-slate-700 transition-opacity"
        title="关闭项目"
        @click="closeProject"
      >
        ×
      </button>
    </div>

    <!-- No Project -->
    <div
      v-else
      class="flex flex-col gap-2"
    >
      <div class="flex flex-col gap-1.5">
        <button
          class="w-full flex items-center gap-2 px-2.5 py-1.5 text-xs border border-gray-200 dark:border-slate-700 bg-white dark:bg-slate-900
                 rounded-md cursor-pointer transition-all
                 hover:bg-gray-50 dark:hover:bg-slate-800 hover:border-blue-400
                 disabled:opacity-50 disabled:cursor-not-allowed"
          :disabled="isLoading"
          @click="openProjectDialog"
        >
          <BaseIcon
            :path="mdiFolderOpen"
            :size="16"
          />
          <span>打开项目</span>
        </button>
        <button
          class="w-full flex items-center gap-2 px-2.5 py-1.5 text-xs border border-gray-200 dark:border-slate-700 bg-white dark:bg-slate-900
                 rounded-md cursor-pointer transition-all
                 hover:bg-gray-50 dark:hover:bg-slate-800 hover:border-blue-400
                 disabled:opacity-50 disabled:cursor-not-allowed"
          :disabled="isLoading"
          @click="handleCreateProject"
        >
          <BaseIcon
            :path="mdiPlus"
            :size="16"
          />
          <span>新建项目</span>
        </button>
      </div>

      <!-- Recent Projects Dropdown -->
      <div
        v-if="recentProjects.length > 0"
        class="relative"
      >
        <button
          class="w-full flex items-center gap-2 px-2.5 py-1.5 text-xs border border-gray-200 dark:border-slate-700 bg-white dark:bg-slate-900
                 rounded-md cursor-pointer
                 hover:bg-gray-50 dark:hover:bg-slate-800"
          @click="showRecentList = !showRecentList"
        >
          <BaseIcon
            :path="mdiPin"
            :size="12"
          />
          <span class="flex-1 text-left">最近项目</span>
          <span class="text-[8px] text-gray-400 dark:text-gray-500">{{ showRecentList ? '▲' : '▼' }}</span>
        </button>

        <!-- Recent List -->
        <div
          v-if="showRecentList"
          class="absolute top-full left-0 right-0 mt-1 bg-white dark:bg-slate-900 border border-gray-100 dark:border-slate-800
                 rounded-md shadow-lg max-h-[200px] overflow-y-auto z-[100]"
        >
          <div
            v-for="project in [...pinnedProjects, ...unpinnedProjects]"
            :key="project.path"
            class="flex items-center gap-2 px-2.5 py-2 cursor-pointer transition-colors
                   hover:bg-gray-50 dark:hover:bg-slate-800"
            :class="project.pinned ? 'bg-blue-50 dark:bg-blue-900/30' : ''"
            @click="handleOpenRecent(project)"
            @mouseenter="hoveredProject = project.path"
            @mouseleave="hoveredProject = null"
          >
            <BaseIcon
              :path="project.pinned ? mdiPin : mdiFolderOpen"
              :size="16"
            />
            <div class="flex-1 min-w-0 flex flex-col">
              <span class="text-xs text-gray-700 dark:text-slate-200 truncate">{{ project.name }}</span>
              <span class="text-[10px] text-gray-400 dark:text-gray-500">{{ formatDate(project.last_opened) }}</span>
            </div>
            <!-- Hover Actions -->
            <div
              v-if="hoveredProject === project.path"
              class="flex gap-1"
            >
              <button
                class="bg-none border-none text-xs cursor-pointer px-1 py-0.5 rounded
                       hover:bg-gray-100 dark:hover:bg-slate-700"
                :title="project.pinned ? '取消固定' : '固定'"
                @click="(e) => handleTogglePin(e, project.path)"
              >
                <BaseIcon
                  :path="project.pinned ? mdiMapMarker : mdiPin"
                  :size="12"
                />
              </button>
              <button
                class="bg-none border-none text-xs cursor-pointer px-1 py-0.5 rounded
                       hover:bg-gray-100 dark:hover:bg-slate-700 text-red-500"
                title="移除"
                @click="(e) => handleRemoveRecent(e, project.path)"
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
