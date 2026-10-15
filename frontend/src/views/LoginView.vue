<script setup lang="ts">
import { ref } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { loginWithCode } from '@/api/account'
import UiIcon from '@/components/UiIcon.vue'
import logoUrl from '@/assets/ret2shell.svg'

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
    <div class="login-card panel">
      <div class="login-content panel-body">
        <form class="login-form" @submit.prevent="handleLogin">
          <div class="login-header">
            <h1>登录 Writeup</h1>
            <p>使用 ret2shell 临时验证码登录</p>
          </div>
          <div class="field">
            <label for="login-code" class="field-label">临时验证码</label>
            <div class="code-field">
              <UiIcon name="key" />
              <input id="login-code" v-model="code" type="text" class="field-input code-input" placeholder="例如：A3F012" maxlength="6" autocomplete="off" spellcheck="false" required />
            </div>
            <p class="field-hint">请输入 6 位十六进制验证码</p>
          </div>
          <div v-if="error" class="status-message error" role="alert">{{ error }}</div>
          <button type="submit" class="btn btn-primary login-submit" :disabled="loading || !code.trim()">
            <span v-if="loading" class="btn-spinner"></span>
            <UiIcon v-else name="arrowRight" :size="18" />
            <span>{{ loading ? '验证中…' : '登录' }}</span>
          </button>
        </form>
        <aside class="login-guide">
          <img :src="logoUrl" width="72" height="72" alt="ret2shell" />
          <h2>ret2shell</h2>
          <p>解题之后，记录思路。</p>
          <ol>
            <li>在 ret2shell 个人中心生成临时识别码</li>
            <li>复制识别码，粘贴到左侧输入框</li>
            <li>登录后选择比赛并提交 Writeup</li>
          </ol>
        </aside>
      </div>
    </div>
  </div>
</template>

<style scoped>
.login-page { min-height: calc(100dvh - var(--nav-height)); display: flex; align-items: center; justify-content: center; padding: 32px 20px; }
.login-card { width: 100%; max-width: 760px; }
.login-content { display: grid; grid-template-columns: 1fr 1fr; gap: 32px; padding: 32px; }
.login-form { display: flex; flex-direction: column; justify-content: center; gap: 20px; min-width: 0; }
.login-header h1 { font-size: 20px; margin-bottom: 8px; }
.login-header p { font-size: 12px; color: var(--text-secondary); }
.code-field { position: relative; }
.code-field > svg { position: absolute; left: 14px; top: 16px; color: var(--text-secondary); pointer-events: none; }
.code-input { min-height: 52px; padding-left: 44px; font-size: 18px; letter-spacing: 0.08em; text-transform: uppercase; }
.code-input::placeholder { font-size: 13px; letter-spacing: 0; }
.login-submit { width: 100%; }
.login-guide { display: flex; flex-direction: column; justify-content: center; align-items: center; border-left: 1px solid var(--border); padding: 16px 0 16px 32px; }
.login-guide h2 { font-size: 20px; font-weight: 400; margin-top: 16px; }
.login-guide > p { font-size: 12px; color: var(--text-secondary); margin-top: 4px; }
.login-guide ol { display: flex; flex-direction: column; gap: 10px; padding-left: 20px; margin-top: 24px; font-size: 12px; color: var(--text-secondary); }
@media (max-width: 620px) {
  .login-card { max-width: 420px; }
  .login-content { grid-template-columns: minmax(0, 1fr); gap: 24px; padding: 24px; }
  .login-guide { border-left: 0; border-top: 1px solid var(--border); padding: 24px 0 0; }
  .login-guide img { width: 48px; height: 48px; }
  .login-guide ol { margin-top: 16px; }
}
</style>
