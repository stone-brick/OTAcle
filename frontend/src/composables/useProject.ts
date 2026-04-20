import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';
import { useLog } from './useLog';

export interface ProjectInfo {
  path: string;
  name: string;
  description?: string;
}

export interface RecentProject {
  path: string;
  name: string;
  last_opened: string;
  pinned: boolean;
}

export interface TemplateConfig {
  name: string;
  description: string;
}

const currentProject = ref<ProjectInfo | null>(null);
const recentProjects = ref<RecentProject[]>([]);
const templates = ref<TemplateConfig[]>([]);
const isLoading = ref(false);

export function useProject() {
  const { addLog } = useLog();

  const isProjectLoaded = computed(() => currentProject.value !== null);

  async function refreshRecentProjects(): Promise<void> {
    try {
      recentProjects.value = await invoke<RecentProject[]>('project_get_recent');
    } catch (e) {
      addLog(`获取最近项目失败: ${e}`, 'error');
    }
  }

  async function refreshTemplates(): Promise<void> {
    try {
      templates.value = await invoke<TemplateConfig[]>('project_list_templates');
    } catch (e) {
      addLog(`获取模板列表失败: ${e}`, 'error');
    }
  }

  async function openProjectDialog(): Promise<void> {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: '选择项目文件夹',
      });

      if (selected) {
        await openProject(selected as string);
      }
    } catch (e) {
      addLog(`打开项目失败: ${e}`, 'error');
    }
  }

  async function openProject(path: string): Promise<void> {
    isLoading.value = true;
    try {
      const project = await invoke<ProjectInfo>('project_open', { path });
      currentProject.value = project;
      await refreshRecentProjects();
      addLog(`已打开项目: ${project.name}`, 'success');
    } catch (e) {
      addLog(`打开项目失败: ${e}`, 'error');
      throw e;
    } finally {
      isLoading.value = false;
    }
  }

  async function createProjectDialog(): Promise<void> {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: '选择新建项目的位置',
      });

      if (selected) {
        const name = prompt('输入项目名称:', 'my-otacle-project');
        if (name) {
          await createProject(name, selected as string);
        }
      }
    } catch (e) {
      addLog(`创建项目失败: ${e}`, 'error');
    }
  }

  async function createProject(name: string, path: string, template?: string): Promise<void> {
    isLoading.value = true;
    try {
      const project = await invoke<ProjectInfo>('project_create', {
        path,
        name,
        template,
      });
      currentProject.value = project;
      await refreshRecentProjects();
      addLog(`已创建项目: ${project.name}`, 'success');
    } catch (e) {
      addLog(`创建项目失败: ${e}`, 'error');
      throw e;
    } finally {
      isLoading.value = false;
    }
  }

  async function closeProject(): Promise<void> {
    try {
      await invoke('project_close');
      currentProject.value = null;
      addLog('已关闭项目', 'info');
    } catch (e) {
      addLog(`关闭项目失败: ${e}`, 'error');
    }
  }

  async function removeRecentProject(path: string): Promise<void> {
    try {
      await invoke('project_remove_recent', { path });
      await refreshRecentProjects();
    } catch (e) {
      addLog(`移除最近项目失败: ${e}`, 'error');
    }
  }

  async function togglePinProject(path: string): Promise<void> {
    try {
      await invoke('project_toggle_pin', { path });
      await refreshRecentProjects();
    } catch (e) {
      addLog(`切换固定状态失败: ${e}`, 'error');
    }
  }

  async function getProjectConfigDir(): Promise<string | null> {
    try {
      return await invoke<string | null>('project_get_config_dir');
    } catch (e) {
      return null;
    }
  }

  async function getProjectActionsConfigPath(): Promise<string | null> {
    try {
      return await invoke<string | null>('project_get_actions_config_path');
    } catch (e) {
      return null;
    }
  }

  async function getProjectObserveConfigPath(): Promise<string | null> {
    try {
      return await invoke<string | null>('project_get_observe_config_path');
    } catch (e) {
      return null;
    }
  }

  async function getProjectCommConfigPath(): Promise<string | null> {
    try {
      return await invoke<string | null>('project_get_comm_config_path');
    } catch (e) {
      return null;
    }
  }

  async function checkCurrentProject(): Promise<void> {
    try {
      const project = await invoke<ProjectInfo | null>('project_get_current');
      currentProject.value = project;
    } catch (e) {
      currentProject.value = null;
    }
  }

  return {
    // State
    currentProject,
    recentProjects,
    templates,
    isLoading,

    // Computed
    isProjectLoaded,

    // Actions
    refreshRecentProjects,
    refreshTemplates,
    openProjectDialog,
    openProject,
    createProjectDialog,
    createProject,
    closeProject,
    removeRecentProject,
    togglePinProject,
    getProjectConfigDir,
    getProjectActionsConfigPath,
    getProjectObserveConfigPath,
    getProjectCommConfigPath,
    checkCurrentProject,
  };
}
