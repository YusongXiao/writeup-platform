<script setup lang="ts">
import { useDismissibleMenu } from '@/composables/useDismissibleMenu'
import { languages, locale, setLocale, t } from '@/i18n'
import UiIcon from './UiIcon.vue'

const popover = useDismissibleMenu()
</script>

<template>
  <details ref="popover" class="language-switcher" @keydown.esc="popover && (popover.open = false)">
    <summary class="btn btn-ghost btn-square" :title="t('language.label')" :aria-label="t('language.label')"><UiIcon name="language" /></summary>
    <div class="language-menu panel" role="group" :aria-label="t('language.label')">
      <button v-for="language in languages" :key="language.value" class="btn btn-ghost btn-sm language-option" :class="{ active: locale === language.value }" :aria-pressed="locale === language.value" :lang="language.value.replace('_', '-')" @click="setLocale(language.value)">
        <span>{{ language.label }}</span><UiIcon v-if="locale === language.value" name="check" :size="16" />
      </button>
    </div>
  </details>
</template>

<style scoped>
.language-switcher { position: relative; }
summary { list-style: none; }
summary::-webkit-details-marker { display: none; }
.language-menu { position: absolute; top: calc(100% + 10px); right: 0; z-index: 110; width: 160px; padding: 8px; background: var(--bg-elevated); }
.language-option { width: 100%; justify-content: flex-start; margin: 2px 0; font-weight: 400; }
.language-option > span { flex: 1; text-align: left; }
.language-option.active { background: var(--overlay); }
</style>
