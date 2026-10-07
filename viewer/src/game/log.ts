import type { LogEvent } from './types'

/**
 * The session log: every engine event plus client-side events, appended as JSONL to
 * logs/<file> by the dev server (see vite.config.ts). Without a dev server the log stays in
 * memory and can be downloaded from the sandbox drawer.
 */
export class SessionLog {
  readonly file: string
  readonly events: LogEvent[] = []
  private queue: LogEvent[] = []
  private timer: ReturnType<typeof setTimeout> | null = null
  private offline = false
  private listeners = new Set<() => void>()

  constructor(seed: number, auto: boolean) {
    const stamp = new Date().toISOString().replace(/[:]/g, '-').replace(/\..*/, '')
    this.file = `${stamp}-seed${seed}${auto ? '-auto' : ''}.jsonl`
  }

  add(events: LogEvent[]) {
    if (events.length === 0) return
    const time = new Date().toISOString()
    for (const e of events) {
      const stamped = { ...e, time }
      this.events.push(stamped)
      this.queue.push(stamped)
    }
    this.listeners.forEach((l) => l())
    this.timer ??= setTimeout(() => this.flush(), 300)
  }

  /** A client-side event: UI choices, errors, and anything the engine can't see. */
  client(type: string, msg: string, data: Record<string, unknown> = {}) {
    this.add([{ type, msg, ...data }])
  }

  async flush() {
    this.timer = null
    if (this.offline || this.queue.length === 0) return
    const lines = this.queue
    this.queue = []
    try {
      const r = await fetch('/api/log', {
        method: 'POST',
        headers: { 'content-type': 'application/json' },
        body: JSON.stringify({ file: this.file, lines }),
      })
      if (!r.ok) throw new Error(String(r.status))
    } catch {
      this.offline = true
      console.warn('Session log server unavailable; the log stays in memory.')
    }
  }

  download() {
    const text = this.events.map((e) => JSON.stringify(e)).join('\n') + '\n'
    const a = document.createElement('a')
    a.href = URL.createObjectURL(new Blob([text], { type: 'application/x-ndjson' }))
    a.download = this.file
    a.click()
    URL.revokeObjectURL(a.href)
  }

  subscribe(l: () => void) {
    this.listeners.add(l)
    return () => {
      this.listeners.delete(l)
    }
  }
}
