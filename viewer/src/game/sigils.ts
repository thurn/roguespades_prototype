import { type Sigil, SIGILS } from '../model'

const BY_ID: Record<string, Sigil> = Object.fromEntries(SIGILS.map((s) => [s.id, s]))

export const getSigil = (id: string): Sigil | undefined => BY_ID[id]
export const sigilName = (id: string) => BY_ID[id]?.name || id

/** Every sigil, kept first, for sandbox pickers. */
export const ALL_SIGILS = [...SIGILS].sort(
  (a, b) =>
    Number(b.status === 'kept') - Number(a.status === 'kept') ||
    (a.name ?? a.id).localeCompare(b.name ?? b.id),
)

/** Rules text with a growth sigil's stored value filled in. */
export function sigilText(id: string, grow?: number): string {
  const text = BY_ID[id]?.text ?? id
  if (grow === undefined) return text
  return text.replace(/\(currently ([+×])[\d.]+\)/, (_, sign) => `(currently ${sign}${grow})`)
}

export const categoryColor = (c: string | undefined) => `var(--cat-${c ?? 'points'})`
export const rarityColor = (r: string | undefined) => `var(--rar-${r ?? 'common'})`
