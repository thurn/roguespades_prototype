import { useEffect, useState } from 'react'
import styles from './App.module.css'

// Every filled Boxicons glyph, loaded on demand so only the icons in use are fetched.
const GLYPHS = import.meta.glob<string>('./bx-*.svg', {
  base: '../node_modules/@boxicons/core/svg/filled/',
  query: '?raw',
  import: 'default',
})

const PLACEHOLDER =
  '<circle cx="12" cy="12" r="8" fill="none" stroke="#000" stroke-width="2" stroke-dasharray="3.2 2.4"/>'

const cache = new Map<string, string | null>()

function globKey(name: string): string {
  const bare = name.replace(/^bxs?-/, '')
  return `./bx-${bare}.svg`
}

/** Inner markup of a Boxicons file, which has a fixed 24px size and no viewBox. */
const inner = (svg: string) => svg.replace(/^[\s\S]*?<svg[^>]*>/, '').replace(/<\/svg>\s*$/, '')

function useGlyph(name: string | null | undefined): string | null {
  const [, setLoaded] = useState(0)
  const key = name ? globKey(name) : null
  const cached = key ? cache.get(key) : undefined
  useEffect(() => {
    if (!key || cached !== undefined) return
    const load = GLYPHS[key]
    if (!load) {
      cache.set(key, null)
      return
    }
    let live = true
    load().then((svg) => {
      cache.set(key, inner(svg))
      if (live) setLoaded((n) => n + 1)
    })
    return () => {
      live = false
    }
  }, [key, cached])
  return cached ?? null
}

/** A Boxicons glyph painted with the --fill custom property; unnamed sigils get a dashed ring. */
export function Icon({ name, className }: { name: string | null | undefined; className?: string }) {
  const glyph = useGlyph(name)
  const body = name ? glyph : PLACEHOLDER
  const url = body
    ? `url("data:image/svg+xml,${encodeURIComponent(
        `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">${body}</svg>`,
      )}")`
    : undefined
  return (
    <span
      className={`${styles.icon} ${className ?? ''}`}
      data-empty={!url}
      style={url ? { maskImage: url, WebkitMaskImage: url } : undefined}
      aria-hidden
    />
  )
}
