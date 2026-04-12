import { ref } from 'vue'
import type { AppStatus } from '../types'

export type PageId = 'observe' | 'think' | 'act'

const currentPage = ref<PageId>('observe')
const appStatus = ref<AppStatus>('ready')

export function useApp() {
  const navigateTo = (page: PageId) => {
    currentPage.value = page
  }

  const setAppStatus = (status: AppStatus) => {
    appStatus.value = status
  }

  return {
    currentPage,
    appStatus,
    navigateTo,
    setAppStatus,
  }
}
