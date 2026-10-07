/** A sigil data file (see docs/sigils/data-format.md). Everything past `id` may be missing. */
export type Sigil = {
  id: string
  rarity?: string
  price?: number
  role?: string
  archetypes?: string[]
  source?: string
  createdIn?: string
  effect?: unknown
  text?: string
  category?: string
  signature?: string
  touchesOpponents?: boolean
  design?: {
    decision?: string
    opponent?: string
    rationale?: string
    partners?: string[]
  }
  status?: string
  replacedBy?: string | null
  history?: { step: string; note: string }[]
  name?: string | null
  icon?: string | null
  iconFamily?: string | null
  iconWord?: string | null
  simplicity?: {
    items?: { burden: string; detail: string; cost: number }[]
    C?: number
    S?: number
  }
  estimates?: Estimates
  reports?: string[]
}

export type Interval = { value: number; lo: number; hi: number }

export type Metric = Interval & {
  key: string
  label: string
  bandLo: number | null
  bandHi: number | null
  n?: number
}

export type Estimates = {
  step?: string
  exposures?: number
  metrics?: Metric[]
  amountHistory?: { step: string; amount: number }[]
  amountSlope?: Interval
  skillGradient?: Interval
  pairs?: { with: string; synergy: number }[]
}

const real = import.meta.glob<Sigil>('../../data/sigils/*.json', { eager: true, import: 'default' })
const fixtures = import.meta.glob<Sigil>('../fixtures/*.json', { eager: true, import: 'default' })

/** True when `data/sigils` is empty and the bundled development fixtures are shown instead. */
export const usingFixtures = Object.keys(real).length === 0

export const SIGILS: Sigil[] = Object.values(usingFixtures ? fixtures : real)

export const RARITIES = ['common', 'uncommon', 'rare', 'legendary']
export const CATEGORIES = ['points', 'mult', 'xmult', 'enabler', 'hybrid']
export const STATUSES = ['candidate', 'kept', 'cut', 'replaced']
export const SOURCES = ['enumerated', 'designed', 'gdd-seed', 'control']

const CATEGORY_LABELS: Record<string, string> = { mult: '+mult', xmult: '×mult' }

export const categoryLabel = (c: string) => CATEGORY_LABELS[c] ?? c

export const rarityColor = (r: string | undefined) =>
  RARITIES.includes(r ?? '') ? `var(--rarity-${r})` : 'var(--ink-faint)'

const rank = (list: string[], v: string | undefined) => {
  const i = list.indexOf(v ?? '')
  return i < 0 ? list.length : i
}

export function compareSigils(a: Sigil, b: Sigil): number {
  return rank(RARITIES, a.rarity) - rank(RARITIES, b.rarity) || a.id.localeCompare(b.id)
}

export const displayName = (s: Sigil) => s.name || s.id

/** Formats a number compactly, keeping a sign when asked. */
export function fmt(n: number, signed = false): string {
  const s = Math.abs(n) >= 100 || Number.isInteger(n) ? String(Math.round(n)) : n.toFixed(1)
  return signed && n > 0 ? `+${s}` : s
}
