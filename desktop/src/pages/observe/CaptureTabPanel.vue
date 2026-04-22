<script setup lang="ts">
import { computed, inject, onMounted } from 'vue'
import { useObserve } from '../../composables/observe/useObserve'
import { useWindows } from '../../composables/useWindows'
import { useDialog } from '../../composables/useDialog'
import BaseButton from '@/components/ui/BaseButton.vue'
import FormControl from '@/components/ui/FormControl.vue'
import CardBox from '@/components/ui/CardBox.vue'
import CardBoxComponentBody from '@/components/ui/CardBoxComponentBody.vue'
import ObservePanel from './ObservePanel.vue'

const {
  isObserving,
  config,
  captureFullFrame,
  startObserve,
  stopObserve,
  getFirstSessionStats,
  formatUptime,
} = useObserve()

const sessionStats = computed(() => getFirstSessionStats())

const { windows, selectedWindow, refreshWindows, selectWindow } = useWindows()

const resetCanvas = inject<() => void>('resetCanvas', () => {})

const { alert } = useDialog()

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
        <h3 class="m-0 text-sm font-semibold text-gray-700 dark:text-slate-200">
          截图配置
        </h3>
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
              v-model.number="config.capture.target_width"
              type="number"
              placeholder="宽度"
            />
            <span class="text-gray-400">×</span>
            <FormControl
              v-model.number="config.capture.target_height"
              type="number"
              placeholder="高度"
            />
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
