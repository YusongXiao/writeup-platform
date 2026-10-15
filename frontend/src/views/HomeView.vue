<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useRoute } from 'vue-router'
import { getGames, type GameItem } from '@/api/game'
import GameCard from '@/components/GameCard.vue'
import UiIcon from '@/components/UiIcon.vue'

const route = useRoute()
const tab = computed(() => route.query.type === 'competition' ? 1 : 0)
const items = ref<GameItem[]>([])
const total = ref(0)
const page = ref(1)
const loading = ref(false)
const error = ref('')
const PAGE_SIZE = 12
let loadVersion = 0

async function load() {
  const version = ++loadVersion
  loading.value = true
  error.value = ''
  try {
    const res = await getGames({ page: page.value, page_size: PAGE_SIZE, host_type: tab.value })
    if (version !== loadVersion) return
    items.value = res.items
    total.value = res.total
  } catch {
    if (version === loadVersion) error.value = '加载失败，请刷新重试'
  } finally {
    if (version === loadVersion) loading.value = false
  }
}

watch(tab, () => { page.value = 1; load() }, { immediate: true })
</script>

<template>
  <div class="home-view page-shell">
    <div class="page-heading">
      <div>
        <h1 class="heading-title"><UiIcon :name="tab === 0 ? 'training' : 'flag'" :size="26" />{{ tab === 0 ? '训练场' : '赛事' }}</h1>
        <p class="heading-sub">选择{{ tab === 0 ? '训练场' : '赛事' }}，提交和管理你的解题报告</p>
      </div>
      <span v-if="!loading && !error" class="tag collection-count">共 {{ total }} 个{{ tab === 0 ? '训练场' : '赛事' }}</span>
    </div>

    <div v-if="loading" class="state-msg" role="status"><div class="spinner"></div><span>加载中…</span></div>
    <div v-else-if="error" class="state-msg error" role="alert">{{ error }}<button class="btn btn-sm" @click="load">重试</button></div>
    <div v-else-if="items.length === 0" class="empty-state panel">
      <UiIcon :name="tab === 0 ? 'training' : 'flag'" :size="48" />
      <p>暂无{{ tab === 0 ? '训练场' : '赛事' }}</p>
    </div>
    <div v-else class="game-grid"><GameCard v-for="g in items" :key="g.id" :game="g" /></div>

    <div v-if="total > PAGE_SIZE" class="pagination">
      <button class="btn btn-sm" :disabled="page === 1" @click="page--; load()">上一页</button>
      <span class="page-info">{{ page }} / {{ Math.ceil(total / PAGE_SIZE) }}</span>
      <button class="btn btn-sm" :disabled="page * PAGE_SIZE >= total" @click="page++; load()">下一页</button>
    </div>
  </div>
</template>

<style scoped>
.home-view { padding-top: 40px; }
.collection-count { margin-top: 5px; }
.game-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(min(100%, 320px), 1fr)); gap: 20px; }
.empty-state { display: flex; flex-direction: column; justify-content: center; align-items: center; gap: 20px; min-height: 300px; color: var(--text-tertiary); }
</style>
