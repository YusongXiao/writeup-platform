<script setup lang="ts">
import type { GameItem } from '@/api/game'
import { useAuthStore } from '@/stores/auth'
import { useRouter } from 'vue-router'

const props = defineProps<{ game: GameItem }>()
const auth = useAuthStore()
const router = useRouter()

function formatDate(s: string | null): string {
  if (!s) return '—'
  return new Date(s).toLocaleDateString('zh-CN', { year: 'numeric', month: 'short', day: 'numeric' })
}

function handleSubmit() {
  if (!auth.isLoggedIn) {
    router.push({ name: 'login', query: { redirect: `/submit/${props.game.id}` } })
  } else {
    router.push({ name: 'submit', params: { gameId: props.game.id } })
  }
}

function handleAdmin() {
  router.push({ name: 'admin', params: { gameId: props.game.id } })
}
</script>

<template>
  <div class="game-card" :class="{ 'can-submit': game.can_submit }">
    <div class="card-header">
      <span class="type-badge" :class="game.host_type === 0 ? 'training' : 'competition'">
        {{ game.host_type === 0 ? '训练场' : '赛事' }}
      </span>
      <span v-if="!game.can_submit" class="archived-badge">已归档</span>
    </div>

    <h3 class="game-name">{{ game.name }}</h3>
    <p class="game-brief">{{ game.brief || '暂无简介' }}</p>

    <div class="game-meta">
      <span v-if="game.start_at">{{ formatDate(game.start_at) }}</span>
      <span v-if="game.end_at"> — {{ formatDate(game.end_at) }}</span>
    </div>

    <div class="card-actions">
      <button
        v-if="game.can_submit"
        class="btn btn-primary"
        @click="handleSubmit"
      >
        提交 Writeup
      </button>
      <button
        v-if="auth.isLoggedIn && auth.isGameAdmin(game.admins ?? [])"
        class="btn btn-secondary"
        @click="handleAdmin"
      >
        管理
      </button>
    </div>
  </div>
</template>

<style scoped>
.game-card {
  background: var(--surface);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 10px;
  transition: all var(--duration-base) var(--ease-out);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
}

.game-card:hover {
  background: var(--surface-hover);
  border-color: var(--border-strong);
  box-shadow: var(--shadow-md);
  transform: translateY(-2px);
}

.card-header {
  display: flex;
  gap: 8px;
  align-items: center;
}

.type-badge {
  font-size: 11px;
  font-weight: 600;
  padding: 2px 8px;
  border-radius: var(--radius-pill);
  letter-spacing: 0.03em;
  text-transform: uppercase;
}

.type-badge.training {
  background: var(--success-muted);
  color: var(--success);
}

.type-badge.competition {
  background: var(--accent-muted);
  color: var(--accent);
}

.archived-badge {
  font-size: 11px;
  padding: 2px 8px;
  border-radius: var(--radius-pill);
  background: var(--overlay);
  color: var(--text-tertiary);
}

.game-name {
  font-family: var(--font-display);
  font-size: 17px;
  font-weight: 600;
  color: var(--text-primary);
  line-height: 1.3;
}

.game-brief {
  font-size: 13px;
  color: var(--text-secondary);
  line-height: 1.5;
  flex: 1;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.game-meta {
  font-size: 12px;
  color: var(--text-tertiary);
}

.card-actions {
  display: flex;
  gap: 8px;
  margin-top: 4px;
}

.btn {
  flex: 1;
  padding: 8px 16px;
  border-radius: var(--radius-md);
  font-size: 13px;
  font-weight: 600;
  font-family: var(--font-sans);
  transition: all var(--duration-fast) var(--ease-out);
  cursor: pointer;
  border: none;
}

.btn-primary {
  background: var(--accent);
  color: var(--text-on-accent);
}

.btn-primary:hover {
  background: var(--accent-hover);
  box-shadow: 0 4px 12px var(--accent-glow);
  transform: translateY(-1px);
}

.btn-secondary {
  background: var(--overlay);
  color: var(--text-secondary);
  border: 1px solid var(--border);
}

.btn-secondary:hover {
  background: var(--surface-hover);
  color: var(--text-primary);
}

/* Cyberpunk card style */
[data-theme="cyberpunk"] .game-card {
  border: 1px solid var(--border);
  box-shadow: var(--neon-glow);
}

[data-theme="cyberpunk"] .game-card:hover {
  border-color: var(--accent);
  box-shadow: var(--neon-glow), 0 0 30px var(--accent-glow);
}

[data-theme="cyberpunk"] .btn-primary {
  clip-path: polygon(8px 0%, 100% 0%, calc(100% - 8px) 100%, 0% 100%);
  border-radius: 0;
}

/* Frutiger Aero card style */
[data-theme="frutiger-aero"] .game-card {
  background: var(--glass-bg);
  box-shadow: var(--shadow-md);
}
</style>
