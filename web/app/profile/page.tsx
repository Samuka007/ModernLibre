'use client'

import { useAuth } from '@/components/auth-provider'
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar"
import Link from 'next/link'
import { ArrowLeft } from 'lucide-react'

export default function ProfilePage() {
  const { user } = useAuth()

  if (!user) {
    return null
  }

  const initials = user.displayName
    ? user.displayName.split(' ').map(n => n[0]).join('').toUpperCase()
    : user.name.substring(0, 2).toUpperCase()

  return (
    <div className="min-h-screen bg-background">
      <header className="border-b">
        <div className="container flex items-center h-16 gap-4">
          <Link 
            href="/home" 
            className="flex items-center gap-2 text-muted-foreground hover:text-foreground transition-colors"
          >
            <ArrowLeft className="h-4 w-4" />
            Back to home
          </Link>
        </div>
      </header>

      <main className="container py-8 max-w-2xl">
        <Card>
          <CardHeader>
            <div className="flex items-center gap-4">
              <Avatar className="h-20 w-20">
                <AvatarImage src={user.avatar} alt={user.displayName || user.name} />
                <AvatarFallback className="text-2xl">{initials}</AvatarFallback>
              </Avatar>
              <div>
                <CardTitle className="text-2xl">{user.displayName || user.name}</CardTitle>
                <p className="text-muted-foreground">Member since {new Date().getFullYear()}</p>
              </div>
            </div>
          </CardHeader>
          <CardContent className="space-y-6">
            <div className="space-y-2">
              <Label>Display Name</Label>
              <Input value={user.displayName || user.name} readOnly />
            </div>
            <div className="space-y-2">
              <Label>Email</Label>
              <Input value={user.email} readOnly />
            </div>
            {user.phone && (
              <div className="space-y-2">
                <Label>Phone</Label>
                <Input value={user.phone} readOnly />
              </div>
            )}
            <div className="pt-4">
              <Button asChild>
                <Link href="https://door.casdoor.com/account" target="_blank">
                  Edit Profile on Casdoor
                </Link>
              </Button>
            </div>
          </CardContent>
        </Card>
      </main>
    </div>
  )
}