import { defineStore } from 'pinia'
import { computed, onScopeDispose, ref, watchEffect } from 'vue'

export type ThemePreference = 'light' | 'dark' | 'system'

function savedPreference(): ThemePreference {
  const saved = localStorage.getItem('theme')
  if (saved === 'light' || saved === 'dark' || saved === 'system') return saved
  if (saved === 'apple-dark' || saved === 'cyberpunk' || saved === 'cyber-dark') return 'dark'
  if (saved === 'apple-light' || saved === 'frutiger-aero' || saved === 'cyber-light') return 'light'
  return 'system'
}

export const useThemeStore = defineStore('theme', () => {
  const preference = ref<ThemePreference>(savedPreference())
  const media = window.matchMedia('(prefers-color-scheme: dark)')
  const systemDark = ref(media.matches)
  const isDark = computed(() => preference.value === 'dark' || (preference.value === 'system' && systemDark.value))
  const current = computed(() => isDark.value ? 'cyber-dark' : 'cyber-light')

  function onSystemChange(event: MediaQueryListEvent) { systemDark.value = event.matches }
  media.addEventListener('change', onSystemChange)
  onScopeDispose(() => media.removeEventListener('change', onSystemChange))

  function set(value: ThemePreference) {
    preference.value = value
    localStorage.setItem('theme', value)
  }

  watchEffect(() => {
    document.documentElement.dataset.theme = current.value
    document.documentElement.dataset.style = isDark.value ? 'dark' : 'light'
  })

  return { current, preference, isDark, set }
})
