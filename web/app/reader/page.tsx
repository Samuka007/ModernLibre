"use client"
import { useSearchParams } from 'next/navigation'
import React, { useState } from 'react'
import { ReactReader } from 'react-reader'

export default function ReaderPage() {
    const [location, setLocation] = useState<string>('')
    const params = useSearchParams()
    const url = params.get('url')
    const extension = params.get('extension')

    //console.log(extension)
    return (
        <div style={{ height: '100vh' }}>
            {extension === 'pdf' ? (
                //test 
                // window.location.href = "https://www.w3.org/WAI/ER/tests/xhtml/testfiles/resources/pdf/dummy.pdf"
                window.location.href = url as string
            ) : (
                <ReactReader
                    url={url as string}
                    location={location}
                    locationChanged={(epubcfi: string) => setLocation(epubcfi)}
                />
            )}
        </div>
    )

}
