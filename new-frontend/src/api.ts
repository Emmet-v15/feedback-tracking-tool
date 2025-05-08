const API_BASE = '/api'

async function request(path: string, options: RequestInit = {}) {
  const token = localStorage.getItem('token')
  const headers = new Headers(options.headers as HeadersInit)
  headers.set('Content-Type', 'application/json')
  if (token) headers.set('Authorization', `Bearer ${token}`)

  const res = await fetch(`${API_BASE}${path}`, { ...options, headers })
  if (res.status === 401) {
    localStorage.removeItem('token')
    window.location.href = '/login'
    return Promise.reject(new Error('Unauthorized'))
  }
  const text = await res.text()
  let data: any = null
  try {
    data = text ? JSON.parse(text) : null
  } catch {
    data = text
  }
  if (!res.ok) {
    const err = new Error(data?.message || res.statusText)
    return Promise.reject(err)
  }
  return data
}

export const api = {
  get: <T = any>(path: string): Promise<T> => request(path, { method: 'GET' }) as Promise<T>,
  post: <T = any>(path: string, body: any): Promise<T> => request(path, { method: 'POST', body: JSON.stringify(body) }) as Promise<T>,
  put: <T = any>(path: string, body: any): Promise<T> => request(path, { method: 'PUT', body: JSON.stringify(body) }) as Promise<T>,
  del: (path: string) => request(path, { method: 'DELETE' }) as Promise<void>,
}
