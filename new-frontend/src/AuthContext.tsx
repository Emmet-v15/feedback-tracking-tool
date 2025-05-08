// @refresh reset
import { createContext, useState, useEffect } from 'react'
import type { ReactNode } from 'react'
import type { User } from './types'

interface AuthContextType {
  token: string | null
  user: User | null
  login: (token: string) => void
  logout: () => void
}

export const AuthContext = createContext<AuthContextType>({
  token: null,
  user: null,
  login: () => {},
  logout: () => {}
})

export const AuthProvider = ({ children }: { children: ReactNode }) => {
  const [token, setToken] = useState<string | null>(localStorage.getItem('token'))
  const [user, setUser] = useState<User | null>(null)

  useEffect(() => {
    if (token) {
      localStorage.setItem('token', token)
      try {
        const payload = JSON.parse(atob(token.split('.')[1]))
        setUser(payload)
      } catch {
        setUser(null)
      }
    } else {
      localStorage.removeItem('token')
      setUser(null)
    }
  }, [token])

  const login = (newToken: string) => setToken(newToken)
  const logout = () => setToken(null)

  return (
    <AuthContext.Provider value={{ token, user, login, logout }}>
      {children}
    </AuthContext.Provider>
  )
}

export type { User, AuthContextType };
