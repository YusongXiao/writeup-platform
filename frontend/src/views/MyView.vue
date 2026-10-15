<script setup lang="ts">
import { t, dateLocale } from '@/i18n'
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
  return new Date(s).toLocaleString(dateLocale)
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
          <h1 class="heading-title"><UiIcon name="file" :size="26" />{{ t('common.mine') }}</h1>
          <p class="heading-sub">{{ t('mine.summary', { count: total }) }}</p>
        </div>
      </div>

      <div v-if="loading" class="state-msg">
        <div class="spinner"></div> {{ t('common.loading') }}
      </div>

      <div v-else-if="items.length === 0" class="state-msg empty">
        {{ t('mine.empty') }}
      </div>

      <div v-else class="table-wrap panel">
        <table class="table">
          <thead>
            <tr>
              <th>{{ t('common.competition') }}</th>
              <th>{{ t('common.filename') }}</th>
              <th>{{ t('common.size') }}</th>
              <th>{{ t('common.type') }}</th>
              <th>{{ t('common.submittedAt') }}</th>
              <th>{{ t('common.actions') }}</th>
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
                  <button class="btn btn-sm" @click="router.push({ name: 'my-reader', params: { gameId: w.game_id } })">{{ t('common.preview') }}</button>
                  <button class="btn btn-sm" @click="downloadMyWriteup(w.game_id)">{{ t('common.download') }}</button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>

      <div v-if="total > PAGE_SIZE" class="pagination">
        <button class="btn btn-sm" :disabled="page === 1" @click="page--; load()">{{ t('common.previous') }}</button>
        <span class="page-info">{{ page }} / {{ Math.ceil(total / PAGE_SIZE) }}</span>
        <button class="btn btn-sm" :disabled="page * PAGE_SIZE >= total" @click="page++; load()">{{ t('common.next') }}</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.table { min-width: 700px; }
.game-name { font-weight: 700; }
</style>
