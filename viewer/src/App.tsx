import { memo, useCallback, useEffect, useMemo, useRef, useState } from 'react'
import { Detail } from './Detail'
import { Icon } from './Icon'
import {
  type Sigil,
  CATEGORIES,
  RARITIES,
  SIGILS,
  SOURCES,
  STATUSES,
  categoryLabel,
  compareSigils,
  displayName,
  rarityColor,
  usingFixtures,
} from './model'
import { RulesText } from './RulesText'
import styles from './App.module.css'

type Facet = {
  key: string
  label: string
  values: string[]
  get: (s: Sigil) => string[]
  format?: (v: string) => string
}

const present = (canonical: string[], get: (s: Sigil) => string[]) => {
  const seen = new Set(SIGILS.flatMap(get))
  return [
    ...canonical.filter((v) => seen.has(v)),
    ...[...seen].filter((v) => !canonical.includes(v)).sort(),
  ]
}

const one = (v: string | undefined) => (v ? [v] : [])

const FACETS: Facet[] = (
  [
    { key: 'status', label: 'Status', values: STATUSES, get: (s) => one(s.status) },
    { key: 'rarity', label: 'Rarity', values: RARITIES, get: (s) => one(s.rarity) },
    {
      key: 'category',
      label: 'Category',
      values: CATEGORIES,
      get: (s) => one(s.category),
      format: categoryLabel,
    },
    { key: 'archetype', label: 'Archetype', values: [], get: (s) => s.archetypes ?? [] },
    { key: 'source', label: 'Source', values: SOURCES, get: (s) => one(s.source) },
  ] as Facet[]
).map((f) => ({ ...f, values: present(f.values, f.get) }))

const BY_ID: Record<string, Sigil> = Object.fromEntries(SIGILS.map((s) => [s.id, s]))
const SORTED = [...SIGILS].sort(compareSigils)

const initialFilters = (): Record<string, string> => ({
  status: SIGILS.some((s) => s.status === 'kept') ? 'kept' : 'all',
})

const readHash = () => decodeURIComponent(location.hash.slice(1))

function matches(s: Sigil, filters: Record<string, string>, query: string, skip?: string) {
  for (const f of FACETS) {
    const want = filters[f.key] ?? 'all'
    if (f.key !== skip && want !== 'all' && !f.get(s).includes(want)) return false
  }
  return !query || [s.id, s.name ?? '', s.text ?? ''].some((v) => v.toLowerCase().includes(query))
}

export default function App() {
  const [filters, setFilters] = useState(initialFilters)
  const [query, setQuery] = useState('')
  const [open, setOpen] = useState(readHash)
  const openedInApp = useRef(false)
  const q = query.trim().toLowerCase()

  useEffect(() => {
    const onHash = () => {
      if (!location.hash) openedInApp.current = false
      setOpen(readHash())
    }
    window.addEventListener('hashchange', onHash)
    return () => window.removeEventListener('hashchange', onHash)
  }, [])

  const visible = useMemo(() => SORTED.filter((s) => matches(s, filters, q)), [filters, q])

  /** For each facet, how many sigils each value would show with the other filters applied. */
  const counts = useMemo(
    () =>
      Object.fromEntries(
        FACETS.map((f) => {
          const pool = SORTED.filter((s) => matches(s, filters, q, f.key))
          const byValue: Record<string, number> = { all: pool.length }
          for (const s of pool) for (const v of f.get(s)) byValue[v] = (byValue[v] ?? 0) + 1
          return [f.key, byValue]
        }),
      ),
    [filters, q],
  )

  const openSigil = useCallback((id: string) => {
    openedInApp.current = true
    location.hash = id
  }, [])

  const close = useCallback(() => {
    if (openedInApp.current) {
      openedInApp.current = false
      history.back()
    } else {
      history.replaceState(null, '', location.pathname + location.search)
      setOpen('')
    }
  }, [])

  const step = useCallback(
    (dir: number) => {
      const list = visible.some((s) => s.id === open) ? visible : SORTED
      const i = list.findIndex((s) => s.id === open)
      const next = list[(i + dir + list.length) % list.length]
      if (next) location.replace(`#${next.id}`)
    },
    [visible, open],
  )

  const selected = BY_ID[open]

  useEffect(() => {
    document.documentElement.classList.toggle(styles.locked, Boolean(selected))
  }, [selected])

  return (
    <div className={styles.page}>
      <header className={styles.header}>
        <div className={styles.titleRow}>
          <h1 className={styles.title}>Sigils</h1>
          <span className={styles.count}>
            {visible.length} of {SIGILS.length}
          </span>
          {usingFixtures && (
            <span className={styles.fixtureNote}>Showing fixtures: data/sigils is empty</span>
          )}
        </div>
        <input
          className={styles.search}
          type="search"
          placeholder="Search ids, names, rules…"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
        />
        <div className={styles.facets}>
          {FACETS.map((f) => (
            <div key={f.key} className={styles.facet}>
              <span className={styles.facetLabel}>{f.label}</span>
              <div className={styles.chips}>
                {['all', ...f.values].map((v) => {
                  const active = (filters[f.key] ?? 'all') === v
                  return (
                    <button
                      key={v}
                      className={styles.chip}
                      data-active={active}
                      style={
                        f.key === 'rarity' && v !== 'all'
                          ? ({ '--chip': rarityColor(v) } as React.CSSProperties)
                          : undefined
                      }
                      onClick={() => setFilters((old) => ({ ...old, [f.key]: v }))}
                    >
                      {v === 'all' ? 'All' : (f.format?.(v) ?? v)}
                      <span className={styles.chipCount}>{counts[f.key][v] ?? 0}</span>
                    </button>
                  )
                })}
              </div>
            </div>
          ))}
        </div>
      </header>

      <main className={styles.grid}>
        {visible.map((s) => (
          <Tile key={s.id} sigil={s} onOpen={openSigil} />
        ))}
        {visible.length === 0 && <p className={styles.empty}>No sigils match.</p>}
      </main>

      {selected && (
        <Detail
          key={selected.id}
          sigil={selected}
          byId={BY_ID}
          onOpen={openSigil}
          onClose={close}
          onStep={step}
        />
      )}
    </div>
  )
}

const Tile = memo(function Tile({ sigil, onOpen }: { sigil: Sigil; onOpen: (id: string) => void }) {
  return (
    <button
      className={styles.card}
      data-status={sigil.status}
      style={{ '--fill': rarityColor(sigil.rarity) } as React.CSSProperties}
      onClick={() => onOpen(sigil.id)}
    >
      <div className={styles.cardHead}>
        <Icon name={sigil.icon} className={styles.badge} />
        <div className={styles.cardTitle}>
          <h2 className={styles.cardName} data-unnamed={!sigil.name}>
            {displayName(sigil)}
          </h2>
          <div className={styles.meta}>
            <span className={styles.rarity}>{sigil.rarity ?? 'no rarity'}</span>
            {sigil.category && <> · {categoryLabel(sigil.category)}</>}
            {sigil.status && sigil.status !== 'kept' && <> · {sigil.status}</>}
          </div>
        </div>
      </div>
      <RulesText text={sigil.text} className={styles.cardText} />
    </button>
  )
})
