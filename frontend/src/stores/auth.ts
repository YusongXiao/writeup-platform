import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export interface TokenPayload {
  id: number
  account: string
  nickname: string
  permissions: number[]
  exp: number
}

function decodeJwt(token: string): TokenPayload | null {
  try {
    const part = token.split('.')[1]
    if (!part) return null
    const b64 = part.replace(/-/g, '+').replace(/_/g, '/')
    const decoded = decodeURIComponent(
      atob(b64).split('').map(c => '%' + c.charCodeAt(0).toString(16).padStart(2, '0')).join('')
    )
    return JSON.parse(decoded)
  } catch {
    return null
  }
}

export const useAuthStore = defineStore('auth', () => {
  const token = ref<string | null>(localStorage.getItem('token'))
  const payload = ref<TokenPayload | null>(
    token.value ? decodeJwt(token.value) : null
  )

  const isLoggedIn = computed(() => {
    if (!token.value || !payload.value) return false
    return payload.value.exp * 1000 > Date.now()
  })

  const isAdmin = computed(() =>
    payload.value?.permissions.includes(5) ?? false
  )

  function setToken(t: string) {
    token.value = t
    payload.value = decodeJwt(t)
    localStorage.setItem('token', t)
  }

  function clearToken() {
    token.value = null
    payload.value = null
    localStorage.removeItem('token')
  }

  function isGameAdmin(admins: number[]): boolean {
    if (!payload.value) return false
    return isAdmin.value && admins.includes(payload.value.id)
  }

  return { token, payload, isLoggedIn, isAdmin, setToken, clearToken, isGameAdmin }
})
