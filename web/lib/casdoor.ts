'use client'

import Sdk from 'casdoor-js-sdk'

let casdoorSDK: Sdk | null = null

export function initCasdoorSDK() {
  if (typeof window === 'undefined') return null
  
  if (!casdoorSDK) {
    const sdkConfig = {
      serverUrl: process.env.NEXT_PUBLIC_CASDOOR_SERVER_URL || '',
      clientId: process.env.NEXT_PUBLIC_CASDOOR_CLIENT_ID || '',
      appName: process.env.NEXT_PUBLIC_CASDOOR_APP_NAME || '',
      organizationName: process.env.NEXT_PUBLIC_CASDOOR_ORG_NAME || '',
      redirectPath: '/callback',
      signinPath: '/api/signin',
    }

    console.log('Initializing Casdoor SDK with config:', {
      serverUrl: sdkConfig.serverUrl,
      clientId: sdkConfig.clientId,
      appName: sdkConfig.appName,
      organizationName: sdkConfig.organizationName,
    })

    try {
      casdoorSDK = new Sdk(sdkConfig)
    } catch (error) {
      console.error('Failed to initialize Casdoor SDK:', error)
      return null
    }
  }

  return casdoorSDK
}

export function getSigninUrl() {
  const sdk = initCasdoorSDK()
  if (!sdk) return ''

  try {
    const state = Math.random().toString(36).substring(7)
    localStorage.setItem('casdoorState', state)
    const signinUrl = sdk.getSigninUrl()
    console.log('Generated signin URL:', signinUrl)
    return signinUrl
  } catch (error) {
    console.error('Failed to get signin URL:', error)
    return ''
  }
}