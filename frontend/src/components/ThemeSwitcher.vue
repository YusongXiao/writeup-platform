<script setup lang="ts">
import { ref } from 'vue'
import { useThemeStore, type ThemePreference } from '@/stores/theme'
import UiIcon from './UiIcon.vue'

const theme = useThemeStore()
const popover = ref<HTMLDetailsElement>()
const options = [
  { value: 'light', label: '浅色', icon: 'sun' },
  { value: 'dark', label: '深色', icon: 'moon' },
  { value: 'system', label: '跟随系统', icon: 'monitor' },
] as const

function select(value: ThemePreference) {
  theme.set(value)
  if (popover.value) popover.value.open = false
}
</script>

<template>
  <details ref="popover" class="theme-switcher" @keydown.esc="popover && (popover.open = false)">
    <summary class="btn btn-ghost btn-square" title="外观设置" aria-label="外观设置"><UiIcon :name="theme.isDark ? 'moon' : 'sun'" /></summary>
    <div class="theme-menu panel" role="group" aria-label="外观模式">
      <button v-for="option in options" :key="option.value" class="btn btn-ghost btn-sm theme-option" :class="{ active: theme.preference === option.value }" :aria-pressed="theme.preference === option.value" @click="select(option.value)">
        <UiIcon :name="option.icon" :size="18" /><span>{{ option.label }}</span><UiIcon v-if="theme.preference === option.value" name="check" :size="16" />
      </button>
    </div>
  </details>
</template>

<style scoped>
.theme-switcher { position: relative; }
summary { list-style: none; }
summary::-webkit-details-marker { display: none; }
.theme-menu { position: absolute; top: calc(100% + 10px); right: 0; z-index: 110; width: 176px; padding: 8px; background: var(--bg-elevated); }
.theme-option { width: 100%; justify-content: flex-start; margin: 2px 0; font-weight: 400; }
.theme-option > span { flex: 1; text-align: left; }
.theme-option.active { background: var(--overlay); }
</style>
