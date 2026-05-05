<script setup lang="ts">
import { ref } from 'vue'
import BaseIcon from '../../components/ui/BaseIcon.vue'
import { mdiPlus, mdiAccountGroup, mdiExitToApp } from '@mdi/js'
import { useRemote } from '../../composables/useRemote'

const {
  currentUser,
  groups,
  selectedGroup,
  isLoading,
  error,
  createGroup,
  leaveGroup,
  selectGroup,
  logout,
} = useRemote()

const showCreateForm = ref(false)
const newGroupName = ref('')
const newGroupDesc = ref('')
const localError = ref<string | null>(null)

async function handleCreateGroup() {
  if (!newGroupName.value.trim()) return
  localError.value = null
  try {
    const group = await createGroup(newGroupName.value.trim(), newGroupDesc.value.trim() || undefined)
    newGroupName.value = ''
    newGroupDesc.value = ''
    showCreateForm.value = false
    selectGroup(group)
  } catch (e) {
    localError.value = String(e)
  }
}

async function handleLeaveGroup(groupId: number) {
  if (confirm('确定要离开该分组吗？')) {
    await leaveGroup(groupId)
  }
}

async function handleSelectGroup(group: any) {
  selectGroup(group)
}

async function handleLogout() {
  if (confirm('确定要退出登录吗？')) {
    await logout()
    emit('logout')
  }
}

const emit = defineEmits<{
  logout: []
}>()
</script>

<template>
  <div class="flex flex-col p-3 gap-3 h-full box-border">
    <div class="flex flex-col flex-1 bg-white dark:bg-slate-900/70 rounded-xl p-4 overflow-y-auto">
      <!-- Header with user info -->
      <div class="flex items-center justify-between mb-6">
        <div class="flex items-center gap-3">
          <div class="w-10 h-10 rounded-full bg-blue-100 dark:bg-blue-900/50 flex items-center justify-center">
            <span class="text-blue-600 dark:text-blue-400 font-semibold text-sm">
              {{ currentUser?.username?.charAt(0).toUpperCase() }}
            </span>
          </div>
          <div>
            <div class="text-sm font-medium text-gray-700 dark:text-gray-200">{{ currentUser?.username }}</div>
            <div class="text-xs text-gray-400">{{ groups.length }} 个分组</div>
          </div>
        </div>
        <button
          class="flex items-center gap-1.5 px-3 py-1.5 text-sm text-gray-500 hover:text-red-500 hover:bg-red-50 dark:hover:bg-red-900/20 rounded-lg transition-colors"
          title="退出登录"
          @click="handleLogout"
        >
          <BaseIcon :path="mdiExitToApp" :size="16" />
          退出
        </button>
      </div>

      <!-- Groups list -->
      <div class="mb-6 flex-1 overflow-y-auto">
        <h3 class="text-sm font-medium text-gray-500 dark:text-gray-400 mb-3">我的分组</h3>
        <div class="space-y-2">
          <div
            v-for="group in groups"
            :key="group.id"
            class="flex items-center gap-3 px-4 py-3 rounded-lg cursor-pointer transition-colors text-sm border border-gray-100 dark:border-slate-700"
            :class="selectedGroup?.id === group.id
              ? 'bg-blue-50 dark:bg-blue-900/30 border-blue-200 dark:border-blue-800'
              : 'bg-white dark:bg-slate-800 hover:bg-gray-50 dark:hover:bg-slate-700'"
            @click="handleSelectGroup(group)"
          >
            <BaseIcon
              :path="mdiAccountGroup"
              :size="20"
              :class="selectedGroup?.id === group.id ? 'text-blue-500' : 'text-gray-400'"
            />
            <div class="flex-1 min-w-0">
              <div class="font-medium text-gray-700 dark:text-gray-200 truncate">{{ group.name }}</div>
              <div v-if="group.description" class="text-xs text-gray-400 truncate">{{ group.description }}</div>
            </div>
            <div class="flex items-center gap-2">
              <span class="text-xs text-gray-400">v{{ group.inviteCode }}</span>
              <button
                class="p-1.5 text-gray-400 hover:text-red-500 hover:bg-red-50 dark:hover:bg-red-900/20 rounded transition-colors"
                @click.stop="handleLeaveGroup(group.id)"
                title="离开分组"
              >
                <BaseIcon :path="mdiExitToApp" :size="16" />
              </button>
            </div>
          </div>

          <div
            v-if="groups.length === 0"
            class="text-center py-8 text-gray-400 text-sm"
          >
            <BaseIcon :path="mdiAccountGroup" :size="32" class="mx-auto mb-2 opacity-50" />
            <div>暂无分组</div>
            <div class="text-xs mt-1">创建一个分组开始使用</div>
          </div>
        </div>
      </div>

      <!-- Create group -->
      <div v-if="!showCreateForm" class="border-t border-gray-100 dark:border-slate-700 pt-4">
        <button
          class="w-full flex items-center justify-center gap-2 px-4 py-2.5 text-sm text-blue-500 hover:bg-blue-50 dark:hover:bg-blue-900/20 border border-dashed border-blue-300 dark:border-blue-700 rounded-lg transition-colors"
          @click="showCreateForm = true"
        >
          <BaseIcon :path="mdiPlus" :size="18" />
          创建分组
        </button>
      </div>

      <div v-else class="border-t border-gray-100 dark:border-slate-700 pt-4 space-y-3">
        <input
          v-model="newGroupName"
          type="text"
          placeholder="分组名称"
          class="w-full px-3 py-2 text-sm border border-gray-200 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800 focus:outline-none focus:border-blue-400"
          @keyup.enter="handleCreateGroup"
        />
        <input
          v-model="newGroupDesc"
          type="text"
          placeholder="分组描述（可选）"
          class="w-full px-3 py-2 text-sm border border-gray-200 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800 focus:outline-none focus:border-blue-400"
          @keyup.enter="handleCreateGroup"
        />
        <div class="flex gap-2">
          <button
            class="flex-1 py-2 text-sm bg-blue-500 hover:bg-blue-600 text-white rounded-lg transition-colors disabled:opacity-50"
            @click="handleCreateGroup"
            :disabled="isLoading || !newGroupName.trim()"
          >
            {{ isLoading ? '创建中...' : '创建' }}
          </button>
          <button
            class="px-4 py-2 text-sm text-gray-500 hover:bg-gray-100 dark:hover:bg-slate-800 rounded-lg transition-colors"
            @click="showCreateForm = false; newGroupName = ''; newGroupDesc = ''"
          >
            取消
          </button>
        </div>
      </div>

      <div v-if="localError || error" class="text-sm text-red-500 mt-3 text-center">
        {{ localError || error }}
      </div>
    </div>
  </div>
</template>