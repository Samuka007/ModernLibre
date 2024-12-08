'use client'

import { useEffect, Suspense } from 'react'
import { useRouter, useSearchParams } from 'next/navigation'
import { initCasdoorSDK } from '@/lib/casdoor'
import { toast } from 'sonner'
import { revalidatePath } from 'next/cache'

function CallbackContent() {
  const router = useRouter()
  const searchParams = useSearchParams()
  
  useEffect(() => {
    const handleCallback = async () => {
      const code = searchParams.get('code')
      const state = searchParams.get('state')

      console.log('Callback received:', { code, state })

      if (!code || !state) {
        console.error('Missing code or state')
        toast.error('Authentication failed: Missing parameters')
        router.push('/login')
        return
      }

      try {
        toast.info('Authenticating...')
        const token_response = await fetch('/api/signin', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({ code, state }),
        })

        const data = await token_response.json()

        if (data.success && data.accessToken) {
          // Get user info with the access token
          const userResponse = await fetch('/api/user', {
            headers: {
              'Authorization': `Bearer ${data.accessToken}`,
            },
          })

          const userData = await userResponse.json()
          localStorage.setItem('casdoorUser', JSON.stringify(userData))
          localStorage.removeItem('casdoorState')
          // Set auth cookie on successful login
          document.cookie = 'auth=true; path=/'
          toast.success('Successfully logged in!')
          
          // Check for redirect URL
          const redirectTo = document.cookie
            .split('; ')
            .find(row => row.startsWith('redirectTo='))
            ?.split('=')[1]

          if (redirectTo) {
            // Clear redirect cookie
            document.cookie = 'redirectTo=; path=/; expires=Thu, 01 Jan 1970 00:00:01 GMT'
            router.push(decodeURIComponent(redirectTo))
            revalidatePath(redirectTo)
          } else {
            router.push('/home')
            revalidatePath('/home')
          }
        } else {
          throw new Error('Failed to get access token')
        }
      } catch (error) {
        console.error('Authentication error:', error)
        toast.error('Authentication failed')
        router.push('/login')
      }
    }

    handleCallback()
  }, [router, searchParams])

  return (
    <div className="min-h-screen flex items-center justify-center">
      <div className="text-center">
        <h2 className="text-2xl font-semibold mb-2">Authenticating...</h2>
        <p className="text-muted-foreground">Please wait while we complete the login process.</p>
      </div>
    </div>
  )
}

function CallbackPageWrapper() {
  return (
    <Suspense fallback={<div>Loading...</div>}>
      <CallbackContent />
    </Suspense>
  )
}

export default CallbackPageWrapper