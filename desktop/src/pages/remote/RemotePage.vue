<script setup lang="ts">
import { ref } from 'vue'
import RemoteTabNav from '../../components/nav/RemoteTabNav.vue'
import AuthTabPanel from './AuthTabPanel.vue'
import GroupsTabPanel from './GroupsTabPanel.vue'
import ConfigTabPanel from './ConfigTabPanel.vue'
import { useRemote } from '../../composables/useRemote'

const { isLoggedIn } = useRemote()

const activeTab = ref<'auth' | 'groups' | 'config'>(isLoggedIn.value ? 'groups' : 'auth')

function onAuthSuccess() {
  activeTab.value = 'groups'
}

function onLogout() {
  activeTab.value = 'auth'
}
</script>

<template>
  <div class="remote-page">
    <RemoteTabNav v-model:active-tab="activeTab" />

    <div class="remote-content">
      <AuthTabPanel v-if="activeTab === 'auth'" @success="onAuthSuccess" />
      <GroupsTabPanel v-else-if="activeTab === 'groups'" @logout="onLogout" />
      <ConfigTabPanel v-else-if="activeTab === 'config'" />
    </div>
  </div>
</template>

<style scoped>
.remote-page {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
}

.remote-content {
  flex: 1;
  overflow: hidden;
}

.remote-content > :deep(*) {
  height: 100%;
  overflow: auto;
}
</style>