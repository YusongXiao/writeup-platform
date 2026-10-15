<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { downloadUserWriteup, getGameWriteups, type WriteupRecord } from '@/api/writeup'
import PdfPreview from '@/components/PdfPreview.vue'

const route = useRoute()
const router = useRouter()
const gameId = computed(() => Number(route.params.gameId))
const items = ref<WriteupRecord[]>([])
const loading = ref(true)
const error = ref('')
const downloadError = ref('')
const downloading = ref<number[]>([])
const search = ref('')
const previewVersion = ref(0)
const selected = computed(() => items.value.find(w => w.id === Number(route.query.writeup)))
const gameName = computed(() => items.value[0]?.game_name || `比赛 #${gameId.value}`)
const filteredItems = computed(() => {
  const query = search.value.trim().toLocaleLowerCase()
  return items.value.filter(w => [w.user_account, w.user_nickname, w.team_name].some(s => s?.toLocaleLowerCase().includes(query)))
})
let loadVersion = 0

async function load() {
  const version = ++loadVersion
  const id = gameId.value
  loading.value = true
  error.value = ''
  items.value = []
  search.value = ''
  try {
    // The API caps each page at 100; include every submission in the reader.
    const records: WriteupRecord[] = []
    for (let page = 1; ; page++) {
      const result = await getGameWriteups(id, { page, page_size: 100 })
      if (version !== loadVersion) return
      records.push(...result.items)
      if (records.length >= result.total || result.items.length === 0) break
    }
    items.value = records
    if (!route.query.writeup && records.length) {
      const first = records.find(w => w.file_type.toLowerCase() === 'pdf') ?? records[0]!
      await router.replace({ query: { ...route.query, writeup: first.id } })
    }
  } catch (e: any) {
    if (version === loadVersion) error.value = e.response?.data?.error || '加载提交列表失败，请重试'
  } finally {
    if (version === loadVersion) loading.value = false
  }
}

function selectWriteup(w: WriteupRecord) {
  router.push({ query: { ...route.query, writeup: w.id } })
}

async function download(w: WriteupRecord) {
  downloadError.value = ''
  downloading.value.push(w.id)
  try {
    await downloadUserWriteup(gameId.value, w.id)
  } catch {
    downloadError.value = `${w.file_name} 下载失败，请重试`
  } finally {
    downloading.value = downloading.value.filter(id => id !== w.id)
  }
}

function formatDate(value: string) {
  return new Date(value).toLocaleString('zh-CN', { hour12: false })
}

watch(gameId, load, { immediate: true })
onBeforeUnmount(() => { loadVersion++ })
watch(() => selected.value?.id, async () => {
  await nextTick()
  document.querySelector('.writeup-card.active')?.scrollIntoView({ block: 'nearest' })
})
</script>

<template>
  <div class="reader-view">
    <header class="reader-header">
      <RouterLink class="back-link" :to="{ name: 'admin', params: { gameId } }">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true"><path d="m15 18-6-6 6-6"/></svg>
        返回管理
      </RouterLink>
      <div class="reader-title">
        <h1>{{ gameName }}</h1>
        <span>批量阅读 · {{ items.length }} 份 Writeup</span>
      </div>
    </header>

    <div v-if="loading" class="reader-state" role="status">正在加载提交列表…</div>
    <div v-else-if="error" class="reader-state" role="alert">
      <p>{{ error }}</p>
      <button class="text-button" @click="load">重新加载</button>
    </div>
    <div v-else-if="!items.length" class="reader-state">当前比赛暂无提交</div>

    <div v-else class="reader-layout">
      <aside class="submission-sidebar" aria-label="Writeup 提交列表">
        <div class="sidebar-header">
          <h2>提交列表 <span>{{ items.length }}</span></h2>
          <input v-model="search" type="search" placeholder="搜索用户名 / 队伍名" aria-label="搜索用户名或队伍名" />
        </div>
        <div class="submission-list">
          <div v-for="w in filteredItems" :key="w.id" class="writeup-card" :class="{ active: selected?.id === w.id }">
            <button class="submission-select" :aria-pressed="selected?.id === w.id" @click="selectWriteup(w)">
              <span class="submitter" :title="`${w.user_nickname} (${w.user_account})`">{{ w.user_nickname || w.user_account }}</span>
              <span class="account">{{ w.user_account }}</span>
              <span class="team" :title="w.team_name || '无队伍'">{{ w.team_name || '无队伍' }}</span>
              <span class="submitted-at">提交时间：{{ formatDate(w.updated_at) }}</span>
              <span class="file-type">{{ w.file_type.toUpperCase() }}</span>
            </button>
            <button class="download-icon" :disabled="downloading.includes(w.id)" :title="`下载 ${w.file_name}`" :aria-label="`下载 ${w.user_nickname || w.user_account} 的 Writeup`" @click="download(w)">
              <svg width="19" height="19" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" aria-hidden="true">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4M12 3v12m-5-5 5 5 5-5"/>
              </svg>
            </button>
          </div>
          <p v-if="!filteredItems.length" class="no-results">没有匹配的提交</p>
        </div>
        <p v-if="downloadError" class="download-error" role="alert">{{ downloadError }}</p>
      </aside>

      <section class="preview-panel" aria-label="Writeup 预览">
        <template v-if="selected">
          <div v-if="selected.file_type.toLowerCase() !== 'pdf'" class="document-header">
            <span class="document-name" :title="selected.file_name">{{ selected.file_name }}</span>
            <span class="document-team">{{ selected.team_name || selected.user_account }}</span>
          </div>
          <PdfPreview v-if="selected.file_type.toLowerCase() === 'pdf'" :key="`${gameId}-${selected.id}-${previewVersion}`" :game-id="gameId" :writeup-id="selected.id" :file-name="selected.file_name" :owner="selected.team_name || selected.user_account" @retry="previewVersion++" />
          <div v-else class="reader-state">
            <p>此文件为 {{ selected.file_type.toUpperCase() }} 格式，暂不支持在线预览</p>
            <button class="text-button" :disabled="downloading.includes(selected.id)" @click="download(selected)">下载文件阅读</button>
          </div>
        </template>
        <div v-else class="reader-state">该提交不存在或已被删除，请从左侧选择其他提交</div>
      </section>
    </div>
  </div>
</template>

<style scoped>
.reader-view { height: calc(100dvh - var(--nav-height)); min-height: 400px; display: flex; flex-direction: column; }
.reader-header { display: flex; align-items: center; gap: 24px; padding: 4px 24px; border-bottom: 1px solid var(--border); background: var(--surface); }
.back-link { display: inline-flex; align-items: center; gap: 4px; flex-shrink: 0; font-size: 14px; color: var(--text-secondary); }
.back-link:hover { color: var(--accent); }
.reader-title { min-width: 0; }
.reader-title h1 { font-size: 16px; line-height: 20px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.reader-title > span { display: block; font-size: 12px; line-height: 16px; color: var(--text-secondary); }
.reader-layout { flex: 1; min-height: 0; display: grid; grid-template-columns: 320px minmax(0, 1fr); }
.submission-sidebar { min-height: 0; display: flex; flex-direction: column; background: var(--surface); border-right: 1px solid var(--border); }
.sidebar-header { padding: 16px; border-bottom: 1px solid var(--border); }
.sidebar-header h2 { display: flex; align-items: center; justify-content: space-between; font-size: 14px; margin-bottom: 12px; }
.sidebar-header h2 span { color: var(--text-secondary); font-weight: 400; }
.sidebar-header input { width: 100%; padding: 8px 10px; border: 1px solid var(--border); border-radius: var(--radius-sm); background: var(--bg); color: var(--text-primary); font-size: 13px; }
.sidebar-header input:focus { outline: 2px solid var(--border-focus); outline-offset: 2px; }
.submission-list { flex: 1; min-height: 0; overflow-y: auto; padding: 10px; }
.writeup-card { display: flex; align-items: center; margin-bottom: 8px; border: 1px solid var(--border); border-radius: var(--radius-md); background: var(--bg-elevated); overflow: hidden; }
.writeup-card:hover { border-color: var(--border-strong); }
.writeup-card.active { border-color: var(--accent); background: var(--accent-muted); }
.submission-select { display: flex; flex: 1; min-width: 0; flex-direction: column; align-items: flex-start; gap: 3px; padding: 14px 4px 14px 14px; color: var(--text-primary); text-align: left; }
.submission-select > span { max-width: 100%; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.submitter { font-size: 14px; font-weight: 600; }
.account, .submitted-at { font-size: 11px; color: var(--text-secondary); }
.team { font-size: 13px; margin-top: 4px; }
.submission-select .file-type { margin-top: 5px; padding: 1px 6px; border-radius: 4px; color: var(--accent); background: var(--accent-muted); font-size: 10px; font-weight: 600; }
.download-icon { display: inline-flex; align-items: center; justify-content: center; flex-shrink: 0; width: 34px; height: 34px; margin: 8px; border-radius: var(--radius-sm); color: var(--text-secondary); }
.download-icon:hover { color: var(--accent); background: var(--accent-muted); }
button:disabled { opacity: 0.5; cursor: wait; }
button:focus-visible, a:focus-visible { outline: 2px solid var(--border-focus); outline-offset: -2px; }
.preview-panel { min-width: 0; min-height: 0; display: flex; flex-direction: column; background: var(--bg-sunken); }
.document-header { display: flex; justify-content: space-between; align-items: center; gap: 16px; padding: 12px 20px; background: var(--surface); border-bottom: 1px solid var(--border); font-size: 13px; }
.document-name { min-width: 0; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; font-weight: 500; }
.document-team { flex-shrink: 0; max-width: 40%; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; color: var(--text-secondary); }
.reader-state { flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 12px; padding: 32px; text-align: center; font-size: 14px; color: var(--text-secondary); }
.text-button { padding: 8px 14px; border-radius: var(--radius-sm); color: var(--accent); background: var(--accent-muted); }
.download-error { padding: 12px 16px; font-size: 12px; color: var(--danger); border-top: 1px solid var(--border); }
.no-results { padding: 24px 8px; text-align: center; color: var(--text-secondary); font-size: 13px; }
@media (max-width: 800px) {
  .reader-layout { grid-template-columns: 240px minmax(0, 1fr); }
  .reader-header { padding: 4px 16px; gap: 16px; }
}
@media (max-width: 580px) {
  .reader-layout { grid-template-columns: minmax(0, 1fr); grid-template-rows: 220px minmax(0, 1fr); }
  .submission-sidebar { border-right: 0; border-bottom: 1px solid var(--border); }
  .sidebar-header { display: flex; align-items: center; gap: 12px; padding: 10px; }
  .sidebar-header h2 { margin: 0; gap: 8px; flex-shrink: 0; }
  .submission-list { display: flex; gap: 8px; overflow: auto; }
  .writeup-card { flex: 0 0 260px; margin-bottom: 0; }
  .document-header { padding: 8px 12px; }
}
</style>
