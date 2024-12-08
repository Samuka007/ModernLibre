'use client'

import Link from 'next/link'
import { usePathname } from 'next/navigation'
import { cn } from "@/lib/utils"
import { Button } from "@/components/ui/button"
import { ScrollArea } from "@/components/ui/scroll-area"
import { 
  Library, 
  Download, 
  Star, 
  Clock,
  Heart,
  BookMarked,
  Tags
} from 'lucide-react'

const libraryLinks = [
  {
    title: "All Books",
    href: "/library",
    icon: Library
  },
  {
    title: "Downloaded",
    href: "/library/downloaded",
    icon: Download
  },
  {
    title: "Top Rated",
    href: "/library/top-rated",
    icon: Star
  },
  {
    title: "Recently Added",
    href: "/library/recently-added",
    icon: Clock
  },
  {
    title: "Favorites",
    href: "/library/favorites",
    icon: Heart
  },
  {
    title: "Reading List",
    href: "/library/reading-list",
    icon: BookMarked
  }
]

const categories = [
  "Fiction",
  "Non-Fiction",
  "Science",
  "Technology",
  "History",
  "Biography",
  "Self-Help",
  "Business",
  "Art & Design",
  "Cooking"
]

interface SidebarProps extends React.HTMLAttributes<HTMLDivElement> {}

export function Sidebar({ className }: SidebarProps) {
  const pathname = usePathname()

  return (
    <div className={cn("pb-12", className)}>
      <div className="space-y-4 py-4">
        <div className="px-3 py-2">
          <h2 className="mb-2 px-4 text-lg font-semibold">Library</h2>
          <div className="space-y-1">
            {libraryLinks.map((link) => {
              const Icon = link.icon
              return (
                <Button
                  key={link.href}
                  variant={pathname === link.href ? 'secondary' : 'ghost'}
                  className="w-full justify-start gap-2"
                  asChild
                >
                  <Link href={link.href}>
                    <Icon className="h-4 w-4" />
                    {link.title}
                  </Link>
                </Button>
              )
            })}
          </div>
        </div>
        <div className="px-3 py-2">
          <h2 className="mb-2 px-4 text-lg font-semibold tracking-tight">
            <div className="flex items-center gap-2">
              <Tags className="h-4 w-4" />
              Categories
            </div>
          </h2>
          <ScrollArea className="h-[300px] px-1">
            <div className="space-y-1">
              {categories.map((category) => (
                <Button
                  key={category}
                  variant={pathname === `/categories/${category.toLowerCase().replace(' ', '-')}` ? 'secondary' : 'ghost'}
                  className="w-full justify-start font-normal"
                  asChild
                >
                  <Link href={`/categories/${category.toLowerCase().replace(' ', '-')}`}>
                    {category}
                  </Link>
                </Button>
              ))}
            </div>
          </ScrollArea>
        </div>
      </div>
    </div>
  )
}