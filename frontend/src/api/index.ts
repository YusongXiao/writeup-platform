import axios from 'axios'
import { useAuthStore } from '@/stores/auth'

const api = axios.create({
  baseURL: '/api',
  timeout: 30000,
})

api.interceptors.request.use((config) => {
  const auth = useAuthStore()
  if (auth.token) {
    config.headers.Authorization = `Bearer ${auth.token}`
  }
  return config
})

api.interceptors.response.use(
  (response) => {
    // Forward Set-Token header from backend
    const newToken = response.headers['set-token']
    if (newToken) {
      const auth = useAuthStore()
      auth.setToken(newToken)
    }
    return response
  },
  (error) => {
    if (error.response?.status === 401) {
      const auth = useAuthStore()
      auth.clearToken()
    }
    return Promise.reject(error)
  }
)

export default api
