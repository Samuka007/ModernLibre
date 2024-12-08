'use client'

import { useState } from 'react'
import { BookGrid } from "../../components/book-grid"
import { BookMarked, Search } from 'lucide-react'
import { LibraryLayout } from '../../components/library-layout'

export default function LibraryPage() {
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