import { createRouter, createWebHistory } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    { path: '/',          name: 'home',   component: () => import('@/views/HomeView.vue') },
    { path: '/login',     name: 'login',  component: () => import('@/views/LoginView.vue') },
    { path: '/submit/:gameId', name: 'submit', component: () => import('@/views/SubmitView.vue'), meta: { requiresAuth: true } },
    { path: '/my',        name: 'my',     component: () => import('@/views/MyView.vue'),    meta: { requiresAuth: true } },
    { path: '/my/:gameId/read', name: 'my-reader', component: () => import('@/views/MyReaderView.vue'), meta: { requiresAuth: true } },
    { path: '/admin/:gameId', name: 'admin', component: () => import('@/views/AdminView.vue'), meta: { requiresAuth: true } },
    { path: '/admin/:gameId/read', name: 'admin-reader', component: () => import('@/views/AdminReaderView.vue'), meta: { requiresAuth: true } },
  ],
})

router.beforeEach((to) => {
  if (to.meta.requiresAuth) {
    const auth = useAuthStore()
    if (!auth.isLoggedIn) {
      return { name: 'login', query: { redirect: to.fullPath } }
    }
  }
})

export default router
