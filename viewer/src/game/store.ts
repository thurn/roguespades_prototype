import { useSyncExternalStore } from 'react'
import { Engine, MODEL_STEP, OFFERABLE } from './engine'
import { SessionLog } from './log'
import type { Action, GameConfig, LastTrick, LogEvent, Response, View } from './types'

// ----- URL parameters -----

const q = new URLSearchParams(location.search)
const flag = (k: string) => q.has(k) && q.get(k) !== '0' && q.get(k) !== 'false'

export const PARAMS = {
  seed: q.has('seed') ? Number(q.get('seed')) : null,
  tier: q.has('tier') ? Number(q.get('tier')) : 1,
  /** The AI plays your seat and shops for your team too. */
  auto: flag('auto'),
  /** No animation pauses. */
  fast: flag('fast'),
  sandbox: flag('sandbox'),
  reveal: flag('reveal'),
  /** Sigil ids granted to your team (`give`) or the opponents (`giveThem`) at the start. */
  give: (q.get('give') ?? '').split(',').filter(Boolean),
  giveThem: (q.get('giveThem') ?? '').split(',').filter(Boolean),
  /** Cards your team owns at the start: AS, 10H, QC* (synthetic), KDb/h/m (engraved). */
  cards: (q.get('cards') ?? '').split(',').filter(Boolean),
  gold: q.has('gold') ? Number(q.get('gold')) : 0,
  /** A log in logs/ to replay. */
  replay: q.get('replay'),
}

const T = PARAMS.fast
  ? { play: 0, lead: 0, bid: 0, swap: 0, shop: 0, next: 50, trick: 60 }
  : PARAMS.auto
    ? { play: 150, lead: 150, bid: 200, swap: 150, shop: 400, next: 2500, trick: 500 }
    : { play: 650, lead: 400, bid: 550, swap: 300, shop: 300, next: 2500, trick: 1100 }

// ----- State -----

export interface ReplayStep {
  view: View
  events: LogEvent[]
}

export interface Replay {
  name: string
  steps: ReplayStep[]
  index: number
  total: number
  diverged: string | null
  loading: boolean
}

export interface GameState {
  view: View | null
  generation: number
  busy: boolean
  /** A finished trick held on the table before it is collected. */
  held: LastTrick | null
  /** Trigger flashes: key → event seq. */
  pulses: Record<string, number>
  toast: { id: number; text: string } | null
  fatal: string | null
  reveal: boolean
  paused: boolean
  sandbox: boolean
  replay: Replay | null
  log: SessionLog | null
  config: GameConfig | null
}

let state: GameState = {
  view: null,
  generation: 0,
  busy: false,
  held: null,
  pulses: {},
  toast: null,
  fatal: null,
  reveal: PARAMS.reveal,
  paused: false,
  sandbox: PARAMS.sandbox,
  replay: null,
  log: null,
  config: null,
}

const listeners = new Set<() => void>()
const set = (patch: Partial<GameState>) => {
  state = { ...state, ...patch }
  listeners.forEach((l) => l())
}

export const getState = () => state
export function subscribe(l: () => void) {
  listeners.add(l)
  return () => {
    listeners.delete(l)
  }
}
export const useGame = () => useSyncExternalStore(subscribe, getState)

let engine: Engine | null = null
let holdTimer: ReturnType<typeof setTimeout> | undefined
let driveTimer: ReturnType<typeof setTimeout> | undefined
let toastId = 0

// ----- Responses -----

function pulsesFrom(events: LogEvent[], view: View | undefined): Record<string, number> {
  const out = { ...state.pulses }
  for (const e of events) {
    if (e.type === 'fire' && e.team !== undefined) out[`s:${e.team}:${e.slot}`] = e.seq ?? 0
    if (e.type === 'engraving' && view?.lastTrick) {
      const w = view.lastTrick.plays.find((p) => p.seat === view.lastTrick!.winner)
      if (w) out[`c:${w.card.id}`] = e.seq ?? 0
      for (const p of view.lastTrick.plays) if (p.card.eng === 2) out[`c:${p.card.id}`] = e.seq ?? 0
    }
  }
  return out
}

function toast(text: string) {
  set({ toast: { id: ++toastId, text } })
}

function handle(res: Response, log: SessionLog | null) {
  log?.add(res.events ?? [])
  if (!res.ok && res.error) {
    toast(res.error)
    if (res.error.startsWith('Error: engine panic')) set({ fatal: res.error })
  }
  const view = res.view ?? state.view
  const patch: Partial<GameState> = { view, pulses: pulsesFrom(res.events ?? [], res.view) }
  if (res.view?.fresh && res.view.lastTrick) {
    patch.held = res.view.lastTrick
    clearTimeout(holdTimer)
    holdTimer = setTimeout(() => {
      set({ held: null })
      drive()
    }, T.trick)
  }
  set(patch)
}

// ----- Live game -----

export async function startGame(opts: { seed?: number; tier?: number; auto?: boolean } = {}) {
  clearTimeout(driveTimer)
  clearTimeout(holdTimer)
  engine ??= new Engine()
  const first = state.generation === 0
  const config: GameConfig = {
    seed: opts.seed ?? (first && PARAMS.seed !== null ? PARAMS.seed : randomSeed()),
    tier: opts.tier ?? state.config?.tier ?? PARAMS.tier,
    auto: opts.auto ?? state.config?.auto ?? PARAMS.auto,
    offerable: OFFERABLE,
  }
  const log = new SessionLog(config.seed, config.auto)
  log.client(
    'client',
    `Client ${__GIT_VERSION__} starts seed ${config.seed} (model ${MODEL_STEP}) at ${location.href}`,
    {
      git: __GIT_VERSION__,
      model: MODEL_STEP,
      url: location.href,
      userAgent: navigator.userAgent,
      viewport: `${innerWidth}x${innerHeight}`,
      config,
    },
  )
  set({
    busy: true,
    held: null,
    pulses: {},
    replay: null,
    log,
    config,
    fatal: null,
    generation: state.generation + 1,
  })
  const res = await engine.newGame(config, state.reveal)
  handle(res, log)
  if (first) {
    const grants: Action[] = [
      ...PARAMS.give.map((sigil): Action => ({ t: 'give', team: 0, sigil })),
      ...PARAMS.giveThem.map((sigil): Action => ({ t: 'give', team: 1, sigil })),
      ...PARAMS.cards.flatMap((c): Action[] => {
        const card = parseCard(c)
        return card ? [{ t: 'giveCard', team: 0, ...card }] : []
      }),
      ...(PARAMS.gold ? [{ t: 'gold', team: 0, amount: PARAMS.gold } as Action] : []),
    ]
    for (const a of grants) handle(await engine.act(a, state.reveal), log)
  }
  set({ busy: false })
  drive()
}

const randomSeed = () => Math.floor(Math.random() * 1_000_000)

function parseCard(code: string) {
  const m = /^(10|[2-9JQKA])([CDHS])([*bhm]?)$/i.exec(code.trim())
  if (!m) return null
  const rank = { J: 11, Q: 12, K: 13, A: 14 }[m[1].toUpperCase()] ?? Number(m[1])
  const suit = 'CDHS'.indexOf(m[2].toUpperCase())
  const eng = { '': 0, b: 1, h: 2, m: 3, '*': 4 }[m[3].toLowerCase()] ?? 0
  return { suit, rank, eng }
}

/** Sends a decision or sandbox action for your seat. */
export async function dispatch(action: Action) {
  if (!engine || state.busy || state.replay || !state.view) return
  clearTimeout(driveTimer)
  set({ busy: true })
  const res = await engine.act(action, state.reveal)
  handle(res, state.log)
  set({ busy: false })
  drive()
}

/** Lets the AI make its pending decision after a short, readable pause. */
function drive() {
  clearTimeout(driveTimer)
  const s = state
  const p = s.view?.pending
  if (!p || !p.ai || s.busy || s.held || s.paused || s.replay || s.fatal) return
  const step = s.view!.step
  driveTimer = setTimeout(() => {
    if (state.view?.step === step && !state.busy && !state.held && !state.paused) {
      dispatch({ t: 'advance' })
    }
  }, T[p.kind])
}

export function setPaused(paused: boolean) {
  state.log?.client('ui', paused ? 'Paused the AI' : 'Resumed the AI')
  set({ paused })
  drive()
}

/** One AI decision while paused. */
export function stepOnce() {
  if (state.view?.pending?.ai) dispatch({ t: 'advance' })
}

export async function setReveal(reveal: boolean) {
  set({ reveal })
  state.log?.client('ui', reveal ? 'Sandbox: show all hands' : 'Sandbox: hide hidden hands')
  if (state.replay) return
  if (engine && state.view) {
    const res = await engine.view(reveal)
    if (res.view) set({ view: res.view })
  }
}

export function setSandbox(open: boolean) {
  set({ sandbox: open })
}

// ----- Replay -----

export async function listLogs(): Promise<{ name: string; size: number; mtime: number }[]> {
  try {
    const r = await fetch('/api/logs')
    return r.ok ? await r.json() : []
  } catch {
    return []
  }
}

/** Replays a session log by feeding every recorded decision back to a fresh engine. */
export async function loadReplay(name: string, text?: string) {
  clearTimeout(driveTimer)
  clearTimeout(holdTimer)
  const body: string =
    text ?? (await fetch(`/api/logs/${encodeURIComponent(name)}`).then((r) => r.text()))
  const events = body
    .split('\n')
    .filter((l) => l.trim())
    .map((l) => JSON.parse(l) as LogEvent)
  const header = events.find((e) => e.type === 'client')
  const start = events.find((e) => e.type === 'start')
  const cfg = (header?.config ?? {}) as Partial<GameConfig>
  const config: GameConfig = {
    seed: Number(start?.seed ?? cfg.seed ?? 1),
    tier: Number(start?.tier ?? cfg.tier ?? 1),
    auto: Boolean(start?.auto ?? cfg.auto ?? false),
    offerable: cfg.offerable ?? OFFERABLE,
  }
  const actions = events.filter((e) => e.action && e.type !== 'rejected').map((e) => e.action!)
  const replay: Replay = {
    name,
    steps: [],
    index: 0,
    total: actions.length,
    diverged: null,
    loading: true,
  }
  set({ replay, held: null, generation: state.generation + 1 })
  const re = new Engine()
  try {
    const first = await re.newGame(config, true)
    if (!first.view) throw new Error(first.error ?? 'could not start the replay')
    replay.steps.push({ view: first.view, events: first.events ?? [] })
    for (const a of actions) {
      const res = await re.act(a, true, true)
      if (!res.ok || !res.view) {
        replay.diverged = `Diverged at ${JSON.stringify(a)}: ${res.error}`
        break
      }
      replay.steps.push({ view: res.view, events: res.events ?? [] })
      if (replay.steps.length % 25 === 0) set({ replay: { ...replay } })
    }
  } catch (e) {
    replay.diverged = String(e)
  } finally {
    re.dispose()
  }
  set({ replay: { ...replay, loading: false } })
}

export function replayTo(index: number) {
  const r = state.replay
  if (!r) return
  set({ replay: { ...r, index: Math.max(0, Math.min(r.steps.length - 1, index)) } })
}

export function exitReplay() {
  set({ replay: null, generation: state.generation + 1 })
  history.replaceState(null, '', location.pathname)
  if (!state.view) startGame()
  else drive()
}

// ----- Debugging -----

declare global {
  interface Window {
    game: unknown
  }
}

window.game = {
  get state() {
    return state
  },
  dispatch,
  startGame,
  loadReplay,
}

window.addEventListener('error', (e) =>
  state.log?.client('jsError', `JS error: ${e.message}`, { stack: String(e.error?.stack ?? '') }),
)
window.addEventListener('unhandledrejection', (e) =>
  state.log?.client('jsError', `Unhandled rejection: ${String(e.reason)}`),
)
window.addEventListener('pagehide', () => state.log?.flush())
