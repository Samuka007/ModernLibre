'use client'

import { useState } from 'react'
import { Sidebar } from "@/components/sidebar"
import { Input } from "@/components/ui/input"
import { ModeToggle } from "@/components/mode-toggle"
import { UserMenu } from "@/components/user-menu"
import { Search } from 'lucide-react'
import Link from 'next/link'
import { LucideIcon } from 'lucide-react'
import { useAuth } from '@/components/auth-provider'

interface LibraryLayoutProps {
  children: React.ReactNode
  title: string
  icon: LucideIcon
  searchPlaceholder?: string
}

export function LibraryLayout({ 
  children, 
  title,
  icon: Icon,
  searchPlaceholder = "Search books..." 
}: LibraryLayoutProps) {
  const [searchTerm, setSearchTerm] = useState('')
  const { user } = useAuth()

  return (
    <div className="min-h-screen bg-background">
      <header className="border-b">
        <div className="container mx-auto px-6 flex items-center justify-between h-16">
          {/* 左侧：Logo */}
          <Link href={user ? "/home" : "/"} className="flex-shrink-0">
            <h1 className="text-2xl font-bold">ModernLibre</h1>
          </Link>

          {/* 中间：搜索栏 */}
          <div className="flex-1 max-w-xl mx-8">
            <div className="relative">
              <Search className="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
              <Input
                placeholder={searchPlaceholder}
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
                className="pl-8"
              />
            </div>
          </div>

          {/* 右侧：用户菜单和模式切换 */}
          <div className="flex items-center gap-4">
            <ModeToggle />
            <UserMenu />
          </div>
        </div>
      </header>

      <div className="container flex pt-6 gap-6 mx-auto px-6">
        {/* 左侧：Sidebar */}
        <Sidebar className="w-64 flex-shrink-0" />

        {/* 主内容区域 */}
        <main className="flex-1">
          {/* 标题和图标 */}
          <div className="flex items-center justify-between mb-6">
            <h2 className="text-2xl font-bold flex items-center gap-2">
              <Icon className="h-6 w-6" />
              {title}
            </h2>
          </div>

          {/* 子内容 */}
          {children}
        </main>
      </div>
    </div>
  )
}