<script setup lang="ts">
import { ref } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { loginWithCode } from '@/api/account'

const router = useRouter()
const route = useRoute()
const auth = useAuthStore()

const code = ref('')
const loading = ref(false)
const error = ref('')

async function handleLogin() {
  const trimmed = code.value.trim().toUpperCase()
  if (!trimmed || !/^[0-9A-F]{1,6}$/.test(trimmed)) {
    error.value = '请输入有效的 6 位验证码（数字和大写字母）'
    return
  }
  loading.value = true
  error.value = ''
  try {
    await loginWithCode(trimmed)
    const redirect = (route.query.redirect as string) || '/'
    router.push(redirect)
  } catch (e: any) {
    error.value = e.response?.data?.error || '验证码无效或已过期，请重新生成'
    code.value = ''
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="login-page">
    <div class="login-card">
      <div class="login-header">
        <h1 class="login-title">登录</h1>
        <p class="login-sub">在 ret2shell 平台生成临时验证码后输入</p>
      </div>

      <form class="login-form" @submit.prevent="handleLogin">
        <div class="field">
          <label class="field-label">临时验证码</label>
          <input
            v-model="code"
            type="text"
            class="field-input code-input"
            placeholder="例如：A3F012"
            inputmode="text"
            maxlength="6"
            autocomplete="off"
            required
          />
          <p class="field-hint">
            前往 ret2shell → 个人中心 → 生成临时识别码
          </p>
        </div>

        <div v-if="error" class="error-msg">{{ error }}</div>

        <button
          type="submit"
          class="submit-btn"
          :disabled="loading || !code.trim()"
        >
          <span v-if="loading" class="btn-spinner"></span>
          <span>{{ loading ? '验证中…' : '登录' }}</span>
        </button>
      </form>
    </div>
  </div>
</template>

<style scoped>
.login-page {
  min-height: calc(100vh - var(--nav-height));
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 24px;
}

.login-card {
  width: 100%;
  max-width: 400px;
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: var(--radius-xl);
  padding: 40px 36px;
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  box-shadow: var(--shadow-lg);
}

.login-header {
  text-align: center;
  margin-bottom: 32px;
}

.login-title {
  font-family: var(--font-display);
  font-size: 28px;
  font-weight: 700;
  color: var(--text-primary);
  letter-spacing: -0.02em;
}

.login-sub {
  margin-top: 6px;
  font-size: 14px;
  color: var(--text-secondary);
}

.login-form {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.field {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.field-label {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-secondary);
}

.field-input {
  padding: 10px 14px;
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  background: var(--bg-elevated);
  color: var(--text-primary);
  font-size: 15px;
  transition: all var(--duration-fast) var(--ease-out);
  outline: none;
}

.code-input {
  font-size: 24px;
  font-family: var(--font-mono, monospace);
  letter-spacing: 0.2em;
  text-align: center;
  padding: 14px;
}

.field-input::placeholder { color: var(--text-tertiary); }

.field-input:focus {
  border-color: var(--border-focus);
  box-shadow: 0 0 0 3px var(--accent-muted);
}

.field-hint {
  font-size: 12px;
  color: var(--text-tertiary);
  line-height: 1.5;
}

.error-msg {
  padding: 10px 14px;
  background: var(--danger-muted);
  border: 1px solid var(--danger);
  border-radius: var(--radius-md);
  font-size: 13px;
  color: var(--danger);
}

.submit-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 12px;
  background: var(--accent);
  color: var(--text-on-accent);
  border: none;
  border-radius: var(--radius-md);
  font-size: 15px;
  font-weight: 600;
  font-family: var(--font-sans);
  cursor: pointer;
  transition: all var(--duration-fast) var(--ease-out);
  margin-top: 4px;
}

.submit-btn:hover:not(:disabled) {
  background: var(--accent-hover);
  box-shadow: 0 4px 16px var(--accent-glow);
  transform: translateY(-1px);
}

.submit-btn:disabled { opacity: 0.5; cursor: not-allowed; transform: none; }

.btn-spinner {
  width: 16px;
  height: 16px;
  border: 2px solid rgba(255,255,255,0.3);
  border-top-color: white;
  border-radius: 50%;
  animation: spin 0.7s linear infinite;
}

@keyframes spin { to { transform: rotate(360deg); } }

[data-theme="cyberpunk"] .login-card {
  border: 1px solid var(--accent);
  box-shadow: var(--neon-glow);
}

[data-theme="cyberpunk"] .submit-btn {
  clip-path: polygon(12px 0%, 100% 0%, calc(100% - 12px) 100%, 0% 100%);
  border-radius: 0;
  letter-spacing: 0.1em;
  text-transform: uppercase;
}

[data-theme="frutiger-aero"] .login-card {
  background: var(--glass-bg);
  box-shadow: var(--shadow-lg);
}
</style>
