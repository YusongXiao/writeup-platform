<script setup lang="ts">
import { computed, onBeforeUnmount, ref, watch } from 'vue'
import { useRoute } from 'vue-router'
import { downloadMyWriteup, getMyWriteupForGame, type WriteupRecord } from '@/api/writeup'
import PdfPreview from '@/components/PdfPreview.vue'
import UiIcon from '@/components/UiIcon.vue'

const route = useRoute()
const gameId = computed(() => Number(route.params.gameId))
const writeup = ref<WriteupRecord | null>(null)
const loading = ref(true)
const error = ref('')
const downloadError = ref('')
const downloading = ref(false)
const previewVersion = ref(0)
let loadVersion = 0

async function load() {
  const version = ++loadVersion
  loading.value = true
  error.value = ''
  downloadError.value = ''
  writeup.value = null
  try {
    const result = await getMyWriteupForGame(gameId.value)
    if (version === loadVersion) writeup.value = result
  } catch (e: any) {
    if (version === loadVersion) error.value = e.response?.data?.error || '加载提交信息失败，请重试'
  } finally {
    if (version === loadVersion) loading.value = false
  }
}

async function download() {
  downloading.value = true
  downloadError.value = ''
  try {
    await downloadMyWriteup(gameId.value)
  } catch {
    downloadError.value = '下载失败，请重试'
  } finally {
    downloading.value = false
  }
}

watch(gameId, load, { immediate: true })
onBeforeUnmount(() => { loadVersion++ })
</script>

<template>
  <div class="my-reader">
    <header class="reader-header">
      <RouterLink to="/my" class="btn btn-ghost btn-sm back-link"><UiIcon name="arrowLeft" :size="16" />返回我的提交</RouterLink>
      <div class="reader-title">
        <h1 :title="writeup?.game_name">{{ writeup?.game_name || 'Writeup 预览' }}</h1>
        <span>我的提交</span>
      </div>
      <button v-if="writeup" class="btn btn-sm" :disabled="downloading" @click="download"><UiIcon name="download" :size="16" />下载</button>
    </header>
    <p v-if="downloadError" class="download-error" role="alert">{{ downloadError }}</p>
    <div v-if="loading" class="reader-state" role="status"><div class="spinner"></div>正在加载提交信息…</div>
    <div v-else-if="error" class="reader-state" role="alert"><p>{{ error }}</p><button class="btn btn-sm" @click="load">重试</button></div>
    <div v-else-if="!writeup" class="reader-state">该比赛暂无提交，或文件已被删除</div>
    <PdfPreview v-else-if="writeup.file_type.toLowerCase() === 'pdf'" :key="`${gameId}-${writeup.id}-${previewVersion}`" source="mine" :game-id="gameId" :writeup-id="writeup.id" :file-name="writeup.file_name" :owner="writeup.team_name || writeup.user_account" @retry="previewVersion++" />
    <div v-else class="reader-state"><UiIcon name="file" :size="40" /><p>{{ writeup.file_name }}</p><p>此文件为 {{ writeup.file_type.toUpperCase() }} 格式，暂不支持在线预览，请下载后阅读</p></div>
  </div>
</template>

<style scoped>
.my-reader { height: calc(100dvh - var(--nav-height)); min-height: 400px; display: flex; flex-direction: column; background: var(--bg-sunken); }
.reader-header { display: flex; align-items: center; gap: 16px; padding: 4px 16px; border-bottom: 1px solid var(--border); background: var(--surface); }
.back-link { color: var(--text-secondary); }
.reader-title { flex: 1; min-width: 0; }
.reader-title h1 { font-size: 16px; line-height: 20px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
.reader-title > span { display: block; font-size: 12px; line-height: 16px; color: var(--text-secondary); }
.reader-state { flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 14px; padding: 24px; font-size: 14px; color: var(--text-secondary); text-align: center; overflow-wrap: anywhere; }
.download-error { padding: 8px 16px; color: var(--danger); font-size: 12px; }
@media (max-width: 580px) {
  .reader-header { gap: 8px; padding: 4px 8px; }
  .reader-title h1 { font-size: 14px; }
}
</style>
