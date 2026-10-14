<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { getGames, type GameItem } from '@/api/game'
import { getGameWriteups, downloadGameWriteups, downloadUserWriteup, deleteUserWriteup, type WriteupRecord } from '@/api/writeup'

const route = useRoute()
const router = useRouter()
const auth = useAuthStore()
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
      getGames({ page: 1, page_size: 100 }),
      getGameWriteups(gameId, { page: page.value, page_size: PAGE_SIZE }),
    ])
    game.value = gamesRes.items.find(g => g.id === gameId) ?? null
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
    await deleteUserWriteup(gameId, w.user_id)
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
  <div class="admin-view">
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
        <div class="page-header">
          <div>
            <h1 class="page-title">{{ game?.name ?? '赛事管理' }}</h1>
            <p class="page-sub">共 {{ total }} 份 Writeup</p>
          </div>
          <button class="dl-all-btn" @click="downloadGameWriteups(gameId)">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
              <polyline points="7 10 12 15 17 10"/>
              <line x1="12" y1="15" x2="12" y2="3"/>
            </svg>
            批量下载 ZIP
          </button>
        </div>

        <div v-if="items.length === 0" class="state-msg empty">暂无提交</div>

        <div v-else class="table-wrap">
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
                <td><span class="type-tag">{{ w.file_type.toUpperCase() }}</span></td>
                <td class="date">{{ formatDate(w.updated_at) }}</td>
                <td>
                  <div class="action-btns">
                    <button class="dl-btn" @click="downloadUserWriteup(gameId, w.user_id)">下载</button>
                    <button class="del-btn" :disabled="deleting === w.id" @click="handleDelete(w)">删除</button>
                  </div>
                </td>
              </tr>
            </tbody>
          </table>
        </div>

        <div v-if="total > PAGE_SIZE" class="pagination">
          <button class="page-btn" :disabled="page === 1" @click="page--; load()">上一页</button>
          <span class="page-info">{{ page }} / {{ Math.ceil(total / PAGE_SIZE) }}</span>
          <button class="page-btn" :disabled="page * PAGE_SIZE >= total" @click="page++; load()">下一页</button>
        </div>
      </template>
    </div>
  </div>
</template>

<style scoped>
.admin-view {
  max-width: var(--content-max);
  margin: 0 auto;
  padding: 40px 24px 80px;
}

.admin-container { max-width: 1000px; margin: 0 auto; }

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

.page-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 16px;
  margin-bottom: 24px;
  flex-wrap: wrap;
}

.page-title {
  font-family: var(--font-display);
  font-size: 26px;
  font-weight: 700;
  color: var(--text-primary);
  letter-spacing: -0.02em;
}

.page-sub { font-size: 14px; color: var(--text-secondary); margin-top: 4px; }

.dl-all-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 20px;
  background: var(--accent);
  color: var(--text-on-accent);
  border: none;
  border-radius: var(--radius-md);
  font-size: 14px;
  font-weight: 600;
  font-family: var(--font-sans);
  cursor: pointer;
  transition: all var(--duration-fast) var(--ease-out);
  white-space: nowrap;
}

.dl-all-btn:hover {
  background: var(--accent-hover);
  box-shadow: 0 4px 12px var(--accent-glow);
  transform: translateY(-1px);
}

.state-msg {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 60px 0;
  color: var(--text-secondary);
  font-size: 15px;
}

.state-msg.error { color: var(--danger); }
.state-msg.empty { color: var(--text-tertiary); }

.spinner {
  width: 20px; height: 20px;
  border: 2px solid var(--border);
  border-top-color: var(--accent);
  border-radius: 50%;
  animation: spin 0.7s linear infinite;
}

@keyframes spin { to { transform: rotate(360deg); } }

.table-wrap {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  overflow: hidden;
  backdrop-filter: blur(12px);
  overflow-x: auto;
}

.table {
  width: 100%;
  border-collapse: collapse;
  font-size: 14px;
  min-width: 700px;
}

.table th {
  padding: 12px 16px;
  text-align: left;
  font-size: 12px;
  font-weight: 600;
  color: var(--text-tertiary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  border-bottom: 1px solid var(--border);
  background: var(--overlay);
  white-space: nowrap;
}

.table td {
  padding: 13px 16px;
  color: var(--text-primary);
  border-bottom: 1px solid var(--border);
}

.table tr:last-child td { border-bottom: none; }
.table tr:hover td { background: var(--overlay); }

.team-name { font-weight: 500; }

.submitter {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.submitter .account {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
}

.submitter .nickname {
  font-size: 12px;
  color: var(--text-tertiary);
}

.file-name {
  color: var(--text-secondary);
  max-width: 180px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.date { color: var(--text-secondary); font-size: 13px; white-space: nowrap; }

.type-tag {
  display: inline-block;
  padding: 2px 8px;
  border-radius: var(--radius-pill);
  font-size: 11px;
  font-weight: 600;
  background: var(--accent-muted);
  color: var(--accent);
}

.dl-btn {
  padding: 5px 14px;
  border-radius: var(--radius-md);
  font-size: 13px;
  font-weight: 500;
  font-family: var(--font-sans);
  background: var(--accent-muted);
  color: var(--accent);
  border: 1px solid transparent;
  cursor: pointer;
  transition: all var(--duration-fast);
}

.dl-btn:hover {
  background: var(--accent);
  color: var(--text-on-accent);
}

.action-btns {
  display: flex;
  gap: 6px;
}

.del-btn {
  padding: 5px 14px;
  border-radius: var(--radius-md);
  font-size: 13px;
  font-weight: 500;
  font-family: var(--font-sans);
  background: var(--danger-muted);
  color: var(--danger);
  border: 1px solid transparent;
  cursor: pointer;
  transition: all var(--duration-fast);
}

.del-btn:hover:not(:disabled) {
  background: var(--danger);
  color: #fff;
}

.del-btn:disabled { opacity: 0.4; cursor: not-allowed; }

.pagination {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 16px;
  margin-top: 24px;
}

.page-btn {
  padding: 7px 18px;
  border-radius: var(--radius-md);
  font-size: 13px;
  font-family: var(--font-sans);
  background: var(--surface);
  color: var(--text-primary);
  border: 1px solid var(--border);
  cursor: pointer;
  transition: all var(--duration-fast);
}

.page-btn:hover:not(:disabled) { border-color: var(--accent); color: var(--accent); }
.page-btn:disabled { opacity: 0.4; cursor: not-allowed; }
.page-info { font-size: 13px; color: var(--text-secondary); }
</style>
