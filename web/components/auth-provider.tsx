'use client'

import { createContext, useContext, useState, useEffect } from 'react'
import { useRouter } from 'next/navigation'
import { getSigninUrl } from '@/lib/casdoor'
import { toast } from 'sonner'

interface User {
  name: string
  displayName: string
  avatar: string
  email: string
  phone: string
  [key: string]: any
}

interface AuthContextType {
  user: User | null
  login: () => void
  logout: () => void
  loading: boolean
}

const AuthContext = createContext<AuthContextType | undefined>(undefined)

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [user, setUser] = useState<User | null>(null)
  const [loading, setLoading] = useState(true)
  const router = useRouter()

  useEffect(() => {
    const checkAuth = () => {
      const storedUser = localStorage.getItem('casdoorUser')
      if (storedUser) {
        setUser(JSON.parse(storedUser))
        document.cookie = 'auth=true; path=/'
      }
      setLoading(false)
    }

    checkAuth()
    window.addEventListener('storage', checkAuth)
    return () => window.removeEventListener('storage', checkAuth)
  }, [])

  const login = () => {
    const signinUrl = getSigninUrl()
    if (signinUrl) {
      toast.info('Redirecting to Casdoor login...')
      console.log('Redirecting to:', signinUrl)
      window.location.href = signinUrl
    } else {
      toast.error('Failed to get signin URL')
      console.error('Failed to get signin URL')
    }
  }

  const logout = () => {
    localStorage.removeItem('casdoorUser')
    localStorage.removeItem('casdoorState')
    document.cookie = 'auth=; path=/; expires=Thu, 01 Jan 1970 00:00:01 GMT'
    setUser(null)
    router.push('/')
    toast.success('Logged out successfully')
  }

  return (
    <AuthContext.Provider value={{ user, login, logout, loading }}>
      {children}
    </AuthContext.Provider>
  )
}

export const useAuth = () => {
  const context = useContext(AuthContext)
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider')
  }
  return context
}