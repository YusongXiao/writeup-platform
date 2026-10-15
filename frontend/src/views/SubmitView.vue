<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { getGames, type GameItem } from '@/api/game'
import { submitWriteup, getMyWriteupForGame, type WriteupRecord } from '@/api/writeup'
import FileUpload from '@/components/FileUpload.vue'
import UiIcon from '@/components/UiIcon.vue'

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
  <div class="submit-view page-shell">
    <div class="submit-container">
      <button class="back-btn" @click="router.push('/')"><UiIcon name="arrowLeft" :size="16" />返回</button>
      <div v-if="loading" class="state-msg" role="status"><div class="spinner"></div>加载中…</div>
      <template v-else-if="game">
        <div class="page-heading">
          <div>
            <h1 class="heading-title"><UiIcon name="upload" :size="26" />{{ game.name }}</h1>
            <p class="heading-sub">提交你的 Writeup，记录解题过程与思路</p>
          </div>
          <span class="tag" :class="game.host_type === 0 ? 'tag-success' : 'tag-primary'">{{ game.host_type === 0 ? '训练场' : '赛事' }}</span>
        </div>
        <section class="panel">
          <div class="panel-body submission-body">
            <div class="section-heading"><UiIcon name="file" /><h2>解题报告</h2><span class="file-formats">PDF / DOCX · 最大 50 MB</span></div>
            <div v-if="existing" class="existing-card">
              <div class="existing-header"><span class="tag tag-success"><UiIcon name="check" :size="14" />已提交</span><span class="existing-date">{{ formatDate(existing.updated_at) }}</span></div>
              <div class="existing-info"><UiIcon name="file" :size="18" /><span class="existing-name">{{ existing.file_name }}</span></div>
              <p class="existing-hint">{{ formatSize(existing.file_size) }} · {{ existing.file_type.toUpperCase() }} · 重新上传将覆盖当前文件</p>
            </div>
            <div v-if="lockReason" class="status-message warning">{{ lockMessage(lockReason) }}</div>
            <template v-else>
              <FileUpload @file="selectedFile = $event" />
              <div v-if="success" class="status-message success" role="status"><UiIcon name="check" :size="18" />提交成功！</div>
              <div v-if="error" class="status-message error" role="alert">{{ error }}</div>
              <div class="submit-footer"><span>请确认文件内容完整后提交</span><button class="btn btn-primary" :disabled="!selectedFile || submitting" @click="handleSubmit"><span v-if="submitting" class="btn-spinner"></span><UiIcon v-else name="upload" :size="18" />{{ submitting ? '提交中…' : (existing ? '更新 Writeup' : '提交 Writeup') }}</button></div>
            </template>
          </div>
        </section>
      </template>
      <div v-else class="state-msg error" role="alert">{{ error || '未找到该赛事' }}</div>
    </div>
  </div>
</template>

<style scoped>
.submit-container { max-width: 800px; margin: 0 auto; }
.submission-body { display: flex; flex-direction: column; gap: 20px; }
.section-heading { display: flex; align-items: center; gap: 8px; padding-bottom: 16px; border-bottom: 1px solid var(--border); }
.section-heading h2 { font-size: 14px; }
.file-formats { margin-left: auto; color: var(--text-tertiary); font-size: 11px; }
.existing-card { padding: 16px; border: 1px solid var(--border); border-radius: var(--radius-md); background: var(--overlay); }
.existing-header { display: flex; justify-content: space-between; align-items: center; gap: 8px; flex-wrap: wrap; margin-bottom: 12px; }
.existing-date, .existing-hint { color: var(--text-tertiary); font-size: 11px; }
.existing-info { display: flex; align-items: center; gap: 8px; }
.existing-name { min-width: 0; overflow-wrap: anywhere; font-size: 13px; }
.existing-hint { margin-top: 6px; }
.submit-footer { display: flex; align-items: center; justify-content: space-between; gap: 16px; padding-top: 4px; }
.submit-footer > span { font-size: 11px; color: var(--text-tertiary); }
@media (max-width: 580px) {
  .section-heading { flex-wrap: wrap; }
  .file-formats { width: 100%; margin: 0; }
  .submit-footer { flex-direction: column; align-items: stretch; }
}
</style>
