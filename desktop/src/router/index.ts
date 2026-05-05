import { createRouter, createWebHistory } from 'vue-router'
import ObservePage from '../pages/observe/ObservePage.vue'
import ThinkPage from '../pages/think/ThinkPage.vue'
import ActPage from '../pages/act/ActPage.vue'
import RemotePage from '../pages/remote/RemotePage.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', redirect: '/observe' },
    { path: '/observe', name: 'observe', component: ObservePage },
    { path: '/think', name: 'think', component: ThinkPage },
    { path: '/act', name: 'act', component: ActPage },
    { path: '/remote', name: 'remote', component: RemotePage },
  ],
})

export default router
