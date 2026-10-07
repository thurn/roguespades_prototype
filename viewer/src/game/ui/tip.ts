/** Tooltip content: sigils (name and rules text) and plain lines; brackets render as cards. */
export interface TipContent {
  sigils?: { id: string; grow?: number; extra?: string }[]
  lines?: string[]
}

type Tip = (TipContent & { x: number; top: number; bottom: number }) | null

let tip: Tip = null
const listeners = new Set<() => void>()
const set = (t: Tip) => {
  tip = t
  listeners.forEach((l) => l())
}

const empty = (c: TipContent) => !c.sigils?.length && !c.lines?.length

export function showTip(c: TipContent, el: Element) {
  if (empty(c)) return
  const r = el.getBoundingClientRect()
  set({ ...c, x: r.left + r.width / 2, top: r.top, bottom: r.bottom })
}

export const hideTip = () => tip && set(null)

let pressTimer: ReturnType<typeof setTimeout> | undefined

/** Hover on desktop, long-press on touch. */
export function tipHandlers(c: TipContent) {
  if (empty(c)) return {}
  return {
    onPointerEnter: (e: React.PointerEvent) => {
      if (e.pointerType === 'mouse') showTip(c, e.currentTarget)
    },
    onPointerLeave: () => {
      clearTimeout(pressTimer)
      hideTip()
    },
    onPointerDown: (e: React.PointerEvent) => {
      if (e.pointerType === 'mouse') return
      const el = e.currentTarget
      clearTimeout(pressTimer)
      pressTimer = setTimeout(() => showTip(c, el), 450)
    },
    onPointerUp: () => clearTimeout(pressTimer),
  }
}

export const getTip = () => tip

export function subscribeTip(l: () => void) {
  listeners.add(l)
  return () => {
    listeners.delete(l)
  }
}
