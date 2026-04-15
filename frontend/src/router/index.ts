import { createRouter, createWebHistory } from 'vue-router'
import ObservePage from '../pages/ObservePage.vue'
import ThinkPage from '../pages/ThinkPage.vue'
import ActPage from '../pages/act/ActPage.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/', redirect: '/observe' },
    { path: '/observe', name: 'observe', component: ObservePage },
    { path: '/think', name: 'think', component: ThinkPage },
    { path: '/act', name: 'act', component: ActPage },
  ],
})

export default router
