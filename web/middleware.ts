import { NextResponse } from 'next/server'
import type { NextRequest } from 'next/server'

export function middleware(request: NextRequest) {
  const isAuthenticated = request.cookies.has('auth')
  const isProtectedRoute = request.nextUrl.pathname.startsWith('/home') || 
                          request.nextUrl.pathname.startsWith('/profile')
  const isLoginRoute = request.nextUrl.pathname === '/login'

  // Store current URL for redirect after login
  if (!isAuthenticated && !isLoginRoute) {
    const redirectTo = encodeURIComponent(request.nextUrl.pathname)
    const response = NextResponse.redirect(new URL('/login', request.url))
    response.cookies.set('redirectTo', redirectTo)
    return response
  }

  // Redirect to home if already logged in and trying to access login
  if (isLoginRoute && isAuthenticated) {
    return NextResponse.redirect(new URL('/home', request.url))
  }

  // Redirect to login if trying to access protected routes without auth
  if (isProtectedRoute && !isAuthenticated) {
    return NextResponse.redirect(new URL('/login', request.url))
  }

  return NextResponse.next()
}

export const config = {
  matcher: ['/home/:path*', '/profile', '/login']
}