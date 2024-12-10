"use client"

import { useSearchParams } from 'next/navigation'
import React, { useEffect, useState } from 'react'
import { ReactReader } from 'react-reader'

export default function ReaderPage() {
    // get access token
    const token = localStorage.getItem('accessToken')

    // get book ID
    const searchParams = useSearchParams()
    const bookId = searchParams?.get('bookId')

    // get book url
    const [location, setLocation] = useState<string | number>(0)
    const [bookUrl, setBookUrl] = useState<string>('')
    const [isLoading, setIsLoading] = useState(true)
    const [error, setError] = useState<string | null>(null)

    useEffect(() => {
        const fetchBook = async () => {
            try {
                setIsLoading(true)
                const response = await fetch(`/api/libre-book/geturl/${bookId}`, {
                    headers: {
                        'Authorization': `Bearer ${token}` // 替换为实际的访问令牌
                    }
                })
                if (!response.ok) {
                    throw new Error('Failed to fetch book')
                }
                const data = await response.json()
                setBookUrl(data.url)
                console.log(bookUrl)
            } catch (err: any) {
                setError(err.message)
            } finally {
                setIsLoading(false)
            }
        }

        if (bookId) {
            fetchBook()
        }
    }, [bookId])

    if (isLoading) return <div>Loading...</div>
    if (error) console.log("book fetch error")


    return (
        <div style={{ height: '100vh' }}>
            <ReactReader
                url="https://react-reader.metabits.no/files/alice.epub"
                location={location}
                locationChanged={(epubcfi: string) => setLocation(epubcfi)}
            />
        </div>
    )
}
