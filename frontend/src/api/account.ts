import api from './index'

export async function loginWithCode(code: string): Promise<void> {
  await api.post('/account/login', { code: parseInt(code.trim(), 16) })
}
