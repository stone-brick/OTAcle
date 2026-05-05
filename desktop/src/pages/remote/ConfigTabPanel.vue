<script setup lang="ts">
import { ref, watch } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import BaseIcon from '../../components/ui/BaseIcon.vue'
import { mdiUpload, mdiDownload, mdiHistory, mdiCloudOffOutline } from '@mdi/js'
import { useRemote, type ConfigVersion, type ConfigDownloadResponse } from '../../composables/useRemote'

const {
  selectedGroup,
  getConfigVersions,
  getLatestConfig,
  getDownloadUrl,
  uploadConfig,
  isLoading,
  error,
} = useRemote()

const characterId = ref('default')
const versions = ref<ConfigVersion[]>([])
const latestConfig = ref<ConfigDownloadResponse | null>(null)
const localError = ref<string | null>(null)

async function loadVersions() {
  if (!selectedGroup.value) return
  try {
    versions.value = await getConfigVersions(selectedGroup.value.id, characterId.value)
    await loadLatest()
  } catch (e) {
    localError.value = String(e)
  }
}

async function loadLatest() {
  if (!selectedGroup.value) return
  try {
    latestConfig.value = await getLatestConfig(selectedGroup.value.id, characterId.value)
  } catch {
    latestConfig.value = null
  }
}

watch([() => selectedGroup.value?.id, characterId], async () => {
  if (selectedGroup.value) {
    await loadVersions()
  }
}, { immediate: true })

async function handleUpload() {
  if (!selectedGroup.value) return
  const file = await open({
    multiple: false,
    filters: [{ name: 'ZIP', extensions: ['zip'] }],
  })
  if (file) {
    try {
      await uploadConfig(selectedGroup.value.id, characterId.value, file as string)
      await loadVersions()
    } catch (e) {
      localError.value = String(e)
    }
  }
}

async function handleDownloadUrl(version?: number) {
  if (!selectedGroup.value) return
  try {
    const url = await getDownloadUrl(selectedGroup.value.id, characterId.value, version)
    window.open(url.downloadUrl, '_blank')
  } catch (e) {
    localError.value = String(e)
  }
}

function formatSize(bytes: number | null): string {
  if (!bytes) return '0 B'
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
}
</script>

<template>
  <div class="flex flex-col p-3 gap-3 h-full box-border">
    <div class="flex flex-col flex-1 bg-white dark:bg-slate-900/70 rounded-xl p-4">
      <!-- No group selected state -->
      <div
        v-if="!selectedGroup"
        class="flex flex-col items-center justify-center h-full text-center"
      >
        <BaseIcon :path="mdiCloudOffOutline" :size="48" class="text-gray-300 dark:text-gray-600 mb-3" />
        <div class="text-gray-500 dark:text-gray-400 text-sm">请先在「分组」中选择一个分组</div>
      </div>

    <!-- Config management -->
    <template v-else>
      <div class="mb-6">
        <div class="flex items-center justify-between mb-4">
          <h3 class="text-sm font-medium text-gray-500 dark:text-gray-400">
            分组：{{ selectedGroup.name }}
          </h3>
          <span class="text-xs text-gray-400">角色 ID</span>
        </div>
        <input
          v-model="characterId"
          type="text"
          placeholder="角色 ID"
          class="w-full px-3 py-2 text-sm border border-gray-200 dark:border-slate-700 rounded-lg bg-white dark:bg-slate-800 focus:outline-none focus:border-blue-400"
        />
      </div>

      <!-- Upload button -->
      <button
        class="w-full flex items-center justify-center gap-2 px-4 py-2.5 mb-6 text-sm bg-blue-500 hover:bg-blue-600 text-white rounded-lg transition-colors disabled:opacity-50"
        @click="handleUpload"
        :disabled="isLoading || !characterId.trim()"
      >
        <BaseIcon :path="mdiUpload" :size="18" />
        {{ isLoading ? '上传中...' : '上传配置 ZIP' }}
      </button>

      <!-- Latest config -->
      <div
        v-if="latestConfig"
        class="mb-6 p-4 bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg"
      >
        <div class="flex items-center justify-between mb-2">
          <span class="text-xs text-blue-600 dark:text-blue-400 font-medium">最新版本</span>
          <button
            class="text-xs text-blue-500 hover:text-blue-600 flex items-center gap-1"
            @click="handleDownloadUrl()"
          >
            <BaseIcon :path="mdiDownload" :size="14" />
            下载
          </button>
        </div>
        <div class="flex items-center justify-between">
          <div>
            <span class="text-sm font-semibold text-gray-700 dark:text-gray-200">v{{ latestConfig.version }}</span>
            <span class="text-sm text-gray-500 ml-2">{{ latestConfig.fileName }}</span>
          </div>
          <span class="text-xs text-gray-400">{{ formatSize(latestConfig.fileSize) }}</span>
        </div>
      </div>

      <!-- Version history -->
      <div v-if="versions.length > 0">
        <div class="text-xs text-gray-500 dark:text-gray-400 mb-3 flex items-center gap-1.5">
          <BaseIcon :path="mdiHistory" :size="14" />
          版本历史
        </div>
        <div class="space-y-2 max-h-80 overflow-y-auto">
          <div
            v-for="v in versions"
            :key="v.id"
            class="flex items-center justify-between p-3 bg-white dark:bg-slate-800 rounded-lg border border-gray-100 dark:border-slate-700"
          >
            <div class="flex items-center gap-3">
              <span class="text-sm font-medium text-gray-700 dark:text-gray-200">v{{ v.version }}</span>
              <span class="text-sm text-gray-400">{{ v.fileName }}</span>
            </div>
            <div class="flex items-center gap-3">
              <span class="text-xs text-gray-400">{{ formatSize(v.fileSize) }}</span>
              <button
                class="p-1.5 text-blue-500 hover:text-blue-600 hover:bg-blue-50 dark:hover:bg-blue-900/20 rounded transition-colors"
                @click="handleDownloadUrl(v.version)"
                title="下载此版本"
              >
                <BaseIcon :path="mdiDownload" :size="16" />
              </button>
            </div>
          </div>
        </div>
      </div>

      <div v-if="localError || error" class="text-sm text-red-500 mt-4 text-center">
        {{ localError || error }}
      </div>
    </template>
    </div>
  </div>
</template>