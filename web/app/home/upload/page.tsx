'use client'

import { BookUpload } from "@/components/book-upload"
import { ArrowLeft } from 'lucide-react'
import Link from 'next/link'

export default function UploadPage() {
  return (
    <>
      <div className="flex items-center gap-4 mb-6">
        <Link 
          href="/home" 
          className="flex items-center gap-2 text-muted-foreground hover:text-foreground"
        >
          <ArrowLeft className="h-4 w-4" />
          Back
        </Link>
        <h2 className="text-2xl font-bold">Upload Books</h2>
      </div>
      <BookUpload />
    </>
  )
}