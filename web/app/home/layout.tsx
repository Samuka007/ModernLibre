import { ModeToggle } from "@/components/mode-toggle"
import { UserMenu } from "@/components/user-menu"
import Link from 'next/link'

export default function HomeLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <div className="min-h-screen bg-background">
      <header className="border-b">
        <div className="container mx-auto px-6 flex items-center justify-between h-16">
          {/* 左侧：Logo */}
          <Link href="/home" className="flex-shrink-0">
            <h1 className="text-2xl font-bold">ModernLibre</h1>
          </Link>

          {/* 右侧：ModeToggle 和 UserMenu */}
          <div className="flex items-center gap-4">
            <ModeToggle />
            <UserMenu />
          </div>
        </div>
      </header>
      <main className="container mx-auto px-6 py-6">
        {children}
      </main>
    </div>
  )
}