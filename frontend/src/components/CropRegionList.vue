<script setup lang="ts">
import { ref, watch } from 'vue'
import type { CropRegion } from '../types'
import BaseButton from '@/components/ui/BaseButton.vue'
import CardBox from '@/components/ui/CardBox.vue'
import CardBoxComponentBody from '@/components/ui/CardBoxComponentBody.vue'
import FormControl from '@/components/ui/FormControl.vue'
import { mdiClose, mdiPlus, mdiContentSave } from '@mdi/js'

const props = defineProps<{
  regions: CropRegion[];
  selectedIndex: number | null;
  hasChanges: boolean;
}>();

const emit = defineEmits<{
  select: [index: number];
  delete: [index: number];
  update: [index: number, region: CropRegion];
  add: [region: CropRegion];
  save: [];
}>();

// 表单数据（新建/编辑共用）
const formData = ref<CropRegion>({ x: 0, y: 0, w: 100, h: 100 })

// 监听选中变化，同步表单数据
watch(() => props.selectedIndex, (idx) => {
  if (idx !== null && props.regions[idx]) {
    formData.value = { ...props.regions[idx] }
  } else {
    formData.value = { x: 0, y: 0, w: 100, h: 100 }
  }
}, { immediate: true })

// 监听 regions 外部变化
watch(() => props.regions, (newRegions) => {
  if (props.selectedIndex !== null && newRegions[props.selectedIndex]) {
    formData.value = { ...newRegions[props.selectedIndex] }
  }
}, { deep: true })

// 应用更新（编辑模式）
function applyUpdate() {
  if (props.selectedIndex !== null) {
    emit('update', props.selectedIndex, { ...formData.value })
  }
}

// 添加新区域
function handleAdd() {
  emit('add', { ...formData.value })
  formData.value = { x: 0, y: 0, w: 100, h: 100 }
}
</script>

<template>
  <CardBox class="flex flex-col flex-1 border-l border-gray-100 dark:border-slate-800">
    <!-- 标题 + 表单 -->
    <div class="px-3 py-2 border-b border-gray-100 dark:border-slate-800">
      <div class="flex items-center justify-between mb-2">
        <div class="flex items-center gap-2">
          <h3 class="m-0 text-sm font-semibold text-gray-700 dark:text-slate-200">
            裁切区域
          </h3>
          <span
            v-if="selectedIndex !== null"
            class="text-xs text-blue-600 dark:text-blue-400"
          >
            (编辑 #{{ selectedIndex }})
          </span>
        </div>
        <div class="flex items-center gap-1">
          <BaseButton
            :icon="mdiContentSave"
            color="whiteDark"
            :disabled="!hasChanges"
            @click="emit('save')"
          />
          <BaseButton
            :icon="mdiPlus"
            @click="handleAdd"
          />
        </div>
      </div>

      <div class="grid grid-cols-2 gap-2">
        <FormControl
          v-model.number="formData.x"
          type="number"
          placeholder="X"
          class="!w-full !text-xs"
          @change="selectedIndex !== null && applyUpdate()"
        />
        <FormControl
          v-model.number="formData.y"
          type="number"
          placeholder="Y"
          class="!w-full !text-xs"
          @change="selectedIndex !== null && applyUpdate()"
        />
        <FormControl
          v-model.number="formData.w"
          type="number"
          placeholder="W"
          class="!w-full !text-xs"
          @change="selectedIndex !== null && applyUpdate()"
        />
        <FormControl
          v-model.number="formData.h"
          type="number"
          placeholder="H"
          class="!w-full !text-xs"
          @change="selectedIndex !== null && applyUpdate()"
        />
      </div>
    </div>

    <!-- 区域列表 -->
    <CardBoxComponentBody
      v-if="regions.length === 0"
      class="flex flex-col items-center justify-center flex-1"
    >
      <p class="text-gray-500 dark:text-slate-400 text-sm">
        暂无裁切区域
      </p>
    </CardBoxComponentBody>

    <CardBoxComponentBody
      v-else
      class="flex-1 overflow-hidden"
    >
      <div class="flex-1 overflow-y-auto p-1.5 flex flex-col gap-1">
        <div
          v-for="(region, idx) in regions"
          :key="idx"
          class="group flex items-center justify-between p-3 rounded-lg cursor-pointer transition-colors border border-transparent"
          :class="selectedIndex === idx
            ? 'bg-blue-50 dark:bg-blue-900/30 border-blue-500 dark:border-blue-400'
            : 'bg-gray-50 dark:bg-slate-800/50 hover:bg-gray-100 dark:hover:bg-slate-800'"
          @click="emit('select', idx)"
        >
          <div class="flex-1 min-w-0">
            <div class="flex items-center gap-2 mb-1">
              <span class="font-mono font-semibold text-xs text-blue-600 dark:text-blue-400">
                #{{ idx }}
              </span>
            </div>
            <div class="text-xs text-gray-500 dark:text-slate-400 font-mono">
              x: {{ region.x }}, y: {{ region.y }}, w: {{ region.w }}, h: {{ region.h }}
            </div>
          </div>

          <div class="ml-2 opacity-0 group-hover:opacity-100 transition-opacity">
            <BaseButton
              :icon="mdiClose"
              small
              transparent-bg
              @click.stop="emit('delete', idx)"
            />
          </div>
        </div>
      </div>
    </CardBoxComponentBody>
  </CardBox>
</template>
