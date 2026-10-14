<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { useRouter } from 'vue-router'
import ThemeSwitcher from '@/components/ThemeSwitcher.vue'

const auth = useAuthStore()
const router = useRouter()

function logout() {
  auth.clearToken()
  router.push('/')
}
</script>

<template>
  <div id="app-root">
    <nav class="navbar">
      <div class="nav-inner">
        <RouterLink to="/" class="nav-brand">
          <span class="brand-icon">⚑</span>
          <span class="brand-text">Writeup</span>
        </RouterLink>

        <div class="nav-center"></div>

        <div class="nav-right">
          <ThemeSwitcher />
          <template v-if="auth.isLoggedIn">
            <RouterLink to="/my" class="nav-link">我的提交</RouterLink>
            <span class="nav-user">{{ auth.payload?.nickname }}</span>
            <button class="nav-logout" @click="logout">退出</button>
          </template>
          <template v-else>
            <RouterLink to="/login" class="nav-btn">登录</RouterLink>
          </template>
        </div>
      </div>
    </nav>

    <main class="main-content">
      <RouterView />
    </main>
  </div>
</template>

<style scoped>
#app-root {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

.navbar {
  position: sticky;
  top: 0;
  z-index: 100;
  height: var(--nav-height);
  background: var(--surface);
  border-bottom: 1px solid var(--border);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
}

.nav-inner {
  max-width: var(--content-max);
  margin: 0 auto;
  padding: 0 24px;
  height: 100%;
  display: flex;
  align-items: center;
  gap: 16px;
}

.nav-brand {
  display: flex;
  align-items: center;
  gap: 8px;
  text-decoration: none;
  flex-shrink: 0;
}

.brand-icon {
  font-size: 20px;
  line-height: 1;
}

.brand-text {
  font-family: var(--font-display);
  font-size: 17px;
  font-weight: 700;
  color: var(--text-primary);
  letter-spacing: -0.01em;
}

.nav-center {
  flex: 1;
  display: flex;
  justify-content: center;
}

.nav-right {
  display: flex;
  align-items: center;
  gap: 12px;
  flex-shrink: 0;
}

.nav-link {
  font-size: 14px;
  color: var(--text-secondary);
  text-decoration: none;
  transition: color var(--duration-fast);
}

.nav-link:hover { color: var(--text-primary); }

.nav-user {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
  max-width: 120px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.nav-logout {
  font-size: 13px;
  color: var(--text-tertiary);
  background: none;
  border: none;
  cursor: pointer;
  font-family: var(--font-sans);
  padding: 4px 8px;
  border-radius: var(--radius-sm);
  transition: all var(--duration-fast);
}

.nav-logout:hover {
  color: var(--danger);
  background: var(--danger-muted);
}

.nav-btn {
  padding: 7px 18px;
  background: var(--accent);
  color: var(--text-on-accent);
  border-radius: var(--radius-md);
  font-size: 14px;
  font-weight: 600;
  text-decoration: none;
  transition: all var(--duration-fast) var(--ease-out);
}

.nav-btn:hover {
  background: var(--accent-hover);
  box-shadow: 0 4px 12px var(--accent-glow);
}

.main-content { flex: 1; }

/* Cyberpunk navbar */
[data-theme="cyberpunk"] .navbar {
  border-bottom: 1px solid var(--accent);
  box-shadow: 0 1px 20px var(--accent-glow);
}

[data-theme="cyberpunk"] .brand-text {
  text-shadow: 0 0 10px var(--accent-glow);
  letter-spacing: 0.05em;
  text-transform: uppercase;
}

/* Frutiger Aero navbar */
[data-theme="frutiger-aero"] .navbar {
  background: var(--glass-bg);
  box-shadow: var(--shadow-md);
}
</style>
