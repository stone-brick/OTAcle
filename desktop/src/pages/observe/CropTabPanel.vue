<script setup lang="ts">
import { ref } from 'vue'
import { useObserve } from '../../composables/observe/useObserve'
import { useProject } from '../../composables/useProject'
import type { CropRegion } from '../../types'
import CropRegionList from '@/components/observe/CropRegionList.vue'
import ObservePanel from './ObservePanel.vue'

const { config, addCropRegion, removeCropRegion, updateCropRegions, saveConfig } = useObserve()
const { isProjectLoaded, getProjectObserveConfigPath } = useProject()

const selectedCropIndex = ref<number | null>(null)
const hasChanges = ref(false)

function handleCropSelect(index: number) {
  selectedCropIndex.value = index
}

function handleCropDelete(index: number) {
  removeCropRegion(index)
  hasChanges.value = true
  if (selectedCropIndex.value === index) {
    selectedCropIndex.value = null
  } else if (selectedCropIndex.value !== null && selectedCropIndex.value > index) {
    selectedCropIndex.value--
  }
}

function handleCropUpdate(index: number, region: CropRegion) {
  const newRegions = [...config.value.crop_regions]
  newRegions[index] = region
  updateCropRegions(newRegions)
  hasChanges.value = true
}

function handleCropAdd(region: CropRegion) {
  addCropRegion(region)
  hasChanges.value = true
  selectedCropIndex.value = config.value.crop_regions.length - 1
}

async function handleCropSave() {
  const path = await getProjectObserveConfigPath()
  if (path) {
    await saveConfig(path)
    hasChanges.value = false
  }
}
</script>

<template>
  <ObservePanel>
    <CropRegionList
      :regions="config.crop_regions"
      :selected-index="selectedCropIndex"
      :has-changes="hasChanges"
      :is-project-loaded="isProjectLoaded"
      @select="handleCropSelect"
      @delete="handleCropDelete"
      @update="handleCropUpdate"
      @add="handleCropAdd"
      @save="handleCropSave"
    />
  </ObservePanel>
</template>
