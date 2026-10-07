/// <reference lib="webworker" />
// Hosts the simulator's wasm session (sim/web) off the main thread, so AI search never blocks
// the table. Requests and responses are JSON strings; see sim/web/src/lib.rs.
import wasmUrl from '../../../sim/target/wasm32-unknown-unknown/release/rsweb.wasm?url'

interface Exports {
  memory: WebAssembly.Memory
  alloc(len: number): number
  dealloc(ptr: number, len: number): void
  call(ptr: number, len: number): number
  out_ptr(): number
  panic_ptr(): number
  panic_len(): number
}

let wasm: Exports | null = null
let broken: string | null = null
const encoder = new TextEncoder()
const decoder = new TextDecoder()

async function load(): Promise<Exports> {
  if (wasm) return wasm
  const bytes = await fetch(wasmUrl, { cache: 'no-store' }).then((r) => r.arrayBuffer())
  const { instance } = await WebAssembly.instantiate(bytes, {})
  wasm = instance.exports as unknown as Exports
  return wasm
}

function call(w: Exports, req: string): string {
  const input = encoder.encode(req)
  const ptr = w.alloc(input.length)
  new Uint8Array(w.memory.buffer, ptr, input.length).set(input)
  try {
    const len = w.call(ptr, input.length)
    return decoder.decode(new Uint8Array(w.memory.buffer, w.out_ptr(), len))
  } catch (e) {
    const msg = decoder.decode(new Uint8Array(w.memory.buffer, w.panic_ptr(), w.panic_len()))
    // A trap leaves the instance unusable; every later call reports the panic.
    broken = `engine panic: ${msg || String(e)}`
    throw new Error(broken, { cause: e })
  } finally {
    if (!broken) w.dealloc(ptr, input.length)
  }
}

self.onmessage = async (e: MessageEvent<{ id: number; req: string }>) => {
  const { id, req } = e.data
  try {
    if (broken) throw new Error(broken)
    const w = await load()
    self.postMessage({ id, res: call(w, req) })
  } catch (err) {
    self.postMessage({ id, res: JSON.stringify({ ok: false, error: String(err) }) })
  }
}
