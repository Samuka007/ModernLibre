'use client'

import { LibraryLayout } from "@/components/library-layout"
import { BookGrid } from "@/components/book-grid"
import { Star } from 'lucide-react'

export default function TopRatedPage() {
  return (
    <LibraryLayout 
      title="Top Rated Books" 
      icon={Star}
      searchPlaceholder="Search top rated books..."
    >
      <BookGrid />
    </LibraryLayout>
  )
}