'use client'

import { createContext, useContext, useState, useEffect } from 'react'
import { useRouter } from 'next/navigation'
import { toast } from 'sonner'

interface User {
  uid: string
  login: string
  name: string
  avatar: string
  email: string
  created_at: string
  admin: boolean
  github_id: string
  casdoor_id: string
}

interface AuthContextType {
  user: User | null
  github: () => void
  casdoor: () => void
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
      const storedUser = localStorage.getItem('user')
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

  const github = () => {
    window.location.href = `${process.env.NEXT_PUBLIC_LIBRE_USER_BACKEND_URL}/oauth/github`
  }

  const casdoor = () => {
    window.location.href = `${process.env.NEXT_PUBLIC_LIBRE_USER_BACKEND_URL}/oauth/casdoor`
  }

  const logout = () => {
    localStorage.removeItem('user')
    document.cookie = 'auth=; path=/; expires=Thu, 01 Jan 1970 00:00:01 GMT'
    setUser(null)
    router.push('/')
    toast.success('Logged out successfully')
  }

  return (
    <AuthContext.Provider value={{ user, github, casdoor, logout, loading }}>
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