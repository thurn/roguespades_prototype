import type { CardV } from './types'

export const CLUBS = 0
export const DIAMONDS = 1
export const HEARTS = 2
export const SPADES = 3

export const SUIT_SYMBOLS = ['♣', '♦', '♥', '♠']
export const SUIT_NAMES = ['clubs', 'diamonds', 'hearts', 'spades']
export const isRed = (s: number) => s === DIAMONDS || s === HEARTS

export const rankLabel = (r: number) => ({ 11: 'J', 12: 'Q', 13: 'K', 14: 'A' })[r] ?? String(r)
export const cardLabel = (c: { s: number; r: number }) => `${rankLabel(c.r)}${SUIT_SYMBOLS[c.s]}`

/** Spades first, then hearts, clubs, diamonds (alternating colors), each high to low. */
const SUIT_ORDER = [2, 1, 3, 0]
export function sortHand(cards: CardV[]): CardV[] {
  return [...cards].sort((a, b) => SUIT_ORDER[a.s] - SUIT_ORDER[b.s] || b.r - a.r || a.id - b.id)
}

export const SEAT_NAMES = ['You', 'Nova', 'Sage', 'Rook']

/** Unit vector from the table center toward each seat, used for card motion. */
export const SEAT_VECTORS = [
  { x: 0, y: 1 },
  { x: -1, y: 0 },
  { x: 0, y: -1 },
  { x: 1, y: 0 },
]

export const ENGRAVINGS: Record<number, { name: string; text: string; icon: string }> = {
  1: { name: 'Bonus', text: '+20 contract points when this card wins a trick', icon: 'star' },
  2: {
    name: 'Herald',
    text: '+20 contract points when your team leads this card',
    icon: 'megaphone',
  },
  3: {
    name: 'Multiplier',
    text: '+5 contract multiplier when this card wins a trick',
    icon: 'bolt',
  },
}

export const SYNTHETIC_ICON = 'copy'

/** Tooltip lines for a card's engraving, synthetic copy, or Opening change. */
export function cardNotes(c: CardV): string[] {
  const out: string[] = []
  if (c.eng && ENGRAVINGS[c.eng]) out.push(`${ENGRAVINGS[c.eng].name}: ${ENGRAVINGS[c.eng].text}`)
  if (c.syn) out.push(`Synthetic: replaces [${c.syn}]`)
  if (c.was) out.push(`Opening: this card was [${c.was}] this round`)
  if (c.own === 0) out.push('Owned: dealt to you every round')
  return out
}

export const signed = (n: number) => (n > 0 ? `+${fmt(n)}` : n < 0 ? `−${fmt(-n)}` : '0')
export const fmt = (n: number) =>
  Number.isInteger(n) ? Math.round(n).toLocaleString('en-US') : n.toFixed(2).replace(/0$/, '')
export const num = (n: number) => (n < 0 ? `−${fmt(-n)}` : fmt(n))
