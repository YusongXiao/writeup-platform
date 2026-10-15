<script setup lang="ts">
import { t } from '@/i18n'
import { ref, watch } from 'vue'
import { RouterLink, RouterView, useRoute, useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import ThemeSwitcher from '@/components/ThemeSwitcher.vue'
import LanguageSwitcher from '@/components/LanguageSwitcher.vue'
import AppBackground from '@/components/AppBackground.vue'
import UiIcon from '@/components/UiIcon.vue'
import logoUrl from '@/assets/ret2shell.svg'

const auth = useAuthStore()
const router = useRouter()
const route = useRoute()
const menuOpen = ref(false)
watch(() => route.fullPath, () => { menuOpen.value = false })

function logout() {
  auth.clearToken()
  router.push('/')
}
</script>

<template>
  <div id="app-root">
    <AppBackground />
    <header class="navbar">
      <div class="nav-inner">
        <button class="btn btn-ghost btn-square mobile-menu-button" :aria-expanded="menuOpen" aria-controls="main-navigation" :aria-label="t('nav.toggle')" @click="menuOpen = !menuOpen"><UiIcon :name="menuOpen ? 'close' : 'menu'" /></button>
        <RouterLink to="/" class="nav-brand btn btn-ghost">
          <img :src="logoUrl" width="28" height="28" alt="ret2shell" />
          <span>Writeup</span>
        </RouterLink>
        <nav id="main-navigation" class="nav-links" :class="{ open: menuOpen }" :aria-label="t('nav.main')">
          <RouterLink :to="{ name: 'home', query: { type: 'training' } }" class="btn btn-ghost nav-link" :class="{ active: route.name === 'home' && route.query.type !== 'competition' }"><UiIcon name="training" /><span>{{ t('common.training') }}</span></RouterLink>
          <RouterLink :to="{ name: 'home', query: { type: 'competition' } }" class="btn btn-ghost nav-link" :class="{ active: route.name === 'home' && route.query.type === 'competition' }"><UiIcon name="flag" /><span>{{ t('common.competition') }}</span></RouterLink>
          <RouterLink v-if="auth.isLoggedIn" to="/my" class="btn btn-ghost nav-link" :class="{ active: route.name === 'my' || route.name === 'my-reader' }"><UiIcon name="file" /><span>{{ t('common.mine') }}</span></RouterLink>
        </nav>
        <div class="nav-right">
          <LanguageSwitcher />
          <ThemeSwitcher />
          <template v-if="auth.isLoggedIn">
            <span class="nav-divider" aria-hidden="true"></span>
            <span class="nav-user" :title="auth.payload?.nickname || auth.payload?.account"><UiIcon name="user" /><span>{{ auth.payload?.nickname || auth.payload?.account }}</span></span>
            <button class="btn btn-ghost btn-square logout-button" :title="t('common.logout')" :aria-label="t('common.logout')" @click="logout"><UiIcon name="logout" /></button>
          </template>
          <RouterLink v-else to="/login" class="btn btn-primary btn-sm login-link"><UiIcon name="user" :size="18" />{{ t('common.login') }}</RouterLink>
        </div>
      </div>
    </header>
    <main class="main-content"><RouterView /></main>
  </div>
</template>

<style scoped>
#app-root { display: flex; flex-direction: column; min-height: 100vh; }
.navbar { position: sticky; top: 0; z-index: 100; height: var(--nav-height); flex-shrink: 0; border-bottom: 1px solid var(--border); background: var(--surface); backdrop-filter: blur(8px); }
.nav-inner { height: 100%; display: flex; align-items: center; gap: 12px; padding: 0 12px; background: var(--overlay); }
.nav-brand { gap: 12px; padding: 0 12px; font-size: 16px; font-weight: 400; }
.nav-links { display: flex; align-items: center; gap: 4px; }
.nav-link { font-weight: 400; }
.nav-link.active { background: var(--overlay); }
.nav-right { display: flex; align-items: center; gap: 8px; margin-left: auto; min-width: 0; }
.nav-divider { height: 24px; width: 1px; background: var(--border); margin: 0 4px; }
.nav-user { display: flex; align-items: center; gap: 8px; font-size: 13px; min-width: 0; }
.nav-user > span { max-width: 140px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.logout-button { color: var(--text-secondary); }
.logout-button:hover { color: var(--danger); }
.mobile-menu-button { display: none; }
.main-content { flex: 1; min-width: 0; }
@media (max-width: 760px) {
  .nav-inner { gap: 4px; padding: 0 8px; }
  .nav-brand { padding: 0 8px; gap: 8px; }
  .mobile-menu-button { display: inline-flex; }
  .nav-links { display: none; position: absolute; top: calc(var(--nav-height) + 8px); left: 8px; min-width: 210px; padding: 8px; border: 1px solid var(--border); border-radius: var(--radius-lg); background: var(--bg-elevated); }
  .nav-links.open { display: flex; flex-direction: column; align-items: stretch; }
  .nav-link { justify-content: flex-start; min-height: 40px; }
  .nav-user > span { max-width: 80px; }
}
@media (max-width: 400px) {
  .nav-user { display: none; }
  .nav-divider { margin: 0; }
}
</style>
