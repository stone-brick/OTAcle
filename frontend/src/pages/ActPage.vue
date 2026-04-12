<script setup lang="ts">
import { ref } from 'vue'
import ActTabNav from '../components/nav/ActTabNav.vue'
import ZmqMonitor from '../components/ZmqMonitor.vue'
import ActionListEditor from '../components/editor/ActionListEditor.vue'
import ActionEditor from '../components/editor/ActionEditor.vue'
import WindowSidebar from '../components/WindowSidebar.vue'
import { useActionEditor } from '../composables/useActionEditor'
import { useWindows } from '../composables/useWindows'

const activeTab = ref<'monitor' | 'config' | 'windows'>('monitor')

const { actions, selectedIndex, selectAction, deleteAction } = useActionEditor()
const { windows, selectedWindow, isLoading, refreshWindows, selectWindow } = useWindows()

function handleSelect(index: number) {
  selectAction(index)
}

function handleDelete(index: number) {
  deleteAction(index)
}
</script>

<template>
  <div class="act-page">
    <!-- Top Navigation -->
    <ActTabNav v-model:activeTab="activeTab" />

    <!-- Tab Content -->
    <div class="act-content">
      <!-- Monitor Tab -->
      <div v-if="activeTab === 'monitor'" class="tab-panel">
        <ZmqMonitor />
      </div>

      <!-- Config Tab -->
      <div v-else-if="activeTab === 'config'" class="tab-panel config-layout">
        <aside class="config-sidebar">
          <ActionListEditor
            :actions="actions"
            :selectedIndex="selectedIndex"
            @select="handleSelect"
            @delete="handleDelete"
          />
        </aside>
        <main class="config-editor">
          <ActionEditor />
        </main>
      </div>

      <!-- Windows Tab -->
      <div v-else-if="activeTab === 'windows'" class="tab-panel">
        <WindowSidebar
          :windows="windows"
          :selectedHwnd="selectedWindow?.hwnd ?? null"
          :isLoading="isLoading"
          @select="selectWindow"
          @refresh="refreshWindows"
        />
      </div>
    </div>
  </div>
</template>

<style scoped>
.act-page {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
}

.act-content {
  flex: 1;
  overflow: hidden;
}

.tab-panel {
  height: 100%;
  overflow: auto;
  background: var(--color-background);
}

.tab-panel.scrollbar-thin::-webkit-scrollbar {
  width: 6px;
}

.tab-panel.scrollbar-thin::-webkit-scrollbar-thumb {
  background: var(--color-border);
  border-radius: 3px;
}

.config-layout {
  display: flex;
}

.config-sidebar {
  width: 30%;
  max-width: 400px;
  min-width: 250px;
  flex-shrink: 0;
  border-right: 1px solid var(--color-border);
  overflow: auto;
  background: var(--color-surface);
}

.config-sidebar.scrollbar-thin::-webkit-scrollbar {
  width: 6px;
}

.config-sidebar.scrollbar-thin::-webkit-scrollbar-thumb {
  background: var(--color-border);
  border-radius: 3px;
}

.config-editor {
  flex: 1;
  overflow: auto;
}

.config-editor.scrollbar-thin::-webkit-scrollbar {
  width: 6px;
}

.config-editor.scrollbar-thin::-webkit-scrollbar-thumb {
  background: var(--color-border);
  border-radius: 3px;
}
</style>
