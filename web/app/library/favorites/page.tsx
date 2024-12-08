'use client'

import { LibraryLayout } from "@/components/library-layout"
import { BookGrid } from "@/components/book-grid"
import { Heart } from 'lucide-react'

export default function FavoritesPage() {
  return (
    <LibraryLayout 
      title="Favorite Books" 
      icon={Heart}
      searchPlaceholder="Search favorite books..."
    >
      <BookGrid />
    </LibraryLayout>
  )
}