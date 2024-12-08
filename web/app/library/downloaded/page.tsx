'use client'

import { useState } from 'react'
import { BookGrid } from "@/components/book-grid"
import { Search, Download, BookMarked } from 'lucide-react'
import { LibraryLayout } from '@/components/library-layout'

export default function DownloadedPage() {
  const [searchTerm, setSearchTerm] = useState('')

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