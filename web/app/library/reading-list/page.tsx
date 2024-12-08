'use client'

import { LibraryLayout } from "@/components/library-layout"
import { BookGrid } from "@/components/book-grid"
import { BookMarked } from 'lucide-react'

export default function ReadingListPage() {
  return (
    <LibraryLayout 
      title="Reading List" 
      icon={BookMarked}
      searchPlaceholder="Search reading list..."
    >
      <BookGrid />
    </LibraryLayout>
  )
}