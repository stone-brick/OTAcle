import { ref, readonly } from 'vue'

export interface ProjectInfo {
  path: string
  name: string
  description?: string
}

export type ProjectEventType = 'opened' | 'closed'

export interface ProjectEvent {
  type: ProjectEventType
  project: ProjectInfo | null
}

const currentEvent = ref<ProjectEvent | null>(null)

type Listener = (event: ProjectEvent) => void
const listeners = new Set<Listener>()

function emit(event: ProjectEvent): void {
  currentEvent.value = event
  listeners.forEach(listener => listener(event))
}

function onProjectEvent(listener: Listener): () => void {
  listeners.add(listener)
  return () => listeners.delete(listener)
}

export function useProjectEvents() {
  return {
    currentEvent: readonly(currentEvent),
    onProjectEvent,
    getCurrentEvent: () => currentEvent.value,
    _emit: emit,
  }
}