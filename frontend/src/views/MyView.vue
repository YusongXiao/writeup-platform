<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import UiIcon from '@/components/UiIcon.vue'
import { getMyWriteups, downloadMyWriteup, type WriteupRecord } from '@/api/writeup'

const items = ref<WriteupRecord[]>([])
const total = ref(0)
const page = ref(1)
const loading = ref(false)
const PAGE_SIZE = 20
const router = useRouter()

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
  <div class="my-view page-shell">
    <div class="my-container">
      <div class="page-heading">
        <div>
          <h1 class="heading-title"><UiIcon name="file" :size="26" />我的提交</h1>
          <p class="heading-sub">共 {{ total }} 份 Writeup · 查看和下载已提交的解题报告</p>
        </div>
      </div>

      <div v-if="loading" class="state-msg">
        <div class="spinner"></div> 加载中…
      </div>

      <div v-else-if="items.length === 0" class="state-msg empty">
        还没有提交记录
      </div>

      <div v-else class="table-wrap panel">
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
              <td><span class="tag tag-primary">{{ w.file_type.toUpperCase() }}</span></td>
              <td class="date">{{ formatDate(w.updated_at) }}</td>
              <td>
                <div class="action-btns">
                  <button class="btn btn-sm" @click="router.push({ name: 'my-reader', params: { gameId: w.game_id } })">预览</button>
                  <button class="btn btn-sm" @click="downloadMyWriteup(w.game_id)">下载</button>
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
    </div>
  </div>
</template>

<style scoped>
.table { min-width: 700px; }
.game-name { font-weight: 700; }
</style>
