/** The engine's view of the table from your seat (see sim/src/session.rs `Session::view`). */

export type Seat = 0 | 1 | 2 | 3

export const HUMAN: Seat = 0
export const PARTNER: Seat = 2

export interface CardV {
  /** The physical card slot; stable for the round. */
  id: number
  s: number
  r: number
  /** Engraving: 0 none, 1 bonus, 2 herald, 3 multiplier. */
  eng: number
  /** Owning team, for owned cards. */
  own?: number
  /** A synthetic copy: the real card it replaces. */
  syn?: string
  /** Changed by an Opening effect this round: what it was. */
  was?: string
  price?: number
  sell?: number
  i?: number
}

export interface SigilV {
  k: number
  id: string
  grow: number
  fires: number
  /** Times fired this round. */
  rf: number
  sell: number
  revealed: boolean
}

export interface TeamV {
  score: number
  gold: number
  sigils: SigilV[]
  hidden: number
  cards: CardV[]
  cardCount: number
  cardSeat: number
}

export type PendingKind = 'shop' | 'swap' | 'bid' | 'play' | 'lead' | 'next'

export interface Pending {
  kind: PendingKind
  seat: Seat
  n: number
  ai: boolean
  /** Swap: the sigil whose Opening swap this is. */
  sigil?: string
}

export interface TrickPlay {
  seat: Seat
  card: CardV
}

export interface LastTrick {
  n: number
  plays: TrickPlay[]
  winner: Seat
  counts: boolean
}

export interface LedgerV {
  k: number
  id: string
  fires: number
  cp: number
  add: number
  x: number
  nilp: number
}

export interface TeamResult {
  bids: [number, number]
  contract: number
  tricks: number
  made: boolean
  exact: boolean
  set: boolean
  nilBids: number
  nilMade: number
  base: number
  cp: number
  cpLost: number
  add: number
  mult: number
  x: number
  multTotal: number
  nilScore: number
  score: number
  total: number
  engCp: number
  engAdd: number
  ledger: LedgerV[]
  income: { interest: number; base: number; contract: number; nil: number; total: number }
  gold: number
  formula: string
}

export interface ShopV {
  sigils: ({ i: number; id: string; price: number } | null)[]
  cards: (CardV | null)[]
  reroll: number
}

export interface View {
  step: number
  phase: 'shop' | 'opening' | 'bidding' | 'playing' | 'roundOver' | 'gameOver'
  round: number
  rounds: number
  auto: boolean
  tier: number
  teams: [TeamV, TeamV]
  pending: Pending | null
  notes: { sigil: string; text: string }[]
  result: { round: number; teams: [TeamResult, TeamResult] } | null
  lastTrick: LastTrick | null
  /** The last action completed `lastTrick`. */
  fresh: boolean
  dealer?: Seat
  hands?: { count: number; cards: CardV[] }[]
  bids?: number[]
  won?: number[]
  trick?: TrickPlay[]
  turn?: Seat
  broken?: boolean
  tricks?: number
  legal?: number[]
  contracts?: [number, number]
  contractTricks?: [number, number]
  live?: { cp: number; add: number; mult: number; x: number; nilp: number }
  shop?: ShopV
  winner?: 0 | 1 | 'draw'
}

/** One session log event (see `Session::log`); the client adds `time`. */
export interface LogEvent {
  seq?: number
  step?: number
  round?: number
  phase?: string
  type: string
  msg: string
  time?: string
  action?: Action
  team?: number
  slot?: number
  [key: string]: unknown
}

export type Action =
  | { t: 'advance' }
  | { t: 'buySigil'; i: number }
  | { t: 'buyCard'; i: number }
  | { t: 'sellSigil'; k: number }
  | { t: 'sellCard'; k: number }
  | { t: 'reroll' }
  | { t: 'shopDone' }
  | { t: 'autoShop' }
  | { t: 'swap'; seat?: number; cards: number[] }
  | { t: 'bid'; seat?: number; bid: number }
  | { t: 'play'; seat?: number; card: number }
  | { t: 'lead'; seat?: number; pass: boolean }
  | { t: 'next' }
  | { t: 'give'; team: number; sigil: string }
  | { t: 'take'; team: number; k: number }
  | { t: 'giveCard'; team: number; suit: number; rank: number; eng: number }
  | { t: 'gold'; team: number; amount: number }
  | { t: 'tier'; tier: number }

export interface GameConfig {
  seed: number
  tier: number
  auto: boolean
  offerable: string[]
}

export interface Response {
  ok: boolean
  error?: string | null
  view?: View
  events?: LogEvent[]
}
