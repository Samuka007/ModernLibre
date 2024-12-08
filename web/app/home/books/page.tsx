'use client'

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { BookList } from "@/components/book-list"
import { Library, ArrowLeft } from 'lucide-react'
import Link from 'next/link'

export default function BooksPage() {
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
        <h2 className="text-2xl font-bold">Books Management</h2>
      </div>

      <div className="space-y-4">
        <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
          <Card>
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
              <CardTitle className="text-sm font-medium">Total Books</CardTitle>
              <Library className="h-4 w-4 text-muted-foreground" />
            </CardHeader>
            <CardContent>
              <div className="text-2xl font-bold">1,234</div>
            </CardContent>
          </Card>
        </div>
        <BookList />
      </div>
    </>
  )
}