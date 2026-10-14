<script setup lang="ts">
import { useThemeStore, THEMES } from '@/stores/theme'
import type { ThemeId } from '@/stores/theme'

const theme = useThemeStore()
</script>

<template>
  <div class="theme-switcher">
    <button
      v-for="t in THEMES"
      :key="t.id"
      class="theme-btn"
      :class="{ active: theme.current === t.id }"
      :title="t.label"
      @click="theme.set(t.id as ThemeId)"
    >
      <span class="icon">{{ t.icon }}</span>
    </button>
  </div>
</template>

<style scoped>
.theme-switcher {
  display: flex;
  gap: 4px;
  padding: 4px;
  background: var(--overlay);
  border: 1px solid var(--border);
  border-radius: var(--radius-pill);
}

.theme-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: var(--radius-pill);
  font-family: var(--font-sans);
  color: var(--text-secondary);
  background: transparent;
  transition: all var(--duration-fast) var(--ease-out);
}

.theme-btn:hover {
  color: var(--text-primary);
  background: var(--surface-hover);
}

.theme-btn.active {
  color: var(--text-on-accent);
  background: var(--accent);
  box-shadow: var(--shadow-sm);
}

.icon { font-size: 15px; line-height: 1; }
</style>
