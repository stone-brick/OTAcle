import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useLog } from './useLog'

// Types
export interface Account {
  id: number
  username: string
  email: string | null
  createdAt: string
  updatedAt: string
}

export interface Group {
  id: number
  name: string
  description: string | null
  inviteCode: string
  createdBy: number
  createdAt: string
  updatedAt: string
}

export interface GroupMember {
  id: number
  accountId: number
  groupId: number
  role: string
  joinedAt: string
}

export interface ConfigVersion {
  id: number
  groupId: number
  uploaderId: number
  characterId: string
  version: number
  filePath: string | null
  fileName: string
  fileSize: number | null
  checksum: string | null
  createdAt: string
}

export interface ConfigDownloadResponse {
  configId: number
  characterId: string
  version: number
  fileName: string
  fileSize: number
  checksum: string
  downloadUrl: string
  createdAt: string
}

interface AuthResponse {
  token: string
  userId: number
  username: string
}

// State
const serverUrl = ref('http://localhost:8080')
const isLoggedIn = ref(false)
const currentUser = ref<Account | null>(null)
const token = ref<string | null>(null)
const groups = ref<Group[]>([])
const selectedGroup = ref<Group | null>(null)
const isLoading = ref(false)
const error = ref<string | null>(null)

// Composable
export function useRemote() {
  const { addLog } = useLog()

  // Load saved server URL from store
  async function loadServerUrl(): Promise<string> {
    try {
      const url = await invoke<string>('remote_load_server_url')
      if (url) {
        serverUrl.value = url
      }
      return serverUrl.value
    } catch (e) {
      return serverUrl.value
    }
  }

  // Save server URL to store
  async function saveServerUrl(url: string): Promise<void> {
    try {
      await invoke('remote_save_server_url', { url })
      serverUrl.value = url
      addLog(`服务器地址已保存: ${url}`, 'success', 'remote')
    } catch (e) {
      addLog(`保存服务器地址失败: ${e}`, 'error', 'remote')
      throw e
    }
  }

  // Initialize remote connection
  async function init(baseUrl?: string): Promise<void> {
    if (baseUrl) {
      serverUrl.value = baseUrl
    } else {
      await loadServerUrl()
    }
    try {
      await invoke('remote_init', { baseUrl: serverUrl.value })
      addLog(`Remote 已连接到: ${serverUrl.value}`, 'success', 'remote')
    } catch (e) {
      addLog(`Remote 初始化失败: ${e}`, 'error', 'remote')
      throw e
    }
  }

  // Check if initialized
  async function checkInitialized(): Promise<boolean> {
    return await invoke<boolean>('remote_is_initialized')
  }

  // Auth: Register
  async function register(username: string, password: string, email?: string): Promise<void> {
    isLoading.value = true
    error.value = null
    try {
      const resp = await invoke<AuthResponse>('remote_auth_register', {
        baseUrl: serverUrl.value,
        username,
        password,
        email: email || null,
      })
      token.value = resp.token
      isLoggedIn.value = true
      await fetchMe()
      addLog(`注册成功: ${username}`, 'success', 'remote')
    } catch (e) {
      error.value = String(e)
      addLog(`注册失败: ${e}`, 'error', 'remote')
      throw e
    } finally {
      isLoading.value = false
    }
  }

  // Auth: Login
  async function login(username: string, password: string): Promise<void> {
    isLoading.value = true
    error.value = null
    try {
      const resp = await invoke<AuthResponse>('remote_auth_login', {
        baseUrl: serverUrl.value,
        username,
        password,
      })
      token.value = resp.token
      isLoggedIn.value = true
      await fetchMe()
      await fetchGroups()
      addLog(`登录成功: ${username}`, 'success', 'remote')
    } catch (e) {
      error.value = String(e)
      addLog(`登录失败: ${e}`, 'error', 'remote')
      throw e
    } finally {
      isLoading.value = false
    }
  }

  // Auth: Get current user
  async function fetchMe(): Promise<void> {
    try {
      currentUser.value = await invoke<Account>('remote_auth_get_me')
    } catch (e) {
      currentUser.value = null
      isLoggedIn.value = false
      addLog(`获取用户信息失败: ${e}`, 'error', 'remote')
    }
  }

  // Auth: Logout
  async function logout(): Promise<void> {
    await invoke('remote_auth_logout')
    token.value = null
    currentUser.value = null
    isLoggedIn.value = false
    groups.value = []
    selectedGroup.value = null
    addLog('已退出登录', 'info', 'remote')
  }

  // Groups: List
  async function fetchGroups(): Promise<void> {
    try {
      groups.value = await invoke<Group[]>('remote_group_list')
    } catch (e) {
      error.value = String(e)
      addLog(`获取分组列表失败: ${e}`, 'error', 'remote')
    }
  }

  // Groups: Create
  async function createGroup(name: string, description?: string): Promise<Group> {
    isLoading.value = true
    error.value = null
    try {
      const group = await invoke<Group>('remote_group_create', {
        name,
        description: description || null,
      })
      groups.value.push(group)
      addLog(`创建分组成功: ${name}`, 'success', 'remote')
      return group
    } catch (e) {
      error.value = String(e)
      addLog(`创建分组失败: ${e}`, 'error', 'remote')
      throw e
    } finally {
      isLoading.value = false
    }
  }

  // Groups: Detail
  async function fetchGroupDetail(groupId: number): Promise<Group> {
    return await invoke<Group>('remote_group_detail', { groupId })
  }

  // Groups: Update
  async function updateGroup(groupId: number, name: string, description?: string): Promise<Group> {
    return await invoke<Group>('remote_group_update', {
      groupId,
      name,
      description: description || null,
    })
  }

  // Groups: Delete
  async function deleteGroup(groupId: number): Promise<void> {
    isLoading.value = true
    error.value = null
    try {
      await invoke('remote_group_delete', { groupId })
      groups.value = groups.value.filter(g => g.id !== groupId)
      if (selectedGroup.value?.id === groupId) {
        selectedGroup.value = null
      }
      addLog(`删除分组成功`, 'success', 'remote')
    } catch (e) {
      error.value = String(e)
      addLog(`删除分组失败: ${e}`, 'error', 'remote')
      throw e
    } finally {
      isLoading.value = false
    }
  }

  // Groups: Members
  async function fetchGroupMembers(groupId: number): Promise<GroupMember[]> {
    return await invoke<GroupMember[]>('remote_group_members', { groupId })
  }

  // Groups: Join
  async function joinGroup(inviteCode: string): Promise<void> {
    isLoading.value = true
    error.value = null
    try {
      // Find group by invite code - need to get group id first
      // For now just call join on all groups or find the right one
      const group = groups.value.find(g => g.inviteCode === inviteCode)
      if (group) {
        await invoke('remote_group_join', { groupId: group.id, inviteCode })
      } else {
        // Try to join by trying each group or use a default
        await invoke('remote_group_join', { groupId: 0, inviteCode })
      }
      addLog(`加入分组成功`, 'success', 'remote')
    } catch (e) {
      error.value = String(e)
      addLog(`加入分组失败: ${e}`, 'error', 'remote')
      throw e
    } finally {
      isLoading.value = false
    }
  }

  // Groups: Leave
  async function leaveGroup(groupId: number): Promise<void> {
    isLoading.value = true
    error.value = null
    try {
      await invoke('remote_group_leave', { groupId })
      groups.value = groups.value.filter(g => g.id !== groupId)
      if (selectedGroup.value?.id === groupId) {
        selectedGroup.value = null
      }
      addLog(`离开分组成功`, 'success', 'remote')
    } catch (e) {
      error.value = String(e)
      addLog(`离开分组失败: ${e}`, 'error', 'remote')
      throw e
    } finally {
      isLoading.value = false
    }
  }

  // Config: Upload
  async function uploadConfig(groupId: number, characterId: string, filePath: string): Promise<ConfigVersion> {
    isLoading.value = true
    error.value = null
    try {
      const version = await invoke<ConfigVersion>('remote_config_upload', {
        groupId,
        characterId,
        filePath,
      })
      addLog(`配置上传成功: v${version.version}`, 'success', 'remote')
      return version
    } catch (e) {
      error.value = String(e)
      addLog(`配置上传失败: ${e}`, 'error', 'remote')
      throw e
    } finally {
      isLoading.value = false
    }
  }

  // Config: Download URL
  async function getDownloadUrl(groupId: number, characterId: string, version?: number): Promise<ConfigDownloadResponse> {
    return await invoke<ConfigDownloadResponse>('remote_config_download_url', {
      groupId,
      characterId,
      version: version || null,
    })
  }

  // Config: Latest
  async function getLatestConfig(groupId: number, characterId: string): Promise<ConfigDownloadResponse> {
    return await invoke<ConfigDownloadResponse>('remote_config_latest', {
      groupId,
      characterId,
    })
  }

  // Config: Versions
  async function getConfigVersions(groupId: number, characterId: string): Promise<ConfigVersion[]> {
    return await invoke<ConfigVersion[]>('remote_config_versions', {
      groupId,
      characterId,
    })
  }

  // Select group
  function selectGroup(group: Group | null) {
    selectedGroup.value = group
  }

  return {
    // State
    serverUrl,
    isLoggedIn,
    currentUser,
    token,
    groups,
    selectedGroup,
    isLoading,
    error,

    // Auth
    init,
    checkInitialized,
    loadServerUrl,
    saveServerUrl,
    register,
    login,
    fetchMe,
    logout,

    // Groups
    fetchGroups,
    createGroup,
    fetchGroupDetail,
    updateGroup,
    deleteGroup,
    fetchGroupMembers,
    joinGroup,
    leaveGroup,
    selectGroup,

    // Config
    uploadConfig,
    getDownloadUrl,
    getLatestConfig,
    getConfigVersions,
  }
}