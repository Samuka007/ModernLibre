'use client'

import { useState, useEffect } from 'react'
import { fetchBooks, prefetchBook } from '@/lib/api'
import type { Book } from './use-book'

export function useBooks() {
  const [books, setBooks] = useState<Book[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<Error | null>(null)

  useEffect(() => {
    let mounted = true;

    async function getBooks() {
      try {
        const data = await fetchBooks()
        if (mounted) {
          setBooks(Array.isArray(data) ? data : [])
          // Prefetch first few books for faster detail view
          data.slice(0, 5).forEach((book: { id: number }) => {
            prefetchBook(book.id)
          })
        }
      } catch (err) {
        if (mounted) {
          setError(err instanceof Error ? err : new Error('Failed to fetch books'))
        }
      } finally {
        if (mounted) {
          setLoading(false)
        }
      }
    }

    getBooks()

    return () => {
      mounted = false
    }
  }, [])

  return { books, loading, error }
}