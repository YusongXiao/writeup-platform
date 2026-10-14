<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { getGames, type GameItem } from '@/api/game'
import GameCard from '@/components/GameCard.vue'

const tab = ref<0 | 1>(0)
const items = ref<GameItem[]>([])
const total = ref(0)
const page = ref(1)
const loading = ref(false)
const error = ref('')
const PAGE_SIZE = 12

async function load() {
  loading.value = true
  error.value = ''
  try {
    const res = await getGames({ page: page.value, page_size: PAGE_SIZE, host_type: tab.value })
    items.value = res.items
    total.value = res.total
  } catch {
    error.value = '加载失败，请刷新重试'
  } finally {
    loading.value = false
  }
}

function switchTab(t: 0 | 1) {
  tab.value = t
  page.value = 1
  load()
}

onMounted(load)
</script>

<template>
  <div class="home-view">
    <div class="hero">
      <h1 class="hero-title">Writeup 提交平台</h1>
      <p class="hero-sub">提交你的解题报告，分享你的思路</p>
    </div>

    <div class="tabs">
      <button class="tab" :class="{ active: tab === 0 }" @click="switchTab(0)">训练场</button>
      <button class="tab" :class="{ active: tab === 1 }" @click="switchTab(1)">赛事</button>
    </div>

    <div v-if="loading" class="state-msg">
      <div class="spinner"></div>
      <span>加载中…</span>
    </div>

    <div v-else-if="error" class="state-msg error">{{ error }}</div>

    <div v-else-if="items.length === 0" class="state-msg empty">
      暂无{{ tab === 0 ? '训练场' : '赛事' }}
    </div>

    <div v-else class="game-grid">
      <GameCard v-for="g in items" :key="g.id" :game="g" />
    </div>

    <div v-if="total > PAGE_SIZE" class="pagination">
      <button class="page-btn" :disabled="page === 1" @click="page--; load()">上一页</button>
      <span class="page-info">{{ page }} / {{ Math.ceil(total / PAGE_SIZE) }}</span>
      <button class="page-btn" :disabled="page * PAGE_SIZE >= total" @click="page++; load()">下一页</button>
    </div>
  </div>
</template>

<style scoped>
.home-view {
  max-width: var(--content-max);
  margin: 0 auto;
  padding: 40px 24px 80px;
}

.hero {
  text-align: center;
  padding: 48px 0 40px;
}

.hero-title {
  font-family: var(--font-display);
  font-size: clamp(28px, 5vw, 48px);
  font-weight: 700;
  color: var(--text-primary);
  letter-spacing: -0.02em;
  line-height: 1.1;
}

.hero-sub {
  margin-top: 12px;
  font-size: 16px;
  color: var(--text-secondary);
}

.tabs {
  display: flex;
  gap: 4px;
  padding: 4px;
  background: var(--overlay);
  border: 1px solid var(--border);
  border-radius: var(--radius-pill);
  width: fit-content;
  margin: 0 auto 32px;
}

.tab {
  padding: 8px 24px;
  border-radius: var(--radius-pill);
  font-size: 14px;
  font-weight: 500;
  font-family: var(--font-sans);
  color: var(--text-secondary);
  background: transparent;
  border: none;
  cursor: pointer;
  transition: all var(--duration-fast) var(--ease-out);
}

.tab:hover { color: var(--text-primary); }

.tab.active {
  background: var(--accent);
  color: var(--text-on-accent);
  box-shadow: var(--shadow-sm);
}

.game-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 16px;
}

.state-msg {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;
  padding: 60px 0;
  color: var(--text-secondary);
  font-size: 15px;
}

.state-msg.error { color: var(--danger); }
.state-msg.empty { color: var(--text-tertiary); }

.spinner {
  width: 20px;
  height: 20px;
  border: 2px solid var(--border);
  border-top-color: var(--accent);
  border-radius: 50%;
  animation: spin 0.7s linear infinite;
}

@keyframes spin { to { transform: rotate(360deg); } }

.pagination {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 16px;
  margin-top: 40px;
}

.page-btn {
  padding: 8px 20px;
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
  background: var(--surface-hover);
  border-color: var(--accent);
  color: var(--accent);
}

.page-btn:disabled { opacity: 0.4; cursor: not-allowed; }

.page-info { font-size: 13px; color: var(--text-secondary); }

[data-theme="cyberpunk"] .hero-title {
  text-shadow: 0 0 20px var(--accent-glow), 0 0 40px var(--accent-glow);
  letter-spacing: 0.05em;
}

[data-theme="frutiger-aero"] .hero-title {
  background: linear-gradient(135deg, var(--text-primary), var(--accent));
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}
</style>
