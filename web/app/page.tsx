import Link from 'next/link'
import { Button } from "@/components/ui/button"
import { ModeToggle } from "@/components/mode-toggle"
import { UserMenu } from '@/components/user-menu'
import { Card, CardContent } from "@/components/ui/card"
import { BookOpen, BookMarked, Users, Download } from 'lucide-react'

export default function HomePage() {
  return (
    <div className="min-h-screen bg-background">
      <header className="border-b">
        <div className="container mx-auto px-6 flex items-center justify-between h-16">
          {/* 左侧：Logo */}
          <Link href="/" className="flex-shrink-0">
            <h1 className="text-2xl font-bold">ModernLibre</h1>
          </Link>

          {/* 右侧：用户菜单和模式切换 */}
          <div className="flex items-center gap-4">
            <ModeToggle />
            <UserMenu />
          </div>
        </div>
      </header>

      <main className="container mx-auto py-12 space-y-12 px-6">
        <section className="max-w-3xl mx-auto text-center">
          <h2 className="text-4xl font-bold tracking-tight mb-4">Your Digital Library Experience</h2>
          <p className="text-xl text-muted-foreground mb-8">
            Discover, read, and manage your digital book collection with ease.
          </p>
          <div className="flex justify-center gap-4">
            <Link href="/library">
              <Button size="lg">Browse Library</Button>
            </Link>
            <Link href="/categories/fiction">
              <Button variant="outline" size="lg">Explore Categories</Button>
            </Link>
          </div>
        </section>

        <section className="grid md:grid-cols-2 lg:grid-cols-4 gap-6">
          <Card>
            <CardContent className="pt-6">
              <div className="flex items-center gap-4">
                <div className="p-2 bg-primary/10 rounded-lg">
                  <BookOpen className="h-6 w-6 text-primary" />
                </div>
                <div>
                  <h3 className="font-semibold">Extensive Collection</h3>
                  <p className="text-sm text-muted-foreground">Access thousands of digital books</p>
                </div>
              </div>
            </CardContent>
          </Card>
          <Card>
            <CardContent className="pt-6">
              <div className="flex items-center gap-4">
                <div className="p-2 bg-primary/10 rounded-lg">
                  <BookMarked className="h-6 w-6 text-primary" />
                </div>
                <div>
                  <h3 className="font-semibold">Easy Organization</h3>
                  <p className="text-sm text-muted-foreground">Manage your reading lists effortlessly</p>
                </div>
              </div>
            </CardContent>
          </Card>
          <Card>
            <CardContent className="pt-6">
              <div className="flex items-center gap-4">
                <div className="p-2 bg-primary/10 rounded-lg">
                  <Users className="h-6 w-6 text-primary" />
                </div>
                <div>
                  <h3 className="font-semibold">Community Features</h3>
                  <p className="text-sm text-muted-foreground">Share and discover recommendations</p>
                </div>
              </div>
            </CardContent>
          </Card>
          <Card>
            <CardContent className="pt-6">
              <div className="flex items-center gap-4">
                <div className="p-2 bg-primary/10 rounded-lg">
                  <Download className="h-6 w-6 text-primary" />
                </div>
                <div>
                  <h3 className="font-semibold">Offline Access</h3>
                  <p className="text-sm text-muted-foreground">Download books for offline reading</p>
                </div>
              </div>
            </CardContent>
          </Card>
        </section>

        <section className="text-center py-8">
          <div className="max-w-2xl mx-auto">
            <h2 className="text-2xl font-bold mb-4">Ready to Start Reading?</h2>
            <p className="text-muted-foreground mb-6">
              Join our community of book lovers and start exploring our collection today.
            </p>
            <Link href="/library">
              <Button size="lg">Get Started</Button>
            </Link>
          </div>
        </section>
      </main>
    </div>
  )
}