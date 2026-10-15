<script setup lang="ts">
import { ref, onMounted } from 'vue'
import UiIcon from '@/components/UiIcon.vue'
import { useRoute, useRouter } from 'vue-router'
import { getGame, type GameItem } from '@/api/game'
import { getGameWriteups, downloadGameWriteups, downloadUserWriteup, deleteUserWriteup, type WriteupRecord } from '@/api/writeup'

const route = useRoute()
const router = useRouter()
const gameId = Number(route.params.gameId)

const game = ref<GameItem | null>(null)
const items = ref<WriteupRecord[]>([])
const total = ref(0)
const page = ref(1)
const loading = ref(false)
const error = ref('')
const PAGE_SIZE = 50

function formatDate(s: string): string {
  return new Date(s).toLocaleString('zh-CN')
}

function formatSize(bytes: number): string {
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB'
  return (bytes / 1024 / 1024).toFixed(1) + ' MB'
}

async function load() {
  loading.value = true
  error.value = ''
  try {
    const [gamesRes, writeupRes] = await Promise.all([
      getGame(gameId),
      getGameWriteups(gameId, { page: page.value, page_size: PAGE_SIZE }),
    ])
    game.value = gamesRes
    items.value = writeupRes.items
    total.value = writeupRes.total
  } catch (e: any) {
    error.value = e.response?.data?.error || '加载失败'
  } finally {
    loading.value = false
  }
}

const deleting = ref<number | null>(null)

async function handleDelete(w: WriteupRecord) {
  if (!confirm(`确认删除 ${w.team_name ?? w.user_account} 的 Writeup？此操作不可撤销。`)) return
  deleting.value = w.id
  try {
    await deleteUserWriteup(gameId, w.id)
    items.value = items.value.filter(i => i.id !== w.id)
    total.value--
  } catch (e: any) {
    alert(e.response?.data?.error || '删除失败')
  } finally {
    deleting.value = null
  }
}

onMounted(load)
</script>

<template>
  <div class="admin-view page-shell">
    <div class="admin-container">
      <button class="back-btn" @click="router.push('/')">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <polyline points="15 18 9 12 15 6"/>
        </svg>
        返回
      </button>

      <div v-if="loading" class="state-msg">
        <div class="spinner"></div> 加载中…
      </div>

      <div v-else-if="error" class="state-msg error">{{ error }}</div>

      <template v-else>
        <div class="page-heading">
          <div>
            <h1 class="heading-title"><UiIcon name="settings" :size="26" />{{ game?.name ?? '赛事管理' }}</h1>
            <p class="heading-sub">共 {{ total }} 份 Writeup</p>
          </div>
          <div class="header-actions">
            <button class="btn" @click="router.push({ name: 'admin-reader', params: { gameId } })">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" aria-hidden="true">
                <path d="M12 7v14M3 3h5a4 4 0 0 1 4 4 4 4 0 0 1 4-4h5v16h-5a4 4 0 0 0-4 2 4 4 0 0 0-4-2H3z"/>
              </svg>
              批量阅读
            </button>
            <button class="btn btn-primary" @click="downloadGameWriteups(gameId)">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
                <polyline points="7 10 12 15 17 10"/>
                <line x1="12" y1="15" x2="12" y2="3"/>
              </svg>
              批量下载 ZIP
            </button>
          </div>
        </div>

        <div v-if="items.length === 0" class="state-msg empty">暂无提交</div>

        <div v-else class="table-wrap panel">
          <table class="table">
            <thead>
              <tr>
                <th>队伍</th>
                <th>提交人</th>
                <th>文件名</th>
                <th>大小</th>
                <th>类型</th>
                <th>提交时间</th>
                <th>操作</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="w in items" :key="w.id">
                <td class="team-name">{{ w.team_name ?? '—' }}</td>
                <td class="submitter">
                  <span class="account">{{ w.user_account }}</span>
                  <span class="nickname">{{ w.user_nickname }}</span>
                </td>
                <td class="file-name">{{ w.file_name }}</td>
                <td>{{ formatSize(w.file_size) }}</td>
                <td><span class="tag tag-primary">{{ w.file_type.toUpperCase() }}</span></td>
                <td class="date">{{ formatDate(w.updated_at) }}</td>
                <td>
                  <div class="action-btns">
                    <button class="btn btn-sm" @click="router.push({ name: 'admin-reader', params: { gameId }, query: { writeup: w.id } })">预览</button>
                    <button class="btn btn-sm" @click="downloadUserWriteup(gameId, w.id)">下载</button>
                    <button class="btn btn-sm btn-danger" :disabled="deleting === w.id" @click="handleDelete(w)">删除</button>
                  </div>
                </td>
              </tr>
            </tbody>
          </table>
        </div>

        <div v-if="total > PAGE_SIZE" class="pagination">
          <button class="btn btn-sm" :disabled="page === 1" @click="page--; load()">上一页</button>
          <span class="page-info">{{ page }} / {{ Math.ceil(total / PAGE_SIZE) }}</span>
          <button class="btn btn-sm" :disabled="page * PAGE_SIZE >= total" @click="page++; load()">下一页</button>
        </div>
      </template>
    </div>
  </div>
</template>

<style scoped>
.admin-container { width: 100%; }
.header-actions { display: flex; gap: 8px; flex-wrap: wrap; }
.table { min-width: 900px; }
.team-name { font-weight: 700; }
.submitter { min-width: 110px; }
.submitter .account, .submitter .nickname { display: block; }
.submitter .account { font-size: 13px; }
.submitter .nickname { font-size: 11px; color: var(--text-tertiary); margin-top: 2px; }
</style>
