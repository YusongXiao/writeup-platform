import { onMounted, onUnmounted, ref } from 'vue'

export function useDismissibleMenu() {
  const menu = ref<HTMLDetailsElement>()

  function dismiss(event: MouseEvent) {
    if (menu.value && !menu.value.contains(event.target as Node)) {
      menu.value.open = false
    }
  }

  onMounted(() => document.addEventListener('click', dismiss, true))
  onUnmounted(() => document.removeEventListener('click', dismiss, true))

  return menu
}
