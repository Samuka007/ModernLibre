import { NextResponse } from 'next/server'

export async function POST(request: Request) {
  try {
    const { code, state } = await request.json()
    
    if (!code || !state) {
      return NextResponse.json(
        { success: false, error: 'Missing code or state' },
        { status: 400 }
      )
    }

    const params = new URLSearchParams({
      grant_type: 'authorization_code',
      client_id: process.env.NEXT_PUBLIC_CASDOOR_CLIENT_ID || '',
      client_secret: process.env.CASDOOR_CLIENT_SECRET || '',
      code: code,
    })

    const response = await fetch(
      `${process.env.NEXT_PUBLIC_CASDOOR_SERVER_URL}/api/login/oauth/access_token?${params}`,
      {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
      }
    )

    const data = await response.json()

    if (data.access_token) {
      return NextResponse.json({
        success: true,
        accessToken: data.access_token,
      })
    } else {
      throw new Error('Failed to get access token')
    }
  } catch (error) {
    console.error('Signin error:', error)
    return NextResponse.json(
      { success: false, error: 'Authentication failed' },
      { status: 500 }
    )
  }
}