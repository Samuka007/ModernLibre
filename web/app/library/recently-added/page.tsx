'use client'

import { LibraryLayout } from "@/components/library-layout"
import { BookGrid } from "@/components/book-grid"
import { Clock } from 'lucide-react'

export default function RecentlyAddedPage() {
  return (
    <LibraryLayout 
      title="Recently Added" 
      icon={Clock}
      searchPlaceholder="Search recently added books..."
    >
      <BookGrid />
    </LibraryLayout>
  )
}