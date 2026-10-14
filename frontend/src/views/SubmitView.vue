<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { getGames, type GameItem } from '@/api/game'
import { submitWriteup, getMyWriteupForGame, type WriteupRecord } from '@/api/writeup'
import FileUpload from '@/components/FileUpload.vue'

const route = useRoute()
const router = useRouter()
const auth = useAuthStore()
const gameId = Number(route.params.gameId)

const game = ref<GameItem | null>(null)
const existing = ref<WriteupRecord | null>(null)
const selectedFile = ref<File | null>(null)
const loading = ref(false)
const submitting = ref(false)
const error = ref('')
const success = ref(false)
const lockReason = ref('')  // non-empty = locked

function formatSize(bytes: number): string {
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB'
  return (bytes / 1024 / 1024).toFixed(1) + ' MB'
}

function formatDate(s: string): string {
  return new Date(s).toLocaleString('zh-CN')
}

function computeLockReason(g: GameItem): string {
  const isAdmin = auth.isGameAdmin(g.admins ?? [])
  if (isAdmin) return ''
  if (g.host_type === 1) {
    const now = Date.now()
    if (g.start_at && new Date(g.start_at).getTime() > now) return 'not_started'
    if (g.archive_at && new Date(g.archive_at).getTime() <= now) return 'ended'
  }
  return ''
}

function lockMessage(reason: string): string {
  if (reason === 'not_started') return '比赛还未开始，暂不接受提交'
  if (reason === 'ended') return '比赛已归档，不再接受提交'
  return ''
}

async function load() {
  loading.value = true
  try {
    const [gamesRes, existingRes] = await Promise.all([
      getGames({ page: 1, page_size: 100 }),
      getMyWriteupForGame(gameId),
    ])
    game.value = gamesRes.items.find(g => g.id === gameId) ?? null
    existing.value = existingRes
    if (game.value) lockReason.value = computeLockReason(game.value)
  } catch {
    error.value = '加载失败'
  } finally {
    loading.value = false
  }
}

async function handleSubmit() {
  if (!selectedFile.value) return
  submitting.value = true
  error.value = ''
  success.value = false
  try {
    existing.value = await submitWriteup(gameId, selectedFile.value)
    success.value = true
    selectedFile.value = null
  } catch (e: any) {
    const msg: string = e.response?.data?.error ?? ''
    if (msg.includes('not registered')) error.value = '你尚未报名参加本次比赛'
    else if (msg.includes('not started')) error.value = '比赛还未开始，暂不接受提交'
    else if (msg.includes('ended')) error.value = '比赛已归档，不再接受提交'
    else error.value = msg || '提交失败'
  } finally {
    submitting.value = false
  }
}

onMounted(load)
</script>

<template>
  <div class="submit-view">
    <div class="submit-container">
      <button class="back-btn" @click="router.push('/')">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <polyline points="15 18 9 12 15 6"/>
        </svg>
        返回
      </button>

      <div v-if="loading" class="state-msg">
        <div class="spinner"></div> 加载中…
      </div>

      <template v-else-if="game">
        <div class="page-header">
          <span class="type-badge" :class="game.host_type === 0 ? 'training' : 'competition'">
            {{ game.host_type === 0 ? '训练场' : '赛事' }}
          </span>
          <h1 class="page-title">{{ game.name }}</h1>
          <p class="page-sub">提交你的 Writeup（PDF 或 docx，最大 50 MB）</p>
        </div>

        <div v-if="existing" class="existing-card">
          <div class="existing-header">
            <span class="existing-label">已提交</span>
            <span class="existing-date">{{ formatDate(existing.updated_at) }}</span>
          </div>
          <div class="existing-info">
            <span class="existing-name">{{ existing.file_name }}</span>
            <span class="existing-size">{{ formatSize(existing.file_size) }}</span>
            <span class="existing-type">{{ existing.file_type.toUpperCase() }}</span>
          </div>
          <p class="existing-hint">重新上传将覆盖当前文件</p>
        </div>

        <div v-if="lockReason" class="lock-card">
          <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <rect x="3" y="11" width="18" height="11" rx="2" ry="2"/>
            <path d="M7 11V7a5 5 0 0 1 10 0v4"/>
          </svg>
          <span>{{ lockMessage(lockReason) }}</span>
        </div>

        <template v-else>
          <div class="upload-section">
            <FileUpload @file="selectedFile = $event" />
          </div>

          <div v-if="success" class="success-msg">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
              <polyline points="20 6 9 17 4 12"/>
            </svg>
            提交成功！
          </div>

          <div v-if="error" class="error-msg">{{ error }}</div>

          <button
            class="submit-btn"
            :disabled="!selectedFile || submitting"
            @click="handleSubmit"
          >
            <span v-if="submitting" class="btn-spinner"></span>
            {{ submitting ? '提交中…' : (existing ? '更新 Writeup' : '提交 Writeup') }}
          </button>
        </template>
      </template>

      <div v-else class="state-msg error">未找到该赛事</div>
    </div>
  </div>
</template>

<style scoped>
.submit-view {
  max-width: var(--content-max);
  margin: 0 auto;
  padding: 32px 24px 80px;
}

.submit-container {
  max-width: 560px;
  margin: 0 auto;
}

.back-btn {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  font-size: 14px;
  color: var(--text-secondary);
  background: none;
  border: none;
  cursor: pointer;
  padding: 6px 0;
  margin-bottom: 24px;
  font-family: var(--font-sans);
  transition: color var(--duration-fast);
}

.back-btn:hover { color: var(--accent); }

.page-header { margin-bottom: 28px; }

.type-badge {
  display: inline-block;
  font-size: 11px;
  font-weight: 600;
  padding: 2px 8px;
  border-radius: var(--radius-pill);
  text-transform: uppercase;
  letter-spacing: 0.04em;
  margin-bottom: 10px;
}

.type-badge.training { background: var(--success-muted); color: var(--success); }
.type-badge.competition { background: var(--accent-muted); color: var(--accent); }

.page-title {
  font-family: var(--font-display);
  font-size: 26px;
  font-weight: 700;
  color: var(--text-primary);
  letter-spacing: -0.02em;
  margin-bottom: 6px;
}

.page-sub { font-size: 14px; color: var(--text-secondary); }

.existing-card {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  padding: 16px 20px;
  margin-bottom: 20px;
  backdrop-filter: blur(12px);
}

.existing-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 8px;
}

.existing-label {
  font-size: 12px;
  font-weight: 600;
  color: var(--success);
  text-transform: uppercase;
  letter-spacing: 0.04em;
}

.existing-date { font-size: 12px; color: var(--text-tertiary); }

.existing-info {
  display: flex;
  align-items: center;
  gap: 10px;
  flex-wrap: wrap;
}

.existing-name {
  font-size: 14px;
  font-weight: 500;
  color: var(--text-primary);
}

.existing-size, .existing-type {
  font-size: 12px;
  color: var(--text-tertiary);
}

.existing-hint {
  margin-top: 8px;
  font-size: 12px;
  color: var(--text-tertiary);
}

.lock-card {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 16px 20px;
  background: var(--warning-muted, rgba(255, 166, 0, 0.08));
  border: 1px solid var(--warning, #ffa600);
  border-radius: var(--radius-md);
  font-size: 14px;
  color: var(--warning, #ffa600);
  margin-bottom: 16px;
}

.upload-section { margin-bottom: 16px; }

.success-msg {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 14px;
  background: var(--success-muted);
  border: 1px solid var(--success);
  border-radius: var(--radius-md);
  font-size: 13px;
  color: var(--success);
  margin-bottom: 12px;
}

.error-msg {
  padding: 10px 14px;
  background: var(--danger-muted);
  border: 1px solid var(--danger);
  border-radius: var(--radius-md);
  font-size: 13px;
  color: var(--danger);
  margin-bottom: 12px;
}

.submit-btn {
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 13px;
  background: var(--accent);
  color: var(--text-on-accent);
  border: none;
  border-radius: var(--radius-md);
  font-size: 15px;
  font-weight: 600;
  font-family: var(--font-sans);
  cursor: pointer;
  transition: all var(--duration-fast) var(--ease-out);
}

.submit-btn:hover:not(:disabled) {
  background: var(--accent-hover);
  box-shadow: 0 4px 16px var(--accent-glow);
  transform: translateY(-1px);
}

.submit-btn:disabled { opacity: 0.5; cursor: not-allowed; transform: none; }

.state-msg {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 60px 0;
  color: var(--text-secondary);
  font-size: 15px;
}

.state-msg.error { color: var(--danger); }

.spinner {
  width: 20px; height: 20px;
  border: 2px solid var(--border);
  border-top-color: var(--accent);
  border-radius: 50%;
  animation: spin 0.7s linear infinite;
}

.btn-spinner {
  width: 16px; height: 16px;
  border: 2px solid rgba(255,255,255,0.3);
  border-top-color: white;
  border-radius: 50%;
  animation: spin 0.7s linear infinite;
}

@keyframes spin { to { transform: rotate(360deg); } }
</style>
