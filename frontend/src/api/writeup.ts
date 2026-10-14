import api from './index'
import { useAuthStore } from '@/stores/auth'

export interface WriteupRecord {
  id: number
  user_id: number
  user_account: string
  user_nickname: string
  team_id?: number
  team_name?: string
  game_id: number
  game_name: string
  file_name: string
  file_size: number
  file_type: string
  submitted_at: string
  updated_at: string
}

export interface WriteupListResponse {
  items: WriteupRecord[]
  total: number
}

export function submitWriteup(gameId: number, file: File): Promise<WriteupRecord> {
  const form = new FormData()
  form.append('file', file)
  return api.post(`/writeup/${gameId}`, form, {
    headers: { 'Content-Type': 'multipart/form-data' },
  }).then(r => r.data)
}

export function getMyWriteups(params?: { page?: number; page_size?: number }): Promise<WriteupListResponse> {
  return api.get('/writeup', { params }).then(r => r.data)
}

export function getMyWriteupForGame(gameId: number): Promise<WriteupRecord | null> {
  return api.get(`/writeup/${gameId}`).then(r => r.data).catch(e => {
    if (e.response?.status === 404) return null
    throw e
  })
}

async function fetchAndDownload(url: string, filename?: string) {
  const auth = useAuthStore()
  const resp = await fetch(url, {
    headers: auth.token ? { Authorization: `Bearer ${auth.token}` } : {},
  })
  if (!resp.ok) throw new Error(`Download failed: ${resp.status}`)
  const blob = await resp.blob()
  const a = document.createElement('a')
  a.href = URL.createObjectURL(blob)
  if (filename) {
    a.download = filename
  } else {
    const cd = resp.headers.get('content-disposition') ?? ''
    const star = cd.match(/filename\*=UTF-8''([^;]+)/i)
    const plain = cd.match(/filename="([^"]+)"/)
    a.download = star ? decodeURIComponent(star[1]!) : (plain?.[1] ?? 'download')
  }
  a.click()
  URL.revokeObjectURL(a.href)
}

export function downloadMyWriteup(gameId: number): Promise<void> {
  return fetchAndDownload(`/api/writeup/${gameId}/file`)
}

export function getGameWriteups(gameId: number, params?: { page?: number; page_size?: number }): Promise<WriteupListResponse> {
  return api.get(`/admin/game/${gameId}/writeup`, { params }).then(r => r.data)
}

export function downloadGameWriteups(gameId: number): Promise<void> {
  return fetchAndDownload(`/api/admin/game/${gameId}/writeup/download`, `writeups-${gameId}.zip`)
}

export function downloadUserWriteup(gameId: number, userId: number): Promise<void> {
  return fetchAndDownload(`/api/admin/game/${gameId}/writeup/${userId}/file`)
}

export function deleteUserWriteup(gameId: number, userId: number): Promise<void> {
  return api.delete(`/admin/game/${gameId}/writeup/${userId}`).then(() => undefined)
}
