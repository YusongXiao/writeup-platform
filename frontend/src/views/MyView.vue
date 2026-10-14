<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getMyWriteups, downloadMyWriteup, type WriteupRecord } from '@/api/writeup'

const items = ref<WriteupRecord[]>([])
const total = ref(0)
const page = ref(1)
const loading = ref(false)
const PAGE_SIZE = 20

function formatDate(s: string): string {
  return new Date(s).toLocaleString('zh-CN')
}

function formatSize(bytes: number): string {
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB'
  return (bytes / 1024 / 1024).toFixed(1) + ' MB'
}

async function load() {
  loading.value = true
  try {
    const res = await getMyWriteups({ page: page.value, page_size: PAGE_SIZE })
    items.value = res.items
    total.value = res.total
  } finally {
    loading.value = false
  }
}

onMounted(load)
</script>

<template>
  <div class="my-view">
    <div class="my-container">
      <h1 class="page-title">我的提交</h1>

      <div v-if="loading" class="state-msg">
        <div class="spinner"></div> 加载中…
      </div>

      <div v-else-if="items.length === 0" class="state-msg empty">
        还没有提交记录
      </div>

      <div v-else class="table-wrap">
        <table class="table">
          <thead>
            <tr>
              <th>赛事</th>
              <th>文件名</th>
              <th>大小</th>
              <th>类型</th>
              <th>提交时间</th>
              <th>操作</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="w in items" :key="w.id">
              <td class="game-name">{{ w.game_name }}</td>
              <td class="file-name">{{ w.file_name }}</td>
              <td>{{ formatSize(w.file_size) }}</td>
              <td><span class="type-tag">{{ w.file_type.toUpperCase() }}</span></td>
              <td class="date">{{ formatDate(w.updated_at) }}</td>
              <td>
                <button class="dl-btn" @click="downloadMyWriteup(w.game_id)">下载</button>
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
    </div>
  </div>
</template>

<style scoped>
.my-view {
  max-width: var(--content-max);
  margin: 0 auto;
  padding: 40px 24px 80px;
}

.my-container { max-width: 900px; margin: 0 auto; }

.page-title {
  font-family: var(--font-display);
  font-size: 28px;
  font-weight: 700;
  color: var(--text-primary);
  letter-spacing: -0.02em;
  margin-bottom: 28px;
}

.state-msg {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 60px 0;
  color: var(--text-secondary);
  font-size: 15px;
}

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
}

.table {
  width: 100%;
  border-collapse: collapse;
  font-size: 14px;
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
}

.table td {
  padding: 14px 16px;
  color: var(--text-primary);
  border-bottom: 1px solid var(--border);
}

.table tr:last-child td { border-bottom: none; }

.table tr:hover td { background: var(--overlay); }

.game-name { font-weight: 500; }

.file-name {
  color: var(--text-secondary);
  max-width: 200px;
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

.page-btn:hover:not(:disabled) {
  border-color: var(--accent);
  color: var(--accent);
}

.page-btn:disabled { opacity: 0.4; cursor: not-allowed; }
.page-info { font-size: 13px; color: var(--text-secondary); }
</style>
