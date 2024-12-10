'use client'

import { useState } from 'react'
import { useAuth } from '@/components/auth-provider'
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { Library, ArrowLeft } from 'lucide-react'
import Link from 'next/link'

export default function LoginPage() {
  const { github, casdoor } = useAuth()

  return (
    <div className="min-h-screen flex items-center justify-center bg-background relative">
      <Link
        href="/library"
        className="absolute top-4 left-4 flex items-center gap-2 text-muted-foreground hover:text-foreground transition-colors"
      >
        <ArrowLeft className="h-4 w-4" />
        Back
      </Link>
      <Card className="w-[400px]">
        <CardHeader className="text-center">
          <div className="flex justify-center mb-2">
            <Library className="h-8 w-8" />
          </div>
          <CardTitle>Welcome back</CardTitle>
          <CardDescription>Choose your preferred login method</CardDescription>
        </CardHeader>
        <CardContent>
          <Button
            className="w-full"
            onClick={casdoor}
            variant="outline"
          >
            Sign in with Casdoor
          </Button>
          <Button
            className="w-full mt-2"
            onClick={github}
            variant="outline"
          >
            Sign in with GitHub
          </Button>
        </CardContent>
      </Card>
    </div>
  )
}