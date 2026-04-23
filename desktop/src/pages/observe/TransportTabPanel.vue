<script setup lang="ts">
import { ref, inject, computed, onMounted, type Ref } from 'vue'
import { useObserve } from '../../composables/observe/useObserve'
import { useComm } from '../../composables/useComm'
import { useProject } from '../../composables/useProject'
import { useLog } from '../../composables/useLog'
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
  loadConfig,
  saveConfig,
  getFirstSessionStats,
  formatBytes,
} = useObserve()

const { config: commConfig, setPubAddress } = useComm()
const { isProjectLoaded } = useProject()
const { addLog } = useLog()
const { prompt } = useDialog()

const pubAddress = computed({
  get: () => commConfig.value.observe_pub_address,
  set: (val: string) => { commConfig.value.observe_pub_address = val }
})

const sessionStats = computed(() => getFirstSessionStats())

const actualFps = inject<Ref<number>>('actualFps', ref(0))

async function savePubAddress() {
  try {
    await setPubAddress(commConfig.value.observe_pub_address)
  } catch (e) {
    addLog('保存发布地址失败', 'error', 'comm')
    throw e
  }
}

const configPath = ref('')

async function handleLoad() {
  const path = await prompt('输入配置文件路径:', configPath.value || 'observe.json')
  if (path) {
    configPath.value = path
    await loadConfig(path)
  }
}

async function handleSave() {
  if (configPath.value) {
    await saveConfig(configPath.value)
  } else {
    handleSaveAs()
  }
}

async function handleSaveAs() {
  const path = await prompt('输入新配置文件路径:', 'observe.json')
  if (path) {
    configPath.value = path
    await saveConfig(path)
  }
}

onMounted(async () => {
  // 初始化时同步 Rust 后端的地址
  try {
    await setPubAddress(commConfig.value.observe_pub_address)
  } catch {
    // 静默处理
  }
})
</script>

<template>
  <ObservePanel>
    <CardBox class="flex flex-col flex-1">
      <!-- 标题栏 -->
      <div class="px-3 py-2 border-b border-gray-100 dark:border-slate-800">
        <div class="flex items-center justify-between">
          <h3 class="m-0 text-sm font-semibold text-gray-700 dark:text-slate-200">
            传输配置
          </h3>
          <BaseButton
            :icon="mdiContentSave"
            color="whiteDark"
            :disabled="!isProjectLoaded"
            @click="savePubAddress"
          />
        </div>
      </div>

      <!-- 配置内容 -->
      <CardBoxComponentBody class="flex flex-col gap-4">
        <!-- 发布地址 -->
        <div class="flex flex-col gap-1.5">
          <label class="text-xs font-medium text-gray-500 dark:text-slate-400">发布地址</label>
          <FormControl
            v-model="pubAddress"
            type="text"
            placeholder="tcp://127.0.0.1:5556"
            @change="savePubAddress"
          />
        </div>

        <!-- 状态信息 -->
        <div class="grid grid-cols-2 gap-2">
          <div class="flex justify-between items-center px-3 py-2 bg-gray-50 dark:bg-slate-800 rounded-lg">
            <span class="text-xs font-medium text-gray-500 dark:text-slate-400">实际帧率</span>
            <span class="text-xs font-medium text-gray-700 dark:text-slate-200">{{ actualFps }} fps</span>
          </div>
          <div class="flex justify-between items-center px-3 py-2 bg-gray-50 dark:bg-slate-800 rounded-lg">
            <span class="text-xs font-medium text-gray-500 dark:text-slate-400">观察状态</span>
            <span :class="['text-xs font-medium', isObserving ? 'text-green-500' : 'text-gray-400']">
              {{ isObserving ? '采集中' : '已停止' }}
            </span>
          </div>
        </div>

        <!-- 裁切区域信息 -->
        <div class="px-3 py-2 bg-gray-50 dark:bg-slate-800 rounded-lg text-xs">
          <span class="text-gray-600 dark:text-slate-300">{{ config.crop_regions.length }} 个区域</span>
          <span
            v-if="config.crop_regions.length > 0"
            class="text-gray-400 ml-1"
          >（在"裁切配置"中编辑）</span>
        </div>

        <!-- 配置文件操作 -->
        <div class="flex flex-col gap-1.5">
          <label class="text-xs font-medium text-gray-500 dark:text-slate-400">配置文件</label>
          <div class="flex gap-2">
            <BaseButton
              label="加载"
              color="whiteDark"
              class="flex-1"
              @click="handleLoad"
            />
            <BaseButton
              label="保存"
              color="whiteDark"
              class="flex-1"
              :disabled="!isProjectLoaded"
              @click="handleSave"
            />
            <BaseButton
              label="另存为"
              color="whiteDark"
              class="flex-1"
              :disabled="!isProjectLoaded"
              @click="handleSaveAs"
            />
          </div>
        </div>

        <!-- 传输统计 -->
        <div class="flex flex-col gap-1.5">
          <label class="text-xs font-medium text-gray-500 dark:text-slate-400">传输统计</label>
          <div class="grid grid-cols-2 gap-2">
            <div class="flex justify-between px-2 py-1.5 bg-gray-50 dark:bg-slate-800 rounded text-xs">
              <span class="text-gray-500">发送消息</span>
              <span class="font-medium">{{ sessionStats?.messages_sent ?? 0 }}</span>
            </div>
            <div class="flex justify-between px-2 py-1.5 bg-gray-50 dark:bg-slate-800 rounded text-xs">
              <span class="text-gray-500">发送数据</span>
              <span class="font-medium">{{ formatBytes(sessionStats?.bytes_sent ?? 0) }}</span>
            </div>
            <div class="flex justify-between px-2 py-1.5 bg-gray-50 dark:bg-slate-800 rounded text-xs">
              <span class="text-gray-500">发送帧数</span>
              <span class="font-medium">{{ sessionStats?.frames_captured ?? 0 }}</span>
            </div>
          </div>
        </div>
      </CardBoxComponentBody>
    </CardBox>
  </ObservePanel>
</template>
