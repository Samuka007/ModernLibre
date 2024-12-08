'use client'

import { useState } from 'react'
import { BookGrid } from "@/components/book-grid"
import { Sidebar } from "@/components/sidebar"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { ModeToggle } from "@/components/mode-toggle"
import { Search } from 'lucide-react'
import Link from 'next/link'
import { useParams } from 'next/navigation'

export default function CategoryPage() {
  const [searchTerm, setSearchTerm] = useState('')
  const params = useParams()
  const category = params.slug as string

  return (
    <div className="min-h-screen bg-background">
      <header className="border-b">
        <div className="container mx-auto px-6 flex items-center justify-between h-16">
          {/* 左侧：Logo */}
          <Link href="/" className="flex-shrink-0">
            <h1 className="text-2xl font-bold">ModernLibre</h1>
          </Link>

          {/* 中间：搜索栏 */}
          <div className="flex-1 max-w-xl mx-8">
            <div className="relative">
              <Search className="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
              <Input
                placeholder="Search books..."
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
                className="pl-8"
              />
            </div>
          </div>

          {/* 右侧：模式切换和登录按钮 */}
          <div className="flex items-center gap-4">
            <ModeToggle />
            <Link href="/login">
              <Button variant="outline">Sign In</Button>
            </Link>
          </div>
        </div>
      </header>

      <div className="container mx-auto py-6 flex gap-6 px-6">
        {/* 左侧：Sidebar */}
        <Sidebar className="w-64 flex-shrink-0" />

        {/* 主内容区域 */}
        <main className="flex-1">
          <h2 className="text-2xl font-bold mb-6 capitalize">{category.replace('-', ' ')}</h2>
          <BookGrid />
        </main>
      </div>
    </div>
  )
}