import { SIGILS } from '../model'
import model from '../../../data/models/round-3.json'
import type { Action, GameConfig, Response } from './types'

/** The model the shop and AI read: the last fitted value model of the sigil design run. */
export const MODEL_STEP = 'round-3'

/** The parts of each sigil file the engine reads (it ignores estimates and history). */
const DEFS = SIGILS.filter((s) => s.effect && s.rarity).map((s) => ({
  id: s.id,
  rarity: s.rarity,
  price: s.price,
  role: s.role,
  archetypes: s.archetypes,
  source: s.source,
  status: s.status,
  effect: s.effect,
  name: s.name ?? null,
  text: s.text ?? null,
}))

/** Sigils the shop offers: the kept pool. */
export const OFFERABLE = SIGILS.filter((s) => s.status === 'kept').map((s) => s.id)

/** One wasm session in its own Web Worker. Requests run one at a time, in order. */
export class Engine {
  private worker = new Worker(new URL('./engine.worker.ts', import.meta.url), { type: 'module' })
  private next = 0
  private waiting = new Map<number, (res: Response) => void>()
  private ready: Promise<Response>

  constructor() {
    this.worker.onmessage = (e: MessageEvent<{ id: number; res: string }>) => {
      const done = this.waiting.get(e.data.id)
      this.waiting.delete(e.data.id)
      done?.(JSON.parse(e.data.res) as Response)
    }
    this.ready = this.request({ op: 'init', defs: DEFS, model })
  }

  private request(req: object): Promise<Response> {
    const id = this.next++
    return new Promise((resolve) => {
      this.waiting.set(id, resolve)
      this.worker.postMessage({ id, req: JSON.stringify(req) })
    })
  }

  private async send(req: object): Promise<Response> {
    const init = await this.ready
    if (!init.ok) return init
    return this.request(req)
  }

  newGame(config: GameConfig, all: boolean) {
    return this.send({ op: 'new', config, all })
  }

  act(action: Action, all: boolean, force = false) {
    return this.send({ op: 'act', action, force, all })
  }

  view(all: boolean) {
    return this.send({ op: 'view', all })
  }

  dispose() {
    this.worker.terminate()
  }
}
