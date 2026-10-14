import { defineStore } from 'pinia'
import { ref, watch } from 'vue'

export type ThemeId = 'apple-light' | 'apple-dark' | 'cyberpunk' | 'frutiger-aero'

export interface ThemeMeta {
  id: ThemeId
  label: string
  icon: string
}

export const THEMES: ThemeMeta[] = [
  { id: 'apple-light',    label: 'Apple Light',    icon: '☀️' },
  { id: 'apple-dark',     label: 'Apple Dark',     icon: '🌙' },
  { id: 'cyberpunk',      label: 'Cyberpunk',      icon: '⚡' },
  { id: 'frutiger-aero',  label: 'Frutiger Aero',  icon: '🌊' },
]

function systemDefault(): ThemeId {
  return window.matchMedia('(prefers-color-scheme: dark)').matches
    ? 'apple-dark'
    : 'apple-light'
}

export const useThemeStore = defineStore('theme', () => {
  const saved = localStorage.getItem('theme') as ThemeId | null
  const current = ref<ThemeId>(saved ?? systemDefault())

  function apply(id: ThemeId) {
    document.documentElement.setAttribute('data-theme', id)
  }

  function set(id: ThemeId) {
    current.value = id
    localStorage.setItem('theme', id)
    apply(id)
  }

  // Apply on init
  apply(current.value)

  watch(current, apply)

  return { current, set, themes: THEMES }
})
