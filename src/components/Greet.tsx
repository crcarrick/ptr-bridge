'use client'

import { useEffect } from 'react'

import { invoke } from '@tauri-apps/api'

export default function Greet() {
  useEffect(() => {
    invoke('greet', { name: 'Chris' }).then((message) => {
      console.log(message)
    })
  }, [])

  return null
}
