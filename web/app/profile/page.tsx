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

  const initials = user.name
    ? user.name.split(' ').map(n => n[0]).join('').toUpperCase()
    : user.login.substring(0, 2).toUpperCase()

  return (
    <div className="min-h-screen bg-background flex flex-col">
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

      <main className="flex-grow flex justify-center items-center py-8">
        <Card className="max-w-2xl w-full">
          <CardHeader>
            <div className="flex items-center gap-4">
              <Avatar className="h-20 w-20">
                <AvatarImage src={user.avatar} alt={user.name || user.login} />
                <AvatarFallback className="text-2xl">{initials}</AvatarFallback>
              </Avatar>
              <div>
                <CardTitle className="text-2xl">{user.name || user.login}</CardTitle>
                <p className="text-muted-foreground">Member since {new Date().getFullYear()}</p>
              </div>
            </div>
          </CardHeader>
          <CardContent className="space-y-6">
            <div className="space-y-2">
              <Label>Display Name</Label>
              <Input value={user.name || user.login} readOnly />
            </div>
            <div className="space-y-2">
              <Label>Email</Label>
              <Input value={user.email} readOnly />
            </div>
          </CardContent>
        </Card>
      </main>
    </div>
  )
}