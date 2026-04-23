<script setup lang="ts">
import { computed, inject, onMounted, ref, watch } from 'vue'
import { useObserve } from '../../composables/observe/useObserve'
import { useWindows } from '../../composables/useWindows'
import { useProject } from '../../composables/useProject'
import { useDialog } from '../../composables/useDialog'
import { mdiContentSave } from '@mdi/js'
import BaseButton from '@/components/ui/BaseButton.vue'
import FormControl from '@/components/ui/FormControl.vue'
import CardBox from '@/components/ui/CardBox.vue'
import CardBoxComponentBody from '@/components/ui/CardBoxComponentBody.vue'
import ObservePanel from './ObservePanel.vue'

const {
  isObserving,
  config,
  fullPreviewFrame,
  captureFullFrame,
  startObserve,
  stopObserve,
  getFirstSessionStats,
  formatUptime,
  saveConfig,
} = useObserve()

const sessionStats = computed(() => getFirstSessionStats())

// 锁定比例状态
const lockAspectRatio = ref(false)
const aspectRatio = ref(0)

// 监听原始帧更新，获取原分辨率和比例
watch(fullPreviewFrame, (frame) => {
  if (frame && frame.original_width > 0 && frame.original_height > 0) {
    aspectRatio.value = frame.original_width / frame.original_height
  }
})

// 宽度变化时，按比例调整高度
function onWidthChange(newWidth: number) {
  config.value.capture.target_width = newWidth
  if (lockAspectRatio.value && aspectRatio.value > 0) {
    config.value.capture.target_height = Math.round(newWidth / aspectRatio.value)
  }
}

// 高度变化时，按比例调整宽度
function onHeightChange(newHeight: number) {
  config.value.capture.target_height = newHeight
  if (lockAspectRatio.value && aspectRatio.value > 0) {
    config.value.capture.target_width = Math.round(newHeight * aspectRatio.value)
  }
}

const { windows, selectedWindow, refreshWindows, selectWindow } = useWindows()

const { isProjectLoaded, getProjectObserveConfigPath } = useProject()

const resetCanvas = inject<() => void>('resetCanvas', () => {})

const { alert } = useDialog()

async function handleSave() {
  const path = await getProjectObserveConfigPath()
  if (path) {
    await saveConfig(path)
  }
}

async function handleStart() {
  if (!selectedWindow.value) {
    await alert('请先选择一个窗口')
    return
  }
  try {
    resetCanvas()
    await startObserve(selectedWindow.value.hwnd.toString())
  } catch (e) {
    await alert(`启动观察失败: ${e}`)
  }
}

async function handleStop() {
  try {
    await stopObserve()
  } catch (e) {
    await alert(`停止观察失败: ${e}`)
  }
}

async function handleWindowChange() {
  if (selectedWindow.value) {
    selectWindow(selectedWindow.value.hwnd)
    try {
      await captureFullFrame(selectedWindow.value.hwnd.toString())
    } catch {
      // 预览失败不阻止窗口选择
    }
  }
}

onMounted(async () => {
  await refreshWindows()
})
</script>

<template>
  <ObservePanel>
    <CardBox class="flex flex-col flex-1">
      <!-- 标题栏 -->
      <div class="px-3 py-2 border-b border-gray-100 dark:border-slate-800">
        <div class="flex items-center justify-between">
          <h3 class="m-0 text-sm font-semibold text-gray-700 dark:text-slate-200">
            截图配置
          </h3>
          <div class="flex items-center gap-1">
            <BaseButton
              :icon="mdiContentSave"
              color="whiteDark"
              :disabled="!isProjectLoaded"
              @click="handleSave"
            />
          </div>
        </div>
      </div>

      <!-- 配置内容 -->
      <CardBoxComponentBody class="flex flex-col gap-4">
        <!-- 窗口选择 -->
        <div class="flex flex-col gap-1.5">
          <div class="flex justify-between items-center">
            <label class="text-xs font-medium text-gray-500 dark:text-slate-400">目标窗口</label>
            <BaseButton
              label="↻"
              color="whiteDark"
              :small="true"
              title="刷新窗口列表"
              @click="refreshWindows"
            />
          </div>
          <FormControl
            v-model="selectedWindow"
            :options="windows.map(w => ({ value: w, label: w.title || 'Untitled' }))"
            @change="handleWindowChange"
          />
        </div>

        <!-- 采集帧率 -->
        <div class="flex flex-col gap-1.5">
          <label class="text-xs font-medium text-gray-500 dark:text-slate-400">采集帧率</label>
          <FormControl
            v-model.number="config.capture.frame_rate"
            type="number"
            placeholder="1-60"
          />
        </div>

        <!-- 目标分辨率 -->
        <div class="flex flex-col gap-1.5">
          <label class="text-xs font-medium text-gray-500 dark:text-slate-400">目标分辨率</label>
          <div class="flex items-center gap-2">
            <FormControl
              :model-value="config.capture.target_width"
              type="number"
              placeholder="宽度"
              @update:model-value="onWidthChange"
            />
            <span class="text-gray-400">×</span>
            <FormControl
              :model-value="config.capture.target_height"
              type="number"
              placeholder="高度"
              @update:model-value="onHeightChange"
            />
          </div>
        </div>

        <!-- 原分辨率和锁定比例 -->
        <div class="flex items-center gap-4">
          <!-- 原分辨率显示 -->
          <div class="flex flex-col gap-1">
            <label class="text-xs font-medium text-gray-500 dark:text-slate-400">原分辨率</label>
            <div class="px-2 py-1.5 bg-gray-50 dark:bg-slate-800 rounded text-xs text-gray-600 dark:text-slate-300 min-w-[100px]">
              {{ fullPreviewFrame ? `${fullPreviewFrame.original_width} × ${fullPreviewFrame.original_height}` : '-- × --' }}
            </div>
          </div>
          <!-- 锁定比例 -->
          <div class="flex items-center gap-1.5 mt-5">
            <input
              type="checkbox"
              v-model="lockAspectRatio"
              id="lock-aspect-ratio"
              class="w-3.5 h-3.5 rounded border-gray-300 dark:border-slate-600"
            />
            <label for="lock-aspect-ratio" class="text-xs font-medium text-gray-500 dark:text-slate-400 cursor-pointer">
              锁定比例
            </label>
          </div>
        </div>

        <!-- 采集统计 -->
        <div class="flex flex-col gap-1.5">
          <label class="text-xs font-medium text-gray-500 dark:text-slate-400">采集统计</label>
          <div class="grid grid-cols-2 gap-2">
            <div class="flex justify-between px-2 py-1.5 bg-gray-50 dark:bg-slate-800 rounded text-xs">
              <span class="text-gray-500">状态</span>
              <span :class="['text-xs font-medium', isObserving ? 'text-green-500' : 'text-gray-400']">
                {{ isObserving ? '采集中' : '已停止' }}
              </span>
            </div>
            <div class="flex justify-between px-2 py-1.5 bg-gray-50 dark:bg-slate-800 rounded text-xs">
              <span class="text-gray-500">运行时长</span>
              <span class="font-medium">{{ formatUptime(sessionStats?.uptime_seconds ?? 0) }}</span>
            </div>
          </div>
        </div>

        <!-- 操作按钮 -->
        <div class="flex gap-2 mt-auto pt-4 border-t border-gray-100 dark:border-slate-800">
          <BaseButton
            label="开始观察"
            color="info"
            class="flex-1"
            :disabled="isObserving"
            @click="handleStart"
          />
          <BaseButton
            label="停止观察"
            color="whiteDark"
            class="flex-1"
            :disabled="!isObserving"
            @click="handleStop"
          />
        </div>
      </CardBoxComponentBody>
    </CardBox>
  </ObservePanel>
</template>
