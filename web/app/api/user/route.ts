import { NextResponse } from 'next/server'
import { headers } from 'next/headers'

export async function GET(request: Request) {
  try {
    const headersList = headers()
    const access_token = (await headersList).get('authorization')?.split(' ')[1]

    if (!access_token) {
      return NextResponse.json(
        { error: 'No token provided' },
        { status: 401 }
      )
    }

    const response = await fetch(`${process.env.NEXT_PUBLIC_CASDOOR_SERVER_URL}/api/userinfo`, {
      headers: {
        'Authorization': `Bearer ${access_token}`
      }
    })

    const data = await response.json()
    return NextResponse.json(data)
  } catch (error) {
    console.error('User info error:', error)
    return NextResponse.json(
      { error: 'Failed to get user info' },
      { status: 500 }
    )
  }
}