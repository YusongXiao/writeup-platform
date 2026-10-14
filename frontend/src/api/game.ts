import api from './index'

export interface GameItem {
  id: number
  name: string
  brief: string
  host_type: number   // 0=training, 1=competition
  start_at: string | null
  end_at: string | null
  archive_at: string | null
  can_submit: boolean
  admins?: number[]
}

export interface GamesResponse {
  items: GameItem[]
  total: number
}

export function getGames(params?: {
  page?: number
  page_size?: number
  host_type?: number
}): Promise<GamesResponse> {
  return api.get('/games', { params }).then(r => r.data)
}
